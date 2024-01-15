//! Programmatically readable representation of the CityGML model.

use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

pub type Map = IndexMap<String, TypeRef, ahash::RandomState>;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Schema {
    pub types: IndexMap<String, TypeDef, ahash::RandomState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeDef {
    #[serde(rename = "stereoType")]
    pub stereo_type: StereoType,
    pub attributes: Map,
    #[serde(default, skip_serializing_if = "is_false")]
    pub any: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum StereoType {
    Data,
    Feature,
    // Object
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypeRef {
    #[serde(rename = "type")]
    pub ty: Type,
    #[serde(default, skip_serializing_if = "is_one")]
    pub min_occurs: u16,
    #[serde(default, skip_serializing_if = "is_some_one")]
    pub max_occurs: Option<u16>,
}

impl TypeRef {
    pub fn new(ty: Type) -> Self {
        Self {
            ty,
            ..Default::default()
        }
    }
}

impl Default for TypeRef {
    fn default() -> Self {
        Self {
            ty: Type::Unknown,
            min_occurs: 1,
            max_occurs: Some(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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
    Property(Vec<TypeRef>),
}

fn is_false(n: &bool) -> bool {
    !(*n)
}

fn is_one(n: &u16) -> bool {
    *n == 1
}

fn is_some_one(n: &Option<u16>) -> bool {
    match n {
        Some(n) => is_one(n),
        None => false,
    }
}
