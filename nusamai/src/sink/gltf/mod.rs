//! gltf sink poc
mod gltf_writer;
mod material;

use std::{f64::consts::FRAC_PI_2, fs::File, io::BufWriter, path::PathBuf, sync::Mutex};

use crate::sink::cesiumtiles::utils::calculate_normal;
use ahash::{HashMap, HashSet, RandomState};
use earcut::{utils3d::project3d_to_2d, Earcut};
use flatgeom::MultiPolygon;
use glam::{DMat4, DVec3, DVec4};
use gltf_writer::write_gltf_glb;
use indexmap::IndexSet;
use itertools::Itertools;
use material::{Material, Texture};
use nusamai_citygml::{object::ObjectStereotype, schema::Schema, GeometryType, Value};
use nusamai_plateau::appearance;
use nusamai_projection::cartesian::geodetic_to_geocentric;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{cesiumtiles::metadata, DataRequirements, DataSink, DataSinkProviderTest, SinkInfo},
    transformer,
    transformer::{
        LodSelection, Selection, TransformerConfig, TransformerConfig2, TransformerOption,
        TransformerOption2, TransformerRegistry, TransformerRegistry2,
    },
};

pub struct GltfSinkProvider {}

impl DataSinkProviderTest for GltfSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "gltf".to_string(),
            name: "glTF".to_string(),
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

    fn available_transformer2(&self) -> TransformerRegistry2 {
        let mut settings: TransformerRegistry2 = TransformerRegistry2::new();

        // settings.insert(TransformerConfig {
        //     key: "use_texture".to_string(),
        //     label: "テクスチャの使用".to_string(),
        //     is_enabled: false,
        //     requirements: vec![transformer::Requirement::UseAppearance],
        // });

        settings.insert(TransformerConfig2 {
            key: "use_lod".to_string(),
            label: "出力LODの選択".to_string(),
            selection: Some(Selection::Lod(LodSelection::MaxLod)),
            requirements: vec![transformer::Requirement2::UseLod(LodSelection::Lod2)],
        });

        settings
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_settings = self.available_transformer2();

        Box::<GltfSink>::new(GltfSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings,
        })
    }
}

pub struct GltfSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry2,
}

