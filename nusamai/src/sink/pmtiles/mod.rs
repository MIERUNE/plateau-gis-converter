//! PMTiles sink

use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
};

use nusamai_citygml::schema::Schema;
use nusamai_projection::crs::EPSG_WEB_MERCATOR;
use pmtiles::{PmTilesWriter, TileCoord, TileType};

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{
        mvt::encode::MvtTileEncoder,
        vector_tile::{
            feature_sorting_stage, generate_stage, report_final_tile_count, report_tile_progress,
            slice_stage, tile_id::TileIdMethod, validate_vector_tile_schema_crs, EncodedTile,
            TilePipelineOptions, DEFAULT_MAX_COMPRESSED_TILE_SIZE, FEATURE_CHANNEL_CAPACITY,
        },
        DataRequirements, DataSink, DataSinkProvider, SinkInfo, SinkInputCrsRequirement,
    },
    transformer,
    transformer::{use_lod_config, TransformerSettings},
};

use super::{option::output_parameter, vector_tile::slice::validate_zoom_range};

const TILE_CHANNEL_CAPACITY: usize = 100;
const PMTILES_HEADER_SIZE: usize = 127;
const PMTILES_VERSION_OFFSET: usize = 7;
const PMTILES_LEAF_OFFSET: usize = 40;
const PMTILES_LEAF_LENGTH_OFFSET: usize = 48;
const PMTILES_TILE_DATA_OFFSET: usize = 56;
const PMTILES_TILE_DATA_LENGTH_OFFSET: usize = 64;
const PMTILES_U64_FIELD_SIZE: usize = size_of::<u64>();

pub struct PmTilesSinkProvider {}

impl DataSinkProvider for PmTilesSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "pmtiles".to_string(),
            name: "PMTiles".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());
        params.define(ParameterDefinition {
            key: "min_z".into(),
            entry: ParameterEntry {
                description: "Minimum zoom level".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: Some(7),
                    min: Some(0),
                    max: Some(20),
                }),
                label: Some("最小ズームレベル".into()),
            },
        });
        params.define(ParameterDefinition {
            key: "max_z".into(),
            entry: ParameterEntry {
                description: "Maximum zoom level".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: Some(15),
                    min: Some(0),
                    max: Some(20),
                }),
                label: Some("最大ズームレベル".into()),
            },
        });
        params
    }

    fn transformer_options(&self) -> TransformerSettings {
        let mut settings = TransformerSettings::new();
        settings.insert(use_lod_config("min_lod", None));
        settings
    }

    fn sink_input_crs_requirement(&self) -> SinkInputCrsRequirement {
        SinkInputCrsRequirement::Fixed(EPSG_WEB_MERCATOR)
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath)
            .as_ref()
            .expect("Output path is required but not provided");
        let min_z = get_parameter_value!(params, "min_z", Integer)
            .expect("min_z parameter is required but not provided") as u8;
        let max_z = get_parameter_value!(params, "max_z", Integer)
            .expect("max_z parameter is required but not provided") as u8;
        validate_zoom_range(min_z, max_z);

        Box::new(PmTilesSink {
            output_path: output_path.into(),
            transform_settings: self.transformer_options(),
            options: PmTilesParams { min_z, max_z },
        })
    }
}

struct PmTilesSink {
    output_path: PathBuf,
    transform_settings: TransformerSettings,
    options: PmTilesParams,
}

struct PmTilesParams {
    min_z: u8,
    max_z: u8,
}

