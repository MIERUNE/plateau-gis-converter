//! PMTiles sink

use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::mpsc,
};

use nusamai_citygml::schema::Schema;
use pmtiles::{PmTilesWriter, TileCoord, TileType};

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{
        mvt::encode::MvtTileEncoder,
        vector_tile::{
            feature_sorting_stage, generate_stage, slice_stage, tile_id::TileIdMethod, EncodedTile,
            TilePipelineOptions, DEFAULT_MAX_COMPRESSED_TILE_SIZE, FEATURE_CHANNEL_CAPACITY,
        },
        DataRequirements, DataSink, DataSinkProvider, SinkInfo,
    },
    transformer,
    transformer::{use_lod_config, TransformerSettings},
};

use super::{option::output_parameter, vector_tile::slice::validate_zoom_range};

const TILE_CHANNEL_CAPACITY: usize = 100;

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

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
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
                let pool = rayon::ThreadPoolBuilder::new().build().map_err(|error| {
                    PipelineError::Other(format!("Failed to build thread pool: {error}"))
                });
                match pool {
                    Ok(pool) => pool.install(|| {
                        if let Err(error) = generate_stage(
                            feedback,
                            receiver_sorted,
                            tile_id_method,
                            pipeline_options.max_compressed_tile_size,
                            &encoder,
                            |tile| send_generated_tile(&sender_tiles, tile),
                        ) {
                            feedback.fatal_error(error);
                        }
                    }),
                    Err(error) => feedback.fatal_error(error),
                }
            });

            scope.spawn(move || {
                if let Err(error) = pmtiles_writing_stage(
                    &self.output_path,
                    self.options.min_z,
                    self.options.max_z,
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
    min_z: u8,
    max_z: u8,
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

    for (tile_id, _) in &tiles {
        let (zoom, x, y) = tile_id_method.id_to_zxy(*tile_id);
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
    let center_zoom = (min_z + max_z) / 2;
    let metadata = if layer_names.is_empty() {
        "{}".to_string()
    } else {
        let vector_layers = layer_names
            .iter()
            .map(|name| {
                serde_json::json!({
                    "id": name,
                    "minzoom": min_z,
                    "maxzoom": max_z
                })
            })
            .collect::<Vec<_>>();
        serde_json::json!({ "vector_layers": vector_layers }).to_string()
    };

    let file = File::create(output_path)?;
    let mut writer = PmTilesWriter::new(TileType::Mvt)
        .min_zoom(min_z)
        .max_zoom(max_z)
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
        if tile_count.is_multiple_of(1_000) {
            feedback.info(format!("Written {tile_count} tiles to PMTiles archive"));
        }
    }

    feedback.info("Finalizing PMTiles archive...".to_string());
    writer
        .finalize()
        .map_err(|error| PipelineError::Other(format!("Failed to finalize PMTiles: {error:?}")))?;
    feedback.info(format!(
        "PMTiles archive created: {} ({} tiles)",
        output_path.display(),
        tile_count
    ));
    Ok(())
}
