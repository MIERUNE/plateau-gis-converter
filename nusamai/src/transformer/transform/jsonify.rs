use nusamai_citygml::{
    object::Value,
    schema::{DataTypeDef, FeatureTypeDef, Schema, TypeDef, TypeRef},
};
use nusamai_plateau::Entity;

use crate::{pipeline::Feedback, transformer::Transform};

/// Jsonify all objects and arrays in the entity.
#[derive(Clone)]
pub struct JsonifyTransform {
    jsonify_array: bool,
}

impl JsonifyTransform {
    pub fn jsonify_array(mut self, jsonify_array: bool) -> Self {
        self.jsonify_array = jsonify_array;
        self
    }
}

impl Default for JsonifyTransform {
    fn default() -> Self {
        Self {
            jsonify_array: true,
        }
    }
}

impl Transform for JsonifyTransform {
    fn transform(&mut self, _feedback: &Feedback, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(obj) = &mut entity.root {
            for value in obj.attributes.values_mut() {
                match value {
                    Value::Object(_) => {
                        *value = Value::String(value.to_attribute_json().to_string())
                    }
                    Value::Array(arr) => {
                        if self.jsonify_array {
                            *value = Value::String(value.to_attribute_json().to_string())
                        } else {
                            for v in arr {
                                if let Value::Object(_) = v {
                                    *v = Value::String(v.to_attribute_json().to_string())
                                }
                            }
                        }
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
                            _ => {
                                if self.jsonify_array && attr.max_occurs != Some(1) {
                                    attr.type_ref = TypeRef::JsonString(attr.clone().into())
                                }
                            }
                        }
                    }
                }
                TypeDef::Property(_) => {}
            }
        }
    }
}
