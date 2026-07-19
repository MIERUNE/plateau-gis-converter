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
