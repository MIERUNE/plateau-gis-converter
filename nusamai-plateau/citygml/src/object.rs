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
