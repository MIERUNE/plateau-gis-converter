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

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct DataTypeDef {
    pub attributes: Map,
    #[serde(default, skip_serializing_if = "is_false")]
    pub additional_attributes: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct FeatureTypeDef {
    pub attributes: Map,
    #[serde(default, skip_serializing_if = "is_false")]
    pub additional_attributes: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PropertyTypeDef {
    pub members: Vec<Attribute>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attribute {
    #[serde(rename = "ref")]
    pub type_ref: TypeRef,
    #[serde(default, skip_serializing_if = "is_zero")]
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
            min_occurs: 0,
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
    /// String containing a valid JSON.
    JsonString,
    URI,
    Date,
    DateTime,
    Measure,
    Point,
    /// Reference to a type defined in the schema.
    Named(String),
}

fn is_false(n: &bool) -> bool {
    !(*n)
}

fn is_zero(n: &u16) -> bool {
    *n == 0
}

fn is_some_one(n: &Option<u16>) -> bool {
    match n {
        Some(n) => *n == 1,
        None => false,
    }
}
