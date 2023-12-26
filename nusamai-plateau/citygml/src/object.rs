//! Objectified representation of the city objects.

use crate::geometry::GeometryRef;
use crate::values::{Code, Point, URI};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureOrData {
    pub typename: String,
    pub id: Option<String>,
    pub attributes: HashMap<String, ObjectValue>,
    pub geometries: Option<GeometryRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectValue {
    String(String),
    Code(Code),
    Integer(i64),
    Double(f64),
    Measure(f64),
    Boolean(bool),
    URI(URI),
    Date(NaiveDate),
    Point(Point),
    Array(Vec<ObjectValue>),
    FeatureOrData(FeatureOrData),
}

pub fn attribute_to_json(fod: &FeatureOrData) -> serde_json::Value {
    let mut map = serde_json::Map::new();

    if let Some(id) = &fod.id {
        map.insert("id".to_string(), serde_json::Value::String(id.clone()));
    }

    for (k, v) in &fod.attributes {
        map.insert(k.clone(), v.to_json_value());
    }

    serde_json::Value::Object(map)
}

impl ObjectValue {
    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            ObjectValue::String(s) => serde_json::Value::String(s.clone()),
            ObjectValue::Code(c) => serde_json::Value::String(c.value.to_string()),
            ObjectValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
            ObjectValue::Double(d) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*d).unwrap())
            }
            ObjectValue::Measure(m) => {
                serde_json::Value::Number(serde_json::Number::from_f64(*m).unwrap())
            }
            ObjectValue::Boolean(b) => serde_json::Value::Bool(*b),
            ObjectValue::URI(u) => serde_json::Value::String(u.to_string()),
            ObjectValue::Date(d) => serde_json::Value::String(d.to_string()),
            // TODO: Handle Point
            // ObjectValue::Point(p) => Value::Array(vec![
            //     Value::Number(serde_json::Number::from_f64(p.x).unwrap()),
            //     Value::Number(serde_json::Number::from_f64(p.y).unwrap()),
            //     Value::Number(serde_json::Number::from_f64(p.z).unwrap()),
            // ]),
            ObjectValue::Array(a) => {
                serde_json::Value::Array(a.iter().map(ObjectValue::to_json_value).collect())
            }
            ObjectValue::FeatureOrData(fod) => {
                let attributes = attribute_to_json(fod);
                serde_json::Value::Object(attributes.as_object().unwrap().clone())
            }
            _ => serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_value() {
        let obj = ObjectValue::String("test".to_string());
        let value = obj.to_json_value();
        assert_eq!(value, serde_json::Value::String("test".to_string()));

        let obj = ObjectValue::Code(Code {
            value: "12345".to_string(),
            code: "12345".to_string(),
        });
        let value = obj.to_json_value();
        assert_eq!(value, serde_json::Value::String("12345".to_string()));

        let obj = ObjectValue::Integer(12345);
        let value = obj.to_json_value();
        assert_eq!(
            value,
            serde_json::Value::Number(serde_json::Number::from(12345))
        );

        let obj = ObjectValue::Double(1.0);
        let value = obj.to_json_value();
        assert_eq!(
            value,
            serde_json::Value::Number(serde_json::Number::from_f64(1.0).unwrap())
        );

        let obj = ObjectValue::Measure(1.0);
        let value = obj.to_json_value();
        assert_eq!(
            value,
            serde_json::Value::Number(serde_json::Number::from_f64(1.0).unwrap())
        );

        let obj = ObjectValue::Boolean(true);
        let value = obj.to_json_value();
        assert_eq!(value, serde_json::Value::Bool(true));

        let obj = ObjectValue::URI(URI("http://example.com".to_string()));
        let value = obj.to_json_value();
        assert_eq!(
            value,
            serde_json::Value::String("http://example.com".to_string())
        );

        let obj = ObjectValue::Date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        let value = obj.to_json_value();
        assert_eq!(value, serde_json::Value::String("2020-01-01".to_string()));

        let obj = ObjectValue::Array(vec![
            ObjectValue::String("test".to_string()),
            ObjectValue::Integer(1),
        ]);
        let value = obj.to_json_value();
        assert_eq!(
            value,
            serde_json::Value::Array(vec![
                serde_json::Value::String("test".to_string()),
                serde_json::Value::Number(serde_json::Number::from(1)),
            ])
        );

        let mut attributes = HashMap::new();
        attributes.insert(
            "String".to_string(),
            ObjectValue::String("test".to_string()),
        );
        attributes.insert("Integer".to_string(), ObjectValue::Integer(1));
        let obj = ObjectValue::FeatureOrData(FeatureOrData {
            typename: "test".to_string(),
            id: Some("test".to_string()),
            attributes,
            geometries: None,
        });

        let value = obj.to_json_value();
        println!("{:?}", value);
        assert_eq!(
            value,
            serde_json::Value::Object(
                vec![
                    (
                        "id".to_string(),
                        serde_json::Value::String("test".to_string())
                    ),
                    (
                        "String".to_string(),
                        serde_json::Value::String("test".to_string())
                    ),
                    (
                        "Integer".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(1))
                    ),
                ]
                .into_iter()
                .collect()
            )
        );
    }
}
