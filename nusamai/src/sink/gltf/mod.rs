//! gltf sink poc
mod attributes;
mod gltf_writer;
mod positions;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use ahash::RandomState;
use earcut_rs::utils_3d::project3d_to_2d;
use earcut_rs::Earcut;
use indexmap::{IndexMap, IndexSet};

use nusamai_citygml::{object::ObjectStereotype, schema::Schema, GeometryType, Value};
use nusamai_projection::cartesian::geographic_to_geocentric;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::parameters::*;
use crate::pipeline::{Feedback, PipelineError, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};

use attributes::Attributes;
use gltf_writer::{append_gltf_extensions, to_gltf, write_3dtiles, write_gltf};
use positions::Vertex;

use self::gltf_writer::build_base_gltf;

pub struct BoundingVolume {
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

pub struct TriangulatedEntity {
    pub positions: Vec<Vertex<f64>>,
    pub attributes: Attributes,
}

pub struct Buffers {
    pub vertices: IndexSet<Vertex<u32>>,
    pub indices: Vec<u32>,
}

pub struct GltfSinkProvider {}

impl DataSinkProvider for GltfSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
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

impl DataSink for GltfSink {
    fn make_transform_requirements(&self) -> transformer::Requirements {
        transformer::Requirements {
            ..Default::default()
        }
    }

