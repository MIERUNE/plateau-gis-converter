//! gltf sink poc
mod gltf_writer;
mod material;
mod metadata;
mod utils;

use indexmap::IndexSet;
use itertools::Itertools;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Mutex;

use ahash::{HashMap, HashSet, RandomState};
use earcut_rs::{utils3d::project3d_to_2d, Earcut};

use nusamai_citygml::{object::ObjectStereotype, schema::Schema, GeometryType, Value};
use nusamai_geometry::MultiPolygon;
use nusamai_geometry::Polygon;
use nusamai_plateau::appearance;
use nusamai_projection::cartesian::geographic_to_geocentric;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::Deserialize;
use serde::Serialize;

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo};

use gltf_writer::{write_3dtiles, write_gltf_glb};
use material::{Material, Texture};
use utils::calculate_normal;

pub struct GltfSinkProvider {}

impl DataSinkProvider for GltfSinkProvider {
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
            },
        );
        params
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);

        Box::<GltfSink>::new(GltfSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct GltfSink {
    output_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct BoundingVolume {
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    // polygons [x, y, z, u, v]
    pub polygons: MultiPolygon<'static, 5>,
    // material ids for each polygon
    pub polygon_material_ids: Vec<u32>,
    // materials
    pub materials: IndexSet<Material>,
    // attribute values
    pub attributes: nusamai_citygml::object::Value,
    // feature_id
    pub feature_id: Option<u32>,
}

pub type Features = Vec<Feature>;
pub type CategorizedFeatures = HashMap<String, Features>;

#[derive(Default)]
pub struct PrimitiveInfo {
    pub indices: Vec<u32>,
    pub feature_ids: HashSet<u32>,
}

pub type Primitives = HashMap<material::Material, PrimitiveInfo>;

impl DataSink for GltfSink {
    fn make_requirements(&self) -> DataRequirements {
        DataRequirements {
            use_appearance: true,
            resolve_appearance: true,
            key_value: crate::transformer::KeyValueSpec::JsonifyObjects,
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let ellipsoid = nusamai_projection::ellipsoid::wgs84();

        // This is the code to verify the operation with Cesium
        let bounding_volume = Mutex::new(BoundingVolume {
            min_lng: f64::MAX,
            max_lng: f64::MIN,
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_height: f64::MAX,
            max_height: f64::MIN,
        });

        let grouped_features: Mutex<CategorizedFeatures> = Default::default();

        let mut glb_files: Vec<String> = Vec::new();

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
                            let poly = idx_poly.transform(|c| geom_store.vertices[c[0] as usize]);
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

                            let mut ling_buf: Vec<[f64; 5]> = Vec::new();
                            for (xyz, uv) in poly
                                .coords()
                                .chunks_exact(3)
                                .zip(poly_uv.coords().chunks_exact(2))
                            {
                                ling_buf.push([xyz[0], xyz[1], xyz[2], uv[0], uv[1]]);
                                let mut bounds = bounding_volume.lock().unwrap();
                                bounds.min_lng = bounds.min_lng.min(xyz[0]);
                                bounds.max_lng = bounds.max_lng.max(xyz[0]);
                                bounds.min_lat = bounds.min_lat.min(xyz[1]);
                                bounds.max_lat = bounds.max_lat.max(xyz[1]);
                                bounds.min_height = bounds.min_height.min(xyz[2]);
                                bounds.max_height = bounds.max_height.max(xyz[2]);
                            }

                            let mut poly_buf: Polygon<5> = Polygon::new();
                            poly_buf.add_ring(ling_buf);

                            feature.polygons.push(&poly_buf);
                            feature.polygon_material_ids.push(mat_idx as u32);
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
            grouped_features
                .lock()
                .unwrap()
                .entry(obj.typename.to_string())
                .or_default()
                .push(feature);

            Ok::<(), PipelineError>(())
        });

        // triangulation and make vertices and primitives
        let translation = {
            let bounds = bounding_volume.lock().unwrap();
            let (tx, ty, tz) = geographic_to_geocentric(
                &ellipsoid,
                (bounds.min_lng + bounds.max_lng) / 2.0,
                (bounds.min_lat + bounds.max_lat) / 2.0,
                0.,
            );
            // z-up to y-up
            let [tx, ty, tz] = [tx, tz, -ty];
            // double-precision to single-precision
            [(tx as f32) as f64, (ty as f32) as f64, (tz as f32) as f64]
        };

        for (typename, mut features) in grouped_features.lock().unwrap().drain() {
            feedback.ensure_not_canceled()?;

            // Triangulation
            let mut earcutter: Earcut<f64> = Earcut::new();
            let mut buf2d: Vec<f64> = Vec::new(); // 2d-projected [x, y]
            let mut index_buf: Vec<u32> = Vec::new();

            let mut vertices: IndexSet<[u32; 9], RandomState> = IndexSet::default(); // [x, y, z, nx, ny, nz, u, v, feature_id]
            let mut primitives: Primitives = Default::default();

            let mut metadata_encoder = metadata::MetadataEncoder::new(schema);

            // make vertices and indices
            let mut feature_id = 0;
            for feature in features.iter_mut() {
                feedback.ensure_not_canceled()?;

                feature.feature_id = Some(feature_id as u32);
                feature
                    .polygons
                    .transform_inplace(|&[lng, lat, height, u, v]| {
                        // geographic to geocentric
                        let (x, y, z) = geographic_to_geocentric(&ellipsoid, lng, lat, height);

                        // z-up to y-up
                        // subtract the translation
                        // flip the texture v-coordinate
                        [
                            x - translation[0],
                            z - translation[1],
                            -y - translation[2],
                            u,
                            1.0 - v,
                        ]
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
                                let [x, y, z, u, v] =
                                    poly.coords()[pos..pos + 5].try_into().unwrap();
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

            // make folders
            std::fs::create_dir_all(&self.output_path)?;

            // write glTFs
            let c_name = typename
                .split_once(':')
                .map(|(_, s)| s)
                .unwrap_or(&typename);
            let file_path = self.output_path.join(&format!("{}.glb", c_name));

            // save filename referenced from 3D Tiles
            glb_files.push(format!("{}.glb", c_name));

            let mut file = File::create(&file_path)?;
            let writer = BufWriter::with_capacity(1024 * 1024, &mut file);

            write_gltf_glb(
                feedback,
                writer,
                translation,
                vertices,
                primitives,
                metadata_encoder,
            )?;
        }

        // write 3DTiles
        let bounds = bounding_volume.lock().unwrap();
        let region: [f64; 6] = [
            bounds.min_lng.to_radians(),
            bounds.min_lat.to_radians(),
            bounds.max_lng.to_radians(),
            bounds.max_lat.to_radians(),
            bounds.min_height,
            bounds.max_height,
        ];
        write_3dtiles(region, &self.output_path, &glb_files)?;

        Ok(())
    }
}
