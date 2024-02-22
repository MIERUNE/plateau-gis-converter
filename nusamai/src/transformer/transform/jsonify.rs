use crate::transformer::Transform;

use nusamai_citygml::object::Value;
use nusamai_citygml::schema::{DataTypeDef, FeatureTypeDef, Schema, TypeDef, TypeRef};
use nusamai_plateau::Entity;

/// Jsonify all objects and arrays in the entity.
#[derive(Default, Clone)]
pub struct JsonifyTransform {}

impl Transform for JsonifyTransform {
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(obj) = &mut entity.root {
            for value in obj.attributes.values_mut() {
                match value {
                    Value::Object(_) | Value::Array(_) => {
                        *value = Value::String(value.to_attribute_json().to_string())
                    }
                    _ => {}
                }
            }
            out.push(entity)
        }
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for ty in schema.types.values_mut() {
            match ty {
                TypeDef::Feature(FeatureTypeDef { attributes, .. })
                | TypeDef::Data(DataTypeDef { attributes, .. }) => {
                    for attr in attributes.values_mut() {
                        match attr.type_ref {
                            TypeRef::Named(_) => {
                                attr.type_ref = TypeRef::JsonString(attr.clone().into());
                            }
                            _ if attr.max_occurs != Some(1) => {
                                attr.type_ref = TypeRef::JsonString(attr.clone().into());
                            }
                            _ => {}
                        }
                    }
                }
                TypeDef::Property(_) => {}
            }
        }
    }
}
