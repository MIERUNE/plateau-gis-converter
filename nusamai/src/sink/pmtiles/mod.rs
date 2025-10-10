// ! PMTiles sink
//!
//! This sink converts CityGML data to PMTiles format, a single-file archive for tiled data.
//! It reuses geometry slicing and sorting logic from the MVT sink, but writes to a single
//! PMTiles archive instead of individual tile files.

use std::{
    convert::Infallible,
    fs::File,
    path::{Path, PathBuf},
    sync::mpsc,
};

use flatgeom::MultiPolygon2;
use hashbrown::HashMap;
use itertools::Itertools;
use nusamai_citygml::{object, schema::Schema};
use pmtiles::{PmTilesWriter, TileCoord, TileType};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tinymvt::{geometry::GeometryEncoder, tag::TagsEncoder, vector_tile};

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{
        mvt::slice::slice_cityobj_geoms, mvt::tags::convert_properties, mvt::tileid::TileIdMethod,
        DataRequirements, DataSink, DataSinkProvider, SinkInfo,
    },
    transformer,
    transformer::{use_lod_config, TransformerSettings},
};

use super::option::output_parameter;

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
        let mut settings: TransformerSettings = TransformerSettings::new();
        settings.insert(use_lod_config("min_lod", None));

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_options = self.transformer_options();
        let min_z = get_parameter_value!(params, "min_z", Integer).unwrap() as u8;
        let max_z = get_parameter_value!(params, "max_z", Integer).unwrap() as u8;

        Box::<PmTilesSink>::new(PmTilesSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings: transform_options,
            pmtiles_options: PmTilesParams { min_z, max_z },
        })
    }
}

struct PmTilesSink {
    output_path: PathBuf,
    transform_settings: TransformerSettings,
    pmtiles_options: PmTilesParams,
}

struct PmTilesParams {
    min_z: u8,
    max_z: u8,
}

/// Sliced feature data structure (same as MVT)
#[derive(Serialize, Deserialize)]
struct SlicedFeature<'a> {
    geometry: MultiPolygon2<'a>,
    properties: nusamai_citygml::object::Value,
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

        for config in properties.configs.iter() {
            let _ = &self.transform_settings.update_transformer(config.clone());
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender_sliced, receiver_sliced) = mpsc::sync_channel(2000);
        let (sender_sorted, receiver_sorted) = mpsc::sync_channel(2000);
        let (sender_tiles, receiver_tiles) = mpsc::sync_channel(100);

        let tile_id_conv = TileIdMethod::Hilbert;

        std::thread::scope(|s| {
            // Stage 1: Slicing geometry along the tile boundaries (parallel, reused from MVT)
            {
                let pmtiles_options = &self.pmtiles_options;
                s.spawn(|| {
                    if let Err(error) = geometry_slicing_stage(
                        feedback,
                        upstream,
                        tile_id_conv,
                        sender_sliced,
                        pmtiles_options,
                    ) {
                        feedback.fatal_error(error);
                    }
                });
            }

            // Stage 2: Sort features by tile_id (using external sorter, reused from MVT)
            {
                s.spawn(move || {
                    if let Err(error) =
                        feature_sorting_stage(feedback, receiver_sliced, sender_sorted)
                    {
                        feedback.fatal_error(error);
                    }
                });
            }

            // Stage 3: Generate MVT tiles in parallel
            {
                s.spawn(move || {
                    let pool = rayon::ThreadPoolBuilder::new()
                        .use_current_thread()
                        .build()
                        .unwrap();
                    pool.install(|| {
                        if let Err(error) = tile_generation_stage(
                            feedback,
                            receiver_sorted,
                            sender_tiles,
                            tile_id_conv,
                        ) {
                            feedback.fatal_error(error);
                        }
                    })
                });
            }

            // Stage 4: Write tiles to PMTiles archive sequentially
            {
                let output_path = &self.output_path;
                let min_z = self.pmtiles_options.min_z;
                let max_z = self.pmtiles_options.max_z;
                s.spawn(move || {
                    if let Err(error) = pmtiles_writing_stage(
                        output_path,
                        min_z,
                        max_z,
                        feedback,
                        receiver_tiles,
                        tile_id_conv,
                    ) {
                        feedback.fatal_error(error);
                    }
                });
            }
        });

        Ok(())
    }
}