impl DataSink for PmTilesSink {
    fn make_requirements(&mut self, properties: TransformerSettings) -> DataRequirements {
        let default_requirements = DataRequirements {
            key_value: transformer::KeyValueSpec::DotNotation,
            lod_filter: transformer::LodFilterSpec {
                mode: transformer::LodFilterMode::Lowest,
                ..Default::default()
            },
            geom_stats: transformer::GeometryStatsSpec::MinMaxHeights,
            ..Default::default()
        };

        for config in &properties.configs {
            self.transform_settings.update_transformer(config.clone());
        }
        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        validate_vector_tile_schema_crs(schema)?;

        let (sender_sliced, receiver_sliced) = mpsc::sync_channel(FEATURE_CHANNEL_CAPACITY);
        let (sender_sorted, receiver_sorted) = mpsc::sync_channel(FEATURE_CHANNEL_CAPACITY);
        let (sender_tiles, receiver_tiles) = mpsc::sync_channel(TILE_CHANNEL_CAPACITY);

        let tile_id_method = TileIdMethod::Hilbert;
        let pipeline_options = TilePipelineOptions {
            min_z: self.options.min_z,
            max_z: self.options.max_z,
            max_compressed_tile_size: DEFAULT_MAX_COMPRESSED_TILE_SIZE,
        };
        let encoder = MvtTileEncoder::for_pmtiles_legacy();

        std::thread::scope(|scope| {
            scope.spawn(move || {
                if let Err(error) = slice_stage(
                    feedback,
                    upstream,
                    tile_id_method,
                    sender_sliced,
                    pipeline_options,
                ) {
                    feedback.fatal_error(error);
                }
            });

            scope.spawn(move || {
                if let Err(error) = feature_sorting_stage(feedback, receiver_sliced, sender_sorted)
                {
                    feedback.fatal_error(error);
                }
            });

            scope.spawn(move || {
                let generated_tile_count = AtomicU64::new(0);
                let pool = rayon::ThreadPoolBuilder::new().build().map_err(|error| {
                    PipelineError::Other(format!("Failed to build thread pool: {error}"))
                });
                match pool {
                    Ok(pool) => pool.install(|| {
                        let result = generate_stage(
                            feedback,
                            receiver_sorted,
                            tile_id_method,
                            pipeline_options.max_compressed_tile_size,
                            &encoder,
                            |tile| {
                                send_generated_tile(&sender_tiles, tile)?;
                                let tile_count =
                                    generated_tile_count.fetch_add(1, Ordering::Relaxed) + 1;
                                report_tile_progress(
                                    feedback,
                                    tile_count,
                                    "Generated",
                                    "tiles for PMTiles archive",
                                );
                                Ok(())
                            },
                        );
                        match result {
                            Ok(()) => report_final_tile_count(
                                feedback,
                                generated_tile_count.load(Ordering::Relaxed),
                                "generating",
                                "tiles for PMTiles archive",
                            ),
                            Err(error) => feedback.fatal_error(error),
                        }
                    }),
                    Err(error) => feedback.fatal_error(error),
                }
            });

            scope.spawn(move || {
                if let Err(error) = pmtiles_writing_stage(
                    &self.output_path,
                    feedback,
                    receiver_tiles,
                    tile_id_method,
                ) {
                    feedback.fatal_error(error);
                }
            });
        });

        Ok(())
    }
}

fn send_generated_tile(
    sender_tiles: &mpsc::SyncSender<(u64, Vec<u8>)>,
    tile: EncodedTile,
) -> Result<()> {
    let (zoom, x, y) = tile.zxy;
    log::debug!(
        "Generated tile: {zoom}/{x}/{y} ({} bytes, {} compressed)",
        bytesize::ByteSize(tile.bytes.len() as u64),
        bytesize::ByteSize(tile.zlib_size as u64),
    );
    sender_tiles
        .send((tile.tile_id, tile.bytes))
        .map_err(|_| PipelineError::Canceled)
}

