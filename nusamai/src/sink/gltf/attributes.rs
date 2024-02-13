use std::{collections::HashMap, io::Write};

use ahash::RandomState;
use hashbrown::HashSet;
use indexmap::IndexMap;

use nusamai_citygml::{
    schema::{Schema, TypeDef, TypeRef},
    Value,
};
use nusamai_gltf_json::extensions;

use super::TriangulatedEntity;

#[derive(Debug, Clone, Default)]
pub struct GltfPropertyType {
    pub class_name: String,
    pub property_name: String,
    pub class_property_type: extensions::gltf::ext_structural_metadata::ClassPropertyType,
    pub component_type:
        Option<extensions::gltf::ext_structural_metadata::ClassPropertyComponentType>,
}

// Attributes per vertex id
#[derive(Debug, Clone)]
pub struct FeatureAttributes {
    pub class_name: String,
    pub feature_id: u32,
    pub attributes: IndexMap<String, Value, RandomState>,
}

fn to_gltf_schema(type_ref: &TypeRef) -> GltfPropertyType {
    // todo: 型定義を正確に行う
    match type_ref {
        TypeRef::String => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
            ..Default::default()
        },
        TypeRef::Integer => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32,
            ),
            ..Default::default()
        },
        TypeRef::Double => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float64,
            ),
            ..Default::default()
        },
        TypeRef::Boolean => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean,
            component_type: None,
            ..Default::default()
        },
        TypeRef::Measure => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32,
            ),
            ..Default::default()
        },
        TypeRef::Code => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
            ..Default::default()
        },
        TypeRef::NonNegativeInteger => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32,
            ),
            ..Default::default()
        },
        TypeRef::JsonString => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
            ..Default::default()
        },
        TypeRef::Point => GltfPropertyType {
            class_property_type: extensions::gltf::ext_structural_metadata::ClassPropertyType::Vec3,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float64,
            ),
            ..Default::default()
        },
        TypeRef::Named(_) => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
            ..Default::default()
        },
        // todo: その他の型についても対応（暫定的にStringとして取り扱う）
        _ => GltfPropertyType {
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
            ..Default::default()
        },
    }
}

pub fn to_gltf_class(
    class_name: &String,
    type_def: &TypeDef,
) -> HashMap<String, extensions::gltf::ext_structural_metadata::Class> {
    let mut gltf_property_types = Vec::new();

    match type_def {
        TypeDef::Feature(f) => {
            for (name, attr) in &f.attributes {
                let mut property_type = to_gltf_schema(&attr.type_ref);
                property_type.class_name = class_name.clone();
                property_type.property_name = name.clone();
                gltf_property_types.push(property_type);
            }
        }
        // todo: feature 以外の型も実装する
        TypeDef::Data(_) => unimplemented!(),
        TypeDef::Property(_) => unimplemented!(),
    }

    let mut class_properties = HashMap::new();
    for gltf_property_type in gltf_property_types.iter() {
        // Create Schema.classes
        class_properties.insert(
            gltf_property_type.property_name.clone(),
            extensions::gltf::ext_structural_metadata::ClassProperty {
                description: Some(gltf_property_type.property_name.clone()),
                type_: gltf_property_type.class_property_type.clone(),
                component_type: gltf_property_type.component_type.clone(),
                ..Default::default()
            },
        );
    }

    let mut class: HashMap<String, extensions::gltf::ext_structural_metadata::Class> =
        HashMap::new();
    class.insert(
        class_name.clone(),
        extensions::gltf::ext_structural_metadata::Class {
            name: Some(class_name.clone()),
            description: None,
            properties: class_properties.clone(),
            ..Default::default()
        },
    );

    class
}

