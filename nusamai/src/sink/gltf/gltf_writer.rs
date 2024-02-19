use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use indexmap::IndexMap;

use nusamai_citygml::schema::Schema;
use nusamai_citygml::Measure;
use nusamai_citygml::Value;
use nusamai_gltf_json::{
    extensions, Accessor, AccessorType, Buffer, BufferView, BufferViewTarget, ComponentType, Gltf,
    Mesh, MeshPrimitive, Node, PrimitiveMode, Scene,
};

use crate::sink::gltf::attributes::{to_gltf_class, to_gltf_property_table};

use super::{Buffers, TriangulatedEntity};

pub fn build_base_gltf(
    buffers: &IndexMap<String, Buffers>,
    translation: [f64; 3],
) -> (Vec<u8>, Gltf) {
    // make base gltf structure
    let mut gltf = Gltf {
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
                attributes: vec![("POSITION".to_string(), 0)].into_iter().collect(),
                indices: Some(1),
                mode: PrimitiveMode::Triangles,
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };

    let mut bin_content: Vec<u8> = Vec::new();
    let mut mesh_primitives = Vec::new();

    for (feature_ids_length, (_, buffer)) in buffers.iter().enumerate() {
        let vertices = buffer.vertices.clone();
        let indices = buffer.indices.clone();

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
        for idx in indices.clone() {
            bin_content.write_all(&idx.to_le_bytes()).unwrap();
            indices_count += 1;
        }
        let indices_len = bin_content.len() - indices_offset;

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

        let buffer_views = vec![
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
        ];
        gltf.buffer_views.extend(buffer_views);

        let accessors = vec![
            Accessor {
                buffer_view: Some(gltf.buffer_views.len() as u32 - 2),
                component_type: ComponentType::Float,
                count: vertices_count,
                min: Some(min.to_vec()),
                max: Some(max.to_vec()),
                type_: AccessorType::Vec3,
                ..Default::default()
            },
            Accessor {
                buffer_view: Some(gltf.buffer_views.len() as u32 - 1),
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            },
        ];
        gltf.accessors.extend(accessors);

        let buffer_view_length = gltf.buffer_views.len() as u32;

        // write feature_ids
        let feature_ids_offset = bin_content.len();
        let mut feature_ids_count = 0;
        for vertex in buffer.vertices.clone() {
            bin_content
                .write_all(&vertex.feature_id.to_le_bytes())
                .unwrap();
            feature_ids_count += 1;
        }
        let feature_ids_len = bin_content.len() - feature_ids_offset;

        let buffer_view = BufferView {
            byte_offset: feature_ids_offset as u32,
            byte_length: feature_ids_len as u32,
            byte_stride: Some(4),
            target: Some(BufferViewTarget::ArrayBuffer),
            ..Default::default()
        };
        gltf.buffer_views.push(buffer_view);

        let feature_id_min = buffer.vertices.iter().map(|v| v.feature_id).min().unwrap();
        let feature_id_max = buffer.vertices.iter().map(|v| v.feature_id).max().unwrap();
        let accessor = Accessor {
            buffer_view: Some(buffer_view_length),
            component_type: ComponentType::UnsignedInt,
            count: feature_ids_count,
            type_: AccessorType::Scalar,
            min: Some(vec![feature_id_min as f64]),
            max: Some(vec![feature_id_max as f64]),
            ..Default::default()
        };
        gltf.accessors.push(accessor);

        let primitives = vec![MeshPrimitive {
            attributes: vec![
                ("POSITION".to_string(), buffer_view_length - 2),
                (
                    format!("_FEATURE_ID_{}", feature_ids_length),
                    buffer_view_length,
                ),
            ]
            .into_iter()
            .collect(),
            indices: Some(buffer_view_length - 1),
            mode: PrimitiveMode::Triangles,
            ..Default::default()
        }];
        mesh_primitives.extend(primitives);
    }
    gltf.meshes[0].primitives = mesh_primitives;

    (bin_content, gltf)
}

