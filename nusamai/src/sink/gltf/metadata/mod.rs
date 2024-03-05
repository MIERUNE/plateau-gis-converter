use byteorder::{LittleEndian, WriteBytesExt};
use bytesize::to_string;
use std::collections::HashMap;

use nusamai_citygml::schema::{Schema as NusamaiSchema, TypeDef, TypeRef};
use nusamai_gltf_json::{
    extensions::gltf::ext_structural_metadata::{
        Class, ClassProperty, ClassPropertyComponentType, ClassPropertyType, Enum, EnumValue,
        EnumValueType, ExtStructuralMetadata, PropertyTable, PropertyTableProperty, Schema,
    },
    BufferView,
};

use super::Features;

pub fn make_metadata(
    num_features: usize,
    typename: &str,
    features: &Features,
    buffer: &mut Vec<u8>,
    buffer_views: &mut Vec<BufferView>,
    schema: &NusamaiSchema,
) -> Option<ExtStructuralMetadata> {
    if num_features == 0 {
        return None;
    }

    let type_def = schema.types.get::<String>(&typename.to_string()).unwrap();

    // schema to gltf `Class`
    let classes = schema_to_gltf_classes(typename, type_def);

    // schema to gltf `Enums`
    // let enums: HashMap<String, Enum> = Default::default();

    // Schema
    let schema = Schema {
        id: typename.to_string(),
        classes,
        // enums,
        ..Default::default()
    };

    let property_tables = {
        let mut properties: HashMap<String, PropertyTableProperty> = Default::default();
        properties.insert(
            "class".to_string(),
            PropertyTableProperty {
                values: buffer_views.len() as u32,
                ..Default::default()
            },
        );

        let start = buffer.len();
        for _ in 0..num_features {
            buffer.write_u32::<LittleEndian>(0).unwrap();
        }
        buffer_views.push(BufferView {
            byte_offset: start as u32,
            byte_length: (buffer.len() - start) as u32,
            ..Default::default()
        });

        let property_table =
            to_gltf_property_table(typename, type_def, buffer_views.len() as u32, num_features);

        // make buffer_view for each property
        let mut properties = property_table.properties.iter().collect::<Vec<_>>();
        properties.sort_by(|a, b| a.1.values.cmp(&b.1.values));

        // featuresをfeature_idの順にソート
        features
            .clone()
            .sort_by(|a, b| a.feature_id.cmp(&b.feature_id));

        // 抽出するべき全ての属性がproperty_nameに入る
        for (property_name, _) in properties {
            // property_nameに対応するbuffer_viewを作成する
            // todo: データ型ごとの対応をする（配列の場合は、array_offsetを書き込む必要があるなど）
            let mut buffer: Vec<u8> = Vec::new();
            let mut string_offset_buffer: Vec<u32> = Vec::new();

            //     // このproperty_nameに対応するbuffer_viewを作成する
            //     for entity in entities {
            //         // 個別のentityから、property_nameに対応する属性を取り出す
            //         // なければ、無視する
            //         let attribute_name_list = entity.attributes.attributes.keys().collect::<Vec<_>>();
            //         let is_hit = attribute_name_list.contains(&property_name);
            //         // classから型を取り出す
            //         let p = class
            //             .get(class_name)
            //             .unwrap()
            //             .properties
            //             .get(property_name)
            //             .unwrap();
            //         let property_type = &p.type_;
            //         let component_type = &p.component_type;

            //         if !is_hit {
            //             match property_type {
            //                 extensions::gltf::ext_structural_metadata::ClassPropertyType::String => {
            //                     string_offset_buffer.push(buffer.len() as u32);
            //                     buffer.write_all(b"0").unwrap();
            //                 }
            //                 extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean => {
            //                     buffer.write_u8(0).unwrap();
            //                 }
            //                 _ => {}
            //             }

            //             // component_typeがSomeなら値を取り出す
            //             if let Some(component_type) = component_type {
            //                 let component_type = component_type.clone();
            //                 match component_type {
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int8 => {
            //                             buffer.write_i8(0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Uint8 => {
            //                             buffer.write_u8(0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int16 => {
            //                             buffer.write_i16::<LittleEndian>(0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Uint16 => {
            //                             buffer.write_u16::<LittleEndian>(0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32 => {
            //                             buffer.write_i32::<LittleEndian>(0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Uint32 => {
            //                             buffer.write_u32::<LittleEndian>(0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float32 => {
            //                             buffer.write_f32::<LittleEndian>(0.0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float64 => {
            //                             buffer.write_f64::<LittleEndian>(0.0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int64 => {
            //                             buffer.write_i64::<LittleEndian>(0).unwrap();
            //                         },
            //                         extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Uint64 => {
            //                             buffer.write_u64::<LittleEndian>(0).unwrap();
            //                         },
            //                     }
            //             };
            //             continue;
            //         }

            //         // 属性名が一致するものがあれば、ちゃんと取り出す
            //         // entity.attributes.attributes.get(&property_name)が効かないので、for文で取り出す
            //         if let Some((_, value)) = entity
            //             .attributes
            //             .attributes
            //             .iter()
            //             .find(|(name, _)| *name == property_name)
            //         {
            //             // bufferに書き込み
            //             value.write_to_buffer(&mut buffer, &mut string_offset_buffer);
            //         }
            //     }
            //     // offsetには文字列の最後にもインデックスが必要
            //     if !string_offset_buffer.is_empty() {
            //         string_offset_buffer.push(buffer.len() as u32);
            //     }

            //     let byte_offset = bin_content.len();
            //     let byte_length = buffer.len();

            //     bin_content.extend(buffer.iter());

            //     buffer_views.push(BufferView {
            //         byte_offset: byte_offset as u32,
            //         byte_length: byte_length as u32,
            //         ..Default::default()
            //     });

            //     if !string_offset_buffer.is_empty() {
            //         let byte_offset = bin_content.len();
            //         let byte_length = string_offset_buffer.len() as u32 * 4;

            //         bin_content.extend(string_offset_buffer.iter().flat_map(|x| x.to_le_bytes()));

            //         buffer_views.push(BufferView {
            //             byte_offset: byte_offset as u32,
            //             byte_length,
            //             ..Default::default()
            //         });
            //     }
        }

        vec![property_table]
    };

    Some(ExtStructuralMetadata {
        schema: Some(schema),
        property_tables: Some(property_tables),
        ..Default::default()
    })
}

pub fn schema_to_gltf_classes(typename: &str, type_def: &TypeDef) -> HashMap<String, Class> {
    let mut classes: HashMap<String, Class> = HashMap::new();

    match type_def {
        TypeDef::Feature(f) => {
            let mut class_properties: HashMap<String, ClassProperty> = HashMap::new();

            for (name, attr) in &f.attributes {
                let (class_property_type, class_property_component_type) =
                    schema_to_gltf_property_type(&attr.type_ref);

                // Create Schema.classes
                let class_property = ClassProperty {
                    description: Some(name.clone()),
                    type_: class_property_type,
                    component_type: class_property_component_type,
                    ..Default::default()
                };
                class_properties.insert(name.clone(), class_property);
            }

            classes.insert(
                typename.to_string(),
                Class {
                    name: Some(typename.to_string()),
                    description: None,
                    properties: class_properties.clone(),
                    ..Default::default()
                },
            );
        }
        TypeDef::Data(_) => {
            // todo: implement
        }
        TypeDef::Property(_) => {
            // todo: implement
        }
    }

    classes
}

fn schema_to_gltf_property_type(
    type_ref: &TypeRef,
) -> (ClassPropertyType, Option<ClassPropertyComponentType>) {
    match type_ref {
        TypeRef::String => (ClassPropertyType::String, None),
        TypeRef::Integer => (
            ClassPropertyType::Scalar,
            Some(ClassPropertyComponentType::Int64),
        ),
        TypeRef::Double => (
            ClassPropertyType::Scalar,
            Some(ClassPropertyComponentType::Float64),
        ),
        TypeRef::Boolean => (ClassPropertyType::Boolean, None),
        TypeRef::Measure => (
            ClassPropertyType::Scalar,
            Some(ClassPropertyComponentType::Float64),
        ),
        TypeRef::Code => (ClassPropertyType::String, None),
        TypeRef::NonNegativeInteger => (
            ClassPropertyType::Scalar,
            Some(ClassPropertyComponentType::Uint64),
        ),
        TypeRef::JsonString(_) => (ClassPropertyType::String, None),
        TypeRef::Point => (
            ClassPropertyType::Vec3,
            Some(ClassPropertyComponentType::Float64),
        ),
        TypeRef::Named(_) => (ClassPropertyType::String, None),
        TypeRef::URI => (ClassPropertyType::String, None),
        TypeRef::Date => (ClassPropertyType::String, None),
        TypeRef::DateTime => (ClassPropertyType::String, None),
        TypeRef::Unknown => todo!(),
    }
}

pub fn to_gltf_property_table(
    typename: &str,
    schema: &TypeDef,
    buffer_view_length: u32,
    num_features: usize,
) -> PropertyTable {
    // Create Schema.propertyTables
    let mut property_table: PropertyTable = PropertyTable {
        class: typename.to_string(),
        properties: HashMap::new(),
        count: num_features as u32,
        ..Default::default()
    };

    let mut buffer_view_length = buffer_view_length;
    match schema {
        TypeDef::Feature(f) => {
            for (name, attr) in &f.attributes {
                let (class_property_type, class_property_component_type) =
                    schema_to_gltf_property_type(&attr.type_ref);
                match class_property_type {
                    ClassPropertyType::String => {
                        property_table.properties.insert(
                            name.clone(),
                            PropertyTableProperty {
                                values: buffer_view_length,
                                string_offsets: Some(buffer_view_length + 1),
                                ..Default::default()
                            },
                        );
                        buffer_view_length += 2;
                    }
                    ClassPropertyType::Scalar => {
                        property_table.properties.insert(
                            name.clone(),
                            PropertyTableProperty {
                                values: buffer_view_length,
                                ..Default::default()
                            },
                        );
                        buffer_view_length += 1;
                    }
                    ClassPropertyType::Boolean => {
                        property_table.properties.insert(
                            name.clone(),
                            PropertyTableProperty {
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
        TypeDef::Data(_) => {
            // todo: implement
        }
        TypeDef::Property(_) => {
            // todo: implement
        }
    }

    property_table
}
