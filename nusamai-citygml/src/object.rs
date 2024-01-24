//! Object representation of the city objects.

use std::borrow::Cow;
use std::sync::{Arc, RwLock};

use crate::geometry::{GeometryRef, GeometryStore};
use crate::values::{Code, Date, Point, URI};
use crate::Measure;
use serde::{Deserialize, Serialize};

pub type Map = indexmap::IndexMap<String, Value, ahash::RandomState>;

/// City objects, features, objects or data
#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    /// Attribute tree
    pub root: Value,
    /// All geometries referenced by the attribute tree
    pub geometry_store: Arc<RwLock<GeometryStore>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Object {
    pub typename: Cow<'static, str>,
    pub stereotype: ObjectStereotype,
    pub attributes: Map,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObjectStereotype {
    Feature { id: String, geometries: GeometryRef },
    Object { id: String },
    Data,
}

impl ObjectStereotype {
    pub fn id(&self) -> Option<&str> {
        match self {
            ObjectStereotype::Feature { id, .. } => Some(id),
            ObjectStereotype::Object { id } => Some(id),
            ObjectStereotype::Data => None,
        }
    }
}

// Nodes for the "Object" representation of the city object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Value {
    String(String),
    Code(Code),
    Integer(i64),
    Double(f64),
    Measure(Measure),
    Boolean(bool),
    URI(URI),
    Date(Date),
    Point(Point),
    Array(Vec<Value>),
    Object(Object),
}

#[cfg(feature = "serde_json")]
impl Value {
    /// Extracts the thematic attribute tree and converts it to a JSON representation.
    pub fn to_attribute_json(&self) -> serde_json::Value {
        use Value::*;
        match &self {
            String(s) => serde_json::Value::String(s.into()),
            Code(c) => serde_json::Value::String(c.value().to_owned()),
            Integer(i) => serde_json::Value::Number((*i).into()),
            Double(d) => serde_json::Value::Number(serde_json::Number::from_f64(*d).unwrap()),
            Measure(m) => {
                serde_json::Value::Number(serde_json::Number::from_f64(m.value()).unwrap())
            }
            Boolean(b) => serde_json::Value::Bool(*b),
            URI(u) => serde_json::Value::String(u.value().clone()),
            Date(d) => serde_json::Value::String(d.to_string()), // ISO 8601
            Point(_) => {
                // TODO: Handle Point
                todo!()
                // json! {
                //     {
                //         "type": "Point",
                //         "coordinates": [x, y, z]
                //     }
                // }
            }
            Array(arr) => {
                serde_json::Value::Array(arr.iter().map(Value::to_attribute_json).collect())
            }
            Object(cls) => {
                let mut m = serde_json::Map::from_iter(
                    cls.attributes
                        .iter()
                        .map(|(k, v)| (k.into(), v.to_attribute_json())),
                );
                m.insert("type".into(), cls.typename.clone().into());
                if let Some(id) = cls.stereotype.id() {
                    m.insert("id".into(), serde_json::Value::String(id.into()));
                }
                serde_json::Value::Object(m)
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "serde_json")]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn to_attribute_json() {
        let obj = Value::String("test".into());
        assert_eq!(obj.to_attribute_json(), json!("test"));

        let obj = Value::Code(Code::new("12345".into(), "12345".into()));
        assert_eq!(obj.to_attribute_json(), json!("12345"));

        let obj = Value::Integer(12345);
        assert_eq!(obj.to_attribute_json(), json!(12345));

        let obj = Value::Double(1.0);
        assert_eq!(obj.to_attribute_json(), json!(1.0));

        let obj = Value::Measure(Measure::new(1.0));
        assert_eq!(obj.to_attribute_json(), json!(1.0));

        let obj = Value::Boolean(true);
        assert_eq!(obj.to_attribute_json(), json!(true));

        let obj = Value::URI(URI::new("http://example.com"));
        assert_eq!(obj.to_attribute_json(), json!("http://example.com"));

        let obj = Value::Date(Date::from_ymd_opt(2020, 1, 1).unwrap());
        assert_eq!(obj.to_attribute_json(), json!("2020-01-01"));

        let obj = Value::Array(vec![Value::String("test".into()), Value::Integer(1)]);
        assert_eq!(obj.to_attribute_json(), json!(["test", 1]));

        let mut attributes = Map::default();
        attributes.insert("string".into(), Value::String("test".into()));
        attributes.insert("integer".into(), Value::Integer(1));
        let obj = Value::Object(Object {
            typename: "test".into(),
            attributes,
            stereotype: ObjectStereotype::Feature {
                id: "test".into(),
                geometries: Vec::default(),
            },
        });
        assert_eq!(
            obj.to_attribute_json(),
            json! {
                {
                    "type": "test",
                    "id": "test",
                    "string": "test",
                    "integer": 1
                }
            }
        );

        let mut attributes = Map::default();
        attributes.insert("string".into(), Value::String("test".into()));
        attributes.insert("integer".into(), Value::Integer(1));
        let obj = Value::Object(Object {
            typename: "test".into(),
            stereotype: ObjectStereotype::Data,
            attributes,
        });
        assert_eq!(
            obj.to_attribute_json(),
            json! {
                {
                    "type": "test",
                    "string": "test",
                    "integer": 1
                }
            }
        );
    }
}
