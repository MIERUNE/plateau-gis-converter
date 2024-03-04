//! 3D Tiles sink

mod gltf;
mod material;
mod metadata;
mod slice;
mod sort;
mod tiling;
mod utils;

use std::cmp::Ordering;
use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};

use ahash::RandomState;
use earcut_rs::{utils3d::project3d_to_2d, Earcut};
use ext_sort::{buffer::mem::MemoryLimitedBufferBuilder, ExternalSorter, ExternalSorterBuilder};
use indexmap::IndexSet;
use itertools::Itertools;
use nusamai_projection::cartesian::geographic_to_geocentric;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use nusamai_citygml::object::Value;
use nusamai_citygml::schema::Schema;
use nusamai_mvt::tileid::TileIdMethod;

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo};
use gltf::write_gltf_glb;
use slice::{slice_to_tiles, SlicedFeature};
use sort::BincodeExternalChunk;
use tiling::{TileContent, TileTree};

use self::utils::calculate_normal;

pub struct CesiumTilesSinkProvider {}

impl DataSinkProvider for CesiumTilesSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "3dtiles".to_string(),
            name: "Cesium 3D Tiles".to_string(),
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
        // TODO: min Zoom
        // TODO: max Zoom
        params
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);

        Box::<CesiumTilesSink>::new(CesiumTilesSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

struct CesiumTilesSink {
    output_path: PathBuf,
}

#[derive(Serialize, Deserialize, deepsize::DeepSizeOf)]
struct SerializedSlicedFeature {
    tile_id: u64,
    typename: String,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

impl DataSink for CesiumTilesSink {
    fn make_requirements(&self) -> DataRequirements {
        DataRequirements {
            // use_appearance: true,
            resolve_appearance: true,
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender_sliced, receiver_sliced) = mpsc::sync_channel(2000);
        let (sender_sorted, receiver_sorted) = mpsc::sync_channel(2000);

        let tile_id_conv = TileIdMethod::Hilbert;

        // TODO: configurable
        let min_zoom = 12;
        let max_zoom = 18;

        // TODO: refactoring

        std::thread::scope(|s| {
            // Slicing geometry along the tile boundaries
            {
                s.spawn(move || {
                    if let Err(error) = geometry_slicing_stage(
                        feedback,
                        upstream,
                        tile_id_conv,
                        sender_sliced,
                        min_zoom,
                        max_zoom,
                    ) {
                        feedback.report_fatal_error(error);
                    }
                });
            }

            // Sort features by tile_id (using external sorter)
            {
                s.spawn(move || {
                    if let Err(error) =
                        feature_sorting_stage(feedback, receiver_sliced, sender_sorted)
                    {
                        feedback.report_fatal_error(error);
                    }
                });
            }

            // Group sorted features and write them into tiles
            {
                let output_path = &self.output_path;
                s.spawn(move || {
                    // Run in a separate thread pool to avoid deadlocks
                    let pool = rayon::ThreadPoolBuilder::new()
                        .use_current_thread()
                        .build()
                        .unwrap();
                    pool.install(|| {
                        if let Err(error) = tile_writing_stage(
                            output_path,
                            feedback,
                            receiver_sorted,
                            tile_id_conv,
                            min_zoom,
                            max_zoom,
                        ) {
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
    min_zoom: u8,
    max_zoom: u8,
) -> Result<()> {
    // Convert CityObjects to sliced features
    upstream.into_iter().par_bridge().try_for_each(|parcel| {
        feedback.ensure_not_canceled()?;

        // TODO: zoom level from parameters
        slice_to_tiles(&parcel.entity, min_zoom, max_zoom, |(z, x, y), feature| {
            feedback.ensure_not_canceled()?;

            if let Value::Object(obj) = &parcel.entity.root {
                let bytes = bincode::serialize(&feature).unwrap();
                let serialized_feature = SerializedSlicedFeature {
                    tile_id: tile_id_conv.zxy_to_id(z, x, y),
                    typename: obj.typename.to_string(),
                    body: bytes,
                };
                if sender_sliced.send(serialized_feature).is_err() {
                    return Err(PipelineError::Canceled);
                };
            }

            Ok(())
        })
    })?;

    Ok(())
}

fn feature_sorting_stage(
    feedback: &Feedback,
    receiver_sliced: mpsc::Receiver<SerializedSlicedFeature>,
    sender_sorted: mpsc::SyncSender<((u64, String), Vec<SerializedSlicedFeature>)>,
) -> Result<()> {
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
            // sort by tile_id and typename
            match a.tile_id.cmp(&b.tile_id) {
                Ordering::Equal => a.typename.cmp(&b.typename),
                ord => ord,
            }
        })
        .unwrap();

    for ((tile_id, typename), ser_feats) in &sorted
        .map(std::result::Result::unwrap)
        .group_by(|ser_feat| (ser_feat.tile_id, ser_feat.typename.clone()))
    {
        feedback.ensure_not_canceled()?;

        let ser_feats: Vec<_> = ser_feats.collect();
        if sender_sorted
            .send(((tile_id, typename), ser_feats))
            .is_err()
        {
            return Err(PipelineError::Canceled);
        };
    }

    Ok(())
}

fn tile_writing_stage(
    output_path: &Path,
    feedback: &Feedback,
    receiver_sorted: mpsc::Receiver<((u64, String), Vec<SerializedSlicedFeature>)>,
    tile_id_conv: TileIdMethod,
    _min_zoom: u8,
    _max_zoom: u8,
) -> Result<()> {
    let ellipsoid = nusamai_projection::ellipsoid::wgs84();
    let contents: Arc<Mutex<Vec<TileContent>>> = Default::default();

    // Make a glTF (.glb) file for each tile
    receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|((tile_id, typename), serialized_feats)| {
            feedback.ensure_not_canceled()?;

            // Tile information
            let (mut content, translation) = {
                let zxy = tile_id_conv.id_to_zxy(tile_id);
                let (tile_zoom, tile_x, tile_y) = zxy;
                let (min_lat, max_lat) = tiling::y_slice_range(tile_zoom, tile_y);
                let (min_lng, max_lng) = tiling::x_slice_range(tile_zoom, tile_x as i32, tiling::x_step(tile_zoom, tile_y));
                // Use the tile center as the translation of the glTF mesh
                let translation = {
                    let (tx, ty, tz) = geographic_to_geocentric(&ellipsoid, (min_lng + max_lng) / 2.0, (min_lat + max_lat) / 2.0, 0.);
                    [tx, tz, -ty]
                };
                let geom_error = tiling::geometric_error(tile_zoom, tile_y);
                log::info!(
                    "tile: z={tile_zoom}, x={tile_x}, y={tile_y} (lng: [{min_lng} => {max_lng}], lat: [{min_lat} => {max_lat}) geometricError: {geom_error}"
                );
                let content_path = {
                    let normalized_typename = typename.replace(':', "_");
                    format!("{tile_zoom}/{tile_x}/{tile_y}_{normalized_typename}.glb")
                };
                let content = TileContent {
                    zxy,
                    content_path,
                    min_lng: f64::MAX,
                    max_lng: f64::MIN,
                    min_lat: f64::MAX,
                    max_lat: f64::MIN,
                    min_height: f64::MAX,
                    max_height: f64::MIN,
                };

                (content, translation)
            };

            // Triangulation
            let mut earcutter = Earcut::new();
            let mut buf2d: Vec<f64> = Vec::new(); // 2d-projected [x, y]
            let mut index_buf: Vec<u32> = Vec::new();

            let mut vertices: IndexSet<[u32; 9], RandomState> = IndexSet::default(); // [x, y, z, u, v, feature_id]
            let mut primitives: gltf::Primitives = Default::default();

            // make vertices and indices
            let mut feature_id = 0;
            for serialized_feat in serialized_feats.into_iter() {
                feedback.ensure_not_canceled()?;

                let mut feature: SlicedFeature = bincode::deserialize(&serialized_feat.body)
                    .map_err(|err| {
                        PipelineError::Other(format!(
                            "Failed to deserialize a sliced feature: {:?}",
                            err
                        ))
                    })?;

                {
                    feature
                        .polygons
                        .transform_inplace(|&[lng, lat, height, u, v]| {
                            // tile boundary
                            content.min_lng = content.min_lng.min(lng);
                            content.max_lng = content.max_lng.max(lng);
                            content.min_lat = content.min_lat.min(lat);
                            content.max_lat = content.max_lat.max(lat);
                            content.min_height = content.min_height.min(height);
                            content.max_height = content.max_height.max(height);

                            let (x, y, z) = geographic_to_geocentric(&ellipsoid, lng, lat, height);
                            [x - translation[0], z - translation[1], -y - translation[2], u, 1.0 - v]
                        });
                }

                for (poly, orig_mat_id) in feature.polygons.iter().zip_eq(feature.polygon_material_ids.iter()) {
                    let num_outer = match poly.hole_indices().first() {
                        Some(&v) => v as usize,
                        None => poly.coords().len() / 5,
                    };

                    let mat = feature.materials[*orig_mat_id as usize].clone();
                    let primitive = primitives.entry(mat).or_default();
                    primitive.feature_ids.insert(feature_id as u32);

                    if let Some((nx, ny, nz)) = calculate_normal(poly.exterior().coords(), 5) {
                        if project3d_to_2d(poly.coords(), num_outer, 5, &mut buf2d) {
                            // earcut
                            earcutter.earcut(&buf2d, poly.hole_indices(), 2, &mut index_buf);

                            // collect triangles
                            primitive.indices.extend(index_buf.iter().map(|idx| {
                                let pos = *idx as usize * 5;
                                let [x, y, z, u, v] = poly.coords()[pos..pos + 5].try_into().unwrap();
                                let vbits = [
                                    (x as f32).to_bits(),
                                    (y as f32).to_bits(),
                                    (z as f32).to_bits(),
                                    (nx as f32).to_bits(),
                                    (ny as f32).to_bits(),
                                    (nz as f32).to_bits(),
                                    (u as f32).to_bits(),
                                    (v as f32).to_bits(),
                                    (feature_id as f32).to_bits(), // UNSIGNED_INT can't be used for vertex attribute
                                ];
                                let (index, _) = vertices.insert_full(vbits);
                                index as u32
                            }));
                        }
                    }
                }

                feature_id += 1;
            }

            // write to file
            let path_glb = output_path.join(Path::new(&content.content_path));
            if let Some(dir) = path_glb.parent() {
                fs::create_dir_all(dir)?;
            }

            contents.lock().unwrap().push(content);

            let mut file = std::fs::File::create(path_glb)?;
            write_gltf_glb(
                &mut BufWriter::new(&mut file),
                translation,
                vertices,
                primitives,
                feature_id, // number of features
            )?;

            Ok::<(), PipelineError>(())
        })?;

    feedback.ensure_not_canceled()?;

    // Generate tileset.json
    let mut tree = TileTree::default();
    for content in contents.lock().unwrap().drain(..) {
        tree.add_content(content);
    }

    let tileset = nusamai_3dtiles_json::tileset::Tileset {
        asset: nusamai_3dtiles_json::tileset::Asset {
            version: "1.1".to_string(),
            ..Default::default()
        },
        root: tree.into_tileset_root(),
        geometric_error: 1e+100,
        ..Default::default()
    };

    let root_tileset_path = output_path.join(Path::new("tileset.json"));
    fs::write(
        root_tileset_path,
        serde_json::to_string_pretty(&tileset).unwrap(),
    )?;

    Ok(())
}