/// Stage 1: Geometry slicing (reused from MVT)
fn geometry_slicing_stage(
    feedback: &Feedback,
    upstream: mpsc::Receiver<crate::pipeline::Parcel>,
    tile_id_conv: TileIdMethod,
    sender_sliced: mpsc::SyncSender<(u64, Vec<u8>)>,
    pmtiles_options: &PmTilesParams,
) -> Result<()> {
    let bincode_config = bincode::config::standard();

    // Convert CityObjects to sliced features
    upstream.into_iter().par_bridge().try_for_each(|parcel| {
        feedback.ensure_not_canceled()?;

        let max_detail = 12; // 4096
        let buffer_pixels = 5;
        slice_cityobj_geoms(
            &parcel.entity,
            pmtiles_options.min_z,
            pmtiles_options.max_z,
            max_detail,
            buffer_pixels,
            |(z, x, y), mpoly| {
                feedback.ensure_not_canceled()?;

                let feature = SlicedFeature {
                    geometry: mpoly,
                    properties: parcel.entity.root.clone(),
                };
                let bytes = bincode::serde::encode_to_vec(&feature, bincode_config).unwrap();
                let tile_id = tile_id_conv.zxy_to_id(z, x, y);
                if sender_sliced.send((tile_id, bytes)).is_err() {
                    return Err(PipelineError::Canceled);
                };
                Ok(())
            },
        )
    })?;
    Ok(())
}

/// Stage 2: Feature sorting (reused from MVT)
fn feature_sorting_stage(
    feedback: &Feedback,
    receiver_sliced: mpsc::Receiver<(u64, Vec<u8>)>,
    sender_sorted: mpsc::SyncSender<(u64, Vec<Vec<u8>>)>,
) -> Result<()> {
    let config = kv_extsort::SortConfig::default()
        .max_chunk_bytes(256 * 1024 * 1024) // TODO: Configurable
        .set_cancel_flag(feedback.get_cancellation_flag());

    let sorted_iter = kv_extsort::sort(
        receiver_sliced
            .into_iter()
            .map(|(tile_id, body)| std::result::Result::<_, Infallible>::Ok((tile_id, body))),
        config,
    );

    for ((_, tile_id), grouped) in &sorted_iter.chunk_by(|feat| match feat {
        Ok((tile_id, _)) => (false, *tile_id),
        Err(_) => (true, 0),
    }) {
        let grouped = grouped
            .into_iter()
            .map_ok(|(_, serialized_feats)| serialized_feats)
            .collect::<kv_extsort::Result<Vec<_>, _>>();
        match grouped {
            Ok(serialized_feats) => {
                feedback.ensure_not_canceled()?;
                if sender_sorted.send((tile_id, serialized_feats)).is_err() {
                    return Err(PipelineError::Canceled);
                }
            }
            Err(kv_extsort::Error::Canceled) => {
                return Err(PipelineError::Canceled);
            }
            Err(err) => {
                return Err(PipelineError::Other(format!(
                    "Failed to sort features: {err:?}"
                )));
            }
        }
    }

    Ok(())
}

/// Stage 3: Tile generation in parallel
fn tile_generation_stage(
    feedback: &Feedback,
    receiver_sorted: mpsc::Receiver<(u64, Vec<Vec<u8>>)>,
    sender_tiles: mpsc::SyncSender<(u64, Vec<u8>)>,
    tile_id_conv: TileIdMethod,
) -> Result<()> {
    let default_detail = 12;
    let min_detail = 9;

    receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|(tile_id, serialized_feats)| {
            feedback.ensure_not_canceled()?;

            let (zoom, x, y) = tile_id_conv.id_to_zxy(tile_id);

            if serialized_feats.len() > 200_000 {
                feedback.warn(format!(
                    "Too many features in a tile ({} features)",
                    serialized_feats.len()
                ));
            }

            // Try different detail levels to keep tile size manageable
            for detail in (min_detail..=default_detail).rev() {
                feedback.ensure_not_canceled()?;

                // Make a MVT tile binary (reused from MVT)
                let bytes = make_tile(detail, &serialized_feats)?;

                // Check compressed tile size
                let compressed_size = {
                    use flate2::{write::ZlibEncoder, Compression};
                    use std::io::prelude::*;
                    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
                    e.write_all(&bytes)?;
                    let compressed_bytes = e.finish()?;
                    compressed_bytes.len()
                };

                if detail != min_detail && compressed_size > 500_000 {
                    // If the tile is too large, try a lower detail level
                    let extent = 1 << detail;
                    feedback.info(format!(
                        "Tile size is too large: {zoom}/{x}/{y} (extent: {extent}), trying a \
                         lower detail level."
                    ));
                    continue;
                }

                feedback.info(format!(
                    "Generated tile: {zoom}/{x}/{y} ({} bytes, {} compressed)",
                    bytesize::ByteSize(bytes.len() as u64),
                    bytesize::ByteSize(compressed_size as u64),
                ));

                // Send to PMTiles writer
                if sender_tiles.send((tile_id, bytes)).is_err() {
                    return Err(PipelineError::Canceled);
                }
                break;
            }

            Ok::<(), PipelineError>(())
        })?;

    Ok(())
}