pub struct BoundingVolume {
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl BoundingVolume {
    fn update(&mut self, other: &Self) {
        self.min_lng = self.min_lng.min(other.min_lng);
        self.max_lng = self.max_lng.max(other.max_lng);
        self.min_lat = self.min_lat.min(other.min_lat);
        self.max_lat = self.max_lat.max(other.max_lat);
        self.min_height = self.min_height.min(other.min_height);
        self.max_height = self.max_height.max(other.max_height);
    }
}

impl Default for BoundingVolume {
    fn default() -> Self {
        Self {
            min_lng: f64::MAX,
            max_lng: f64::MIN,
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_height: f64::MAX,
            max_height: f64::MIN,
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
    fn make_requirements(&mut self, properties: Vec<TransformerOption>) -> DataRequirements {
        let default_requirements: DataRequirements = DataRequirements {
            resolve_appearance: true,
            key_value: crate::transformer::KeyValueSpec::JsonifyObjectsAndArrays,
            // NOTE: test
            lod_filter: transformer::LodFilterSpec {
                mask: transformer::LodMask::all(),
                ..Default::default() // mode: transformer::LodFilterMode::Lod4,
            },
            ..Default::default()
        };

        for prop in properties {
            let _ = &self.transform_settings;
            println!("{:?}", prop);
            // .update_transformer(&prop.key, prop.is_enabled);
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let ellipsoid = nusamai_projection::ellipsoid::wgs84();

        let classified_features: Mutex<ClassifiedFeatures> = Default::default();

        // Construct a Feature classified by typename from Entity
        // Features have polygons, attributes and materials
        // The coordinates of polygon store the actual coordinate values (WGS84) and UV coordinates, not the index.
        let _ = upstream.into_iter().par_bridge().try_for_each(|parcel| {
            feedback.ensure_not_canceled()?;

            let entity = parcel.entity;
            println!("runshink!!!!!!!!!!!!!!!!!!!");

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

                                            local_bvol.min_lng = local_bvol.min_lng.min(lng);
                                            local_bvol.max_lng = local_bvol.max_lng.max(lng);
                                            local_bvol.min_lat = local_bvol.min_lat.min(lat);
                                            local_bvol.max_lat = local_bvol.max_lat.max(lat);
                                            local_bvol.min_height =
                                                local_bvol.min_height.min(height);
                                            local_bvol.max_height =
                                                local_bvol.max_height.max(height);
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

        let transform_matrix = {
            let bounds = &global_bvol;
            let center_lng = (bounds.min_lng + bounds.max_lng) / 2.0;
            let center_lat = (bounds.min_lat + bounds.max_lat) / 2.0;

            let psi = ((1. - ellipsoid.e_sq()) * center_lat.to_radians().tan()).atan();

            let (tx, ty, tz) = geodetic_to_geocentric(&ellipsoid, center_lng, center_lat, 0.);
            let h = (tx * tx + ty * ty + tz * tz).sqrt();

            DMat4::from_translation(DVec3::new(0., -h, 0.))
                * DMat4::from_rotation_x(-(FRAC_PI_2 - psi))
                * DMat4::from_rotation_y((-center_lng - 90.).to_radians())
        };
        let _ = transform_matrix.inverse();

        classified_features
            .into_par_iter()
            .try_for_each(|(typename, mut features)| {
                feedback.ensure_not_canceled()?;

                // Triangulation
                let mut earcutter: Earcut<f64> = Earcut::new();
                let mut buf3d: Vec<[f64; 3]> = Vec::new();
                let mut buf2d: Vec<[f64; 2]> = Vec::new(); // 2d-projected [x, y]
                let mut index_buf: Vec<u32> = Vec::new();

                let mut vertices: IndexSet<[u32; 9], RandomState> = IndexSet::default(); // [x, y, z, nx, ny, nz, u, v, feature_id]
                let mut primitives: Primitives = Default::default();

                let mut metadata_encoder = metadata::MetadataEncoder::new(schema);

                // make vertices and indices
                let mut feature_id = 0;
                for feature in features.features.iter_mut() {
                    feedback.ensure_not_canceled()?;

                    feature.feature_id = Some(feature_id as u32);
                    feature
                        .polygons
                        .transform_inplace(|&[lng, lat, height, u, v]| {
                            // geographic to geocentric
                            let (x, y, z) = geodetic_to_geocentric(&ellipsoid, lng, lat, height);
                            // z-up to y-up
                            let v_xyz = DVec4::new(x, z, -y, 1.0);
                            // local ENU coordinate
                            let v_enu = transform_matrix * v_xyz;
                            // println!("enu: {:?}", v_enu);

                            // flip the texture v-coordinate
                            [v_enu[0], v_enu[1], v_enu[2], u, 1.0 - v]
                        });

                    // Encode properties
                    if metadata_encoder
                        .add_feature(&typename, &feature.attributes)
                        .is_err()
                    {
                        feedback.warn("Failed to encode feature attributes".to_string());
                        continue;
                    }

                    for (poly, orig_mat_id) in feature
                        .polygons
                        .iter()
                        .zip_eq(feature.polygon_material_ids.iter())
                    {
                        let num_outer = match poly.hole_indices().first() {
                            Some(&v) => v as usize,
                            None => poly.raw_coords().len(),
                        };

                        let mat = feature.materials[*orig_mat_id as usize].clone();
                        let primitive = primitives.entry(mat).or_default();
                        primitive.feature_ids.insert(feature_id as u32);

                        if let Some((nx, ny, nz)) =
                            calculate_normal(poly.exterior().iter().map(|v| [v[0], v[1], v[2]]))
                        {
                            buf3d.clear();
                            buf3d.extend(poly.raw_coords().iter().map(|c| [c[0], c[1], c[2]]));

                            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                                // earcut
                                earcutter.earcut(
                                    buf2d.iter().copied(),
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
                    }
                    feature_id += 1;
                }

                // Ensure that the parent directory exists
                std::fs::create_dir_all(&self.output_path)?;

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
