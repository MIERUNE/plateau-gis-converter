use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use ahash::RandomState;
use byteorder::{ByteOrder, LittleEndian};
use indexmap::{IndexMap, IndexSet};

use itertools::Itertools;
use nusamai_citygml::schema::Schema;
use nusamai_citygml::{attribute, Value};
use nusamai_gltf_json::{
    extensions, Accessor, AccessorType, Buffer, BufferView, BufferViewTarget, ComponentType, Gltf,
    Mesh, MeshPrimitive, Node, PrimitiveMode, Scene,
};

use crate::sink::gltf::attributes::{
    attributes_to_buffer, to_gltf_class, to_gltf_property_table, FeatureAttributes,
};

use super::positions::Vertex;
use super::{Buffers, TriangulatedEntity};

pub fn build_base_gltf(
    buffers: &IndexMap<String, Buffers>,
    translation: [f64; 3],
) -> (Vec<u8>, Gltf) {
    let mut vertices = IndexSet::new();
    let mut indices = Vec::new();

    for (_class_name, buffer) in buffers.iter() {
        vertices.extend(buffer.vertices.clone());
        indices.extend(buffer.indices.clone());
    }

    let (bin_content, gltf) = to_gltf(&vertices, &indices, translation);

    // let mut gltf_list = Vec::new();
    // for (_class_name, buffer) in buffers.iter() {
    //     let (bin_content, gltf) = to_gltf(&buffer.vertices, &buffer.indices, translation);
    //     bin_contents.extend(bin_content);
    //     gltf_list.push(gltf);
    // }

    // let mut base_gltf = Gltf {
    //     extensions_used: vec![
    //         "EXT_mesh_features".to_string(),
    //         "EXT_structural_metadata".to_string(),
    //     ],
    //     scenes: vec![Scene {
    //         nodes: Some(vec![0]),
    //         ..Default::default()
    //     }],
    //     nodes: vec![Node {
    //         mesh: Some(0),
    //         translation,
    //         ..Default::default()
    //     }],
    //     ..Default::default()
    // };

    // let mut vertices_offset = 0;
    // let mut indices_offset = 0;
    // let mut feature_ids_offset = 0;

    // for gltf in gltf_list {
    //     // accessorsを整列
    //     for accessor in gltf.accessors {
    //         let mut new_accessor = accessor.clone();
    //         new_accessor.buffer_view =
    //             Some(accessor.buffer_view.unwrap() + base_gltf.buffer_views.len() as u32);
    //         base_gltf.accessors.push(new_accessor);
    //     }

    //     // buffer_viewsを整列
    //     for buffer_view in gltf.buffer_views {
    //         let mut new_buffer_view = buffer_view.clone();
    //         new_buffer_view.byte_offset += bin_contents.len() as u32;
    //         base_gltf.buffer_views.push(new_buffer_view);
    //     }

    //     // meshes.primitivesを整列
    //     for mesh in gltf.meshes.iter() {
    //         for primitive in mesh.primitives.iter() {
    //             let mut new_primitive = primitive.clone();
    //             new_primitive.indices =
    //                 Some(primitive.indices.unwrap() + bin_contents.len() as u32);
    //             let mut new_attributes = IndexMap::new();
    //             for (key, value) in primitive.attributes.clone() {
    //                 new_attributes.insert(key, value + bin_contents.len() as u32);
    //             }
    //             let new_attributes: HashMap<String, u32> = new_attributes.into_iter().collect();
    //             new_primitive.attributes = new_attributes;
    //         }
    //         base_gltf.meshes.push(mesh.clone());
    //     }
    // }

    // let mut bin_content: Vec<u8> = Vec::new();
    // for content in bin_contents {
    //     bin_content.extend(content.iter());
    // }

    (bin_content, gltf)
}

