//! gltf sink poc
mod gltf_writer;
mod material;

use std::{f64::consts::FRAC_PI_2, fs::File, io::BufWriter, path::PathBuf, sync::Mutex};

use crate::sink::cesiumtiles::utils::calculate_normal;
use atlas_packer::{
    export::{AtlasExporter as _, JpegAtlasExporter},
    pack::AtlasPacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::{
        cache::{TextureCache, TextureSizeCache},
        DownsampleFactor, PolygonMappedTexture,
    },
};
use earcut::{utils3d::project3d_to_2d, Earcut};
use flatgeom::MultiPolygon;
use foldhash::{fast::RandomState, HashMap, HashSet};
use geocentric::geodetic_to_geocentric;
use glam::{DMat4, DVec3, DVec4};
use gltf_writer::write_gltf_glb;
use indexmap::IndexSet;
use itertools::Itertools;
use material::{Material, Texture};
use nusamai_citygml::{object::ObjectStereotype, schema::Schema, GeometryType, Value};
use nusamai_plateau::appearance;
use nusamai_projection::crs::{self, EpsgCode};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use url::Url;

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{cesiumtiles::metadata, DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer::{use_lod_config, TransformerSettings},
};

use super::option::{
    center_at_origin_parameter, limit_texture_resolution_parameter, output_parameter,
};
use super::texture_resolution::get_texture_downsample_scale_of_polygon;
pub struct GltfSinkProvider {}

impl DataSinkProvider for GltfSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "gltf".to_string(),
            name: "glTF".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());
        params.define(limit_texture_resolution_parameter(false));
        params.define(center_at_origin_parameter(true)); // デフォルトtrue

        params
    }

    fn transformer_options(&self) -> TransformerSettings {
        let mut settings: TransformerSettings = TransformerSettings::new();
        settings.insert(use_lod_config("max_lod", Some(&["textured_max_lod"])));

        settings
    }
    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let limit_texture_resolution =
            *get_parameter_value!(params, "limit_texture_resolution", Boolean);
        let center_at_origin = get_parameter_value!(params, "center_at_origin", Boolean)
            .map(|v| v)
            .unwrap_or(true);
        let transform_settings = self.transformer_options();

        Box::<GltfSink>::new(GltfSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings,
            limit_texture_resolution,
            center_at_origin,
        })
    }
}

pub struct GltfSink {
    output_path: PathBuf,
    transform_settings: TransformerSettings,
    limit_texture_resolution: Option<bool>,
    center_at_origin: bool,
}

pub struct BoundingVolume {
    pub min_x: f64, // lng or x
    pub max_x: f64, // lng or x
    pub min_y: f64, // lat or y
    pub max_y: f64, // lat or y
    pub min_z: f64, // height or z
    pub max_z: f64, // height or z
}

impl BoundingVolume {
    fn update(&mut self, other: &Self) {
        self.min_x = self.min_x.min(other.min_x);
        self.max_x = self.max_x.max(other.max_x);
        self.min_y = self.min_y.min(other.min_y);
        self.max_y = self.max_y.max(other.max_y);
        self.min_z = self.min_z.min(other.min_z);
        self.max_z = self.max_z.max(other.max_z);
    }
}

