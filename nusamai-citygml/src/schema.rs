//! Programmatically readable representation of the CityGML model.

use indexmap::IndexMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Schema {
    pub types: IndexMap<String, TypeDef, ahash::RandomState>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum TypeDef {
    Feature(FeatureTypeDef),
    Data(DataTypeDef),
    Property(PropertyTypeDef),
}

pub type Map = IndexMap<String, Attribute, ahash::RandomState>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DataTypeDef {
    pub attributes: Map,
    #[serde(default, skip_serializing_if = "is_false")]
    pub any: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FeatureTypeDef {
    pub attributes: Map,
    #[serde(default, skip_serializing_if = "is_false")]
    pub any: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PropertyTypeDef {
    pub members: Vec<Attribute>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attribute {
    #[serde(rename = "ref")]
    pub type_ref: TypeRef,
    #[serde(default, skip_serializing_if = "is_one")]
    pub min_occurs: u16,
    #[serde(default, skip_serializing_if = "is_some_one")]
    pub max_occurs: Option<u16>,
}

impl Attribute {
    pub fn new(ty: TypeRef) -> Self {
        Self {
            type_ref: ty,
            ..Default::default()
        }
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            type_ref: TypeRef::Unknown,
            min_occurs: 1,
            max_occurs: Some(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum TypeRef {
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
