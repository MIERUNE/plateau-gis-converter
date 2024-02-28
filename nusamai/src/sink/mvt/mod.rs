//! Mapbox Vector Tiles (MVT) sink

mod slice;
mod sort;
mod tags;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use ext_sort::{buffer::mem::MemoryLimitedBufferBuilder, ExternalSorter, ExternalSorterBuilder};
use hashbrown::HashMap;
use itertools::Itertools;

use prost::Message;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use nusamai_citygml::object;
use nusamai_citygml::schema::Schema;
use nusamai_geometry::MultiPolygon;
use nusamai_mvt::geometry::GeometryEncoder;
use nusamai_mvt::tag::TagsEncoder;
use nusamai_mvt::{tileid::TileIdMethod, vector_tile};

use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};
use slice::slice_cityobj_geoms;
use sort::BincodeExternalChunk;
use tags::convert_properties;

pub struct MvtSinkProvider {}

impl DataSinkProvider for MvtSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "mvt".to_string(),
            name: "Mapbox Vector Tiles (MVT)".to_string(),
        }
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(
            "@output".into(),
            ParameterEntry {
                description: "Output file path".into(),
                required: true,
                parameter: ParameterType::FileSystemPath(FileSystemPathParameter {
                    value: None,
                    must_exist: false,
                }),
            },
        );
        params.define(
            "min_z".into(),
            ParameterEntry {
                description: "Minumum zoom level".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: Some(7),
                    min: Some(0),
                    max: Some(20),
                }),
            },
        );
        params.define(
            "max_z".into(),
            ParameterEntry {
                description: "Maximum zoom level".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: Some(15),
                    min: Some(0),
                    max: Some(20),
                }),
            },
        );
        params
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let min_z = get_parameter_value!(params, "min_z", Integer).unwrap() as u8;
        let max_z = get_parameter_value!(params, "max_z", Integer).unwrap() as u8;

        Box::<MvtSink>::new(MvtSink {
            output_path: output_path.as_ref().unwrap().into(),
            mvt_options: MvtParams { min_z, max_z },
        })
    }
}

struct MvtSink {
    output_path: PathBuf,
    mvt_options: MvtParams,
}

struct MvtParams {
    min_z: u8,
    max_z: u8,
}

#[derive(Serialize, Deserialize, deepsize::DeepSizeOf)]
struct SerializedSlicedFeature {
    tile_id: u64,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct SlicedFeature<'a> {
    geometry: MultiPolygon<'a, 2, i16>,
    properties: nusamai_citygml::object::Value,
}

impl DataSink for MvtSink {
    fn make_transform_requirements(&self) -> transformer::Requirements {
        transformer::Requirements {
            key_value: transformer::KeyValueSpec::DotNotation,
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender_sliced, receiver_sliced) = mpsc::sync_channel(2000);
        let (sender_sorted, receiver_sorted) = mpsc::sync_channel(2000);

        let tile_id_conv = TileIdMethod::Hilbert;

        // TODO: refactoring

        std::thread::scope(|s| {
            // Slicing geometry along the tile boundaries
            {
                s.spawn(|| {
                    if let Err(error) = geometry_slicing_stage(
                        feedback,
                        upstream,
                        tile_id_conv,
                        sender_sliced,
                        &self.mvt_options,
                    ) {
                        feedback.report_fatal_error(error);
                    }
                });
            }

            // Sort features by tile_id (using external sorter)
            {
                s.spawn(move || {
                    feature_sorting_stage(receiver_sliced, sender_sorted);
                });
            }

            // Group sorted features and write them into MVT tiles
            {
                let output_path = &self.output_path;
                s.spawn(move || {
                    // Run in a separate thread pool to avoid deadlocks
                    let pool = rayon::ThreadPoolBuilder::new()
                        .use_current_thread()
                        .build()
                        .unwrap();
                    pool.install(|| {
                        if let Err(error) =
                            tile_writing_stage(output_path, feedback, receiver_sorted, tile_id_conv)
                        {
                            feedback.report_fatal_error(error);
                        }
                    })
                });
            }
        });

        Ok(())
    }
}

fn geometry_slicing_stage(
    feedback: &Feedback,
    upstream: mpsc::Receiver<crate::pipeline::Parcel>,
    tile_id_conv: TileIdMethod,
    sender_sliced: mpsc::SyncSender<SerializedSlicedFeature>,
    mvt_options: &MvtParams,
) -> Result<()> {
    // Convert CityObjects to sliced features
    upstream.into_iter().par_bridge().try_for_each(|parcel| {
        feedback.ensure_not_canceled()?;

        let max_detail = 12; // 4096
        let buffer_pixels = 5;
        slice_cityobj_geoms(
            &parcel.entity,
            mvt_options.min_z,
            mvt_options.max_z,
            max_detail,
            buffer_pixels,
            |(z, x, y), mpoly| {
                let feature = SlicedFeature {
                    geometry: mpoly,
                    properties: parcel.entity.root.clone(),
                };
                let bytes = bincode::serialize(&feature).unwrap();
                let sfeat = SerializedSlicedFeature {
                    tile_id: tile_id_conv.zxy_to_id(z, x, y),
                    body: bytes,
                };

                if sender_sliced.send(sfeat).is_err() {
                    return Err(PipelineError::Canceled);
                };
                Ok(())
            },
        )
    })?;
    Ok(())
}

