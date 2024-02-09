use std::collections::{hash_map::RandomState, HashMap};

use indexmap::IndexMap;

use nusamai_citygml::{
    schema::{TypeDef, TypeRef},
    Value,
};
use nusamai_gltf_json::extensions;

#[derive(Debug, Clone)]
pub struct GltfPropertyType {
    pub property_name: String,
    pub class_property_type: extensions::gltf::ext_structural_metadata::ClassPropertyType,
    pub component_type:
        Option<extensions::gltf::ext_structural_metadata::ClassPropertyComponentType>,
}

pub struct Attributes {
    pub class_name: String,
    pub feature_id: u32,
    pub attributes: IndexMap<String, Value, RandomState>,
}

fn to_gltf_schema(type_ref: &TypeRef) -> GltfPropertyType {
    // todo: 型定義を正確に行う
    match type_ref {
        TypeRef::String => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
        },
        TypeRef::Integer => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32,
            ),
        },
        TypeRef::Double => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float64,
            ),
        },
        TypeRef::Boolean => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean,
            component_type: None,
        },
        TypeRef::Measure => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32,
            ),
        },
        TypeRef::Code => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
        },
        TypeRef::NonNegativeInteger => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32,
            ),
        },
        TypeRef::JsonString => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
        },
        TypeRef::DataTime => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
        },
        TypeRef::Point => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type: extensions::gltf::ext_structural_metadata::ClassPropertyType::Vec3,
            component_type: Some(
                extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float64,
            ),
        },
        TypeRef::Named(_) => GltfPropertyType {
            property_name: "".to_string(),
            class_property_type:
                extensions::gltf::ext_structural_metadata::ClassPropertyType::String,
            component_type: None,
        },
        _ => unimplemented!(),
    }
}

pub fn to_gltf_classes(
    class_name: &String,
    schema: &TypeDef,
) -> HashMap<String, extensions::gltf::ext_structural_metadata::Class> {
    let mut gltf_property_types = Vec::new();

    match schema {
        TypeDef::Feature(f) => {
            for (name, attr) in &f.attributes {
                let mut property_type = to_gltf_schema(&attr.type_ref);
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

    let mut classes: HashMap<String, extensions::gltf::ext_structural_metadata::Class> =
        HashMap::new();
    classes.insert(
        class_name.clone(),
        extensions::gltf::ext_structural_metadata::Class {
            name: Some(class_name.clone()),
            description: None,
            properties: class_properties.clone(),
            ..Default::default()
        },
    );

    classes
}

pub fn to_gltf_property_tables(
    schema: &TypeDef,
    class_name: &String,
    buffer_view_length: u32,
    feature_count: u32,
) -> Vec<extensions::gltf::ext_structural_metadata::PropertyTable> {
    // todo: 複数の地物型が存在している時の対応を考える
    let mut property_tables: Vec<extensions::gltf::ext_structural_metadata::PropertyTable> =
        Vec::new();

    // Create Schema.property_tables
    let mut property_table: extensions::gltf::ext_structural_metadata::PropertyTable =
        extensions::gltf::ext_structural_metadata::PropertyTable {
            class: class_name.clone(),
            properties: HashMap::new(),
            count: feature_count,
            ..Default::default()
        };

    let mut count = buffer_view_length;
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
                                values: count,
                                string_offsets: Some(count + 1),
                                ..Default::default()
                            },
                        );
                        count += 2;
                    }
                    extensions::gltf::ext_structural_metadata::ClassPropertyType::Scalar => {
                        property_table.properties.insert(
                            name.clone(),
                            extensions::gltf::ext_structural_metadata::PropertyTableProperty {
                                values: count,
                                ..Default::default()
                            },
                        );
                    }
                    extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean => {
                        property_table.properties.insert(
                            name.clone(),
                            extensions::gltf::ext_structural_metadata::PropertyTableProperty {
                                values: count,
                                ..Default::default()
                            },
                        );
                    }
                    _ => unimplemented!(),
                }
            }
        }
        // todo: feature 以外の型も実装する
        TypeDef::Data(_) => unimplemented!(),
        TypeDef::Property(_) => unimplemented!(),
    }

    property_tables.push(property_table);

    property_tables
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

        let classes = to_gltf_classes(&class_name, &feature_type_def);
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

        let property_tables = to_gltf_property_tables(&feature_type_def, &class_name, 0, 1);
        assert_eq!(property_tables.len(), 1);
    }
}
