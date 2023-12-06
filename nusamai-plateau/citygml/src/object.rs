//! Objectify CityGML features and their thematic/geometric attributes

use crate::geometric::GeometryRef;
use crate::model::{Code, URI};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FeatureOrData<'a> {
    pub typename: &'a str,
    pub id: Option<&'a str>,
    pub attributes: HashMap<String, ObjectValue<'a>>,
    pub geometries: Option<&'a GeometryRef>,
}

#[derive(Debug)]
pub enum ObjectValue<'a> {
    Unknown,
    Null,
    String(&'a str),
    Code(&'a Code),
    Integer(i64),
    Double(f64),
    Measure(f64),
    Bool(bool),
    URI(&'a URI),
    // Date(Date),
    Array(Vec<ObjectValue<'a>>),
    FeatureOrData(FeatureOrData<'a>),
}
