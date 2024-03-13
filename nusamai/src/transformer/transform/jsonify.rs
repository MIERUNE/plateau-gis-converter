use crate::transformer::Transform;

use nusamai_citygml::object::Value;
use nusamai_citygml::schema::{DataTypeDef, FeatureTypeDef, Schema, TypeDef, TypeRef};
use nusamai_plateau::Entity;

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
    fn transform(&mut self, mut entity: Entity, out: &mut Vec<Entity>) {
        if let Value::Object(obj) = &mut entity.root {
            let mut attrs = nusamai_citygml::object::Map::default();
            for (key, value) in obj.attributes.drain(..) {
                match value {
                    Value::Object(_) => {
                        attrs.insert(key, Value::String(value.to_attribute_json().to_string()));
                    }
                    Value::Array(mut arr) => {
                        if self.jsonify_array {
                            attrs.insert(
                                key,
                                Value::String(Value::Array(arr).to_attribute_json().to_string()),
                            );
                        } else {
                            for v in arr.iter_mut() {
                                if let Value::Object(_) = v {
                                    *v = Value::String(v.to_attribute_json().to_string())
                                }
                            }
                            attrs.insert(key, Value::Array(arr));
                        }
                    }
                    value => {
                        attrs.insert(key, value);
                    }
                }
            }
            obj.attributes = attrs;
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