/// Stage 4: Write tiles to PMTiles archive sequentially
fn pmtiles_writing_stage(
    output_path: &Path,
    min_z: u8,
    max_z: u8,
    feedback: &Feedback,
    receiver_tiles: mpsc::Receiver<(u64, Vec<u8>)>,
    tile_id_conv: TileIdMethod,
) -> Result<()> {
    use prost::Message;
    use std::collections::BTreeSet;

    // Collect all tiles first to calculate bounds
    let tiles: Vec<_> = receiver_tiles.into_iter().collect();

    if tiles.is_empty() {
        return Err(PipelineError::Other("No tiles to write".to_string()));
    }

    // Extract layer names from the first tile for metadata
    let layer_names: BTreeSet<String> = if let Some((_, first_tile_data)) = tiles.first() {
        match vector_tile::Tile::decode(&first_tile_data[..]) {
            Ok(tile) => tile.layers.into_iter().map(|layer| layer.name).collect(),
            Err(e) => {
                feedback.warn(format!(
                    "Failed to decode first tile for metadata extraction: {e:?}"
                ));
                BTreeSet::new()
            }
        }
    } else {
        BTreeSet::new()
    };

    // Calculate bounds from tile coordinates
    let mut min_x = u32::MAX;
    let mut max_x = 0u32;
    let mut min_y = u32::MAX;
    let mut max_y = 0u32;
    let mut bounds_z = 0u8;

    for (tile_id, _) in &tiles {
        let (z, x, y) = tile_id_conv.id_to_zxy(*tile_id);

        // Use the maximum zoom level tiles for bounds calculation
        if z >= bounds_z {
            if z > bounds_z {
                // Reset bounds when we find a higher zoom level
                min_x = x;
                max_x = x;
                min_y = y;
                max_y = y;
                bounds_z = z;
            } else {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    // Convert tile coordinates to lon/lat bounds
    // Using Web Mercator tile math: lon = (x / 2^z) * 360 - 180
    let n = 1u32 << bounds_z; // 2^z
    let min_lon = (min_x as f64 / n as f64) * 360.0 - 180.0;
    let max_lon = ((max_x + 1) as f64 / n as f64) * 360.0 - 180.0;

    // lat = atan(sinh(π * (1 - 2 * y / 2^z))) * 180 / π
    let min_lat = {
        let y_mercator = std::f64::consts::PI * (1.0 - 2.0 * (max_y + 1) as f64 / n as f64);
        y_mercator.sinh().atan() * 180.0 / std::f64::consts::PI
    };
    let max_lat = {
        let y_mercator = std::f64::consts::PI * (1.0 - 2.0 * min_y as f64 / n as f64);
        y_mercator.sinh().atan() * 180.0 / std::f64::consts::PI
    };

    // Calculate center
    let center_lon = (min_lon + max_lon) / 2.0;
    let center_lat = (min_lat + max_lat) / 2.0;
    let center_zoom = (min_z + max_z) / 2;

    // Generate TileJSON metadata with vector_layers (required for MVT)
    let metadata_json = if !layer_names.is_empty() {
        let vector_layers: Vec<_> = layer_names
            .iter()
            .map(|name| {
                serde_json::json!({
                    "id": name,
                    "minzoom": min_z,
                    "maxzoom": max_z
                })
            })
            .collect();

        serde_json::json!({
            "vector_layers": vector_layers
        })
        .to_string()
    } else {
        "{}".to_string()
    };

    // Create PMTiles writer with bounds, center, and metadata
    let file = File::create(output_path)?;
    let mut writer = PmTilesWriter::new(TileType::Mvt)
        .min_zoom(min_z)
        .max_zoom(max_z)
        .bounds(
            min_lon as f32,
            min_lat as f32,
            max_lon as f32,
            max_lat as f32,
        )
        .center(center_lon as f32, center_lat as f32)
        .center_zoom(center_zoom)
        .metadata(&metadata_json)
        .create(file)
        .map_err(|e| PipelineError::Other(format!("Failed to create PMTiles writer: {e:?}")))?;

    let mut tile_count = 0u64;

    // Write tiles sequentially
    for (tile_id, tile_data) in tiles {
        feedback.ensure_not_canceled()?;

        let (z, x, y) = tile_id_conv.id_to_zxy(tile_id);
        let coord = TileCoord::new(z, x, y)
            .map_err(|e| PipelineError::Other(format!("Invalid tile coord: {e:?}")))?;

        writer
            .add_tile(coord, &tile_data)
            .map_err(|e| PipelineError::Other(format!("Failed to add tile: {e:?}")))?;

        tile_count += 1;
        if tile_count % 1000 == 0 {
            feedback.info(format!("Written {} tiles to PMTiles archive", tile_count));
        }
    }

    // Finalize PMTiles archive
    feedback.info("Finalizing PMTiles archive...".to_string());
    writer
        .finalize()
        .map_err(|e| PipelineError::Other(format!("Failed to finalize PMTiles: {e:?}")))?;

    feedback.info(format!(
        "PMTiles archive created: {} ({} tiles)",
        output_path.display(),
        tile_count
    ));

    Ok(())
}

/// Layer data structure (from MVT)
#[derive(Default)]
struct LayerData {
    pub features: Vec<vector_tile::tile::Feature>,
    pub tags_enc: TagsEncoder,
}

/// Generate MVT tile from sliced features (reused from MVT)
fn make_tile(default_detail: i32, serialized_feats: &[Vec<u8>]) -> Result<Vec<u8>> {
    use flatgeom::MultiPolygon;
    use prost::Message;

    let mut layers: HashMap<String, LayerData> = HashMap::new();
    let mut int_ring_buf = Vec::new();
    let mut int_ring_buf2 = Vec::new();
    let extent = 1 << default_detail;
    let bincode_config = bincode::config::standard();

    for serialized_feat in serialized_feats {
        let (feature, _): (SlicedFeature, _) =
            bincode::serde::decode_from_slice(serialized_feat, bincode_config).map_err(|err| {
                PipelineError::Other(format!("Failed to deserialize a sliced feature: {err:?}"))
            })?;

        let mpoly = feature.geometry;
        let mut int_mpoly = MultiPolygon::<[i16; 2]>::new();

        for poly in &mpoly {
            for (ri, ring) in poly.rings().enumerate() {
                int_ring_buf.clear();
                int_ring_buf.extend(ring.into_iter().map(|[x, y]| {
                    let x = (x * extent as f64 + 0.5) as i16;
                    let y = (y * extent as f64 + 0.5) as i16;
                    [x, y]
                }));

                // some simplification
                {
                    int_ring_buf2.clear();
                    int_ring_buf2.push(int_ring_buf[0]);
                    for c in int_ring_buf.windows(3) {
                        let &[prev, curr, next] = c.try_into().unwrap();

                        // Remove duplicate points
                        if prev == curr {
                            continue;
                        }

                        // Reject collinear points
                        let [curr_x, curr_y] = curr;
                        let [prev_x, prev_y] = prev;
                        let [next_x, next_y] = next;
                        if curr != next
                            && ((next_y - prev_y) as i32 * (curr_x - prev_x) as i32).abs()
                                == ((curr_y - prev_y) as i32 * (next_x - prev_x) as i32).abs()
                        {
                            continue;
                        }

                        int_ring_buf2.push(curr);
                    }
                    int_ring_buf2.push(*int_ring_buf.last().unwrap());
                }

                match ri {
                    0 => int_mpoly.add_exterior(int_ring_buf2.drain(..)),
                    _ => int_mpoly.add_interior(int_ring_buf2.drain(..)),
                }
            }
        }

        // encode geometry
        let mut geom_enc = GeometryEncoder::new();
        for poly in &int_mpoly {
            let exterior = poly.exterior();
            if exterior.signed_ring_area() > 0.0 {
                geom_enc.add_ring(&exterior);
                for interior in poly.interiors() {
                    if interior.is_cw() {
                        geom_enc.add_ring(&interior);
                    }
                }
            }
        }
        let geometry = geom_enc.into_vec();
        if geometry.is_empty() {
            continue;
        }

        let mut id = None;
        let layer = if let object::Value::Object(obj) = &feature.properties {
            let typename = obj.typename.as_ref();

            // id
            if let Some(object::Value::String(gml_id)) = obj.attributes.get("gml_id") {
                id = Some(gml_id.to_string());
            }

            layers.entry(typename.to_string()).or_default()
        } else {
            continue;
        };

        let mut mvt_feature = vector_tile::tile::Feature::default();
        mvt_feature.set_type(vector_tile::tile::GeomType::Polygon);
        mvt_feature.geometry = geometry;

        // encode tags
        if let object::Value::Object(obj) = &feature.properties {
            for (k, v) in obj.attributes.iter() {
                convert_properties(&mut layer.tags_enc, k, v);
            }
        }
        mvt_feature.tags = layer.tags_enc.take_tags();

        layer.features.push(mvt_feature);
    }

    // Encode tile (same as MVT sink)
    let layers_vec = layers
        .into_iter()
        .flat_map(|(name, layer_data)| {
            if layer_data.features.is_empty() {
                return None;
            }
            let (keys, values) = layer_data.tags_enc.into_keys_and_values();
            Some(vector_tile::tile::Layer {
                version: 2,
                name: name.to_string(),
                features: layer_data.features,
                keys,
                values,
                extent: Some(extent),
            })
        })
        .collect();

    let tile = vector_tile::Tile { layers: layers_vec };

    let bytes = tile.encode_to_vec();
    Ok(bytes)
}