// value to bytes
trait ToBytes {
    fn write_to_bytes(&self, buffer: &mut Vec<u8>);
}

impl ToBytes for i64 {
    fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
        buffer.write_i64::<LittleEndian>(*self).unwrap();
    }
}

impl ToBytes for u64 {
    fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
        buffer.write_u64::<LittleEndian>(*self).unwrap();
    }
}

impl ToBytes for f64 {
    fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
        buffer.write_f64::<LittleEndian>(*self).unwrap();
    }
}

impl ToBytes for bool {
    fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
        let value = if *self { 1 } else { 0 };
        buffer.write_i32::<LittleEndian>(value).unwrap();
    }
}

impl ToBytes for Measure {
    fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
        let value = self.value();
        buffer.write_f64::<LittleEndian>(value).unwrap();
    }
}

impl ToBytes for Value {
    fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
        match self {
            Value::Integer(i) => i.write_to_bytes(buffer),
            Value::NonNegativeInteger(u) => u.write_to_bytes(buffer),
            Value::Double(d) => d.write_to_bytes(buffer),
            Value::Boolean(b) => b.write_to_bytes(buffer),
            Value::Measure(m) => m.write_to_bytes(buffer),
            // todo: 他の型にも対応する
            _ => {}
        }
    }
}

trait ToBuffer {
    fn write_to_buffer(&self, buffer: &mut Vec<u8>, string_offset_buffer: &mut Vec<u32>);
}

impl ToBuffer for Value {
    fn write_to_buffer(&self, buffer: &mut Vec<u8>, string_offset_buffer: &mut Vec<u32>) {
        match self {
            Value::String(s) => {
                string_offset_buffer.push(buffer.len() as u32);
                buffer.write_all(s.as_bytes()).unwrap();
            }
            Value::Code(c) => {
                let json = c.value();
                string_offset_buffer.push(buffer.len() as u32);
                buffer.write_all(json.as_bytes()).unwrap();
            }
            Value::Array(a) => {
                let json = serde_json::to_string(a).unwrap();
                string_offset_buffer.push(buffer.len() as u32);
                buffer.write_all(json.as_bytes()).unwrap();
            }
            Value::Object(o) => {
                let json = serde_json::to_string(o).unwrap();
                string_offset_buffer.push(buffer.len() as u32);
                buffer.write_all(json.as_bytes()).unwrap();
            }
            _ => self.write_to_bytes(buffer),
        }
    }
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

    let mut buffer_views: Vec<BufferView> = Vec::new();