fn pmtiles_writing_stage(
    output_path: &Path,
    feedback: &Feedback,
    receiver_tiles: mpsc::Receiver<(u64, Vec<u8>)>,
    tile_id_method: TileIdMethod,
) -> Result<()> {
    use prost::Message;
    use std::collections::BTreeSet;
    use tinymvt::vector_tile;

    let mut tiles = receiver_tiles.into_iter().collect::<Vec<_>>();
    if tiles.is_empty() {
        feedback.ensure_not_canceled()?;
        return Err(PipelineError::Other("No tiles to write".to_string()));
    }

    // Tile generation runs in parallel, while PMTiles requires ascending tile IDs.
    tiles.sort_unstable_by_key(|(tile_id, _)| *tile_id);

    let layer_names = if let Some((_, first_tile_data)) = tiles.first() {
        match vector_tile::Tile::decode(&first_tile_data[..]) {
            Ok(tile) => tile
                .layers
                .into_iter()
                .map(|layer| layer.name)
                .collect::<BTreeSet<_>>(),
            Err(error) => {
                feedback.warn(format!(
                    "Failed to decode first tile for metadata extraction: {error:?}"
                ));
                BTreeSet::new()
            }
        }
    } else {
        BTreeSet::new()
    };

    let mut global_min_lon = f64::INFINITY;
    let mut global_max_lon = f64::NEG_INFINITY;
    let mut global_min_lat = f64::INFINITY;
    let mut global_max_lat = f64::NEG_INFINITY;
    let mut actual_min_z = u8::MAX;
    let mut actual_max_z = u8::MIN;

    for (tile_id, _) in &tiles {
        let (zoom, x, y) = tile_id_method.id_to_zxy(*tile_id);
        actual_min_z = actual_min_z.min(zoom);
        actual_max_z = actual_max_z.max(zoom);
        let scale = 1_u32 << zoom;
        let tile_min_lon = (f64::from(x) / f64::from(scale)) * 360.0 - 180.0;
        let tile_max_lon = (f64::from(x + 1) / f64::from(scale)) * 360.0 - 180.0;
        let tile_max_lat = {
            let mercator = std::f64::consts::PI * (1.0 - 2.0 * f64::from(y) / f64::from(scale));
            mercator.sinh().atan() * 180.0 / std::f64::consts::PI
        };
        let tile_min_lat = {
            let mercator = std::f64::consts::PI * (1.0 - 2.0 * f64::from(y + 1) / f64::from(scale));
            mercator.sinh().atan() * 180.0 / std::f64::consts::PI
        };

        global_min_lon = global_min_lon.min(tile_min_lon);
        global_max_lon = global_max_lon.max(tile_max_lon);
        global_min_lat = global_min_lat.min(tile_min_lat);
        global_max_lat = global_max_lat.max(tile_max_lat);
    }

    let center_lon = (global_min_lon + global_max_lon) / 2.0;
    let center_lat = (global_min_lat + global_max_lat) / 2.0;
    let center_zoom = actual_min_z + (actual_max_z - actual_min_z) / 2;
    let metadata = if layer_names.is_empty() {
        "{}".to_string()
    } else {
        let vector_layers = layer_names
            .iter()
            .map(|name| {
                serde_json::json!({
                    "id": name,
                    "minzoom": actual_min_z,
                    "maxzoom": actual_max_z
                })
            })
            .collect::<Vec<_>>();
        serde_json::json!({ "vector_layers": vector_layers }).to_string()
    };

    let file = File::create(output_path)?;
    let mut writer = PmTilesWriter::new(TileType::Mvt)
        .min_zoom(actual_min_z)
        .max_zoom(actual_max_z)
        .bounds(
            global_min_lon,
            global_min_lat,
            global_max_lon,
            global_max_lat,
        )
        .center(center_lon, center_lat)
        .center_zoom(center_zoom)
        .metadata(&metadata)
        .create(file)
        .map_err(|error| {
            PipelineError::Other(format!("Failed to create PMTiles writer: {error:?}"))
        })?;

    let mut tile_count = 0_u64;
    for (tile_id, tile_data) in tiles {
        feedback.ensure_not_canceled()?;
        let (zoom, x, y) = tile_id_method.id_to_zxy(tile_id);
        let coordinate = TileCoord::new(zoom, x, y)
            .map_err(|error| PipelineError::Other(format!("Invalid tile coord: {error:?}")))?;
        writer
            .add_tile(coordinate, &tile_data)
            .map_err(|error| PipelineError::Other(format!("Failed to add tile: {error:?}")))?;

        tile_count += 1;
        report_tile_progress(feedback, tile_count, "Written", "tiles to PMTiles archive");
    }

    feedback.info("Finalizing PMTiles archive...".to_string());
    writer
        .finalize()
        .map_err(|error| PipelineError::Other(format!("Failed to finalize PMTiles: {error:?}")))?;
    correct_empty_leaf_directory_offset_in_file(output_path)?;
    feedback.info(format!(
        "PMTiles archive created: {} ({} tiles)",
        output_path.display(),
        tile_count
    ));
    Ok(())
}