fn feature_sorting_stage(
    receiver_sliced: mpsc::Receiver<SerializedSlicedFeature>,
    sender_sorted: mpsc::SyncSender<(u64, Vec<SerializedSlicedFeature>)>,
) {
    let sorter: ExternalSorter<
        SerializedSlicedFeature,
        std::io::Error,
        MemoryLimitedBufferBuilder,
        BincodeExternalChunk<_>,
        // TODO: Implement an external sorter by ourselves?
    > = ExternalSorterBuilder::new()
        .with_buffer(MemoryLimitedBufferBuilder::new(200 * 1024 * 1024)) // TODO
        .with_threads_number(8) // TODO
        .build()
        .unwrap();
    let sorted = sorter
        .sort_by(receiver_sliced.into_iter().map(Ok), |a, b| {
            a.tile_id.cmp(&b.tile_id)
        })
        .unwrap();

    for (tile_id, ser_feats) in &sorted
        .map(std::result::Result::unwrap)
        .group_by(|ser_feat| ser_feat.tile_id)
    {
        let ser_feats: Vec<_> = ser_feats.collect();
        if sender_sorted.send((tile_id, ser_feats)).is_err() {
            return;
        };
    }
}

#[derive(Default)]
struct LayerData {
    pub features: Vec<vector_tile::tile::Feature>,
    pub tags_enc: TagsEncoder,
}

fn tile_writing_stage(
    output_path: &Path,
    feedback: &Feedback,
    receiver_sorted: mpsc::Receiver<(u64, Vec<SerializedSlicedFeature>)>,
    tile_id_conv: TileIdMethod,
) -> Result<()> {
    let detail = 12;
    let extent = 1 << detail;

    receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|(tile_id, sfeats)| {
            feedback.ensure_not_canceled()?;
            let (zoom, x, y) = tile_id_conv.id_to_zxy(tile_id);

            let mut layers: HashMap<String, LayerData> = HashMap::new();

            for ser_feat in sfeats {
                let feat: SlicedFeature = bincode::deserialize(&ser_feat.body).unwrap();
                let mpoly = feat.geometry;

                // encode geometry
                let mut geom_enc = GeometryEncoder::new();
                for poly in &mpoly {
                    let exterior = poly.exterior();
                    if exterior.is_ccw() {
                        geom_enc.add_ring(&exterior);
                    }
                    for interior in poly.interiors() {
                        if interior.is_cw() {
                            geom_enc.add_ring(&interior);
                        }
                    }
                }
                let geometry = geom_enc.into_vec();
                if geometry.is_empty() {
                    continue;
                }

                let mut id = None;
                let mut tags: Vec<u32> = Vec::new();

                let layer = if let object::Value::Object(obj) = &feat.properties {
                    let layer = layers.entry_ref(obj.typename.as_ref()).or_default();

                    // Encode attributes as MVT tags
                    for (key, value) in &obj.attributes {
                        convert_properties(&mut tags, &mut layer.tags_enc, key, value);
                    }

                    // Make a MVT feature id (u64) by hashing the original feature id string.
                    id = obj.stereotype.id().map(|id| {
                        id.as_bytes()
                            .iter()
                            .fold(5381u64, |a, c| a.wrapping_mul(33) ^ *c as u64)
                    });

                    layer
                } else {
                    layers.entry_ref("Unknown").or_default()
                };

                layer.features.push(vector_tile::tile::Feature {
                    id,
                    tags,
                    r#type: Some(vector_tile::tile::GeomType::Polygon as i32),
                    geometry,
                });
            }

            let layers = layers
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

            let tile = vector_tile::Tile { layers };

            let path = output_path.join(Path::new(&format!("{}/{}/{}.pbf", zoom, x, y)));

            if let Some(dir) = path.parent() {
                fs::create_dir_all(dir)?;
            }
            let bytes = tile.encode_to_vec();

            log::info!(
                "Writing a tile: {} ({})",
                &path.to_string_lossy(),
                bytesize::to_string(bytes.len() as u64, true)
            );

            fs::write(&path, &bytes)?;

            Ok::<(), PipelineError>(())
        })?;

    Ok(())
}
