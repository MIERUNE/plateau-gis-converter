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

pub fn make_metadata(
    num_features: usize,
    typename: &str,
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