    fn run(
        &mut self,
        upstream: Receiver,
        feedback: &Feedback,
        schema: &Schema,
    ) -> Result<(), PipelineError> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        rayon::join(
            || {
                let ellipsoid = nusamai_projection::ellipsoid::wgs84();

                // Convert Entity to CzmlPolygon objects
                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_canceled() {
                            return Err(PipelineError::Canceled);
                        }

                        // This is the code to verify the operation with Cesium
                        let mut bounding_volume = BoundingVolume {
                            min_lng: f64::MAX,
                            max_lng: f64::MIN,
                            min_lat: f64::MAX,
                            max_lat: f64::MIN,
                            min_height: f64::MAX,
                            max_height: f64::MIN,
                        };

                        let entity = parcel.entity;
                        let geom_store = entity.geometry_store.read().unwrap();

                        // todo: Exception handling
                        let Value::Object(obj) = &entity.root else {
                            unimplemented!()
                        };
                        // todo: Support for other stereotypes
                        let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype
                        else {
                            unimplemented!()
                        };

                        let typename = obj.typename.clone();

                        // Divide polygons into triangles
                        let mut earcutter = Earcut::new();
                        let mut buf3d: Vec<f64> = Vec::new();
                        let mut buf2d: Vec<f64> = Vec::new();
                        let mut triangles_buf: Vec<u32> = Vec::new();
                        let mut triangles = Vec::new();

                        // extract triangles from entity
                        geometries.iter().for_each(|entry| match entry.ty {
                            GeometryType::Solid
                            | GeometryType::Surface
                            | GeometryType::Triangle => {
                                for idx_poly in geom_store.multipolygon.iter_range(
                                    entry.pos as usize..(entry.pos + entry.len) as usize,
                                ) {
                                    let mut poly = idx_poly.transform(|idx| {
                                        let [lng, lat, height] =
                                            geom_store.vertices[idx[0] as usize];
                                        [lng, lat, height]
                                    });

                                    poly.transform_inplace(|[lng, lat, height]| {
                                        bounding_volume.min_lng = bounding_volume.min_lng.min(*lng);
                                        bounding_volume.max_lng = bounding_volume.max_lng.max(*lng);
                                        bounding_volume.min_lat = bounding_volume.min_lat.min(*lat);
                                        bounding_volume.max_lat = bounding_volume.max_lat.max(*lat);
                                        bounding_volume.min_height =
                                            bounding_volume.min_height.min(*height);
                                        bounding_volume.max_height =
                                            bounding_volume.max_height.max(*height);

                                        // Convert to geocentric (x, y, z) coordinate.
                                        // (Earcut do not work in geographic space)

                                        // WGS84 to Geocentric
                                        let (x, y, z) = geographic_to_geocentric(
                                            &ellipsoid, *lng, *lat, *height,
                                        );
                                        // OpenGL is a right-handed y-up
                                        [x, z, -y]
                                    });

                                    let num_outer = match poly.hole_indices().first() {
                                        Some(&v) => v as usize,
                                        None => poly.coords().len() / 3,
                                    };

                                    buf3d.clear();
                                    buf3d.extend(poly.coords());

                                    if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                                        // earcut
                                        earcutter.earcut(
                                            &buf2d,
                                            poly.hole_indices(),
                                            2,
                                            &mut triangles_buf,
                                        );
                                        triangles.extend(triangles_buf.iter().map(|idx| {
                                            [
                                                buf3d[*idx as usize * 3],
                                                buf3d[*idx as usize * 3 + 1],
                                                buf3d[*idx as usize * 3 + 2],
                                            ]
                                        }));
                                    }
                                }
                            }
                            GeometryType::Curve => unimplemented!(),
                            GeometryType::Point => unimplemented!(),
                        });

                        // extract attributes from entity
                        let Value::Object(obj) = &entity.root else {
                            unimplemented!()
                        };
                        let attributes = obj.attributes.clone();

                        // send triangles and attributes to sender
                        if sender
                            .send((triangles, attributes, bounding_volume, typename))
                            .is_err()
                        {
                            return Err(PipelineError::Canceled);
                        }

                        Ok(())
                    },
                );
            },
            || {
                // Write glTF to a file

                // Max and min values of all vertices
                let mut all_max: [f64; 3] = [f64::MIN; 3];
                let mut all_min: [f64; 3] = [f64::MAX; 3];

                // BoundingVolume.region required for 3DTiles
                let mut bounding_volume = BoundingVolume {
                    min_lng: f64::MAX,
                    max_lng: f64::MIN,
                    min_lat: f64::MAX,
                    max_lat: f64::MIN,
                    min_height: f64::MAX,
                    max_height: f64::MIN,
                };

                // // Maintain a list of type names as multiple types are mixed.
                // let mut class_names = HashSet::new();

                // // holds all vertex coordinates and feature_id
                // let mut all_positions: Vec<[f64; 4]> = Vec::new();

                // // Holds all attributes of the entity
                // let mut all_attributes: Vec<Attributes> = Vec::new();

                let mut all_triangulated_entities: IndexMap<String, Vec<TriangulatedEntity>> =
                    IndexMap::new();

                for (feature_id, (triangles, attributes, bounds, class_name)) in
                    receiver.into_iter().enumerate()
                {
                    let vertices = triangles
                        .iter()
                        .map(|&[x, y, z]| Vertex {
                            position: [x, y, z],
                            feature_id: feature_id as u32,
                            ..Default::default()
                        })
                        .collect();
                    let triangulated_entity = TriangulatedEntity {
                        positions: vertices,
                        attributes: Attributes {
                            class_name: class_name.as_ref().to_string(),
                            feature_id: feature_id as u32,
                            attributes,
                        },
                    };
                    all_triangulated_entities
                        .entry(class_name.as_ref().to_string())
                        .or_insert_with(Vec::new)
                        .push(triangulated_entity);

                    // all_attributes.push(Attributes {
                    //     class_name: class_name.as_ref().to_string(),
                    //     feature_id: feature_id as u32,
                    //     attributes,
                    // });

                    // class_names.insert(class_name);

                    let mut pos_max = [f64::MIN; 3];
                    let mut pos_min = [f64::MAX; 3];

                    // // add feature_id to all_positions
                    // triangles.iter().for_each(|&[x, y, z]| {
                    //     all_positions.push([x, y, z, feature_id as f64]);
                    // });

                    // calculate the centroid and min/max of the entity
                    triangles.iter().for_each(|&[x, y, z]| {
                        pos_min = [
                            f64::min(pos_min[0], x),
                            f64::min(pos_min[1], y),
                            f64::min(pos_min[2], z),
                        ];
                        pos_max = [
                            f64::max(pos_max[0], x),
                            f64::max(pos_max[1], y),
                            f64::max(pos_max[2], z),
                        ];
                    });

                    // calculate the centroid of all entities
                    all_min = [
                        f64::min(all_min[0], pos_min[0]),
                        f64::min(all_min[1], pos_min[1]),
                        f64::min(all_min[2], pos_min[2]),
                    ];
                    all_max = [
                        f64::max(all_max[0], pos_max[0]),
                        f64::max(all_max[1], pos_max[1]),
                        f64::max(all_max[2], pos_max[2]),
                    ];

                    // calculate the bounding volume of all entities
                    bounding_volume.min_lng = f64::min(bounding_volume.min_lng, bounds.min_lng);
                    bounding_volume.max_lng = f64::max(bounding_volume.max_lng, bounds.max_lng);
                    bounding_volume.min_lat = f64::min(bounding_volume.min_lat, bounds.min_lat);
                    bounding_volume.max_lat = f64::max(bounding_volume.max_lat, bounds.max_lat);
                    bounding_volume.min_height =
                        f64::min(bounding_volume.min_height, bounds.min_height);
                    bounding_volume.max_height =
                        f64::max(bounding_volume.max_height, bounds.max_height);
                }

                // calculate the centroid of all entities
                let mut all_translation = [0.; 3];
                all_translation[0] = (all_max[0] + all_min[0]) / 2.;
                all_translation[1] = (all_max[1] + all_min[1]) / 2.;
                all_translation[2] = (all_max[2] + all_min[2]) / 2.;

                all_max[0] -= all_translation[0];
                all_min[0] -= all_translation[0];
                all_max[1] -= all_translation[1];
                all_min[1] -= all_translation[1];
                all_max[2] -= all_translation[2];
                all_min[2] -= all_translation[2];

                // ベースとなるglTFを作成するために、クラスごとに頂点・インデックス・頂点IDを分ける
                // 2クラスあるなら、6個のbufferViewが必要で、それに伴い6個のaccessorが必要になる
                // primitivesも、クラスごとに作成する必要があるので2つになる
                // クラス名をキーとしたIndexMapを作成し、クラスごとに頂点・インデックス・頂点IDを格納する
                let mut buffers = IndexMap::new();
                for (class_name, entities) in all_triangulated_entities {
                    let mut vertices: IndexSet<Vertex<u32>> = IndexSet::default();
                    let mut indices: Vec<u32> = Vec::new();
                    let mut feature_ids: Vec<u32> = Vec::new();

                    for entity in entities {
                        let mut entity_vertices: Vec<Vertex<u32>> = Vec::new();
                        let mut entity_indices: Vec<u32> = Vec::new();

                        for vertex in entity.positions {
                            let (x, y, z) = (
                                vertex.position[0] - all_translation[0],
                                vertex.position[1] - all_translation[1],
                                vertex.position[2] - all_translation[2],
                            );
                            let vbits = [
                                (x as f32).to_bits(),
                                (y as f32).to_bits(),
                                (z as f32).to_bits(),
                            ];

                            let vertex = Vertex {
                                position: vbits,
                                feature_id: vertex.feature_id,
                                ..Default::default()
                            };

                            let (index, _) = vertices.insert_full(vertex);
                            entity_vertices.push(vertex);
                            entity_indices.push(index as u32);
                        }

                        indices.extend(entity_indices);
                    }

                    buffers.insert(class_name, Buffers { vertices, indices });
                }

                // make all vertices and indices for binary buffer
                // let mut vertices: IndexSet<Vertex<u32>, RandomState> = IndexSet::default();
                // let indices: Vec<u32> = all_positions
                //     .iter()
                //     .map(|&[x, y, z, feature_id]| {
                //         let (x, y, z) = (
                //             x - all_translation[0],
                //             y - all_translation[1],
                //             z - all_translation[2],
                //         );
                //         let vbits = [
                //             (x as f32).to_bits(),
                //             (y as f32).to_bits(),
                //             (z as f32).to_bits(),
                //         ];

                //         let vertex = Vertex {
                //             position: vbits,
                //             feature_id: feature_id as u32,
                //             ..Default::default()
                //         };

                //         let (index, _) = vertices.insert_full(vertex);

                //         index as u32
                //     })
                //     .collect();
                // let indices: Vec<u32> = indices
                //     .chunks_exact(3)
                //     .filter(|idx| (idx[0] != idx[1] && idx[1] != idx[2] && idx[2] != idx[0]))
                //     .flatten()
                //     .copied()
                //     .collect();

                let mut file = File::create(&self.output_path).unwrap();
                let writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                // write glTF
                let (bin_content, gltf) =
                    build_base_gltf(&buffers, all_translation, all_min, all_max);

                // let (mut bin_content, mut gltf) =
                //     to_gltf(&vertices, &indices, all_translation, all_min, all_max);
                // append_gltf_extensions(
                //     &mut gltf,
                //     &mut bin_content,
                //     class_names,
                //     schema,
                //     vertices,
                //     all_attributes,
                // );
                write_gltf(gltf, bin_content, writer);

                // write 3DTiles
                let region: [f64; 6] = [
                    bounding_volume.min_lng.to_radians(),
                    bounding_volume.min_lat.to_radians(),
                    bounding_volume.max_lng.to_radians(),
                    bounding_volume.max_lat.to_radians(),
                    bounding_volume.min_height,
                    bounding_volume.max_height,
                ];
                write_3dtiles(region, &self.output_path);
            },
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use nusamai_citygml::{
        object::{Entity, Object},
        GeometryRefEntry, Value,
    };
    use nusamai_geometry::MultiPolygon;
    use nusamai_projection::crs::EPSG_JGD2011_GEOGRAPHIC_3D;

    #[test]
    fn test_entity_multipolygon() {
        let vertices: Vec<[f64; 3]> = vec![
            // 1st polygon, exterior (vertex 0~3)
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
            // 1st polygon, interior 1 (vertex 4~7)
            [1., 1., 111.],
            [2., 1., 111.],
            [2., 2., 111.],
            [1., 2., 111.],
            // 1st polygon, interior 2 (vertex 8~11)
            [3., 3., 111.],
            [4., 3., 111.],
            [4., 4., 111.],
            [3., 4., 111.],
            // 2nd polygon, exterior (vertex 12~15)
            [4., 0., 222.],
            [7., 0., 222.],
            [7., 3., 222.],
            [4., 3., 222.],
            // 2nd polygon, interior (vertex 16~19)
            [5., 1., 222.],
            [6., 1., 222.],
            [6., 2., 222.],
            [5., 2., 222.],
            // 3rd polygon, exterior (vertex 20~23)
            [4., 0., 333.],
            [7., 0., 333.],
            [7., 3., 333.],
            [4., 3., 333.],
        ];

        let mut mpoly = MultiPolygon::<1, u32>::new();
        // 1st polygon
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        mpoly.add_interior([[4], [5], [6], [7], [4]]);
        mpoly.add_interior([[8], [9], [10], [11], [8]]);
        // 2nd polygon
        mpoly.add_exterior([[12], [13], [14], [15], [12]]);
        mpoly.add_interior([[16], [17], [18], [19], [16]]);
        // 3rd polygon
        mpoly.add_exterior([[20], [21], [22], [23], [20]]);

        let geometries = nusamai_citygml::GeometryStore {
            epsg: EPSG_JGD2011_GEOGRAPHIC_3D,
            vertices,
            multipolygon: mpoly,
            multilinestring: Default::default(),
            multipoint: Default::default(),
        };

        let entity = Entity {
            root: Value::Object(Object {
                typename: "dummy".into(),
                attributes: Default::default(),
                stereotype: nusamai_citygml::object::ObjectStereotype::Feature {
                    id: "dummy".into(),
                    geometries: vec![
                        GeometryRefEntry {
                            ty: GeometryType::Solid,
                            pos: 0,
                            len: 1,
                            lod: 1,
                        },
                        GeometryRefEntry {
                            ty: GeometryType::Solid,
                            pos: 1,
                            len: 1,
                            lod: 1,
                        },
                        GeometryRefEntry {
                            ty: GeometryType::Solid,
                            pos: 2,
                            len: 1,
                            lod: 1,
                        },
                    ],
                },
            }),
            geometry_store: RwLock::new(geometries).into(),
        };
    }
}
