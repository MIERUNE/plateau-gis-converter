//! Objectified representation of the city objects.

use crate::geometry::GeometryRef;
use crate::values::{Code, Point, URI};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json;
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

impl ObjectValue {
    pub fn to_value(&self) -> serde_json::Value {
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
            ObjectValue::URI(u) => serde_json::Value::String(serde_json::to_string(u).unwrap()),
            ObjectValue::Date(d) => serde_json::Value::String(d.to_string()),
            // TODO: Handle Point
            // ObjectValue::Point(p) => Value::Array(vec![
            //     Value::Number(serde_json::Number::from_f64(p.x).unwrap()),
            //     Value::Number(serde_json::Number::from_f64(p.y).unwrap()),
            //     Value::Number(serde_json::Number::from_f64(p.z).unwrap()),
            // ]),
            ObjectValue::Array(a) => {
                serde_json::Value::Array(a.iter().map(ObjectValue::to_value).collect())
            }
            ObjectValue::FeatureOrData(fod) => {
                let attributes = self.attribute_to_json(fod);
                let attributes_map: serde_json::Map<String, serde_json::Value> =
                    attributes.into_iter().collect();
                serde_json::Value::Object(attributes_map)
            }
            _ => serde_json::Value::Null,
        }
    }

    pub fn attribute_to_json(&self, fod: &FeatureOrData) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        if let Some(id) = &fod.id {
            map.insert("id".to_string(), serde_json::Value::String(id.clone()));
        }

        for (k, v) in &fod.attributes {
            map.insert(k.clone(), v.to_value());
        }

        map
    }
}
