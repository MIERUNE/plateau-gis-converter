//! 3D Tiles PoC sink

mod gltf;
mod material;
pub(crate) mod metadata;
mod slice;
mod tiling;
pub(crate) mod utils;

use std::{
    convert::Infallible,
    fs,
    io::BufWriter,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, Mutex},
};

use ahash::RandomState;
use bytemuck::Zeroable;
use earcut::{utils3d::project3d_to_2d, Earcut};
use gltf::write_gltf_glb;
use hashbrown::HashMap;
use indexmap::IndexSet;
use itertools::Itertools;
use rayon::prelude::*;
use slice::{slice_to_tiles, SlicedFeature};
use tempfile::tempdir;
use tiling::{TileContent, TileTree};
use url::Url;

use nusamai_atlas::{
    export::{AtlasExporter as _, PngAtlasExporter},
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::{self, DownsampleFactor, TextureCache},
};
use nusamai_citygml::{object::Value, schema::Schema};
use nusamai_mvt::tileid::TileIdMethod;
use nusamai_projection::cartesian::geodetic_to_geocentric;

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
    transformer::{TransformerConfig, TransformerOption, TransformerRegistry},
};
use utils::calculate_normal;

pub struct CesiumTilesPocSinkProvider {}

impl DataSinkProvider for CesiumTilesPocSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "3dtiles_poc".to_string(),
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
                label: None,
            },
        );
        // TODO: min Zoom
        // TODO: max Zoom

        params.define(
            "transform".into(),
            ParameterEntry {
                description: "transform option".into(),
                required: false,
                parameter: ParameterType::String(StringParameter { value: None }),
                label: None,
            },
        );
        params
    }

    fn available_transformer(&self) -> TransformerRegistry {
        let mut settings: TransformerRegistry = TransformerRegistry::new();

        settings.insert(TransformerConfig {
            key: "use_texture".to_string(),
            label: "テクスチャの使用".to_string(),
            is_enabled: false,
            requirements: vec![transformer::Requirement::UseAppearance],
        });

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transformer_registry = self.available_transformer();

        Box::<CesiumTilesPocSink>::new(CesiumTilesPocSink {
            output_path: output_path.as_ref().unwrap().into(),
            transformer_registry,
        })
    }
}

struct CesiumTilesPocSink {
    output_path: PathBuf,
    transformer_registry: TransformerRegistry,
}

