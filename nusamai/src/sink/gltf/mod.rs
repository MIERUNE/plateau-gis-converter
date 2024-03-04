//! gltf sink poc
mod attributes;
mod gltf_writer;
mod material;
mod positions;

use std::fs::File;
use std::io::BufWriter;
use std::ops::Mul;
use std::path::PathBuf;
use std::sync::Mutex;

use ahash::{HashMap, HashSet, RandomState};
use earcut_rs::{utils3d::project3d_to_2d, Earcut};
use indexmap::{IndexMap, IndexSet};

use itertools::Itertools;
use kml::types::LineString;
use nusamai_citygml::{object::ObjectStereotype, schema::Schema, GeometryType, Value};
use nusamai_geometry::MultiPolygon;
use nusamai_geometry::Polygon;
use nusamai_plateau::{appearance, Entity};
use nusamai_projection::cartesian::geographic_to_geocentric;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver, Result};
use crate::sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo};

use attributes::FeatureAttributes;
use gltf_writer::{append_gltf_extensions, write_3dtiles, write_gltf};
// use positions::Vertex;

use self::gltf_writer::build_base_gltf;
use self::material::Material;
use self::material::Texture;

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

pub struct BoundingVolume {
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

#[derive(Debug, Clone)]
pub struct TriangulatedEntity {
    pub positions: Vec<Vertex<f64>>,
    pub attributes: FeatureAttributes,
}

pub struct Buffers {
    pub vertices: IndexSet<Vertex<u32>>,
    pub indices: Vec<u32>,
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
}

pub type Features = Vec<Feature>;
pub type CategorizedFeatures = HashMap<String, Features>;

#[derive(Default)]
pub struct PrimitiveInfo {
    pub indices: Vec<u32>,
    pub feature_ids: HashSet<u32>,
}

pub type Primitives = HashMap<material::Material, PrimitiveInfo>;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vertex<T> {
    pub position: [T; 3],
    pub tex_coord: [T; 2],
    pub feature_id: u32,
}

pub type Vertices = Vec<Vertex<f64>>;

pub struct Attributes {
    pub typename: String,
    pub feature_id: u32,
    pub attributes: nusamai_citygml::object::Value,
}

impl DataSink for GltfSink {
    fn make_requirements(&self) -> DataRequirements {
        DataRequirements {
            resolve_appearance: true,
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema) -> Result<()> {
        let ellipsoid = nusamai_projection::ellipsoid::wgs84();
        // This is the code to verify the operation with Cesium
        let mut bounding_volume = Mutex::new(BoundingVolume {
            min_lng: f64::MAX,
            max_lng: f64::MIN,
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_height: f64::MAX,
            max_height: f64::MIN,
        });

        let mut categorized_features: Mutex<CategorizedFeatures> = Mutex::new(Default::default());

        // Construct a Feature classified by typename from Entity
        // Features have polygons, attributes and materials
        // The coordinates of polygon store the actual coordinate values (WGS84) and UV coordinates, not the index.
        {
            let _ = upstream.into_iter().par_bridge().try_for_each(|parcel| {
                if feedback.is_canceled() {
                    return Err(PipelineError::Canceled);
                }

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
                let typename = obj.typename.clone();

                let mut materials: IndexSet<Material> = IndexSet::new();
                let default_mat = appearance::Material::default();

                let mut feature = Feature {
                    polygons: MultiPolygon::new(),
                    attributes: entity.root.clone(),
                    polygon_material_ids: Default::default(),
                    materials: Default::default(),
                };

                geometries.iter().for_each(|entry| {
                    match entry.ty {
                        GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
                            // for each polygon
                            for (((idx_poly, poly_uv), poly_mat), poly_tex) in geom_store
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
                                let poly =
                                    idx_poly.transform(|c| geom_store.vertices[c[0] as usize]);
                                let orig_mat = poly_mat
                                    .and_then(|idx| appearance_store.materials.get(idx as usize))
                                    .unwrap_or(&default_mat)
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
                categorized_features
                    .lock()
                    .unwrap()
                    .entry(typename.into_owned())
                    .or_default()
                    .push(feature);

                Ok(())
            });
        }

        // triangulation and make vertices and primitives
        {
            let bbox = bounding_volume.lock().unwrap();
            let translation = {
                let (tx, ty, tz) = geographic_to_geocentric(
                    &ellipsoid,
                    (bbox.min_lng + bbox.max_lng) / 2.0,
                    (bbox.min_lat + bbox.max_lat) / 2.0,
                    (bbox.min_height + bbox.max_height) / 2.0,
                );
                [tx, tz, -ty]
            };

            for (typename, mut features) in categorized_features.lock().unwrap().clone().into_iter()
            {
                // Triangulation
                let mut earcutter: Earcut<f64> = Earcut::new();
                let mut buf2d: Vec<f64> = Vec::new(); // 2d-projected [x, y]
                let mut index_buf: Vec<u32> = Vec::new();

                let mut vertices: IndexSet<[u32; 6], RandomState> = IndexSet::default(); // [x, y, z, u, v, feature_id]
                let mut primitives: Primitives = Default::default();

                // make vertices and indices
                let mut feature_id = 0;

                for mut feature in features.into_iter() {
                    feature
                        .polygons
                        .transform_inplace(|&[lng, lat, height, u, v]| {
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
                                    (u as f32).to_bits(),
                                    (v as f32).to_bits(),
                                    (feature_id as f32).to_bits(),
                                ];
                                let (index, _) = vertices.insert_full(vbits);
                                index as u32
                            }));
                        }
                    }
                }

                feature_id += 1;
            }
        }

        // build glTF
        {
            // make accessors of positions and tex_coords and feature_ids from vertices
            {}

            // make primitives from indices and feature_ids
            {}

            // make classes from schema
            {}

            // make propertyTable and bufferView from attributes
            {}
        }

        // let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        // rayon::join(
        //     || {
        //         let ellipsoid = nusamai_projection::ellipsoid::wgs84();

        //         // Convert Entity to CzmlPolygon objects
        //         let _ = upstream.into_iter().par_bridge().try_for_each_with(
        //             sender,
        //             |sender, parcel| {
        //                 if feedback.is_canceled() {
        //                     return Err(PipelineError::Canceled);
        //                 }

        //                 // This is the code to verify the operation with Cesium
        //                 let mut bounding_volume = BoundingVolume {
        //                     min_lng: f64::MAX,
        //                     max_lng: f64::MIN,
        //                     min_lat: f64::MAX,
        //                     max_lat: f64::MIN,
        //                     min_height: f64::MAX,
        //                     max_height: f64::MIN,
        //                 };

        //                 let entity = parcel.entity;
        //                 let geom_store = entity.geometry_store.read().unwrap();

        //                 let Value::Object(obj) = &entity.root else {
        //                     // Since root is always assumed to be an Object, skip if unexpected data comes in
        //                     return Ok(());
        //                 };
        //                 let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype
        //                 else {
        //                     // Since root is always assumed to be an Object, skip if unexpected data comes in
        //                     return Ok(());
        //                 };

        //                 let typename = obj.typename.clone();

        //                 // Divide polygons into triangles
        //                 let mut earcutter = Earcut::new();
        //                 let mut buf3d: Vec<f64> = Vec::new();
        //                 let mut buf2d: Vec<f64> = Vec::new();
        //                 let mut triangles_buf: Vec<u32> = Vec::new();
        //                 let mut triangles = Vec::new();

        //                 // extract triangles from entity
        //                 geometries.iter().for_each(|entry| match entry.ty {
        //                     GeometryType::Solid
        //                     | GeometryType::Surface
        //                     | GeometryType::Triangle => {
        //                         for idx_poly in geom_store.multipolygon.iter_range(
        //                             entry.pos as usize..(entry.pos + entry.len) as usize,
        //                         ) {
        //                             let mut poly = idx_poly.transform(|idx| {
        //                                 let [lng, lat, height] =
        //                                     geom_store.vertices[idx[0] as usize];
        //                                 [lng, lat, height]
        //                             });

        //                             poly.transform_inplace(|[lng, lat, height]| {
        //                                 bounding_volume.min_lng = bounding_volume.min_lng.min(*lng);
        //                                 bounding_volume.max_lng = bounding_volume.max_lng.max(*lng);
        //                                 bounding_volume.min_lat = bounding_volume.min_lat.min(*lat);
        //                                 bounding_volume.max_lat = bounding_volume.max_lat.max(*lat);
        //                                 bounding_volume.min_height =
        //                                     bounding_volume.min_height.min(*height);
        //                                 bounding_volume.max_height =
        //                                     bounding_volume.max_height.max(*height);

        //                                 // Convert to geocentric (x, y, z) coordinate.
        //                                 // (Earcut do not work in geographic space)

        //                                 // WGS84 to Geocentric
        //                                 let (x, y, z) = geographic_to_geocentric(
        //                                     &ellipsoid, *lng, *lat, *height,
        //                                 );
        //                                 // OpenGL is a right-handed y-up
        //                                 [x, z, -y]
        //                             });

        //                             let num_outer = match poly.hole_indices().first() {
        //                                 Some(&v) => v as usize,
        //                                 None => poly.coords().len() / 3,
        //                             };

        //                             buf3d.clear();
        //                             buf3d.extend(poly.coords());

        //                             if project3d_to_2d(&buf3d, num_outer, 3, &mut buf2d) {
        //                                 // earcut
        //                                 earcutter.earcut(
        //                                     &buf2d,
        //                                     poly.hole_indices(),
        //                                     2,
        //                                     &mut triangles_buf,
        //                                 );
        //                                 triangles.extend(triangles_buf.iter().map(|idx| {
        //                                     [
        //                                         buf3d[*idx as usize * 3],
        //                                         buf3d[*idx as usize * 3 + 1],
        //                                         buf3d[*idx as usize * 3 + 2],
        //                                     ]
        //                                 }));
        //                             }
        //                         }
        //                     }
        //                     GeometryType::Curve => unimplemented!(),
        //                     GeometryType::Point => unimplemented!(),
        //                 });

        //                 // extract attributes from entity
        //                 let Value::Object(obj) = &entity.root else {
        //                     unimplemented!()
        //                 };
        //                 let attributes = obj.attributes.clone();

        //                 // send triangles and attributes to sender
        //                 if sender
        //                     .send((triangles, attributes, bounding_volume, typename))
        //                     .is_err()
        //                 {
        //                     return Err(PipelineError::Canceled);
        //                 }

        //                 Ok(())
        //             },
        //         );
        //     },
        //     || {
        //         // Write glTF to a file

        //         // Max and min values of all vertices
        //         let mut all_max: [f64; 3] = [f64::MIN; 3];
        //         let mut all_min: [f64; 3] = [f64::MAX; 3];

        //         // BoundingVolume.region required for 3DTiles
        //         let mut bounding_volume = BoundingVolume {
        //             min_lng: f64::MAX,
        //             max_lng: f64::MIN,
        //             min_lat: f64::MAX,
        //             max_lat: f64::MIN,
        //             min_height: f64::MAX,
        //             max_height: f64::MIN,
        //         };

        //         let mut all_triangulated_entities: IndexMap<String, Vec<TriangulatedEntity>> =
        //             IndexMap::new();

        //         for (feature_id, (triangles, attributes, bounds, class_name)) in
        //             receiver.into_iter().enumerate()
        //         {
        //             let vertices = triangles
        //                 .iter()
        //                 .map(|&[x, y, z]| Vertex {
        //                     position: [x, y, z],
        //                     feature_id: feature_id as u32,
        //                     ..Default::default()
        //                 })
        //                 .collect();
        //             let triangulated_entity = TriangulatedEntity {
        //                 positions: vertices,
        //                 attributes: FeatureAttributes {
        //                     class_name: class_name.as_ref().to_string(),
        //                     feature_id: feature_id as u32,
        //                     attributes,
        //                 },
        //             };
        //             all_triangulated_entities
        //                 .entry(class_name.as_ref().to_string())
        //                 .or_default()
        //                 .push(triangulated_entity);

        //             let mut pos_max = [f64::MIN; 3];
        //             let mut pos_min = [f64::MAX; 3];

        //             // calculate the centroid and min/max of the entity
        //             triangles.iter().for_each(|&[x, y, z]| {
        //                 pos_min = [
        //                     f64::min(pos_min[0], x),
        //                     f64::min(pos_min[1], y),
        //                     f64::min(pos_min[2], z),
        //                 ];
        //                 pos_max = [
        //                     f64::max(pos_max[0], x),
        //                     f64::max(pos_max[1], y),
        //                     f64::max(pos_max[2], z),
        //                 ];
        //             });

        //             // calculate the centroid of all entities
        //             all_min = [
        //                 f64::min(all_min[0], pos_min[0]),
        //                 f64::min(all_min[1], pos_min[1]),
        //                 f64::min(all_min[2], pos_min[2]),
        //             ];
        //             all_max = [
        //                 f64::max(all_max[0], pos_max[0]),
        //                 f64::max(all_max[1], pos_max[1]),
        //                 f64::max(all_max[2], pos_max[2]),
        //             ];

        //             // calculate the bounding volume of all entities
        //             bounding_volume.min_lng = f64::min(bounding_volume.min_lng, bounds.min_lng);
        //             bounding_volume.max_lng = f64::max(bounding_volume.max_lng, bounds.max_lng);
        //             bounding_volume.min_lat = f64::min(bounding_volume.min_lat, bounds.min_lat);
        //             bounding_volume.max_lat = f64::max(bounding_volume.max_lat, bounds.max_lat);
        //             bounding_volume.min_height =
        //                 f64::min(bounding_volume.min_height, bounds.min_height);
        //             bounding_volume.max_height =
        //                 f64::max(bounding_volume.max_height, bounds.max_height);
        //         }

        //         // calculate the centroid of all entities
        //         let mut all_translation = [0.; 3];
        //         all_translation[0] = (all_max[0] + all_min[0]) / 2.;
        //         all_translation[1] = (all_max[1] + all_min[1]) / 2.;
        //         all_translation[2] = (all_max[2] + all_min[2]) / 2.;

        //         all_max[0] -= all_translation[0];
        //         all_min[0] -= all_translation[0];
        //         all_max[1] -= all_translation[1];
        //         all_min[1] -= all_translation[1];
        //         all_max[2] -= all_translation[2];
        //         all_min[2] -= all_translation[2];

        //         let mut buffers = IndexMap::new();

        //         // class名ごとにentitiesに入っているfeature_idを振り直す
        //         all_triangulated_entities
        //             .iter_mut()
        //             .for_each(|(_, entities)| {
        //                 entities
        //                     .iter_mut()
        //                     .enumerate()
        //                     .for_each(|(feature_id, entity)| {
        //                         entity.positions.iter_mut().for_each(|vertex| {
        //                             vertex.feature_id = feature_id as u32;
        //                         });
        //                     });
        //             });

        //         for (class_name, entities) in &all_triangulated_entities {
        //             let mut vertices: IndexSet<Vertex<u32>> = IndexSet::default();
        //             let mut indices: Vec<u32> = Vec::new();

        //             for entity in entities.iter() {
        //                 let mut entity_vertices: Vec<Vertex<u32>> = Vec::new();
        //                 let mut entity_indices: Vec<u32> = Vec::new();

        //                 for vertex in entity.positions.clone().into_iter() {
        //                     let (x, y, z) = (
        //                         vertex.position[0] - all_translation[0],
        //                         vertex.position[1] - all_translation[1],
        //                         vertex.position[2] - all_translation[2],
        //                     );
        //                     let vbits = [
        //                         (x as f32).to_bits(),
        //                         (y as f32).to_bits(),
        //                         (z as f32).to_bits(),
        //                     ];

        //                     let vertex = Vertex {
        //                         position: vbits,
        //                         feature_id: vertex.feature_id,
        //                         ..Default::default()
        //                     };

        //                     let (index, _) = vertices.insert_full(vertex);
        //                     entity_vertices.push(vertex);
        //                     entity_indices.push(index as u32);
        //                 }

        //                 indices.extend(entity_indices);
        //             }

        //             buffers.insert(class_name.clone(), Buffers { vertices, indices });
        //         }

        //         // make folders
        //         std::fs::create_dir_all(&self.output_path).unwrap();

        //         // write glTFs
        //         let mut filenames = Vec::new();
        //         for (class_name, buffer) in &buffers {
        //             let mut file_path = self.output_path.clone();
        //             let c_name = class_name.split(':').last().unwrap();
        //             // 作成されたフォルダにファイルを保存
        //             file_path.push(&format!("{}.glb", c_name));
        //             filenames.push(format!("{}.glb", c_name));

        //             let mut file = File::create(&file_path).unwrap();
        //             let writer = BufWriter::with_capacity(1024 * 1024, &mut file);

        //             let mut content = build_base_gltf(class_name, buffer, all_translation);
        //             append_gltf_extensions(&mut content, schema, &all_triangulated_entities);
        //             write_gltf(content.gltf, content.bin_content, writer);
        //         }

        //         // write 3DTiles
        //         let region: [f64; 6] = [
        //             bounding_volume.min_lng.to_radians(),
        //             bounding_volume.min_lat.to_radians(),
        //             bounding_volume.max_lng.to_radians(),
        //             bounding_volume.max_lat.to_radians(),
        //             bounding_volume.min_height,
        //             bounding_volume.max_height,
        //         ];
        //         write_3dtiles(region, &self.output_path, &filenames);
        //     },
        // );
        Ok(())
    }
}
