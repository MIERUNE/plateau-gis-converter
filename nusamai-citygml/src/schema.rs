//! Programmatically readable representation of the CityGML model.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub types: HashMap<String, TypeDef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeDef {
    pub stereo_type: StereoType,
    /// name -> type
    pub attributes: HashMap<String, TypeRef>,
    pub any: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StereoType {
    Data,
    Feature,
    // Object
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeRef {
    pub ty: Type,
    pub min_occurs: u16,
    pub max_occurs: Option<u16>,
}

impl TypeRef {
    pub fn new(ty: Type) -> Self {
        Self {
            ty,
            min_occurs: 1,
            max_occurs: Some(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    Unknown,
    String,
    Code,
    Integer,
    NonNegativeInteger,
    Double,
    Boolean,
    URI,
    Date,
    DataTime,
    Measure,
    Point,
    Ref(String),
}
