//! 3D Tiles sink

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

use crate::sink::mvt::tileid::TileIdMethod;
use ahash::RandomState;
use atlas_packer::{
    export::{AtlasExporter as _, WebpAtlasExporter},
    pack::AtlasPacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::{
        cache::{TextureCache, TextureSizeCache},
        DownsampleFactor, PolygonMappedTexture,
    },
};
use bytemuck::Zeroable;
use earcut::{utils3d::project3d_to_2d, Earcut};
use gltf::write_gltf_glb;
use indexmap::IndexSet;
use itertools::Itertools;
use nusamai_citygml::{object::Value, schema::Schema};
use nusamai_projection::cartesian::geodetic_to_geocentric;
use rayon::prelude::*;
use slice::{slice_to_tiles, SlicedFeature};
use tempfile::tempdir;
use tiling::{TileContent, TileTree};
use url::Url;

use crate::{
    get_parameter_value,
    option::use_textured_lod_config,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer::TransformerRegistry,
};
use utils::calculate_normal;

use super::texture_resolution::get_texture_downsample_scale_of_polygon;
use super::{
    option::{limit_texture_resolution_parameter, output_parameter},
    texture_resolution::apply_downsample_factor,
};

pub struct CesiumTilesSinkProvider {}

impl DataSinkProvider for CesiumTilesSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "3dtiles".to_string(),
            name: "Cesium 3D Tiles".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());
        params.define(ParameterDefinition {
            key: "min_z".into(),
            entry: ParameterEntry {
                description: "Minumum zoom level".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: Some(15),
                    min: Some(0),
                    max: Some(22),
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
                    value: Some(18),
                    min: Some(0),
                    max: Some(22),
                }),
                label: Some("最大ズームレベル".into()),
            },
        });
        params.define(limit_texture_resolution_parameter(false));
        params.define(ParameterDefinition {
            key: "gzip".into(),
            entry: ParameterEntry {
                description: "gzip compress".into(),
                required: false,
                parameter: ParameterType::Boolean(BooleanParameter { value: Some(false) }),
                label: Some("gzipで圧縮する".into()),
            },
        });

        params
    }

    fn transformer_options(&self) -> TransformerRegistry {
        let mut settings: TransformerRegistry = TransformerRegistry::new();
        settings.insert(use_textured_lod_config("max_lod"));

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let min_z = get_parameter_value!(params, "min_z", Integer).unwrap() as u8;
        let max_z = get_parameter_value!(params, "max_z", Integer).unwrap() as u8;
        let limit_texture_resolution =
            *get_parameter_value!(params, "limit_texture_resolution", Boolean);
        let gzip_compress = *get_parameter_value!(params, "gzip", Boolean);
        let transform_settings = self.transformer_options();

        Box::<CesiumTilesSink>::new(CesiumTilesSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings,
            limit_texture_resolution,
            gzip_compress,
            min_z,
            max_z,
        })
    }
}

struct CesiumTilesSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
    limit_texture_resolution: Option<bool>,
    gzip_compress: Option<bool>,
    min_z: u8,
    max_z: u8,
}