impl DataSink for CesiumTilesPocSink {
    fn make_requirements(&mut self, properties: Vec<TransformerOption>) -> DataRequirements {
        let default_requirements = DataRequirements {
            resolve_appearance: true,
            key_value: crate::transformer::KeyValueSpec::JsonifyObjects,
            ..Default::default()
        };

        for prop in properties {
            let _ = &self
                .transformer_registry
                .update_transformer(&prop.key, prop.is_enabled);
        }

        self.transformer_registry.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
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
                        feedback.fatal_error(error);
                    }
                });
            }

            // Sort features by tile_id (using external sorter)
            {
                s.spawn(move || {
                    if let Err(error) =
                        feature_sorting_stage(feedback, receiver_sliced, sender_sorted)
                    {
                        feedback.fatal_error(error);
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
                            schema,
                        ) {
                            feedback.fatal_error(error);
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
    sender_sliced: mpsc::SyncSender<(u64, String, Vec<u8>)>,
    min_zoom: u8,
    max_zoom: u8,
) -> Result<()> {
    let bincode_config = bincode::config::standard();

    // Convert CityObjects to sliced features
    upstream.into_iter().par_bridge().try_for_each(|parcel| {
        feedback.ensure_not_canceled()?;

        // TODO: zoom level from parameters
        slice_to_tiles(&parcel.entity, min_zoom, max_zoom, |(z, x, y), feature| {
            feedback.ensure_not_canceled()?;

            if let Value::Object(obj) = &parcel.entity.root {
                let bytes = bincode::serde::encode_to_vec(&feature, bincode_config).unwrap();
                let serialized_feature = (
                    tile_id_conv.zxy_to_id(z, x, y),
                    obj.typename.to_string(),
                    bytes,
                );
                if sender_sliced.send(serialized_feature).is_err() {
                    return Err(PipelineError::Canceled);
                };
            }

            Ok(())
        })
    })?;

    Ok(())
}

#[derive(
    bytemuck::Pod, bytemuck::Zeroable, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, std::fmt::Debug,
)]
#[repr(C)]
struct SortKey {
    tile_id: u64,
    type_seq: u64,
}

fn feature_sorting_stage(
    feedback: &Feedback,
    receiver_sliced: mpsc::Receiver<(u64, String, Vec<u8>)>,
    sender_sorted: mpsc::SyncSender<(u64, String, Vec<Vec<u8>>)>,
) -> Result<()> {
    let mut typename_to_seq: IndexSet<String, ahash::RandomState> = Default::default();

    let config = kv_extsort::SortConfig::default()
        .max_chunk_bytes(256 * 1024 * 1024) // TODO: Configurable
        .set_cancel_flag(feedback.get_cancellation_flag());

    let sorted_iter = kv_extsort::sort(
        receiver_sliced
            .into_iter()
            .map(|(tile_id, typename, body)| {
                let (idx, _) = typename_to_seq.insert_full(typename);
                let type_seq = idx as u64;
                std::result::Result::<_, Infallible>::Ok((SortKey { tile_id, type_seq }, body))
            }),
        config,
    );

    for ((_, key), grouped) in &sorted_iter.chunk_by(|feat| match feat {
        Ok((key, _)) => (false, *key),
        Err(_) => (true, SortKey::zeroed()),
    }) {
        let grouped = grouped
            .into_iter()
            .map_ok(|(_, serialized_feats)| serialized_feats)
            .collect::<kv_extsort::Result<Vec<_>, _>>();
        match grouped {
            Ok(serialized_feats) => {
                feedback.ensure_not_canceled()?;
                let tile_id = key.tile_id;
                let typename = typename_to_seq[key.type_seq as usize].clone();
                if sender_sorted
                    .send((tile_id, typename, serialized_feats))
                    .is_err()
                {
                    return Err(PipelineError::Canceled);
                }
            }
            Err(kv_extsort::Error::Canceled) => {
                return Err(PipelineError::Canceled);
            }
            Err(err) => {
                return Err(PipelineError::Other(format!(
                    "Failed to sort features: {:?}",
                    err
                )));
            }
        }
    }

    Ok(())
}

fn tile_writing_stage(
    output_path: &Path,
    feedback: &Feedback,
    receiver_sorted: mpsc::Receiver<(u64, String, Vec<Vec<u8>>)>,
    tile_id_conv: TileIdMethod,
    schema: &Schema,
) -> Result<()> {
    let ellipsoid = nusamai_projection::ellipsoid::wgs84();
    let contents: Arc<Mutex<Vec<TileContent>>> = Default::default();
    let bincode_config = bincode::config::standard();

    // Texture cache
    let texture_cache = TextureCache::new(1_000_000_000);
    // Use a temporary directory for embedding in glb.
    let atlas_dir = tempdir()?;

    // Make a glTF (.glb) file for each tile
    receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|(tile_id, typename, feats)| {
            feedback.ensure_not_canceled()?;

            // Tile information
            let (mut content, translation) = {
                let zxy = tile_id_conv.id_to_zxy(tile_id);
                let (tile_zoom, tile_x, tile_y) = zxy;
                let (min_lat, max_lat) = tiling::y_slice_range(tile_zoom, tile_y);
                let (min_lng, max_lng) = tiling::x_slice_range(
                    tile_zoom,
                    tile_x as i32,
                    tiling::x_step(tile_zoom, tile_y),
                );

                // Use the tile center as the translation of the glTF mesh
                let translation = {
                    let (tx, ty, tz) = geodetic_to_geocentric(
                        &ellipsoid,
                        (min_lng + max_lng) / 2.0,
                        (min_lat + max_lat) / 2.0,
                        0.,
                    );
                    // z-up to y-up
                    let [tx, ty, tz] = [tx, tz, -ty];
                    // double-precision to single-precision
                    [(tx as f32) as f64, (ty as f32) as f64, (tz as f32) as f64]
                };

                let geom_error = tiling::geometric_error(tile_zoom, tile_y);
                feedback.info(format!(
                    "tile: z={tile_zoom}, x={tile_x}, y={tile_y} (lng: [{min_lng} => {max_lng}], \
                    lat: [{min_lat} => {max_lat}) geometricError: {geom_error}"
                ));
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

            let mut earcutter = Earcut::new();
            let mut buf3d: Vec<[f64; 3]> = Vec::new();
            let mut buf2d: Vec<[f64; 2]> = Vec::new(); // 2d-projected [x, y]
            let mut index_buf: Vec<u32> = Vec::new();

            // [x, y, z, nx, ny, nz, u, v, feature_id]
            let mut vertices: IndexSet<[u32; 9], RandomState> = IndexSet::default();
            let mut primitives: gltf::Primitives = Default::default();

            let mut metadata_encoder = metadata::MetadataEncoder::new(schema);

            // initialize texture packer
            let config = TexturePlacerConfig {
                width: 4096,
                height: 4096,
                padding: 0,
            };
            let placer = GuillotineTexturePlacer::new(config.clone());
            let exporter = PngAtlasExporter::default();
            let ext = exporter.clone().get_extension().to_string();
            let mut packer = TexturePacker::new(placer, exporter);

            type Vertex = Vec<(f64, f64, f64, f64, f64, f64, f64, f64)>;
            type BaseColor = [f32; 4];
            let mut atlas_files: HashMap<String, (Vertex, BaseColor)> = HashMap::new();

            // For each feature
            let mut feature_id = 0;
            for serialized_feat in feats.into_iter() {
                feedback.ensure_not_canceled()?;

                let feature = {
                    let (mut feature, _): (SlicedFeature, _) =
                        bincode::serde::decode_from_slice(&serialized_feat, bincode_config)
                            .map_err(|err| {
                                PipelineError::Other(format!(
                                    "Failed to deserialize a sliced feature: {:?}",
                                    err
                                ))
                            })?;

                    feature
                        .polygons
                        .transform_inplace(|&[lng, lat, height, u, v]| {
                            // Update tile boundary
                            content.min_lng = content.min_lng.min(lng);
                            content.max_lng = content.max_lng.max(lng);
                            content.min_lat = content.min_lat.min(lat);
                            content.max_lat = content.max_lat.max(lat);
                            content.min_height = content.min_height.min(height);
                            content.max_height = content.max_height.max(height);

                            // Coordinate transformation
                            // - geographic to geocentric
                            // - z-up to y-up
                            // - subtract the translation
                            // - flip the texture v-coordinate
                            let (x, y, z) = geodetic_to_geocentric(&ellipsoid, lng, lat, height);
                            [
                                x - translation[0],
                                z - translation[1],
                                -y - translation[2],
                                u,
                                1.0 - v,
                            ]
                        });

                    feature
                };

                // Encode properties
                if metadata_encoder
                    .add_feature(&typename, &feature.attributes)
                    .is_err()
                {
                    feedback.warn("Failed to encode feature attributes".to_string());
                    continue;
                }

                // Triangulation, etc.
                for (poly_count, (mut poly, orig_mat_id)) in feature
                    .polygons
                    .iter()
                    .zip_eq(feature.polygon_material_ids.iter())
                    .enumerate()
                {
                    let mat = &feature.materials[*orig_mat_id as usize];
                    let t = mat.base_texture.clone();
                    match t {
                        Some(base_texture) => {
                            // textured
                            let original_vertices = poly
                                .raw_coords()
                                .iter()
                                .map(|[x, y, z, u, v]| (*x, *y, *z, *u, *v))
                                .collect::<Vec<(f64, f64, f64, f64, f64)>>();

                            let texture = texture_cache.get_or_insert(
                                &original_vertices
                                    .iter()
                                    .map(|(_, _, _, u, v)| (*u, *v))
                                    .collect::<Vec<_>>(),
                                &base_texture.uri.to_file_path().unwrap(),
                                &DownsampleFactor::new(&1.0).value(),
                            );

                            // Unique id required for placement in atlas
                            let texture_id = format!("{}_{}_{}", tile_id, feature_id, poly_count);
                            let info = packer.add_texture(texture_id, texture);

                            let atlas_placed_uv_coords = info
                                .placed_uv_coords
                                .iter()
                                .map(|(u, v)| ({ *u }, { *v }))
                                .collect::<Vec<(f64, f64)>>();
                            let updated_vertices = original_vertices
                                .iter()
                                .zip(atlas_placed_uv_coords.iter())
                                .map(|((x, y, z, _, _), (u, v))| (*x, *y, *z, *u, *v))
                                .collect::<Vec<(f64, f64, f64, f64, f64)>>();

                            // update_verticesを利用して、polyのtransform_inplaceメソッドで頂点を更新する
                            poly.transform_inplace(|&[x, y, z, _, _]| {
                                let (u, v) = updated_vertices
                                    .iter()
                                    .find(|(x_, y_, z_, _, _)| {
                                        (*x_ - x).abs() < 1e-6
                                            && (*y_ - y).abs() < 1e-6
                                            && (*z_ - z).abs() < 1e-6
                                    })
                                    .map(|(_, _, _, u, v)| (*u, *v))
                                    .unwrap();
                                [x, y, z, u, v]
                            });

                            let atlas_file_name = format!("{}/{}", tile_id, &info.atlas_id);

                            let atlas_uri = atlas_dir
                                .path()
                                .join(atlas_file_name)
                                .with_extension(ext.clone());

                            // update material
                            let mat = material::Material {
                                base_color: mat.base_color,
                                base_texture: Some(material::Texture {
                                    uri: Url::from_file_path(atlas_uri).unwrap(),
                                }),
                            };
                        }
                        None => {}
                    }

                    let num_outer_points = match poly.hole_indices().first() {
                        Some(&v) => v as usize,
                        None => poly.raw_coords().len(),
                    };

                    let primitive = primitives.entry(mat.clone()).or_default();
                    primitive.feature_ids.insert(feature_id as u32);

                    if let Some((nx, ny, nz)) =
                        calculate_normal(poly.exterior().iter().map(|v| [v[0], v[1], v[2]]))
                    {
                        buf3d.clear();
                        buf3d.extend(poly.raw_coords().iter().map(|c| [c[0], c[1], c[2]]));

                        if project3d_to_2d(&buf3d, num_outer_points, &mut buf2d) {
                            // earcut
                            earcutter.earcut(
                                buf2d.iter().cloned(),
                                poly.hole_indices(),
                                &mut index_buf,
                            );

                            // collect triangles
                            primitive.indices.extend(index_buf.iter().map(|&idx| {
                                let [x, y, z, u, v] = poly.raw_coords()[idx as usize];
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

                    // // ----------------------------------------
                    // let num_outer_points = match poly.hole_indices().first() {
                    //     Some(&v) => v as usize,
                    //     None => poly.raw_coords().len(),
                    // };

                    // let primitive = primitives.entry(mat.clone()).or_default();
                    // primitive.feature_ids.insert(feature_id as u32);

                    // if let Some((nx, ny, nz)) =
                    //     calculate_normal(poly.exterior().iter().map(|v| [v[0], v[1], v[2]]))
                    // {
                    //     buf3d.clear();
                    //     buf3d.extend(poly.raw_coords().iter().map(|c| [c[0], c[1], c[2]]));

                    //     if project3d_to_2d(&buf3d, num_outer_points, &mut buf2d) {
                    //         // earcut
                    //         earcutter.earcut(
                    //             buf2d.iter().cloned(),
                    //             poly.hole_indices(),
                    //             &mut index_buf,
                    //         );

                    //         // collect triangles
                    //         primitive.indices.extend(index_buf.iter().map(|&idx| {
                    //             let [x, y, z, u, v] = poly.raw_coords()[idx as usize];
                    //             let vbits = [
                    //                 (x as f32).to_bits(),
                    //                 (y as f32).to_bits(),
                    //                 (z as f32).to_bits(),
                    //                 (nx as f32).to_bits(),
                    //                 (ny as f32).to_bits(),
                    //                 (nz as f32).to_bits(),
                    //                 (u as f32).to_bits(),
                    //                 (v as f32).to_bits(),
                    //                 (feature_id as f32).to_bits(), // UNSIGNED_INT can't be used for vertex attribute
                    //             ];
                    //             let (index, _) = vertices.insert_full(vbits);
                    //             index as u32
                    //         }));
                    //     }
                    // }

                    // // FIXME: material handling
                    // // let mat = &feature.materials[*orig_mat_id as usize];
                    // // not textured
                    // let Some(base_texture) = mat.base_texture.clone() else {
                    //     let primitive = primitives.entry(mat.clone()).or_default();
                    //     primitive.feature_ids.insert(feature_id as u32);

                    //     if let Some((nx, ny, nz)) =
                    //         calculate_normal(poly.exterior().iter().map(|v| [v[0], v[1], v[2]]))
                    //     {
                    //         buf3d.clear();
                    //         buf3d.extend(poly.raw_coords().iter().map(|c| [c[0], c[1], c[2]]));

                    //         if project3d_to_2d(&buf3d, num_outer_points, &mut buf2d) {
                    //             // earcut
                    //             earcutter.earcut(
                    //                 buf2d.iter().cloned(),
                    //                 poly.hole_indices(),
                    //                 &mut index_buf,
                    //             );

                    //             // collect triangles
                    //             primitive.indices.extend(index_buf.iter().map(|&idx| {
                    //                 let [x, y, z, u, v] = poly.raw_coords()[idx as usize];
                    //                 let vbits = [
                    //                     (x as f32).to_bits(),
                    //                     (y as f32).to_bits(),
                    //                     (z as f32).to_bits(),
                    //                     (nx as f32).to_bits(),
                    //                     (ny as f32).to_bits(),
                    //                     (nz as f32).to_bits(),
                    //                     (u as f32).to_bits(),
                    //                     (v as f32).to_bits(),
                    //                     (feature_id as f32).to_bits(), // UNSIGNED_INT can't be used for vertex attribute
                    //                 ];
                    //                 let (index, _) = vertices.insert_full(vbits);
                    //                 index as u32
                    //             }));
                    //         }
                    //     }
                    //     continue;
                    // };

                    // // textured
                    // if let Some((nx, ny, nz)) =
                    //     calculate_normal(poly.exterior().iter().map(|v| [v[0], v[1], v[2]]))
                    // {
                    //     buf3d.clear();
                    //     buf3d.extend(poly.raw_coords().iter().map(|c| [c[0], c[1], c[2]]));

                    //     if project3d_to_2d(&buf3d, num_outer_points, &mut buf2d) {
                    //         // earcut
                    //         earcutter.earcut(
                    //             buf2d.iter().cloned(),
                    //             poly.hole_indices(),
                    //             &mut index_buf,
                    //         );

                    //         // adding triangles to the atlas
                    //         for (triangle_count, chunks_idx) in index_buf.chunks(3).enumerate() {
                    //             let original_vertices = chunks_idx
                    //                 .iter()
                    //                 .map(|&idx| {
                    //                     let [x, y, z, u, v] = poly.raw_coords()[idx as usize];
                    //                     (x, y, z, u, v, nx, ny, nz)
                    //                 })
                    //                 .collect::<Vertex>();
                    //             let original_uv_coords = original_vertices
                    //                 .iter()
                    //                 .map(|(_, _, _, u, v, _, _, _)| (*u, *v))
                    //                 .collect::<Vec<(f64, f64)>>();

                    //             let texture = texture_cache.get_or_insert(
                    //                 &original_uv_coords,
                    //                 &base_texture.uri.to_file_path().unwrap(),
                    //                 &DownsampleFactor::new(&1.0).value(),
                    //             );

                    //             // Unique id required for placement in atlas
                    //             let texture_id = format!(
                    //                 "{}_{}_{}_{}",
                    //                 tile_id, feature_id, poly_count, triangle_count
                    //             );
                    //             let info = packer.add_texture(texture_id, texture);

                    //             let atlas_placed_uv_coords = info
                    //                 .placed_uv_coords
                    //                 .iter()
                    //                 .map(|(u, v)| ({ *u }, { *v }))
                    //                 .collect::<Vec<(f64, f64)>>();
                    //             let updated_vertices = original_vertices
                    //                 .iter()
                    //                 .zip(atlas_placed_uv_coords.iter())
                    //                 .map(|((x, y, z, _, _, nx, ny, nz), (u, v))| {
                    //                     (*x, *y, *z, *u, *v, *nx, *ny, *nz)
                    //                 })
                    //                 .collect::<Vertex>();

                    //             let atlas_file_name = format!("{}/{}", tile_id, &info.atlas_id);

                    //             match atlas_files.get_mut(&atlas_file_name) {
                    //                 Some((vertex, _)) => {
                    //                     vertex.extend(updated_vertices);
                    //                 }
                    //                 None => {
                    //                     atlas_files.insert(
                    //                         atlas_file_name,
                    //                         (updated_vertices, mat.base_color),
                    //                     );
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                }

                packer.finalize();

                // for (atlas_file_name, (updated_vertices, base_color)) in atlas_files.iter() {
                //     let atlas_uri = atlas_dir
                //         .path()
                //         .join(atlas_file_name)
                //         .with_extension(ext.clone());

                //     // update material
                //     let new_mat = material::Material {
                //         base_color: *base_color,
                //         base_texture: Some(material::Texture {
                //             uri: Url::from_file_path(atlas_uri).unwrap(),
                //         }),
                //     };

                //     let primitive = primitives.entry(new_mat).or_default();
                //     primitive.feature_ids.insert(feature_id as u32);

                //     // collect triangles
                //     primitive
                //         .indices
                //         .extend(updated_vertices.iter().map(|vertex| {
                //             let (x, y, z, u, v, nx, ny, nz) = vertex;
                //             let vbits = [
                //                 (*x as f32).to_bits(),
                //                 (*y as f32).to_bits(),
                //                 (*z as f32).to_bits(),
                //                 (*nx as f32).to_bits(),
                //                 (*ny as f32).to_bits(),
                //                 (*nz as f32).to_bits(),
                //                 (*u as f32).to_bits(),
                //                 (*v as f32).to_bits(),
                //                 (feature_id as f32).to_bits(), // UNSIGNED_INT can't be used for vertex attribute
                //             ];
                //             let (index, _) = vertices.insert_full(vbits);

                //             index as u32
                //         }));
                // }
            }
            feature_id += 1;

            // Write to atlas
            let atlas_path = atlas_dir.path().join(tile_id.to_string());
            fs::create_dir_all(&atlas_path)?;
            packer.export(&atlas_path, &texture_cache, config.width, config.height);

            // Write to file
            let path_glb = output_path.join(Path::new(&content.content_path));
            fs::create_dir_all(path_glb.parent().unwrap())?;

            contents.lock().unwrap().push(content);

            let mut file = std::fs::File::create(path_glb)?;
            write_gltf_glb(
                feedback,
                &mut BufWriter::new(&mut file),
                translation,
                vertices,
                primitives,
                feature_id, // number of features
                metadata_encoder,
            )
            .unwrap_or_else(|_| panic!("Failed to write glTF {:?}", tile_id));

            Ok::<(), PipelineError>(())
        })?;

    feedback.ensure_not_canceled()?;

    // Generate tileset.json
    let mut tree = TileTree::default();
    for content in contents.lock().unwrap().drain(..) {
        tree.add_content(content);
    }

    let tileset = cesiumtiles::tileset::Tileset {
        asset: cesiumtiles::tileset::Asset {
            version: "1.1".to_string(),
            ..Default::default()
        },
        root: tree.into_tileset_root(),
        geometric_error: 1e+100,
        ..Default::default()
    };

    let root_tileset_path = output_path.join(Path::new("tileset.json"));
    fs::create_dir_all(root_tileset_path.parent().unwrap())?;
    fs::write(
        root_tileset_path,
        serde_json::to_string_pretty(&tileset).unwrap(),
    )?;

    Ok(())
}