pub fn to_gltf_property_table(
    class_name: &String,
    schema: &TypeDef,
    buffer_view_length: u32,
    feature_count: u32,
) -> (
    extensions::gltf::ext_structural_metadata::PropertyTable,
    u32,
) {
    // Create Schema.property_tables
    let mut property_table: extensions::gltf::ext_structural_metadata::PropertyTable =
        extensions::gltf::ext_structural_metadata::PropertyTable {
            class: class_name.clone(),
            properties: HashMap::new(),
            count: feature_count,
            ..Default::default()
        };

    let mut buffer_view_length = buffer_view_length;
    match schema {
        TypeDef::Feature(f) => {
            for (name, attr) in &f.attributes {
                let property_type = to_gltf_schema(&attr.type_ref);
                // property_typeによって、PropertyTablePropertyの構造が変化する
                // todo: その他の型についても対応
                match property_type.class_property_type {
                    extensions::gltf::ext_structural_metadata::ClassPropertyType::String => {
                        property_table.properties.insert(
                            name.clone(),
                            extensions::gltf::ext_structural_metadata::PropertyTableProperty {
                                values: buffer_view_length,
                                string_offsets: Some(buffer_view_length + 1),
                                ..Default::default()
                            },
                        );
                        buffer_view_length += 2;
                    }
                    extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar => {
                        property_table.properties.insert(
                            name.clone(),
                            extensions::gltf::ext_structural_metadata::PropertyTableProperty {
                                values: buffer_view_length,
                                ..Default::default()
                            },
                        );
                        buffer_view_length += 1;
                    }
                    extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean => {
                        property_table.properties.insert(
                            name.clone(),
                            extensions::gltf::ext_structural_metadata::PropertyTableProperty {
                                values: buffer_view_length,
                                ..Default::default()
                            },
                        );
                        buffer_view_length += 1;
                    }
                    _ => unimplemented!(),
                }
            }
        }
        // todo: feature 以外の型も実装する
        TypeDef::Data(_) => unimplemented!(),
        TypeDef::Property(_) => unimplemented!(),
    }

    (property_table, buffer_view_length)
}