impl Default for BoundingVolume {
    fn default() -> Self {
        Self {
            min_x: f64::MAX,
            max_x: f64::MIN,
            min_y: f64::MAX,
            max_y: f64::MIN,
            min_z: f64::MAX,
            max_z: f64::MIN,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    // polygons [x, y, z, u, v]
    pub polygons: MultiPolygon<'static, [f64; 5]>,
    // material ids for each polygon
    pub polygon_material_ids: Vec<u32>,
    // materials
    pub materials: IndexSet<Material>,
    // attribute values
    pub attributes: nusamai_citygml::object::Value,
    // feature_id
    pub feature_id: Option<u32>,
}

type ClassifiedFeatures = HashMap<String, ClassFeatures>;

#[derive(Default)]
struct ClassFeatures {
    features: Vec<Feature>,
    bounding_volume: BoundingVolume,
}

#[derive(Default)]
pub struct PrimitiveInfo {
    pub indices: Vec<u32>,
    pub feature_ids: HashSet<u32>,
}

pub type Primitives = HashMap<material::Material, PrimitiveInfo>;

impl DataSink for GltfSink {
    fn make_requirements(&mut self, properties: TransformerSettings) -> DataRequirements {
        let default_requirements: DataRequirements = DataRequirements {
            resolve_appearance: true,
            key_value: crate::transformer::KeyValueSpec::JsonifyObjectsAndArrays,
            ..Default::default()
        };

        for config in properties.configs.iter() {
            let _ = &self.transform_settings.update_transformer(config.clone());
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let ellipsoid = nusamai_projection::ellipsoid::wgs84();

        // 座標系の判定
        let output_epsg = schema.epsg.unwrap_or(crs::EPSG_WGS84_GEOGRAPHIC_3D);
        let use_enu_transform = self.center_at_origin && is_geographic(output_epsg);
        let is_projected = is_japan_plane_rectangular(output_epsg);

        let classified_features: Mutex<ClassifiedFeatures> = Default::default();

        // Construct a Feature classified by typename from Entity
        // Features have polygons, attributes and materials
        // The coordinates of polygon store the actual coordinate values (WGS84) and UV coordinates, not the index.
        let _ = upstream.into_iter().par_bridge().try_for_each(|parcel| {
            feedback.ensure_not_canceled()?;

            let entity = parcel.entity;

            // entity must be a Feature
            let Value::Object(obj) = &entity.root else {
                return Ok(());
            };
            let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype else {
                return Ok(());
            };

            let geom_store = entity.geometry_store.read().unwrap();
            if geom_store.multipolygon.is_empty() {
                return Ok(());
            }
            let appearance_store = entity.appearance_store.read().unwrap();

            let mut materials: IndexSet<Material> = IndexSet::new();
            let default_material = appearance::Material::default();

            let mut feature = Feature {
                polygons: MultiPolygon::new(),
                attributes: entity.root.clone(),
                polygon_material_ids: Default::default(),
                materials: Default::default(),
                feature_id: None, // feature_id is set later
            };

            let mut local_bvol = BoundingVolume::default();

            geometries.iter().for_each(|entry| {
                match entry.ty {
                    GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
                        // extract the polygon, material, and texture
                        for (((idx_poly, poly_uv), poly_mat), poly_tex) in
                            geom_store
                                .multipolygon
                                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
                                .zip_eq(geom_store.polygon_uvs.iter_range(
                                    entry.pos as usize..(entry.pos + entry.len) as usize,
                                ))
                                .zip_eq(
                                    geom_store.polygon_materials
                                        [entry.pos as usize..(entry.pos + entry.len) as usize]
                                        .iter(),
                                )
                                .zip_eq(
                                    geom_store.polygon_textures
                                        [entry.pos as usize..(entry.pos + entry.len) as usize]
                                        .iter(),
                                )
                        {
                            // convert to idx_poly to polygon
                            let poly = idx_poly.transform(|c| geom_store.vertices[*c as usize]);
                            let orig_mat = poly_mat
                                .and_then(|idx| appearance_store.materials.get(idx as usize))
                                .unwrap_or(&default_material)
                                .clone();
                            let orig_tex = poly_tex
                                .and_then(|idx| appearance_store.textures.get(idx as usize));

                            let mat = Material {
                                base_color: orig_mat.diffuse_color.into(),
                                base_texture: orig_tex.map(|tex| Texture {
                                    uri: tex.image_url.clone(),
                                }),
                            };
                            let (mat_idx, _) = materials.insert_full(mat);

                            let mut ring_buffer: Vec<[f64; 5]> = Vec::new();

                            poly.rings().zip_eq(poly_uv.rings()).enumerate().for_each(
                                |(ri, (ring, uv_ring))| {
                                    ring.iter_closed().zip_eq(uv_ring.iter_closed()).for_each(
                                        |(c, uv)| {
                                            let [lng, lat, height] = c;
                                            ring_buffer.push([lng, lat, height, uv[0], uv[1]]);

                                            local_bvol.min_x = local_bvol.min_x.min(lng);
                                            local_bvol.max_x = local_bvol.max_x.max(lng);
                                            local_bvol.min_y = local_bvol.min_y.min(lat);
                                            local_bvol.max_y = local_bvol.max_y.max(lat);
                                            local_bvol.min_z = local_bvol.min_z.min(height);
                                            local_bvol.max_z = local_bvol.max_z.max(height);
                                        },
                                    );
                                    if ri == 0 {
                                        feature.polygons.add_exterior(ring_buffer.drain(..));
                                        feature.polygon_material_ids.push(mat_idx as u32);
                                    } else {
                                        feature.polygons.add_interior(ring_buffer.drain(..));
                                    }
                                },
                            );
                        }
                    }
                    GeometryType::Curve => {
                        // TODO: implement
                    }
                    GeometryType::Point => {
                        // TODO: implement
                    }
                }
            });

            feature.materials = materials;

            {
                let mut locked_features = classified_features.lock().unwrap();
                let feats = locked_features.entry(obj.typename.to_string()).or_default();
                feats.features.push(feature);
                feats.bounding_volume.update(&local_bvol);
            }

            Ok::<(), PipelineError>(())
        });

        let classified_features = classified_features.into_inner().unwrap();

        // Bounding volume for the entire dataset
        let global_bvol = {
            let mut global_bvol = BoundingVolume::default();
            for features in classified_features.values() {
                global_bvol.update(&features.bounding_volume);
            }
            global_bvol
        };

        let tileset_content_files = Mutex::new(Vec::new());

        let transform_matrix = if use_enu_transform {
            // 現行のENU変換処理（地心座標経由）
            let bounds = &global_bvol;
            let center_lng = (bounds.min_x + bounds.max_x) / 2.0;
            let center_lat = (bounds.min_y + bounds.max_y) / 2.0;

            let psi = ((1. - ellipsoid.e_sq()) * center_lat.to_radians().tan()).atan();

            let (tx, ty, tz) =
                geodetic_to_geocentric(ellipsoid.a(), ellipsoid.e_sq(), center_lng, center_lat, 0.);
            let h = (tx * tx + ty * ty + tz * tz).sqrt();

            DMat4::from_translation(DVec3::new(0., -h, 0.))
                * DMat4::from_rotation_x(-(FRAC_PI_2 - psi))
                * DMat4::from_rotation_y((-center_lng - 90.).to_radians())
        } else if is_projected {
            if self.center_at_origin {
                // 平面直角座標系で重心を原点に設定
                let bounds = &global_bvol;
                let center_x = (bounds.min_x + bounds.max_x) / 2.0;
                let center_y = (bounds.min_y + bounds.max_y) / 2.0;
                let center_z = (bounds.min_z + bounds.max_z) / 2.0;
                DMat4::from_translation(DVec3::new(-center_x, -center_z, center_y))
            } else {
                // 平面直角座標系：座標変換なし
                DMat4::IDENTITY
            }
        } else {
            // その他の座標系：単位行列
            DMat4::IDENTITY
        };
        let _ = transform_matrix.inverse();

        let use_enu_transform = use_enu_transform;
        let is_projected = is_projected;
        let center_at_origin = self.center_at_origin;

        classified_features
            .into_par_iter()
            .try_for_each(move |(typename, features)| {
                feedback.ensure_not_canceled()?;

                // The decoded image file is cached
                let texture_cache = TextureCache::new(100_000_000);
                // The image size is cached to avoid unnecessary decoding
                let texture_size_cache = TextureSizeCache::new();

                let mut vertices: IndexSet<[u32; 9], RandomState> = IndexSet::default(); // [x, y, z, nx, ny, nz, u, v, feature_id]
                let mut primitives: Primitives = Default::default();

                let mut metadata_encoder = metadata::MetadataEncoder::new(schema);

                // Use a temporary directory for embedding in glb.
                let binding = tempdir().unwrap();
                let folder_path = binding.path();
                let base_name = typename.replace(':', "_");

                let texture_folder_name = "textures";
                let atlas_dir = folder_path.join(texture_folder_name);
                std::fs::create_dir_all(&atlas_dir)?;

                // Check the size of all the textures and calculate the power of 2 of the largest size
                let mut max_width = 0;
                let mut max_height = 0;
                for feature in features.features.iter() {
                    feedback.ensure_not_canceled()?;

                    for (_, orig_mat_id) in feature
                        .polygons
                        .iter()
                        .zip_eq(feature.polygon_material_ids.iter())
                    {
                        let mat = feature.materials[*orig_mat_id as usize].clone();
                        let t = mat.base_texture.clone();
                        if let Some(base_texture) = t {
                            let texture_uri = base_texture.uri.to_file_path().unwrap();
                            let texture_size = texture_size_cache.get_or_insert(&texture_uri);
                            max_width = max_width.max(texture_size.0);
                            max_height = max_height.max(texture_size.1);
                        }
                    }
                }
                let max_width = max_width.next_power_of_two();
                let max_height = max_height.next_power_of_two();

                // initialize texture packer
                // To reduce unnecessary draw calls, set the lower limit for max_width and max_height to 8192
                let config = TexturePlacerConfig {
                    width: max_width.max(8192),
                    height: max_height.max(8192),
                    padding: 0,
                };

                let packer = Mutex::new(AtlasPacker::default());

                // Transform features
                let features = {
                    let mut features = features.features;
                    features.iter_mut().for_each(|feature| {
                        feature
                            .polygons
                            .transform_inplace(|&[lng, lat, height, u, v]| {
                                if use_enu_transform {
                                    // ENU変換の場合：現行処理を維持
                                    let (x, y, z) = geodetic_to_geocentric(
                                        ellipsoid.a(),
                                        ellipsoid.e_sq(),
                                        lng,
                                        lat,
                                        height,
                                    );
                                    // z-up to y-up
                                    let v_xyz = DVec4::new(x, z, -y, 1.0);
                                    // local ENU coordinate
                                    let v_enu = transform_matrix * v_xyz;
                                    [v_enu[0], v_enu[1], v_enu[2], u, v]
                                } else if is_projected {
                                    // 平面直角座標系の場合
                                    // 入力: x=lng, y=lat, z=height
                                    // Z-up to Y-up変換: [x, z, -y]
                                    let v_xyz = DVec4::new(lng, height, -lat, 1.0);
                                    let v_transformed = transform_matrix * v_xyz;
                                    [v_transformed[0], v_transformed[1], v_transformed[2], u, v]
                                } else {
                                    // その他の座標系
                                    [lng, lat, height, u, v]
                                }
                            });
                    });
                    features
                };

                // Encode metadata
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
                let generate_texture_id =
                    |folder_name: &str, feature_id: usize, poly_count: usize| {
                        format!("{}_{}_{}", folder_name, feature_id, poly_count)
                    };

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

                            let downsample_scale = if self.limit_texture_resolution.unwrap_or(false)
                            {
                                get_texture_downsample_scale_of_polygon(
                                    &original_vertices,
                                    texture_size,
                                ) as f32
                            } else {
                                1.0
                            };

                            let downsample_factor = DownsampleFactor::new(&downsample_scale);

                            let texture = PolygonMappedTexture::new(
                                &texture_uri,
                                texture_size,
                                &uv_coords,
                                downsample_factor,
                            );

                            // Unique id required for placement in atlas

                            let texture_id =
                                generate_texture_id(&base_name, feature_id, poly_count);

                            packer.lock().unwrap().add_texture(texture_id, texture);
                        }
                    }
                }

                let placer = GuillotineTexturePlacer::new(config.clone());
                let packer = packer.into_inner().unwrap();

                // Packing the loaded textures into an atlas
                let packed = packer.pack(placer);

                let exporter = JpegAtlasExporter::default();
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

                        let texture_id = generate_texture_id(&base_name, feature_id, poly_count);

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

                            let atlas_uri =
                                atlas_dir.join(atlas_file_name).with_extension(ext.clone());

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

                // Ensure that the parent directory exists
                std::fs::create_dir_all(&self.output_path)?;

                packed.export(
                    exporter,
                    &atlas_dir,
                    &texture_cache,
                    config.width,
                    config.height,
                );

                // Write glTF (.glb)
                let file_path = {
                    let filename = format!("{}.glb", typename.replace(':', "_"));
                    // Save the filename to the content list of the tileset.json (3D Tiles)
                    tileset_content_files.lock().unwrap().push(filename.clone());

                    self.output_path.join(filename)
                };

                let mut file = File::create(file_path)?;
                let writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                write_gltf_glb(feedback, writer, vertices, primitives, metadata_encoder)?;

                Ok::<(), PipelineError>(())
            })?;

        Ok(())
    }
}

/// Check if the EPSG code represents a geographic coordinate system
fn is_geographic(epsg: EpsgCode) -> bool {
    matches!(
        epsg,
        crs::EPSG_WGS84_GEOGRAPHIC_3D | crs::EPSG_JGD2011_GEOGRAPHIC_3D
    )
}

/// Check if the EPSG code represents a Japan Plane Rectangular coordinate system
fn is_japan_plane_rectangular(epsg: EpsgCode) -> bool {
    matches!(epsg, 6669..=6687 | 10162..=10174)
}
