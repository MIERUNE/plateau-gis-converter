//! gltf sink poc

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use ahash::RandomState;
use byteorder::{ByteOrder, LittleEndian};
use earcut_rs::utils_3d::project3d_to_2d;
use earcut_rs::Earcut;
use indexmap::IndexSet;
use rayon::prelude::*;

use nusamai_citygml::object::{Entity, ObjectStereotype};
use nusamai_citygml::schema::Schema;
use nusamai_citygml::{GeometryType, Value};
use nusamai_geometry::{MultiPolygon, Polygon};
use nusamai_gltf_json::*;

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

                        // todo: transformerからschemaを受け取る必要がある

                        let entity = parcel.entity;
                        let geom_store = entity.geometry_store.read().unwrap();

                        let Value::Object(obj) = &entity.root else {
                            unimplemented!()
                        };
                        let ObjectStereotype::Feature { id: _, geometries } = &obj.stereotype
                        else {
                            unimplemented!()
                        };

                        // Vertex indices are taken from 1D polygons and converted to 3D polygons
                        let mut mpoly3 = MultiPolygon::<3, f64>::new();

                        geometries.iter().for_each(|entry| match entry.ty {
                            GeometryType::Solid
                            | GeometryType::Surface
                            | GeometryType::Triangle => {
                                for idx_poly in geom_store.multipolygon.iter_range(
                                    entry.pos as usize..(entry.pos + entry.len) as usize,
                                ) {
                                    let exterior_rings: Vec<[f64; 3]> = idx_poly
                                        .exterior()
                                        .into_iter()
                                        .map(|idx| geom_store.vertices[idx[0] as usize])
                                        .collect();

                                    let interior_rings: Vec<Vec<[f64; 3]>> = idx_poly
                                        .interiors()
                                        .map(|interior| {
                                            interior
                                                .into_iter()
                                                .map(|idx| geom_store.vertices[idx[0] as usize])
                                                .collect()
                                        })
                                        .collect();

                                    let mut poly = Polygon::<3, f64>::new();
                                    poly.add_ring(exterior_rings);
                                    for interior in interior_rings {
                                        poly.add_ring(interior);
                                    }

                                    mpoly3.push(poly);
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

                        // send mpoly3 and attributes to sender
                        if sender.send((mpoly3, attributes)).is_err() {
                            return Err(());
                        }

                        Ok(())
                    },
                );
            },
            || {
                // Write CZML to a file
                let mut file = File::create(&self.output_path).unwrap();
                let mut writer = BufWriter::with_capacity(1024 * 1024, &mut file);

                // Write each glTF
                // todo: indices, vertices, feature_idsのVecを作成し、pushしていく
                // todo: mpolyを受け取るので、先頭の地物をfeature_id: 0とし、頂点の個数と同じ分だけ、idを振っていく
                // todo: 三角分割する
                // todo: 三角分割したものをindicesとverticesに追加していく

                let mut iter = receiver.into_iter().peekable();
                // iterの先頭を0として、頂点の個数と同じ分だけ、idを振っていく
                let mut feature_id = 0;
                while let Some((mpoly3, attributes)) = iter.next() {
                    triangulate(mpoly3);
                }
            },
        );
    }
}

