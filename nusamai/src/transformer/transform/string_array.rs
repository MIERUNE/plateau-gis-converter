use nusamai_citygml::{
    object::{Object, Value},
    schema::{Attribute, DataTypeDef, FeatureTypeDef, Schema, TypeDef, TypeRef},
};
use nusamai_plateau::Entity;

use crate::{pipeline::Feedback, transformer::Transform};

/// Convert array attributes that contain only scalar values into comma-separated strings.
#[derive(Clone)]
pub struct FlattenStringArrayTransform {
    separator: &'static str,
}

impl FlattenStringArrayTransform {
    pub fn new(separator: &'static str) -> Self {
        Self { separator }
    }
}

impl Default for FlattenStringArrayTransform {
    fn default() -> Self {
        Self { separator: "," }
    }
}

impl Transform for FlattenStringArrayTransform {
    fn transform(&mut self, _feedback: &Feedback, mut entity: Entity, out: &mut Vec<Entity>) {
        flatten_value(&mut entity.root, self.separator);
        out.push(entity);
    }

    fn transform_schema(&self, schema: &mut Schema) {
        for ty in schema.types.values_mut() {
            match ty {
                TypeDef::Feature(FeatureTypeDef { attributes, .. })
                | TypeDef::Data(DataTypeDef { attributes, .. }) => {
                    for attr in attributes.values_mut() {
                        if is_multi_occurrence(attr) && is_scalar_type(&attr.type_ref) {
                            attr.max_occurs = Some(1);
                        }
                    }
                }
                TypeDef::Property(_) => {}
            }
        }
    }
}

fn flatten_value(value: &mut Value, separator: &str) {
    match value {
        Value::Object(Object { attributes, .. }) => {
            for attr_value in attributes.values_mut() {
                flatten_value(attr_value, separator);
            }
        }
        Value::Array(arr) => {
            if arr.iter().any(|v| matches!(v, Value::Array(_))) {
                let json = value.to_attribute_json().to_string();
                *value = Value::String(json);
                return;
            }

            for v in arr.iter_mut() {
                flatten_value(v, separator);
            }

            let mut parts = Vec::with_capacity(arr.len());
            for v in arr.iter() {
                if let Some(text) = scalar_to_string(v) {
                    if text.eq_ignore_ascii_case("null") || text.is_empty() {
                        continue;
                    }
                    parts.push(text);
                } else {
                    let json = value.to_attribute_json().to_string();
                    *value = Value::String(json);
                    return;
                }
            }

            let joined = if parts.is_empty() {
                String::new()
            } else {
                parts.join(separator)
            };
            *value = Value::String(joined);
        }
        _ => {}
    }
}

fn scalar_to_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Code(code) => Some(code.value().to_string()),
        Value::Integer(i) => Some(i.to_string()),
        Value::NonNegativeInteger(i) => Some(i.to_string()),
        Value::Double(d) => Some(d.to_string()),
        Value::Measure(m) => Some(m.value().to_string()),
        Value::Boolean(b) => Some(b.to_string()),
        Value::Uri(uri) => Some(uri.value().to_string()),
        Value::Date(date) => Some(date.to_string()),
        Value::Array(_) | Value::Object(_) | Value::Point(_) => None,
    }
}

fn is_scalar_type(ty: &TypeRef) -> bool {
    matches!(
        ty,
        TypeRef::String
            | TypeRef::Code
            | TypeRef::Integer
            | TypeRef::NonNegativeInteger
            | TypeRef::Double
            | TypeRef::Measure
            | TypeRef::Boolean
            | TypeRef::URI
            | TypeRef::Date
            | TypeRef::JsonString(_)
    )
}

fn is_multi_occurrence(attr: &Attribute) -> bool {
    match attr.max_occurs {
        Some(occur) => occur > 1,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::feedback;
    use nusamai_citygml::{
        object::{Map, ObjectStereotype},
        GeometryStore,
    };
    use std::sync::RwLock;

    fn build_entity(value: Value) -> Entity {
        Entity {
            root: value,
            base_url: url::Url::parse("file:///dummy").unwrap(),
            geometry_store: RwLock::new(GeometryStore::default()).into(),
            appearance_store: Default::default(),
        }
    }

    #[test]
    fn flatten_single_string() {
        let mut attributes = Map::default();
        attributes.insert(
            "name".into(),
            Value::Array(vec![Value::String("Foo".into())]),
        );
        let entity = build_entity(Value::Object(Object {
            typename: "bldg:Building".into(),
            attributes,
            stereotype: ObjectStereotype::Data,
        }));

        let mut transform = FlattenStringArrayTransform::new(",");
        let mut out = Vec::new();
        let (_watcher, feedback, _canceller) = feedback::watcher();
        transform.transform(&feedback, entity, &mut out);

        match &out[0].root {
            Value::Object(obj) => {
                assert_eq!(
                    obj.attributes.get("name"),
                    Some(&Value::String("Foo".into()))
                );
            }
            _ => panic!("unexpected root"),
        }
    }

    #[test]
    fn flatten_multiple_strings() {
        let mut attributes = Map::default();
        attributes.insert(
            "alias".into(),
            Value::Array(vec![
                Value::String("A".into()),
                Value::String("B".into()),
                Value::String("C".into()),
            ]),
        );
        let entity = build_entity(Value::Object(Object {
            typename: "bldg:Building".into(),
            attributes,
            stereotype: ObjectStereotype::Data,
        }));

        let mut transform = FlattenStringArrayTransform::default();
        let mut out = Vec::new();
        let (_watcher, feedback, _canceller) = feedback::watcher();
        transform.transform(&feedback, entity, &mut out);

        match &out[0].root {
            Value::Object(obj) => {
                assert_eq!(
                    obj.attributes.get("alias"),
                    Some(&Value::String("A,B,C".into()))
                );
            }
            _ => panic!("unexpected root"),
        }
    }

    #[test]
    fn skip_non_scalar_array() {
        let mut attributes = Map::default();
        attributes.insert(
            "complex".into(),
            Value::Array(vec![Value::Array(vec![Value::String("nested".into())])]),
        );
        let entity = build_entity(Value::Object(Object {
            typename: "bldg:Building".into(),
            attributes,
            stereotype: ObjectStereotype::Data,
        }));

        let mut transform = FlattenStringArrayTransform::default();
        let mut out = Vec::new();
        let (_watcher, feedback, _canceller) = feedback::watcher();
        transform.transform(&feedback, entity, &mut out);

        match &out[0].root {
            Value::Object(obj) => {
                match obj.attributes.get("complex") {
                    Some(Value::String(s)) => assert_eq!(s, "[[\"nested\"]]"),
                    _ => panic!("complex attribute should remain serialized JSON"),
                }
            }
            _ => panic!("unexpected root"),
        }
    }
}
