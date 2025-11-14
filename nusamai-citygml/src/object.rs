//! Object representation of the city objects.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    geometry::GeometryRefs,
    values::{Code, Date, Point, Uri},
    Measure,
};

// TODO: Cow<'static, str> insted of String ??
pub type Map = indexmap::IndexMap<String, Value, ahash::RandomState>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Object {
    pub typename: Cow<'static, str>,
    pub stereotype: ObjectStereotype,
    pub attributes: Map,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ObjectStereotype {
    Feature {
        id: String,
        geometries: GeometryRefs,
    },
    Object {
        id: String,
    },
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
    NonNegativeInteger(u64),
    Double(f64),
    Measure(Measure),
    Boolean(bool),
    Uri(Uri),
    Date(Date),
    Point(Point),
    Array(Vec<Value>),
    Object(Object),
}

impl Value {
    pub fn id(&self) -> Option<&str> {
        match self {
            Value::Object(obj) => obj.stereotype.id(),
            _ => None,
        }
    }

    pub fn typename(&self) -> Option<&str> {
        match self {
            Value::Object(obj) => Some(obj.typename.as_ref()),
            _ => None,
        }
    }

    /// Traverses the attribute tree and apply the function to each object.
    pub fn traverse_object_mut(&mut self, mut f: impl FnMut(&mut Object)) {
        self.traverse_object_mut_inner(&mut f);
    }

    fn traverse_object_mut_inner(&mut self, f: &mut impl FnMut(&mut Object)) {
        match self {
            Value::Object(obj) => {
                f(obj);
            }
            Value::Array(arr) => {
                for v in arr.iter_mut() {
                    v.traverse_object_mut_inner(f);
                }
            }
            _ => {}
        }
    }
}

#[cfg(feature = "serde_json")]
impl Value {
    /// Extracts the thematic attribute tree and converts it to a JSON representation.
    /// Lossy conversion: Measure are converted to f64, Date to ISO 8601 string.
    pub fn to_attribute_json(&self) -> serde_json::Value {
        use Value::*;
        match &self {
            String(s) => serde_json::Value::String(s.into()),
            Code(c) => serde_json::Value::String(c.value().to_owned()),
            Integer(i) => serde_json::Value::Number((*i).into()),
            NonNegativeInteger(i) => serde_json::Value::Number((*i).into()),
            Double(d) => serde_json::Value::Number(serde_json::Number::from_f64(*d).unwrap()),
            Measure(m) => {
                let value = m.value().parse::<f64>().unwrap_or(0.0);
                serde_json::Value::Number(serde_json::Number::from_f64(value).unwrap())
            }
            Boolean(b) => serde_json::Value::Bool(*b),
            Uri(u) => serde_json::Value::String(u.value().to_string()),
            Date(d) => serde_json::Value::String(d.to_string()), // ISO 8601
            Point(p) => {
                json! {
                    {
                        "type": "Point",
                        "coordinates": p.coordinates()
                    }
                }
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
                if let Some(id) = cls.stereotype.id() {
                    m.insert("id".into(), serde_json::Value::String(id.into()));
                }
                m.insert("type".into(), cls.typename.clone().into());
                serde_json::Value::Object(m)
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "serde_json")]
mod tests {
    use serde_json::json;

    use super::*;

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

        let obj = Value::Measure(Measure::new("1.0".to_string(), None));
        assert_eq!(obj.to_attribute_json(), json!(1.0));

        let obj = Value::Boolean(true);
        assert_eq!(obj.to_attribute_json(), json!(true));

        let obj = Value::Uri(Uri::new(url::Url::parse("http://example.com").unwrap()));
        assert_eq!(obj.to_attribute_json(), json!("http://example.com/"));

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