pub fn to_gltf(
    vertices: &IndexSet<Vertex<u32>>,
    indices: &Vec<u32>,
    translation: [f64; 3],
) -> (Vec<u8>, Gltf) {
    let mut bin_content: Vec<u8> = Vec::new();

    // write vertices
    let vertices_offset = bin_content.len();
    let mut buf = [0; 12];
    let mut vertices_count = 0;
    for vertex in vertices.clone() {
        LittleEndian::write_u32_into(&vertex.position, &mut buf);
        bin_content.write_all(&buf).unwrap();
        vertices_count += 1;
    }
    let vertices_len: usize = bin_content.len() - vertices_offset;

    // write indices
    let indices_offset = bin_content.len();
    let mut indices_count = 0;
    for idx in indices {
        bin_content.write_all(&idx.to_le_bytes()).unwrap();
        indices_count += 1;
    }
    let indices_len = bin_content.len() - indices_offset;

    // write feature_ids
    let feature_ids_offset = bin_content.len();
    let mut feature_ids_count = 0;
    for vertex in vertices.clone() {
        bin_content
            .write_all(&vertex.feature_id.to_le_bytes())
            .unwrap();
        feature_ids_count += 1;
    }
    let feature_ids_len = bin_content.len() - feature_ids_offset;

    let mut min = [f64::MAX; 3];
    let mut max = [f64::MIN; 3];
    for vertex in vertices.iter() {
        for v in 0..3 {
            let value = f32::from_bits(vertex.position[v]) as f64;
            if value < min[v] {
                min[v] = value;
            }
            if value > max[v] {
                max[v] = value;
            }
        }
    }

    // make base gltf structure
    let gltf = Gltf {
        extensions_used: vec![
            "EXT_mesh_features".to_string(),
            "EXT_structural_metadata".to_string(),
        ],
        scenes: vec![Scene {
            nodes: Some(vec![0]),
            ..Default::default()
        }],
        nodes: vec![Node {
            mesh: Some(0),
            translation,
            ..Default::default()
        }],
        meshes: vec![Mesh {
            primitives: vec![MeshPrimitive {
                attributes: vec![
                    ("POSITION".to_string(), 0),
                    ("_FEATURE_ID_0".to_string(), 2),
                ]
                .into_iter()
                .collect(),
                indices: Some(1),
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
            Accessor {
                buffer_view: Some(2),
                component_type: ComponentType::UnsignedInt,
                count: feature_ids_count,
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
            BufferView {
                byte_offset: feature_ids_offset as u32,
                byte_length: feature_ids_len as u32,
                target: Some(BufferViewTarget::ArrayBuffer),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    bin_content.extend(vec![0x0; (4 - (bin_content.len() % 4)) % 4].iter());
    (bin_content, gltf)
}

pub fn append_gltf_extensions(
    gltf: &mut Gltf,
    bin_content: &mut Vec<u8>,
    schema: &Schema,
    entities_list: &IndexMap<String, Vec<TriangulatedEntity>>,
) {
    let mut buffer_view_length = gltf.buffer_views.len() as u32;
    let mut classes = HashMap::new();
    let mut property_tables = Vec::new();
    let mut feature_ids = Vec::new();

    let mut buffers: IndexMap<String, IndexMap<String, Vec<u8>>> = IndexMap::new();
    let mut buffer_views: Vec<BufferView> = Vec::new();

    // glTF拡張のext_structural_metadataを作成
    for (class_name, entities) in entities_list.iter() {
        let feature_count = entities.len() as u32 - 1;

        let type_def = schema.types.get::<String>(&class_name).unwrap();

        // classesを作成
        let class = to_gltf_class(&class_name, type_def);
        let classes_length = classes.len() as u32;
        classes.extend(class);

        // property_tableを作成
        let (property_table, count) =
            to_gltf_property_table(&class_name, type_def, buffer_view_length, feature_count);
        let property_tables_length = property_tables.len() as u32;

        // このタイミングでbuffer_viewも作成しないと、論理的におかしくなる
        // buffer_viewはbyte_offsetとbyte_lengthを必要とする
        // また、propertyがstringなのかどうなのか、で複数のbuffer_viewを作成するかどうか変わる
        // countの数によってbuffer_viewの個数が変更になる
        // property_table.properties.0が属性名
        // properties.1を見て、valuesの順番でソートする
        // その後、entitiesから、valuesが若い属性名を取り出し、属性値に応じてbufferを作成する
        // lengthを計算する
        // その後、bufferとstring_offsetに相当するbuffer_viewを作成する
        let mut properties = property_table.properties.iter().collect::<Vec<_>>();
        properties.sort_by(|a, b| a.1.values.cmp(&b.1.values));

        let mut buffer: IndexMap<String, Vec<u8>> = IndexMap::new();

        // 抽出するべき全ての属性がproperty_nameに入る
        for (property_name, property) in properties {
            let mut buf = Vec::new();
            let mut string_offset_buffer = Vec::new();

            // このproperty_nameに対応するbuffer_viewを作成する
            // 全ての地物から一つ取り出す
            for entity in entities {
                // 地物には複数の属性があるので、全て処理する
                for (name, value) in entity.attributes.attributes.iter() {
                    // 属性名が一致するものがあれば、ちゃんと取り出す
                    if property_name == name {
                        match value {
                            // todo: 型ごとの処理をきちんと定義する
                            Value::String(s) => {
                                if s.is_empty() {
                                    buf.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buf.write_all(s.as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            Value::Integer(i) => {
                                buf.write_all(&i.to_le_bytes()).unwrap();
                            }
                            Value::NonNegativeInteger(u) => {
                                buf.write_all(&u.to_le_bytes()).unwrap();
                            }
                            Value::Double(d) => {
                                buf.write_all(&d.to_le_bytes()).unwrap();
                            }
                            Value::Boolean(b) => {
                                let boolean: u8 = if *b { 1 } else { 0 };
                                buf.write_all(&boolean.to_le_bytes()).unwrap();
                            }
                            Value::Code(c) => {
                                let json = c.value();
                                if json.is_empty() {
                                    buf.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buf.write_all(&json.as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            Value::Measure(m) => {
                                let json = m.value();
                                buf.write_all(&json.to_le_bytes()).unwrap();
                            }
                            Value::URI(u) => {
                                let json = u.value();
                                if json.is_empty() {
                                    buf.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buf.write_all(u.value().as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            Value::Array(a) => {
                                let json = serde_json::to_string(a).unwrap();
                                if json.is_empty() {
                                    buf.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buf.write_all(&json.as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            Value::Object(o) => {
                                let json = serde_json::to_string(o).unwrap();
                                if json.is_empty() {
                                    buf.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buf.write_all(&json.as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buf.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            _ => {
                                buf.write_all(&[0u8]).unwrap();
                            }
                        }
                    } else {
                        // 一致しない場合は、property_nameが存在した場合のスキーマに従い、密なバッファを作成する
                        // propertyを確認する
                        // property.string_offsetが存在する場合は、string_offset_bufferを作成する
                        buf.write_all(&[0u8]).unwrap();
                        if !property.string_offsets.is_none() {
                            string_offset_buffer
                                .write_all(&(buf.len() as u32).to_le_bytes())
                                .unwrap();
                        }
                    }
                }
            }
            buffer.insert(property_name.clone(), buf);
            if !string_offset_buffer.is_empty() {
                buffer.insert(
                    format!("{}_string_offset", property_name),
                    string_offset_buffer,
                );
            }
        }

        // buffer_viewを作成
        for (property_name, content) in buffer.iter() {
            let byte_offset = bin_content.len();
            let byte_length = content.len();

            bin_content.extend(content.iter());

            buffer_views.push(BufferView {
                byte_offset: byte_offset as u32,
                byte_length: byte_length as u32,
                ..Default::default()
            });
        }

        buffers.insert(class_name.clone(), buffer);

        property_tables.push(property_table);

        // feature_idsを作成
        let feature_id = extensions::mesh::ext_mesh_features::FeatureId {
            attribute: Some(classes_length),
            feature_count,
            property_table: Some(property_tables_length),
            ..Default::default()
        };
        feature_ids.push(feature_id);

        buffer_view_length = count;
    }

    let extensions = extensions::gltf::Gltf {
        ext_structural_metadata: Some(
            extensions::gltf::ext_structural_metadata::ExtStructuralMetadata {
                schema: Some(extensions::gltf::ext_structural_metadata::Schema {
                    classes,
                    ..Default::default()
                }),
                property_tables: Some(property_tables.clone()),
                ..Default::default()
            },
        ),
        ..Default::default()
    };

    let mesh_primitive_extensions = Some(extensions::mesh::MeshPrimitive {
        ext_mesh_features: Some(extensions::mesh::ext_mesh_features::ExtMeshFeatures {
            feature_ids,
            ..Default::default()
        }),
        ..Default::default()
    });
    gltf.meshes[0].primitives[0].extensions = mesh_primitive_extensions;

    // 作成したproperty_tableやclassesで指定した順にbufferに属性を追加していく
    // また、属性情報に対応したbuffer_viewも追加する
    // let mut attributes: Vec<FeatureAttributes> = Vec::new();
    // for (_, entities) in entities_list.iter() {
    //     for entity in entities.iter() {
    //         attributes.push(entity.attributes.clone());
    //     }
    // }

    // let attributes_bin_contents = attributes_to_buffer(&property_tables, entities_list);
    // let mut buffer_views: Vec<BufferView> = Vec::new();
    // for (_, content) in attributes_bin_contents.iter() {
    //     let byte_offset = bin_content.len();
    //     let byte_length = content.len();

    //     bin_content.extend(content.iter());

    //     buffer_views.push(BufferView {
    //         byte_offset: byte_offset as u32,
    //         byte_length: byte_length as u32,
    //         ..Default::default()
    //     });
    // }

    gltf.buffer_views.extend(buffer_views);

    gltf.extensions = Some(extensions);

    // Add after all binary buffers have been written
    let buffers = vec![Buffer {
        byte_length: bin_content.len() as u32,
        ..Default::default()
    }];
    gltf.buffers = buffers;
}

pub fn write_gltf<W: Write>(gltf: Gltf, mut bin_content: Vec<u8>, mut writer: W) {
    let mut json_content = serde_json::to_vec(&gltf).unwrap();

    // append padding
    json_content.extend(vec![0x20; (4 - (json_content.len() % 4)) % 4].iter());
    bin_content.extend(vec![0x0; (4 - (bin_content.len() % 4)) % 4].iter());

    let total_size = 12 + 8 + json_content.len() + 8 + bin_content.len();

    writer.write_all(b"glTF").unwrap();
    // magic
    writer.write_all(&2u32.to_le_bytes()).unwrap();
    // version: 2
    writer
        .write_all(&(total_size as u32).to_le_bytes())
        .unwrap();
    // total size

    writer
        .write_all(&(json_content.len() as u32).to_le_bytes())
        .unwrap();
    // json content
    writer.write_all(b"JSON").unwrap();
    // chunk type
    writer.write_all(&json_content).unwrap();
    // json content

    writer
        .write_all(&(bin_content.len() as u32).to_le_bytes())
        .unwrap();
    // json content
    writer.write_all(b"BIN\0").unwrap();
    // chunk type
    writer.write_all(&bin_content).unwrap();
    // bin content

    writer.flush().unwrap();
}

// This is the code to verify the operation with Cesium
pub fn write_3dtiles(bounding_volume: [f64; 6], output_path: &PathBuf) {
    // write 3DTiles
    let tileset_path = output_path.with_file_name("tileset.json");
    let content_uri = output_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    let tileset = nusamai_3dtiles_json::tileset::Tileset {
        geometric_error: 1e+100,
        asset: nusamai_3dtiles_json::tileset::Asset {
            version: "1.1".to_string(),
            ..Default::default()
        },
        root: nusamai_3dtiles_json::tileset::Tile {
            bounding_volume: nusamai_3dtiles_json::tileset::BoundingVolume {
                region: Some(bounding_volume),
                ..Default::default()
            },
            content: Some(nusamai_3dtiles_json::tileset::Content {
                uri: content_uri,
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tileset_file = File::create(&tileset_path).unwrap();
    let tileset_writer = BufWriter::with_capacity(1024 * 1024, &mut tileset_file);
    serde_json::to_writer_pretty(tileset_writer, &tileset).unwrap();
}
