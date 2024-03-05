use byteorder::{LittleEndian, WriteBytesExt};
use bytesize::to_string;
use std::{collections::HashMap, io::Write};

use nusamai_citygml::{
    attribute,
    object::Object,
    schema::{Schema as NusamaiSchema, TypeDef, TypeRef},
    Measure, Value,
};
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

    let property_tables = {
        let property_table = schema_to_gltf_property_table(
            typename,
            type_def,
            buffer_views.len() as u32,
            num_features,
        );

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
            let mut buf: Vec<u8> = Vec::new();
            let mut string_offset_buffer: Vec<u32> = Vec::new();

            for feature in features {
                if let Value::Object(object) = &feature.attributes {
                    let attribute_name_list: Vec<&String> =
                        object.attributes.keys().collect::<Vec<_>>();
                    let is_hit = attribute_name_list.contains(&property_name);

                    // classから型を取り出す
                    let p = classes
                        .get(typename)
                        .unwrap()
                        .properties
                        .get(property_name)
                        .unwrap();
                    let property_type = &p.type_;
                    let component_type = &p.component_type;

                    // featureのattributesに属性が存在しない場合は、bufferにデフォルト値を書き込む
                    if !is_hit {
                        match property_type {
                            ClassPropertyType::String => {
                                string_offset_buffer.push(buf.len() as u32);
                                buf.write_all(b"0").unwrap();
                            }
                            ClassPropertyType::Boolean => {
                                buf.write_u8(0).unwrap();
                            }
                            _ => {}
                        }

                        // component_typeがSomeなら数値だが、その場合は0を書き込む
                        if let Some(component_type) = component_type {
                            let component_type = component_type.clone();
                            match component_type {
                                ClassPropertyComponentType::Int8 => {
                                    buf.write_i8(0).unwrap();
                                }
                                ClassPropertyComponentType::Uint8 => {
                                    buf.write_u8(0).unwrap();
                                }
                                ClassPropertyComponentType::Int16 => {
                                    buf.write_i16::<LittleEndian>(0).unwrap();
                                }
                                ClassPropertyComponentType::Uint16 => {
                                    buf.write_u16::<LittleEndian>(0).unwrap();
                                }
                                ClassPropertyComponentType::Int32 => {
                                    buf.write_i32::<LittleEndian>(0).unwrap();
                                }
                                ClassPropertyComponentType::Uint32 => {
                                    buf.write_u32::<LittleEndian>(0).unwrap();
                                }
                                ClassPropertyComponentType::Float32 => {
                                    buf.write_f32::<LittleEndian>(0.0).unwrap();
                                }
                                ClassPropertyComponentType::Float64 => {
                                    buf.write_f64::<LittleEndian>(0.0).unwrap();
                                }
                                ClassPropertyComponentType::Int64 => {
                                    buf.write_i64::<LittleEndian>(0).unwrap();
                                }
                                ClassPropertyComponentType::Uint64 => {
                                    buf.write_u64::<LittleEndian>(0).unwrap();
                                }
                            }
                        };
                        // 次のfeatureへ
                        continue;
                    }

                    if let Some((_, value)) = object
                        .attributes
                        .iter()
                        .find(|(name, _)| *name == property_name)
                    {
                        // 値があるなら、ルールに従ってbufferに書き込み
                        value.write_to_buffer(&mut buf, &mut string_offset_buffer);
                    }
                }
            }
            // offsetには文字列の最後にもインデックスが必要
            if !string_offset_buffer.is_empty() {
                string_offset_buffer.push(buf.len() as u32);
            }

            let byte_offset = buffer.len();
            let byte_length = buf.len();

            buffer.extend(buf.iter());

            buffer_views.push(BufferView {
                byte_offset: byte_offset as u32,
                byte_length: byte_length as u32,
                ..Default::default()
            });

            if !string_offset_buffer.is_empty() {
                let byte_offset = buffer.len();
                let byte_length = string_offset_buffer.len() as u32 * 4;

                buffer.extend(string_offset_buffer.iter().flat_map(|x| x.to_le_bytes()));

                buffer_views.push(BufferView {
                    byte_offset: byte_offset as u32,
                    byte_length,
                    ..Default::default()
                });
            }
        }

        vec![property_table]
    };

    // Schema
    let schema = Schema {
        id: typename.to_string(),
        classes: classes.clone(),
        // enums,
        ..Default::default()
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

pub fn schema_to_gltf_property_table(
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
                let (class_property_type, _) = schema_to_gltf_property_type(&attr.type_ref);
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