fn mpoly3_to_vertices(mpoly3: MultiPolygon<'_, 3>) {}

/// Create glTF Packet from a Entity
fn triangulate(mpoly3: MultiPolygon<'_, 3>) {
    // todo: mpoly3からindices, vertices, feature_idsを生成する関数を作る
    // todo: gltf書き込みを行う関数を作る

    // todo: bufferには頂点インデックス + 頂点座標 + 頂点IDの順で書き込む（法線とか・RGBを書き込むなら、考慮する必要がある）
    // todo: bufferViewは上記に合わせ、3つ作る
    // todo: accessorは上記に合わせ、3つ作るが頂点インデックスと頂点IDはSCALAR、頂点座標はVEC3になると思う

    // todo: 属性部分をbufferにするコードを書く

    let mut earcutter = Earcut::new();
    let mut buf3d: Vec<f64> = Vec::new();
    let mut buf2d: Vec<f64> = Vec::new();
    let mut triangles_buf: Vec<u32> = Vec::new();
    let mut triangles = Vec::new();

    for poly in mpoly3.iter() {
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

    println!("{:?}", indices);
    println!("{:?}", vertices);
    println!("{:?}", translation);
    println!("{:?}", pos_min);
    println!("{:?}", pos_max);
}

fn write_gltf<W: Write>(
    mut writer: W,
    min: [f64; 3],
    max: [f64; 3],
    translation: [f64; 3],
    vertices: impl IntoIterator<Item = [u32; 3]>,
    indices: impl IntoIterator<Item = u32>,
) {
    let path_glb = "/Users/satoru/Downloads/plateau/test.glb";
    let mut file = std::fs::File::create(path_glb).unwrap();
    let mut writer = BufWriter::new(&mut file);

    let mut bin_content: Vec<u8> = Vec::new();

    let vertices_offset = bin_content.len();
    let mut buf = [0; 12];
    let mut vertices_count = 0;
    for v in vertices {
        LittleEndian::write_u32_into(&v, &mut buf);
        bin_content.write_all(&buf).unwrap();
        vertices_count += 1;
    }
    let vertices_len = bin_content.len() - vertices_offset;

    let indices_offset = bin_content.len();
    let mut indices_count = 0;
    for idx in indices {
        bin_content.write_all(&idx.to_le_bytes()).unwrap();
        indices_count += 1;
    }
    let indices_len = bin_content.len() - indices_offset;

    let gltf = Gltf {
        scenes: vec![Scene {
            nodes: Some(vec![0]),
            ..Default::default()
        }],
        nodes: vec![Node {
            mesh: Some(0),
            translation,
            ..Default::default()
        }],
        materials: vec![Material {
            pbr_metallic_roughness: Some(MaterialPbrMetallicRoughness {
                base_color_factor: [0.5, 0.7, 0.7, 1.0],
                metallic_factor: 0.5,
                roughness_factor: 0.5,
                ..Default::default()
            }),
            ..Default::default()
        }],
        meshes: vec![Mesh {
            primitives: vec![MeshPrimitive {
                attributes: vec![("POSITION".to_string(), 0)].into_iter().collect(),
                indices: Some(1),
                material: Some(0),
                mode: PrimitiveMode::Triangles,
                ..Default::default()
            }],
            ..Default::default()
        }],
        accessors: vec![
            Accessor {
                buffer_view: Some(0),
                component_type: ComponentType::Float,
                count: vertices_count,
                min: Some(min.to_vec()),
                max: Some(max.to_vec()),
                type_: AccessorType::Vec3,
                ..Default::default()
            },
            Accessor {
                buffer_view: Some(1),
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            },
        ],
        buffer_views: vec![
            BufferView {
                byte_offset: vertices_offset as u32,
                byte_length: vertices_len as u32,
                target: Some(BufferViewTarget::ArrayBuffer),
                ..Default::default()
            },
            BufferView {
                byte_offset: indices_offset as u32,
                byte_length: indices_len as u32,
                target: Some(BufferViewTarget::ElementArrayBuffer),
                ..Default::default()
            },
        ],
        buffers: vec![Buffer {
            byte_length: bin_content.len() as u32,
            ..Default::default()
        }],
        ..Default::default()
    };

    {
        let mut json_content = serde_json::to_vec(&gltf).unwrap();

        // append padding
        json_content.extend(vec![0x20; (4 - (json_content.len() % 4)) % 4].iter());
        bin_content.extend(vec![0x0; (4 - (bin_content.len() % 4)) % 4].iter());

        let total_size = 12 + 8 + json_content.len() + 8 + bin_content.len();

        writer.write_all(b"glTF").unwrap(); // magic
        writer.write_all(&2u32.to_le_bytes()).unwrap(); // version: 2
        writer
            .write_all(&(total_size as u32).to_le_bytes())
            .unwrap(); // total size

        writer
            .write_all(&(json_content.len() as u32).to_le_bytes())
            .unwrap(); // json content
        writer.write_all(b"JSON").unwrap(); // chunk type
        writer.write_all(&json_content).unwrap(); // json content

        writer
            .write_all(&(bin_content.len() as u32).to_le_bytes())
            .unwrap(); // json content
        writer.write_all(b"BIN\0").unwrap(); // chunk type
        writer.write_all(&bin_content).unwrap(); // bin content

        writer.flush().unwrap();
    }
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
    }
}
