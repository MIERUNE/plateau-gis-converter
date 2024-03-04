use byteorder::{LittleEndian, WriteBytesExt};
use std::collections::HashMap;

use nusamai_gltf_json::{
    extensions::gltf::ext_structural_metadata::{
        Class, ClassProperty, ClassPropertyType, Enum, EnumValue, EnumValueType,
        ExtStructuralMetadata, PropertyTable, PropertyTableProperty, Schema,
    },
    BufferView,
};

pub fn make_dummy_metadata(
    num_features: usize,
    buffer: &mut Vec<u8>,
    buffer_views: &mut Vec<BufferView>,
) -> Option<ExtStructuralMetadata> {
    if num_features == 0 {
        return None;
    }

    // Schema
    let schema = {
        let classes = {
            let mut properties: HashMap<String, ClassProperty> = Default::default();
            let mut classes: HashMap<String, Class> = HashMap::new();
            properties.insert(
                "class".to_string(),
                ClassProperty {
                    type_: ClassPropertyType::Enum,
                    enum_type: "Enum_01".to_string().into(),
                    ..Default::default()
                },
            );
            classes.insert(
                "Class_01".to_string(),
                Class {
                    properties,
                    ..Default::default()
                },
            );
            classes
        };

        let enums = {
            let mut enums: HashMap<String, Enum> = HashMap::new();
            enums.insert(
                "Enum_01".to_string(),
                Enum {
                    value_type: EnumValueType::Uint32,
                    values: vec![EnumValue {
                        value: 0,
                        name: "dummy_enum_name".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            );
            enums
        };

        Schema {
            id: "dummy".to_string(),
            classes,
            enums,
            ..Default::default()
        }
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

        vec![PropertyTable {
            class: "Class_01".to_string(),
            count: num_features as u32,
            properties,
            ..Default::default()
        }]
    };

    Some(ExtStructuralMetadata {
        schema: Some(schema),
        property_tables: Some(property_tables),
        ..Default::default()
    })
}