pub fn attributes_to_buffer(
    property_tables: &Vec<extensions::gltf::ext_structural_metadata::PropertyTable>,
    entity_list: &IndexMap<String, Vec<TriangulatedEntity>>,
) -> IndexMap<String, IndexMap<String, Vec<u8>>> {
    // 属性名がキーで、値がバッファのマップを作成する
    let mut buffers: IndexMap<String, IndexMap<String, Vec<u8>>> = IndexMap::new();

    // クラスごとの属性名を取得していく
    for property_table in property_tables {
        let class_name = &property_table.class;
        println!("\nprocessing {:?}...\n", class_name);

        let target_entities = entity_list.get(class_name).unwrap();

        for (property_name, property) in &property_table.properties {
            let mut buffer: Vec<u8> = Vec::new();
            let mut string_offset_buffer: Vec<u8> = Vec::new();

            for entity in target_entities {
                for (name, value) in entity.attributes.attributes.iter() {
                    if property_name == name {
                        println!("found: {:?}", &property_name);
                        println!("name: {:?}", name);
                        println!("value: {:?}", value);
                        match value {
                            // todo: 型ごとの処理をきちんと定義する
                            Value::String(s) => {
                                if s.is_empty() {
                                    buffer.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buffer.write_all(s.as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            Value::Integer(i) => {
                                buffer.write_all(&i.to_le_bytes()).unwrap();
                            }
                            Value::NonNegativeInteger(u) => {
                                buffer.write_all(&u.to_le_bytes()).unwrap();
                            }
                            Value::Double(d) => {
                                buffer.write_all(&d.to_le_bytes()).unwrap();
                            }
                            Value::Boolean(b) => {
                                let buf: u8 = if *b { 1 } else { 0 };
                                buffer.write_all(&buf.to_le_bytes()).unwrap();
                            }
                            Value::Code(c) => {
                                let json = c.value();
                                if json.is_empty() {
                                    buffer.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buffer.write_all(&json.as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            Value::Measure(m) => {
                                let json = m.value();
                                buffer.write_all(&json.to_le_bytes()).unwrap();
                            }
                            Value::Point(_) => {
                                // todo: implement
                            }
                            Value::URI(u) => {
                                let json = u.value();
                                if json.is_empty() {
                                    buffer.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buffer.write_all(u.value().as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            Value::Date(_) => {
                                // todo: implement
                            }
                            Value::Array(a) => {
                                let json = serde_json::to_string(a).unwrap();
                                if json.is_empty() {
                                    buffer.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buffer.write_all(&json.as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                            Value::Object(o) => {
                                let json = serde_json::to_string(o).unwrap();
                                if json.is_empty() {
                                    buffer.write_all(&[0u8]).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                } else {
                                    buffer.write_all(&json.as_bytes()).unwrap();
                                    string_offset_buffer
                                        .write_all(&(buffer.len() as u32).to_le_bytes())
                                        .unwrap();
                                }
                            }
                        }
                    } else {
                        println!("not found: {:?}", &property_name);
                        println!("property: {:?}", &property);
                    }
                }
            }
        }

        let mut buffer: Vec<u8> = Vec::new();
        let mut string_offset_buffer: Vec<u8> = Vec::new();

        // スキーマの順番に従って、属性名を抽出
        // for property_name in property_names {
        //     for a in entity_list {
        //         // todo: キー名が正しいのに、値が取得できない
        //         println!(
        //             "value: {:?}",
        //             a.attributes.get("vegetationDataQualityAttribute").unwrap()
        //         );
        //         println!("value: {:?}", a.attributes.get("class").unwrap());
        //         println!("value: {:?}", a.attributes.get("averageHeight").unwrap());

        //         if let Some(value) = a.attributes.get(property_name) {
        //             println!("found: {:?}", &property_name);
        //             match value {
        //                 // todo: 型ごとの処理をきちんと定義する
        //                 Value::String(s) => {
        //                     if s.is_empty() {
        //                         buffer.write_all(&[0u8]).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     } else {
        //                         buffer.write_all(s.as_bytes()).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     }
        //                 }
        //                 Value::Integer(i) => {
        //                     buffer.write_all(&i.to_le_bytes()).unwrap();
        //                 }
        //                 Value::NonNegativeInteger(u) => {
        //                     buffer.write_all(&u.to_le_bytes()).unwrap();
        //                 }
        //                 Value::Double(d) => {
        //                     buffer.write_all(&d.to_le_bytes()).unwrap();
        //                 }
        //                 Value::Boolean(b) => {
        //                     let buf: u8 = if *b { 1 } else { 0 };
        //                     buffer.write_all(&buf.to_le_bytes()).unwrap();
        //                 }
        //                 Value::Code(c) => {
        //                     let json = c.value();
        //                     if json.is_empty() {
        //                         buffer.write_all(&[0u8]).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     } else {
        //                         buffer.write_all(&json.as_bytes()).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     }
        //                 }
        //                 Value::Measure(m) => {
        //                     let json = m.value();
        //                     buffer.write_all(&json.to_le_bytes()).unwrap();
        //                 }
        //                 Value::Point(_) => {
        //                     // todo: implement
        //                 }
        //                 Value::URI(u) => {
        //                     let json = u.value();
        //                     if json.is_empty() {
        //                         buffer.write_all(&[0u8]).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     } else {
        //                         buffer.write_all(u.value().as_bytes()).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     }
        //                 }
        //                 Value::Date(_) => {
        //                     // todo: implement
        //                 }
        //                 Value::Array(a) => {
        //                     let json = serde_json::to_string(a).unwrap();
        //                     if json.is_empty() {
        //                         buffer.write_all(&[0u8]).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     } else {
        //                         buffer.write_all(&json.as_bytes()).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     }
        //                 }
        //                 Value::Object(o) => {
        //                     let json = serde_json::to_string(o).unwrap();
        //                     if json.is_empty() {
        //                         buffer.write_all(&[0u8]).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     } else {
        //                         buffer.write_all(&json.as_bytes()).unwrap();
        //                         string_offset_buffer
        //                             .write_all(&(buffer.len() as u32).to_le_bytes())
        //                             .unwrap();
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     // 該当する属性があるかどうかを確認する
        // }
    }
    //     for attr in attributes {
    //         if let Some(value) = attr.attributes.get(&property_name) {
    //             match value {
    //                 // todo: 型ごとの処理をきちんと定義する
    //                 Value::String(s) => {
    //                     if s.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(s.as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //                 Value::Integer(i) => {
    //                     buffer.write_all(&i.to_le_bytes()).unwrap();
    //                 }
    //                 Value::NonNegativeInteger(u) => {
    //                     buffer.write_all(&u.to_le_bytes()).unwrap();
    //                 }
    //                 Value::Double(d) => {
    //                     buffer.write_all(&d.to_le_bytes()).unwrap();
    //                 }
    //                 Value::Boolean(b) => {
    //                     let buf: u8 = if *b { 1 } else { 0 };
    //                     buffer.write_all(&buf.to_le_bytes()).unwrap();
    //                 }
    //                 Value::Code(c) => {
    //                     let json = c.value();
    //                     if json.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(&json.as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //                 Value::Measure(m) => {
    //                     let json = m.value();
    //                     buffer.write_all(&json.to_le_bytes()).unwrap();
    //                 }
    //                 Value::Point(_) => {
    //                     // todo: implement
    //                 }
    //                 Value::URI(u) => {
    //                     let json = u.value();
    //                     if json.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(u.value().as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //                 Value::Date(_) => {
    //                     // todo: implement
    //                 }
    //                 Value::Array(a) => {
    //                     let json = serde_json::to_string(a).unwrap();
    //                     if json.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(&json.as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //                 Value::Object(o) => {
    //                     let json = serde_json::to_string(o).unwrap();
    //                     if json.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(&json.as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //             }
    //         } else {
    //             // If defined in the schema but not in the entity
    //             match p {
    //                 GltfPropertyType {
    //                     class_property_type:
    //                         extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
    //                     ..
    //                 } => {
    //                     buffer.write_all(&[0u8]).unwrap();
    //                     string_offset_buffer
    //                         .write_all(&(buffer.len() as u32).to_le_bytes())
    //                         .unwrap();
    //                 }
    //                 GltfPropertyType {
    //                     class_property_type:
    //                         extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
    //                     ..
    //                 } => {
    //                     buffer.write_all(&[0u8; 4]).unwrap();
    //                 }
    //                 GltfPropertyType {
    //                     class_property_type:
    //                         extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean,
    //                     ..
    //                 } => {
    //                     buffer.write_all(&[0u8]).unwrap();
    //                 }
    //                 _ => {
    //                     // 全てJSON Stringとして扱う
    //                     buffer.write_all(&[0u8]).unwrap();
    //                     string_offset_buffer
    //                         .write_all(&(buffer.len() as u32).to_le_bytes())
    //                         .unwrap();
    //                 }
    //             }
    //         }
    //     }

    //     let mut value = IndexMap::new();
    //     value.insert(p.property_name.clone(), buffer);
    //     buffers.insert(current_class_name.clone(), value);
    //     // todo: array_offset_bufferの対応を実装する
    //     if !string_offset_buffer.is_empty() {
    //         let mut value = IndexMap::new();
    //         value.insert(
    //             p.property_name.clone() + "_string_offsets",
    //             string_offset_buffer,
    //         );
    //         buffers.insert(current_class_name.clone() + "_string_offsets", value);
    //     }
    // }

    // for p in gltf_properties {
    //     let mut buffer: Vec<u8> = Vec::new();
    //     let mut string_offset_buffer: Vec<u8> = Vec::new();
    //     // let mut array_offset_buffer: Vec<u32> = Vec::new();

    //     let mut current_class_name = &"".to_string();

    //     // スキーマの順番に従って、全ての地物から特定の属性のみを取り出し、バイナリに変換する
    //     // それを、対象クラス全ての属性に対して行う
    //     for attr in attributes {
    //         current_class_name = &attr.class_name;
    //         if let Some(value) = attr.attributes.get(&p.property_name) {
    //             match value {
    //                 // todo: 型ごとの処理をきちんと定義する
    //                 Value::String(s) => {
    //                     if s.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(s.as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //                 Value::Integer(i) => {
    //                     buffer.write_all(&i.to_le_bytes()).unwrap();
    //                 }
    //                 Value::NonNegativeInteger(u) => {
    //                     buffer.write_all(&u.to_le_bytes()).unwrap();
    //                 }
    //                 Value::Double(d) => {
    //                     buffer.write_all(&d.to_le_bytes()).unwrap();
    //                 }
    //                 Value::Boolean(b) => {
    //                     let buf: u8 = if *b { 1 } else { 0 };
    //                     buffer.write_all(&buf.to_le_bytes()).unwrap();
    //                 }
    //                 Value::Code(c) => {
    //                     let json = c.value();
    //                     if json.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(&json.as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //                 Value::Measure(m) => {
    //                     let json = m.value();
    //                     buffer.write_all(&json.to_le_bytes()).unwrap();
    //                 }
    //                 Value::Point(_) => {
    //                     // todo: implement
    //                 }
    //                 Value::URI(u) => {
    //                     let json = u.value();
    //                     if json.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(u.value().as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //                 Value::Date(_) => {
    //                     // todo: implement
    //                 }
    //                 Value::Array(a) => {
    //                     let json = serde_json::to_string(a).unwrap();
    //                     if json.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(&json.as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //                 Value::Object(o) => {
    //                     let json = serde_json::to_string(o).unwrap();
    //                     if json.is_empty() {
    //                         buffer.write_all(&[0u8]).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     } else {
    //                         buffer.write_all(&json.as_bytes()).unwrap();
    //                         string_offset_buffer
    //                             .write_all(&(buffer.len() as u32).to_le_bytes())
    //                             .unwrap();
    //                     }
    //                 }
    //             }
    //         } else {
    //             // If defined in the schema but not in the entity
    //             match p {
    //                 GltfPropertyType {
    //                     class_property_type:
    //                         extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
    //                     ..
    //                 } => {
    //                     buffer.write_all(&[0u8]).unwrap();
    //                     string_offset_buffer
    //                         .write_all(&(buffer.len() as u32).to_le_bytes())
    //                         .unwrap();
    //                 }
    //                 GltfPropertyType {
    //                     class_property_type:
    //                         extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
    //                     ..
    //                 } => {
    //                     buffer.write_all(&[0u8; 4]).unwrap();
    //                 }
    //                 GltfPropertyType {
    //                     class_property_type:
    //                         extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean,
    //                     ..
    //                 } => {
    //                     buffer.write_all(&[0u8]).unwrap();
    //                 }
    //                 _ => {
    //                     // 全てJSON Stringとして扱う
    //                     buffer.write_all(&[0u8]).unwrap();
    //                     string_offset_buffer
    //                         .write_all(&(buffer.len() as u32).to_le_bytes())
    //                         .unwrap();
    //                 }
    //             }
    //         }
    //     }

    //     let mut value = IndexMap::new();
    //     value.insert(p.property_name.clone(), buffer);
    //     buffers.insert(current_class_name.clone(), value);
    //     // todo: array_offset_bufferの対応を実装する
    //     if !string_offset_buffer.is_empty() {
    //         let mut value = IndexMap::new();
    //         value.insert(
    //             p.property_name.clone() + "_string_offsets",
    //             string_offset_buffer,
    //         );
    //         buffers.insert(current_class_name.clone() + "_string_offsets", value);
    //     }
    // }
    // todo: buffersの数がpropertyの数と合わない
    buffers
}

#[cfg(test)]
mod tests {
    use ahash::RandomState;
    use indexmap::IndexMap;
    use nusamai_citygml::schema::FeatureTypeDef;

    use super::*;

    #[test]
    fn test_to_gltf_schema() {
        let type_ref = TypeRef::String;
        let gltf_property_type = to_gltf_schema(&type_ref);
        assert_eq!(
            gltf_property_type.class_property_type,
            extensions::gltf::ext_structural_metadata::ClassPropertyType::String
        );

        let type_ref = TypeRef::Integer;
        let gltf_property_type = to_gltf_schema(&type_ref);
        assert_eq!(
            gltf_property_type.class_property_type,
            extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar
        );
        assert_eq!(
            gltf_property_type.component_type,
            Some(extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32)
        );

        let type_ref = TypeRef::Double;
        let gltf_property_type = to_gltf_schema(&type_ref);
        assert_eq!(
            gltf_property_type.class_property_type,
            extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar
        );
        assert_eq!(
            gltf_property_type.component_type,
            Some(extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float64)
        );

        let type_ref = TypeRef::Boolean;
        let gltf_property_type = to_gltf_schema(&type_ref);
        assert_eq!(
            gltf_property_type.class_property_type,
            extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean
        );

        let type_ref = TypeRef::Measure;
        let gltf_property_type = to_gltf_schema(&type_ref);
        assert_eq!(
            gltf_property_type.class_property_type,
            extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar
        );
        assert_eq!(
            gltf_property_type.component_type,
            Some(extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32)
        );
    }

    #[test]
    fn test_to_gltf_classes() {
        let class_name = "Building".to_string();
        let attribute = TypeRef::String;
        let mut attributes: IndexMap<String, nusamai_citygml::schema::Attribute, RandomState> =
            IndexMap::default();

        attributes.insert(
            class_name.clone(),
            nusamai_citygml::schema::Attribute {
                type_ref: attribute,
                ..Default::default()
            },
        );

        let feature_type_def = TypeDef::Feature(FeatureTypeDef {
            attributes,
            ..Default::default()
        });

        let classes = to_gltf_class(&class_name, &feature_type_def);
        assert_eq!(classes.len(), 1);
    }

    #[test]
    fn test_to_gltf_property_tables() {
        let class_name = "Building".to_string();
        let attribute = TypeRef::String;
        let mut attributes: IndexMap<String, nusamai_citygml::schema::Attribute, RandomState> =
            IndexMap::default();

        attributes.insert(
            class_name.clone(),
            nusamai_citygml::schema::Attribute {
                type_ref: attribute,
                ..Default::default()
            },
        );

        let feature_type_def = TypeDef::Feature(FeatureTypeDef {
            attributes,
            ..Default::default()
        });

        let property_tables = to_gltf_property_table(&class_name, &feature_type_def, 0, 1);
        assert_eq!(property_tables.len(), 1);
    }
}