impl DataSink for CesiumTilesSink {
    fn make_requirements(&mut self, properties: TransformerRegistry) -> DataRequirements {
        let default_requirements = DataRequirements {
            resolve_appearance: true,
            key_value: crate::transformer::KeyValueSpec::JsonifyObjects,
            ..Default::default()
        };

        for config in properties.configs.iter() {
            let _ = &self.transform_settings.update_transformer(config.clone());
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let (sender_sliced, receiver_sliced) = mpsc::sync_channel(2000);
        let (sender_sorted, receiver_sorted) = mpsc::sync_channel(2000);

        let tile_id_conv = TileIdMethod::Hilbert;

        let min_zoom = self.min_z;
        let max_zoom = self.max_z;

        let limit_texture_resolution = self.limit_texture_resolution;
        let gzip_compress = self.gzip_compress;

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
                            limit_texture_resolution,
                            gzip_compress,
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
    limit_texture_resolution: Option<bool>,
    gzip_compress: Option<bool>,
) -> Result<()> {
    let ellipsoid = nusamai_projection::ellipsoid::wgs84();
    let contents: Arc<Mutex<Vec<TileContent>>> = Default::default();
    let bincode_config = bincode::config::standard();

    // Texture cache
    // use default cache size
    let texture_cache = TextureCache::new(200_000_000);
    let texture_size_cache = TextureSizeCache::new();

    // Use a temporary directory for embedding in glb.
    let binding = tempdir().unwrap();
    let folder_path = binding.path();
    let texture_folder_name = "textures";
    let atlas_dir = folder_path.join(texture_folder_name);
    std::fs::create_dir_all(&atlas_dir)?;

    // Make a glTF (.glb) file for each tile
    receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|(tile_id, typename, feats)| {
            feedback.ensure_not_canceled()?;
            let (tile_zoom, tile_x, tile_y) = tile_id_conv.id_to_zxy(tile_id);

            // Tile information
            let (mut content, translation) = {
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
                     lat: [{min_lat} => {max_lat}] geometricError: {geom_error}"
                ));
                let content_path = {
                    let normalized_typename = typename.replace(':', "_");
                    format!("{tile_zoom}/{tile_x}/{tile_y}_{normalized_typename}.glb")
                };
                let content = TileContent {
                    zxy: (tile_zoom, tile_x, tile_y),
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

            let mut vertices: IndexSet<[u32; 9], RandomState> = IndexSet::default(); // [x, y, z, u, v, feature_id]
            let mut primitives: gltf::Primitives = Default::default();

            let mut metadata_encoder = metadata::MetadataEncoder::new(schema);

            let packer = Mutex::new(AtlasPacker::default());

            // transform features
            let features = {
                let mut features = Vec::new();
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
                                // - The origin of atlas-packer is in the lower right.
                                let (x, y, z) =
                                    geodetic_to_geocentric(&ellipsoid, lng, lat, height);
                                [
                                    x - translation[0],
                                    z - translation[1],
                                    -y - translation[2],
                                    u,
                                    v,
                                ]
                            });

                        feature
                    };
                    features.push(feature);
                }
                features
            };

            // metadata encoding
            let features = features
                .iter()
                .filter(|feature| {
                    if metadata_encoder
                        .add_feature(&typename, &feature.attributes)
                        .is_err()
                    {
                        feedback.warn("Failed to encode feature attributes".to_string());
                        false
                    } else {
                        true
                    }
                })
                .collect::<Vec<_>>();

            // A unique ID used when planning the atlas layout
            //  and when obtaining the UV coordinates after the layout has been completed
            let generate_texture_id = |z, x, y, feature_id, poly_count| {
                format!("{}_{}_{}_{}_{}", z, x, y, feature_id, poly_count)
            };

            // Check the size of all the textures and calculate the power of 2 of the largest size
            let mut max_width = 0;
            let mut max_height = 0;

            // Load all textures into the Packer
            for (feature_id, feature) in features.iter().enumerate() {
                for (poly_count, (mat, poly)) in feature
                    .polygons
                    .iter()
                    .zip_eq(feature.polygon_material_ids.iter())
                    .map(move |(poly, orig_mat_id)| {
                        (feature.materials[*orig_mat_id as usize].clone(), poly)
                    })
                    .enumerate()
                {
                    let t = mat.base_texture.clone();
                    if let Some(base_texture) = t {
                        // texture packing
                        let original_vertices = poly
                            .raw_coords()
                            .iter()
                            .map(|[x, y, z, u, v]| (*x, *y, *z, *u, *v))
                            .collect::<Vec<(f64, f64, f64, f64, f64)>>();

                        let uv_coords = original_vertices
                            .iter()
                            .map(|(_, _, _, u, v)| (*u, *v))
                            .collect::<Vec<(f64, f64)>>();

                        let texture_uri = base_texture.uri.to_file_path().unwrap();
                        let texture_size = texture_size_cache.get_or_insert(&texture_uri);

                        let downsample_scale = if limit_texture_resolution.unwrap_or(false) {
                            get_texture_downsample_scale_of_polygon(
                                &original_vertices,
                                texture_size,
                            ) as f32
                        } else {
                            1.0
                        };

                        let geom_error = tiling::geometric_error(tile_zoom, tile_y);
                        let factor = apply_downsample_factor(geom_error, downsample_scale as f32);
                        let downsample_factor = DownsampleFactor::new(&factor);
                        let cropped_texture = PolygonMappedTexture::new(
                            &texture_uri,
                            texture_size,
                            &uv_coords,
                            downsample_factor,
                        );

                        let scaled_width = (texture_size.0 as f32 * factor) as u32;
                        let scaled_height = (texture_size.1 as f32 * factor) as u32;

                        max_width = max_width.max(scaled_width);
                        max_height = max_height.max(scaled_height);

                        // Unique id required for placement in atlas
                        let (z, x, y) = tile_id_conv.id_to_zxy(tile_id);
                        let texture_id = generate_texture_id(z, x, y, feature_id, poly_count);

                        packer
                            .lock()
                            .unwrap()
                            .add_texture(texture_id, cropped_texture);
                    }
                }
            }

            let max_width = max_width.next_power_of_two();
            let max_height = max_height.next_power_of_two();

            // initialize texture packer
            // To reduce unnecessary draw calls, set the lower limit for max_width and max_height to 1024
            let config = TexturePlacerConfig {
                width: max_width.max(1024),
                height: max_height.max(1024),
                padding: 0,
            };

            let placer = GuillotineTexturePlacer::new(config.clone());
            let packer = packer.into_inner().unwrap();

            // Packing the loaded textures into an atlas
            let packed = packer.pack(placer);

            let exporter = WebpAtlasExporter::default();
            let ext = exporter.clone().get_extension().to_string();

            // Obtain the UV coordinates placed in the atlas by specifying the ID
            //  and apply them to the original polygon.
            for (feature_id, feature) in features.iter().enumerate() {
                for (poly_count, (mut mat, mut poly)) in feature
                    .polygons
                    .iter()
                    .zip_eq(feature.polygon_material_ids.iter())
                    .map(move |(poly, orig_mat_id)| {
                        (feature.materials[*orig_mat_id as usize].clone(), poly)
                    })
                    .enumerate()
                {
                    let original_vertices = poly
                        .raw_coords()
                        .iter()
                        .map(|[x, y, z, u, v]| (*x, *y, *z, *u, *v))
                        .collect::<Vec<(f64, f64, f64, f64, f64)>>();

                    let (z, x, y) = tile_id_conv.id_to_zxy(tile_id);
                    let texture_id = generate_texture_id(z, x, y, feature_id, poly_count);

                    if let Some(info) = packed.get_texture_info(&texture_id) {
                        // Place the texture in the atlas
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

                        // Apply the UV coordinates placed in the atlas to the original polygon
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

                        let atlas_file_name = info.atlas_id.to_string();

                        let atlas_uri = atlas_dir
                            .join(format!("{}/{}/{}/{}", z, x, y, atlas_file_name))
                            .with_extension(ext.clone());

                        // update material
                        mat = material::Material {
                            base_color: mat.base_color,
                            base_texture: Some(material::Texture {
                                uri: Url::from_file_path(atlas_uri).unwrap(),
                            }),
                        };
                    }

                    let primitive = primitives.entry(mat).or_default();
                    primitive.feature_ids.insert(feature_id as u32);

                    if let Some((nx, ny, nz)) =
                        calculate_normal(poly.exterior().iter().map(|v| [v[0], v[1], v[2]]))
                    {
                        let num_outer_points = match poly.hole_indices().first() {
                            Some(&v) => v as usize,
                            None => poly.raw_coords().len(),
                        };
                        let mut earcutter = Earcut::new();
                        let mut buf3d: Vec<[f64; 3]> = Vec::new();
                        let mut buf2d: Vec<[f64; 2]> = Vec::new();
                        let mut index_buf: Vec<u32> = Vec::new();

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
                                    // flip the texture v-coordinate
                                    ((1.0 - v) as f32).to_bits(),
                                    (feature_id as f32).to_bits(), // UNSIGNED_INT can't be used for vertex attribute
                                ];
                                let (index, _) = vertices.insert_full(vbits);
                                index as u32
                            }));
                        }
                    }
                }
            }

            // Write to atlas
            let (z, x, y) = tile_id_conv.id_to_zxy(tile_id);
            let atlas_path = atlas_dir.join(format!("{}/{}/{}", z, x, y));
            fs::create_dir_all(&atlas_path)?;
            packed.export(
                exporter,
                &atlas_path,
                &texture_cache,
                config.width,
                config.height,
            );

            // Write to file
            let path_glb = output_path.join(Path::new(&content.content_path));
            if let Some(dir) = path_glb.parent() {
                fs::create_dir_all(dir)?;
            }

            contents.lock().unwrap().push(content);

            let mut file = std::fs::File::create(path_glb)?;
            write_gltf_glb(
                feedback,
                &mut BufWriter::new(&mut file),
                translation,
                vertices,
                primitives,
                features.len(),
                metadata_encoder,
                gzip_compress.unwrap_or_default(),
            )?;

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
