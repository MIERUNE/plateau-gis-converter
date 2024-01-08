//! Object representation of the city objects.

use crate::geometry::{self, GeometryRef};
use crate::values::{Code, Point, URI};
use crate::Measure;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type Map = indexmap::IndexMap<String, Value, ahash::RandomState>;

#[derive(Debug, Deserialize, Serialize)]
pub struct CityObject {
    pub root: Value,
    pub geometries: geometry::GeometryStore,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Feature {
    pub typename: String,
    pub id: Option<String>,
    pub attributes: Map,
    pub geometries: Option<GeometryRef>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
    pub typename: String,
    pub attributes: Map,
}

/// Nodes for the "Object" representation of the city object.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Value {
    String(String),
    Code(Code),
    Integer(i64),
    Double(f64),
    Measure(Measure),
    Boolean(bool),
    URI(URI),
    Date(NaiveDate),
    Point(Point),
    Array(Vec<Value>),
    Feature(Feature),
    Data(Data),
}

#[cfg(feature = "serde_json")]
impl Value {
    /// Extracts the thematic attribute tree and converts it to a JSON representation.
    pub fn to_attribute_json(&self) -> serde_json::Value {
        use Value::*;
        match &self {
            String(s) => serde_json::Value::String(s.clone()),
            Code(c) => serde_json::Value::String(c.value().to_owned()),
            Integer(i) => serde_json::Value::Number((*i).into()),
            Double(d) => serde_json::Value::Number(serde_json::Number::from_f64(*d).unwrap()),
            Measure(m) => serde_json::Value::Number(serde_json::Number::from_f64(m.value).unwrap()),
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
            Array(a) => serde_json::Value::Array(a.iter().map(Value::to_attribute_json).collect()),
            Feature(feat) => {
                let mut m = serde_json::Map::from_iter(
                    feat.attributes
                        .iter()
                        .map(|(k, v)| (k.clone(), v.to_attribute_json())),
                );
                m.insert("type".into(), feat.typename.clone().into());
                m.insert("id".into(), feat.id.clone().into());
                serde_json::Value::Object(m)
            }
            Data(feat) => {
                let mut m = serde_json::Map::from_iter(
                    feat.attributes
                        .iter()
                        .map(|(k, v)| (k.clone(), v.to_attribute_json())),
                );
                m.insert("type".into(), feat.typename.clone().into());
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
        let value = obj.to_attribute_json();
        assert_eq!(value, json!("test"));

        let obj = Value::Code(Code::new("12345".into(), "12345".into()));
        let value = obj.to_attribute_json();
        assert_eq!(value, json!("12345"));

        let obj = Value::Integer(12345);
        let value = obj.to_attribute_json();
        assert_eq!(value, json!(12345));

        let obj = Value::Double(1.0);
        let value = obj.to_attribute_json();
        assert_eq!(value, json!(1.0));

        let obj = Value::Measure(Measure { value: 1.0 });
        let value = obj.to_attribute_json();
        assert_eq!(value, json!(1.0));

        let obj = Value::Boolean(true);
        let value = obj.to_attribute_json();
        assert_eq!(value, json!(true));

        let obj = Value::URI(URI::new("http://example.com"));
        let value = obj.to_attribute_json();
        assert_eq!(value, json!("http://example.com"));

        let obj = Value::Date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        let value = obj.to_attribute_json();
        assert_eq!(value, json!("2020-01-01"));

        let obj = Value::Array(vec![Value::String("test".into()), Value::Integer(1)]);
        let value = obj.to_attribute_json();
        assert_eq!(value, json!(["test", 1]));

        let mut attributes = Map::default();
        attributes.insert("String".into(), Value::String("test".into()));
        attributes.insert("Integer".into(), Value::Integer(1));
        let obj = Value::Feature(Feature {
            typename: "test".into(),
            id: Some("test".into()),
            attributes,
            geometries: None,
        });
        let value = obj.to_attribute_json();
        assert_eq!(
            value,
            json! {
                {
                    "type": "test",
                    "id": "test",
                    "String": "test",
                    "Integer": 1
                }
            }
        );

        let mut attributes = Map::default();
        attributes.insert("String".into(), Value::String("test".into()));
        attributes.insert("Integer".into(), Value::Integer(1));
        let obj = Value::Data(Data {
            typename: "test".into(),
            attributes,
        });
        let value = obj.to_attribute_json();
        assert_eq!(
            value,
            json! {
                {
                    "type": "test",
                    "String": "test",
                    "Integer": 1
                }
            }
        );
    }
}