    // glTF拡張のext_structural_metadataを作成
    // トップレベルのクラスごとに処理
    for (class_name, entities) in entities_list.iter() {
        let feature_count = entities.len() as u32;

        let type_def = schema.types.get::<String>(class_name).unwrap();

        // classesを作成
        let class = to_gltf_class(class_name, type_def);
        let classes_length = classes.len() as u32;
        classes.extend(class.clone());

        // property_tableを作成
        let (property_table, count) =
            to_gltf_property_table(class_name, type_def, buffer_view_length, feature_count);
        let property_tables_length = property_tables.len() as u32;

        // buffer_viewを作成
        let mut properties = property_table.properties.iter().collect::<Vec<_>>();
        properties.sort_by(|a, b| a.1.values.cmp(&b.1.values));

        // 抽出するべき全ての属性がproperty_nameに入る
        for (property_name, _) in properties {
            // property_nameに対応するbuffer_viewを作成する

            // todo: データ型ごとの対応をする
            // 配列の場合は、array_offsetを書き込む必要があるなど
            let mut buffer: Vec<u8> = Vec::new();
            let mut string_offset_buffer: Vec<u32> = Vec::new();

            // このproperty_nameに対応するbuffer_viewを作成する
            for entity in entities {
                // 個別のentityから、property_nameに対応する属性を取り出す
                // なければ、無視する
                let attribute_name_list = entity.attributes.attributes.keys().collect::<Vec<_>>();
                let is_hit = attribute_name_list.contains(&property_name);
                // classから型を取り出す
                let p = class
                    .get(class_name)
                    .unwrap()
                    .properties
                    .get(property_name)
                    .unwrap();
                let property_type = &p.type_;
                let component_type = &p.component_type;

                if !is_hit {
                    match property_type {
                        extensions::gltf::ext_structural_metadata::ClassPropertyType::String => {
                            string_offset_buffer.push(buffer.len() as u32);
                            buffer.write_all("0".to_string().as_bytes()).unwrap();
                        }
                        extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean => {
                            buffer.write_i32::<LittleEndian>(0).unwrap();
                        }
                        _ => {}
                    }

                    // component_typeがSomeなら値を取り出す
                    if let Some(component_type) = component_type {
                        let component_type = component_type.clone();
                        match component_type {
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int8 => {
                                buffer.write_i8(0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::UInt8 => {
                                buffer.write_u8(0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int16 => {
                                buffer.write_i16::<LittleEndian>(0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::UInt16 => {
                                buffer.write_u16::<LittleEndian>(0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32 => {
                                buffer.write_i32::<LittleEndian>(0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::UInt32 => {
                                buffer.write_u32::<LittleEndian>(0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float32 => {
                                buffer.write_f32::<LittleEndian>(0.0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float64 => {
                                buffer.write_f64::<LittleEndian>(0.0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int64 => {
                                buffer.write_i64::<LittleEndian>(0).unwrap();
                            },
                            extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::UInt64 => {
                                buffer.write_u64::<LittleEndian>(0).unwrap();
                            },
                        }
                    };
                    continue;
                }

                // 属性名が一致するものがあれば、ちゃんと取り出す
                // entity.attributes.attributes.get(&property_name)が効かないので、for文で取り出す
                if let Some((_, value)) = entity
                    .attributes
                    .attributes
                    .iter()
                    .find(|(name, _)| *name == property_name)
                {
                    // bufferに書き込み
                    value.write_to_buffer(&mut buffer, &mut string_offset_buffer);
                }
            }
            // offsetには文字列の最後にもインデックスが必要
            if !string_offset_buffer.is_empty() {
                string_offset_buffer.push(buffer.len() as u32);
            }

            let byte_offset = bin_content.len();
            let byte_length = buffer.len();

            bin_content.extend(buffer.iter());

            buffer_views.push(BufferView {
                byte_offset: byte_offset as u32,
                byte_length: byte_length as u32,
                ..Default::default()
            });

            if !string_offset_buffer.is_empty() {
                let byte_offset = bin_content.len();
                let byte_length = string_offset_buffer.len() as u32 * 4;

                bin_content.extend(string_offset_buffer.iter().flat_map(|x| x.to_le_bytes()));

                buffer_views.push(BufferView {
                    byte_offset: byte_offset as u32,
                    byte_length,
                    ..Default::default()
                });
            }
        }

        property_tables.push(property_table);

        // feature_idsを作成
        let feature_id = extensions::mesh::ext_mesh_features::FeatureId {
            attribute: Some(classes_length),
            feature_count,
            property_table: Some(property_tables_length),
            ..Default::default()
        };

        buffer_view_length = count;

        let mesh_primitive_extensions = Some(extensions::mesh::MeshPrimitive {
            ext_mesh_features: Some(extensions::mesh::ext_mesh_features::ExtMeshFeatures {
                feature_ids: vec![feature_id],
                ..Default::default()
            }),
            ..Default::default()
        });
        gltf.meshes[0].primitives[property_tables.len() - 1].extensions = mesh_primitive_extensions;
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
pub fn write_3dtiles(bounding_volume: [f64; 6], output_path: &Path) {
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

    let mut tileset_file = File::create(tileset_path).unwrap();
    let tileset_writer = BufWriter::with_capacity(1024 * 1024, &mut tileset_file);
    serde_json::to_writer_pretty(tileset_writer, &tileset).unwrap();
}
