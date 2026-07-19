use std::collections::BTreeMap;

use geojson::JsonObject;
use nusamai_citygml::schema::{Attribute, FeatureTypeDef, Map, TypeRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InferredType {
    NullOnly,
    Boolean,
    Integer,
    Double,
    String,
    JsonString,
}

impl InferredType {
    fn from_json(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Self::NullOnly,
            serde_json::Value::Bool(_) => Self::Boolean,
            serde_json::Value::Number(number) if number.as_i64().is_some() => Self::Integer,
            serde_json::Value::Number(_) => Self::Double,
            serde_json::Value::String(_) => Self::String,
            serde_json::Value::Array(_) | serde_json::Value::Object(_) => Self::JsonString,
        }
    }

    fn widen(self, other: Self) -> Self {
        match (self, other) {
            (Self::NullOnly, inferred) | (inferred, Self::NullOnly) => inferred,
            (left, right) if left == right => left,
            (Self::Integer, Self::Double) | (Self::Double, Self::Integer) => Self::Double,
            _ => Self::String,
        }
    }

    fn into_type_ref(self) -> TypeRef {
        match self {
            Self::NullOnly | Self::String => TypeRef::String,
            Self::Boolean => TypeRef::Boolean,
            Self::Integer => TypeRef::Integer,
            Self::Double => TypeRef::Double,
            Self::JsonString => TypeRef::JsonString(Box::new(Attribute::new(TypeRef::String))),
        }
    }
}

#[derive(Debug, Default)]
pub(super) struct GeoJsonSchemaBuilder {
    attributes: BTreeMap<String, InferredType>,
}

impl GeoJsonSchemaBuilder {
    pub(super) fn observe_properties(&mut self, properties: Option<&JsonObject>) {
        let Some(properties) = properties else {
            return;
        };

        for (name, value) in properties {
            let observed = InferredType::from_json(value);
            self.attributes
                .entry(name.clone())
                .and_modify(|inferred| *inferred = inferred.widen(observed))
                .or_insert(observed);
        }
    }

    pub(super) fn finish(self) -> FeatureTypeDef {
        let attributes: Map = self
            .attributes
            .into_iter()
            .map(|(name, inferred)| (name, Attribute::new(inferred.into_type_ref())))
            .collect();

        FeatureTypeDef {
            attributes,
            additional_attributes: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn properties(value: serde_json::Value) -> JsonObject {
        value
            .as_object()
            .expect("test value must be an object")
            .clone()
    }

    #[test]
    fn collects_union_in_deterministic_order_with_optional_attributes() {
        let first = properties(json!({"zeta": 1, "alpha": "first"}));
        let second = properties(json!({"middle": true, "alpha": "second"}));
        let mut builder = GeoJsonSchemaBuilder::default();

        builder.observe_properties(Some(&first));
        builder.observe_properties(None);
        builder.observe_properties(Some(&second));
        let feature_type = builder.finish();

        assert_eq!(
            feature_type
                .attributes
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["alpha", "middle", "zeta"]
        );
        assert!(!feature_type.additional_attributes);
        assert!(feature_type.attributes.values().all(|attribute| {
            attribute.min_occurs == 0
                && attribute.max_occurs == Some(1)
                && attribute.original_name.is_none()
        }));
    }

    #[test]
    fn maps_each_json_value_kind_to_schema_type() {
        let values = properties(json!({
            "array": [1, 2],
            "boolean": true,
            "double": 1.5,
            "integer": -1,
            "object": {"nested": "value"},
            "string": "001"
        }));
        let mut builder = GeoJsonSchemaBuilder::default();

        builder.observe_properties(Some(&values));
        let feature_type = builder.finish();

        assert_eq!(
            feature_type.attributes["array"].type_ref,
            TypeRef::JsonString(Box::new(Attribute::new(TypeRef::String)))
        );
        assert_eq!(
            feature_type.attributes["boolean"].type_ref,
            TypeRef::Boolean
        );
        assert_eq!(feature_type.attributes["double"].type_ref, TypeRef::Double);
        assert_eq!(
            feature_type.attributes["integer"].type_ref,
            TypeRef::Integer
        );
        assert_eq!(
            feature_type.attributes["object"].type_ref,
            TypeRef::JsonString(Box::new(Attribute::new(TypeRef::String)))
        );
        assert_eq!(feature_type.attributes["string"].type_ref, TypeRef::String);
    }

    #[test]
    fn widens_numeric_and_null_types_deterministically() {
        let first = properties(json!({
            "all_null": null,
            "nullable_boolean": null,
            "number": 1
        }));
        let second = properties(json!({
            "all_null": null,
            "nullable_boolean": false,
            "number": 1.25
        }));
        let mut builder = GeoJsonSchemaBuilder::default();

        builder.observe_properties(Some(&first));
        builder.observe_properties(Some(&second));
        let feature_type = builder.finish();

        assert_eq!(
            feature_type.attributes["all_null"].type_ref,
            TypeRef::String
        );
        assert_eq!(
            feature_type.attributes["nullable_boolean"].type_ref,
            TypeRef::Boolean
        );
        assert_eq!(feature_type.attributes["number"].type_ref, TypeRef::Double);
    }

    #[test]
    fn widens_incompatible_types_to_string() {
        let first = properties(json!({"mixed": true, "structured": [1]}));
        let second = properties(json!({"mixed": 1, "structured": {"key": "value"}}));
        let third = properties(json!({"structured": "plain"}));
        let mut builder = GeoJsonSchemaBuilder::default();

        builder.observe_properties(Some(&first));
        builder.observe_properties(Some(&second));
        builder.observe_properties(Some(&third));
        let feature_type = builder.finish();

        assert_eq!(feature_type.attributes["mixed"].type_ref, TypeRef::String);
        assert_eq!(
            feature_type.attributes["structured"].type_ref,
            TypeRef::String
        );
    }
}