fn read_header_u64(header: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(
        header[offset..offset + PMTILES_U64_FIELD_SIZE]
            .try_into()
            .unwrap(),
    )
}

/// Exists only to patch the header bytes emitted by `pmtiles-rs 0.22.0`.
/// This cannot be removed until the upstream writer records a non-zero
/// offset for an empty leaf-directory section; remove it together with
/// `correct_empty_leaf_directory_offset_in_file`.
fn write_header_u64(header: &mut [u8], offset: usize, value: u64) {
    header[offset..offset + PMTILES_U64_FIELD_SIZE].copy_from_slice(&value.to_le_bytes());
}

fn validate_section_range(
    header: &[u8],
    offset_field_offset: usize,
    length_field_offset: usize,
    section_name: &str,
    file_length: u64,
) -> Result<u64> {
    let offset = read_header_u64(header, offset_field_offset);
    let length = read_header_u64(header, length_field_offset);
    let end = offset.checked_add(length).ok_or_else(|| {
        PipelineError::Other(format!(
            "Invalid PMTiles header: {section_name} range overflows"
        ))
    })?;
    if end > file_length {
        return Err(PipelineError::Other(format!(
            "Invalid PMTiles header: {section_name} range ends at {end}, beyond file length {file_length}"
        )));
    }
    if length > 0 && offset == 0 {
        return Err(PipelineError::Other(format!(
            "Invalid PMTiles header: non-empty {section_name} has offset 0"
        )));
    }
    Ok(end)
}

fn correct_empty_leaf_directory_offset(header: &mut [u8], file_length: u64) -> Result<Option<u64>> {
    if header.len() < PMTILES_HEADER_SIZE {
        return Err(PipelineError::Other(format!(
            "Invalid PMTiles header: expected {PMTILES_HEADER_SIZE} bytes, got {}",
            header.len()
        )));
    }
    if &header[..PMTILES_VERSION_OFFSET] != b"PMTiles" {
        return Err(PipelineError::Other(
            "Invalid PMTiles header: unexpected magic number".to_string(),
        ));
    }
    if header[PMTILES_VERSION_OFFSET] != 3 {
        return Err(PipelineError::Other(format!(
            "Unsupported PMTiles version: {}",
            header[PMTILES_VERSION_OFFSET]
        )));
    }

    let data_end = validate_section_range(
        header,
        PMTILES_TILE_DATA_OFFSET,
        PMTILES_TILE_DATA_LENGTH_OFFSET,
        "tile data",
        file_length,
    )?;
    validate_section_range(
        header,
        PMTILES_LEAF_OFFSET,
        PMTILES_LEAF_LENGTH_OFFSET,
        "leaf directory",
        file_length,
    )?;

    let leaf_offset = read_header_u64(header, PMTILES_LEAF_OFFSET);
    let leaf_length = read_header_u64(header, PMTILES_LEAF_LENGTH_OFFSET);
    if leaf_length == 0 && leaf_offset == 0 {
        write_header_u64(header, PMTILES_LEAF_OFFSET, data_end);
        Ok(Some(data_end))
    } else {
        Ok(None)
    }
}

