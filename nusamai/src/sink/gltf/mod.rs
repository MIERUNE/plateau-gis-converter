//! czml sink

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use ahash::RandomState;
use earcut_rs::utils_3d::project3d_to_2d;
use earcut_rs::Earcut;
use indexmap::IndexSet;
use nusamai_citygml::{GeometryType, Value};
use nusamai_geometry::{MultiPolygon, Polygon};
use rayon::prelude::*;

use nusamai_citygml::object::{Entity, ObjectStereotype};
use nusamai_citygml::schema::Schema;

use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};

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
        use transformer::RequirementItem;

        transformer::Requirements {
            mergedown: RequirementItem::Required(transformer::Mergedown::Geometry),
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        rayon::join(
            || {
                // Convert Entity to CzmlPolygon objects
                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        let features = entity_to_gltf(&parcel.entity);
                        // for feature in features {
                        //     let Ok(bytes) = serde_json::to_vec(&feature) else {
                        //         // TODO: fatal error
                        //         return Err(());
                        //     };
                        //     if sender.send(bytes).is_err() {
                        //         log::info!("sink cancelled");
                        //         return Err(());
                        //     };
                        // }

                        let Ok(bytes) = serde_json::to_vec(&features) else {
                            return Err(());
                        };
                        if sender.send(bytes).is_err() {
                            log::info!("sink cancelled");
                            return Err(());
                        };

                        Ok(())
                    },
                );
            },
            || {
                // Write CZML to a file
                let mut file = File::create(&self.output_path).unwrap();
                let mut writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                // Write each Packet
                let mut iter = receiver.into_iter().peekable();
                while let Some(bytes) = iter.next() {
                    writer.write_all(&bytes).unwrap();
                    if iter.peek().is_some() {
                        writer.write_all(b",").unwrap();
                    };
                }

                // Write the FeautureCollection footer and EOL
                writer.write_all(b"]\n").unwrap();
            },
        );
    }
}

fn triangulate_polygon(
    multi_polygon: &MultiPolygon<3, f64>,
) -> (IndexSet<[u32; 3], RandomState>, Vec<u32>) {
    let mut earcutter = Earcut::new();
    let mut buf3d: Vec<f64> = Vec::new();
    let mut buf2d: Vec<f64> = Vec::new();
    let mut triangles_buf: Vec<u32> = Vec::new();
    let mut triangles = Vec::new();

    for poly in multi_polygon.iter() {
        let num_outer = match poly.hole_indices().first() {
            Some(&v) => v as usize,
            None => poly.coords().len() / 3,
        };

        buf3d.clear();
        buf3d.extend(poly.coords());

        if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
            earcutter.earcut(&buf2d, poly.hole_indices(), 2, &mut triangles_buf);
            triangles.extend(triangles_buf.iter().map(|idx| {
                [
                    buf3d[*idx as usize * 3],
                    buf3d[*idx as usize * 3 + 1],
                    buf3d[*idx as usize * 3 + 2],
                ]
            }));
        }
    }

    // calculate the centroid and min/max
    let mut pos_max = [f64::MIN; 3];
    let mut pos_min = [f64::MAX; 3];
    let mut translation = [0.; 3];

    for &[x, y, z] in &triangles {
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
    }
    // TODO: Use a library for 3d linalg
    translation[0] = (pos_max[0] + pos_min[0]) / 2.;
    translation[1] = (pos_max[1] + pos_min[1]) / 2.;
    translation[2] = (pos_max[2] + pos_min[2]) / 2.;
    pos_min[0] -= translation[0];
    pos_max[0] -= translation[0];
    pos_min[1] -= translation[1];
    pos_max[1] -= translation[1];
    pos_min[2] -= translation[2];
    pos_max[2] -= translation[2];

    // make vertices and indices
    let mut vertices: IndexSet<[u32; 3], RandomState> = IndexSet::default();
    let indices: Vec<_> = triangles
        .iter()
        .map(|&[x, y, z]| {
            let (x, y, z) = (x - translation[0], y - translation[1], z - translation[2]);
            let vbits = [
                (x as f32).to_bits(),
                (y as f32).to_bits(),
                (z as f32).to_bits(),
            ];
            let (index, _) = vertices.insert_full(vbits);
            index as u32
        })
        .collect();

    (vertices, indices)
}

/// Create glTF Packet from a Entity
pub fn entity_to_gltf(entity: &Entity) {
    let geom_store = entity.geometry_store.read().unwrap();

    let Value::Object(obj) = &entity.root else {
        unimplemented!()
    };
    let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype else {
        unimplemented!()
    };

    let mut mpoly = nusamai_geometry::MultiPolygon::<1, u32>::new();

    geometries.iter().for_each(|entry| match entry.ty {
        GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
            for idx_poly in geom_store
                .multipolygon
                .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
            {
                mpoly.push(idx_poly);
            }
        }
        GeometryType::Curve => unimplemented!(),
        GeometryType::Point => unimplemented!(),
    });

    mpoly.iter().for_each(|poly| {
        log::info!("{:?}", poly);
    });

    let mut exteriors = Vec::new();
    for poly in mpoly.iter() {
        for idx in poly.exterior().iter() {
            exteriors.push(&geom_store.vertices[idx[0] as usize]);
        }
    }

    let mut interiors = Vec::new();
    for poly in mpoly.iter() {
        for interior in poly.interiors() {
            let mut interior_vec = Vec::new();
            for idx in interior.iter() {
                interior_vec.push(&geom_store.vertices[idx[0] as usize]);
            }
            interiors.push(interior_vec);
        }
    }

    let mut mpoly3: MultiPolygon<'_, 3> = MultiPolygon::<3, f64>::new();

    // todo: 3次元ポリゴンを作る・3次元ポリゴンを3次元マルチポリゴンに追加する

    let (vertices, indices) = triangulate_polygon(&mpoly3);
}

#[cfg(test)]
mod tests {
    use std::sync::RwLock;

    use super::*;
    use nusamai_citygml::{object::Object, GeometryRefEntry, Value};
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

        entity_to_gltf(&entity);
    }
}