/// Work around `pmtiles-rs 0.22.0` leaving the leaf-directory offset at zero
/// when an archive has no leaf directories. `pmtiles verify 1.27.0` rejects
/// that header even when the leaf-directory length is zero.
///
/// Remove this post-finalize correction after upgrading to an upstream writer
/// that records a non-zero offset for an empty leaf-directory section.
fn correct_empty_leaf_directory_offset_in_file(output_path: &Path) -> Result<Option<u64>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(output_path)?;
    let file_length = file.metadata()?.len();
    let mut header = [0_u8; PMTILES_HEADER_SIZE];
    file.read_exact(&mut header)?;
    let corrected = correct_empty_leaf_directory_offset(&mut header, file_length)?;

    if corrected.is_some() {
        file.seek(SeekFrom::Start(PMTILES_LEAF_OFFSET as u64))?;
        file.write_all(&header[PMTILES_LEAF_OFFSET..PMTILES_LEAF_OFFSET + PMTILES_U64_FIELD_SIZE])?;
        file.flush()?;
    }

    Ok(corrected)
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Read, sync::mpsc};

    use flate2::read::GzDecoder;
    use pmtiles::Header;
    use prost::Message;
    use tinymvt::vector_tile;

    use super::*;

    const PMTILES_METADATA_OFFSET: usize = 24;
    const PMTILES_METADATA_LENGTH_OFFSET: usize = 32;

    fn sample_mvt() -> Vec<u8> {
        vector_tile::Tile {
            layers: vec![vector_tile::tile::Layer {
                version: 2,
                name: "sample".to_string(),
                features: Vec::new(),
                keys: Vec::new(),
                values: Vec::new(),
                extent: Some(4096),
            }],
        }
        .encode_to_vec()
    }

    fn valid_header(file_length: u64) -> [u8; PMTILES_HEADER_SIZE] {
        let mut header = [0_u8; PMTILES_HEADER_SIZE];
        header[..PMTILES_VERSION_OFFSET].copy_from_slice(b"PMTiles");
        header[PMTILES_VERSION_OFFSET] = 3;
        let tile_data_offset = PMTILES_HEADER_SIZE as u64 + 2;
        write_header_u64(&mut header, PMTILES_LEAF_OFFSET, 0);
        write_header_u64(&mut header, PMTILES_LEAF_LENGTH_OFFSET, 0);
        write_header_u64(&mut header, PMTILES_TILE_DATA_OFFSET, tile_data_offset);
        write_header_u64(
            &mut header,
            PMTILES_TILE_DATA_LENGTH_OFFSET,
            file_length.saturating_sub(tile_data_offset),
        );
        header
    }

    #[track_caller]
    fn assert_header_error(
        mut header: Vec<u8>,
        file_length: u64,
        mutate: impl FnOnce(&mut Vec<u8>),
        expected_message: &str,
    ) {
        mutate(&mut header);
        let error = correct_empty_leaf_directory_offset(&mut header, file_length)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(expected_message),
            "expected error containing {expected_message:?}, got {error:?}"
        );
    }

    #[test]
    fn populated_or_already_positioned_leaf_directory_is_unchanged() {
        let file_length = 512;

        let mut populated = valid_header(file_length);
        write_header_u64(&mut populated, PMTILES_LEAF_OFFSET, 480);
        write_header_u64(&mut populated, PMTILES_LEAF_LENGTH_OFFSET, 32);
        let populated_before = populated;
        assert_eq!(
            correct_empty_leaf_directory_offset(&mut populated, file_length).unwrap(),
            None
        );
        assert_eq!(populated, populated_before);

        let mut positioned_empty = valid_header(file_length);
        write_header_u64(&mut positioned_empty, PMTILES_LEAF_OFFSET, file_length);
        let positioned_empty_before = positioned_empty;
        assert_eq!(
            correct_empty_leaf_directory_offset(&mut positioned_empty, file_length).unwrap(),
            None
        );
        assert_eq!(positioned_empty, positioned_empty_before);
    }

    #[test]
    fn malformed_or_out_of_range_header_is_rejected() {
        let file_length = 512;
        let valid = valid_header(file_length).to_vec();
        assert_header_error(
            valid.clone(),
            file_length,
            |header| header.truncate(PMTILES_HEADER_SIZE - 1),
            "expected 127 bytes, got 126",
        );
        assert_header_error(
            valid.clone(),
            file_length,
            |header| header[0] = b'X',
            "Invalid PMTiles header: unexpected magic number",
        );
        assert_header_error(
            valid.clone(),
            file_length,
            |header| header[PMTILES_VERSION_OFFSET] = 2,
            "Unsupported PMTiles version: 2",
        );
        assert_header_error(
            valid.clone(),
            file_length,
            |header| {
                write_header_u64(header, PMTILES_TILE_DATA_OFFSET, u64::MAX);
                write_header_u64(header, PMTILES_TILE_DATA_LENGTH_OFFSET, 2);
            },
            "tile data range overflows",
        );
        assert_header_error(
            valid.clone(),
            file_length,
            |header| {
                write_header_u64(header, PMTILES_TILE_DATA_LENGTH_OFFSET, file_length);
            },
            "tile data range ends at 641, beyond file length 512",
        );
        assert_header_error(
            valid,
            file_length,
            |header| {
                write_header_u64(header, PMTILES_LEAF_OFFSET, 500);
                write_header_u64(header, PMTILES_LEAF_LENGTH_OFFSET, 13);
            },
            "leaf directory range ends at 513, beyond file length 512",
        );
    }

    #[test]
    fn writing_stage_uses_actual_zoom_range_and_valid_empty_leaf_offset() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("sink-output.pmtiles");

        let (sender, receiver) = mpsc::channel();
        let tile_id_method = TileIdMethod::Hilbert;
        sender
            .send((tile_id_method.zxy_to_id(8, 0, 0), sample_mvt()))
            .unwrap();
        sender
            .send((tile_id_method.zxy_to_id(14, 0, 0), sample_mvt()))
            .unwrap();
        drop(sender);

        let (_watcher, feedback, _canceller) = crate::pipeline::watcher();
        pmtiles_writing_stage(&output_path, &feedback, receiver, tile_id_method).unwrap();

        let archive = fs::read(&output_path).unwrap();
        let header =
            Header::try_from_bytes(archive[..PMTILES_HEADER_SIZE].to_vec().into()).unwrap();
        assert_eq!(header.min_zoom, 8);
        assert_eq!(header.max_zoom, 14);
        assert_eq!(header.center_zoom, 11);

        let leaf_offset = read_header_u64(&archive, PMTILES_LEAF_OFFSET);
        let tile_data_offset = read_header_u64(&archive, PMTILES_TILE_DATA_OFFSET);
        let tile_data_length = read_header_u64(&archive, PMTILES_TILE_DATA_LENGTH_OFFSET);
        assert_eq!(leaf_offset, tile_data_offset + tile_data_length);
        assert_eq!(leaf_offset, archive.len() as u64);

        let metadata_offset = read_header_u64(&archive, PMTILES_METADATA_OFFSET) as usize;
        let metadata_length = read_header_u64(&archive, PMTILES_METADATA_LENGTH_OFFSET) as usize;
        let mut metadata = String::new();
        GzDecoder::new(&archive[metadata_offset..metadata_offset + metadata_length])
            .read_to_string(&mut metadata)
            .unwrap();
        let metadata: serde_json::Value = serde_json::from_str(&metadata).unwrap();
        assert_eq!(metadata["vector_layers"][0]["minzoom"], 8);
        assert_eq!(metadata["vector_layers"][0]["maxzoom"], 14);
    }
}
