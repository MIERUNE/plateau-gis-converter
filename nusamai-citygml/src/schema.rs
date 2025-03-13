//! Programmatically readable representation of the CityGML model.

use std::collections::HashSet;

use indexmap::IndexMap;
use nusamai_projection::crs::EpsgCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Schema {
    pub types: TypeMap,
    pub epsg: Option<EpsgCode>,
}

pub type TypeMap = IndexMap<String, TypeDef, ahash::RandomState>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum TypeDef {
    Feature(FeatureTypeDef),
    Data(DataTypeDef),
    Property(PropertyTypeDef),
}

impl TypeDef {
    pub fn attributes(&self) -> Map {
        match self {
            TypeDef::Feature(ft) => ft.attributes.clone(),
            TypeDef::Data(dt) => dt.attributes.clone(),
            TypeDef::Property(pt) => {
                let mut map = Map::default();
                for (i, attr) in pt.members.iter().enumerate() {
                    map.insert(i.to_string(), attr.clone());
                }
                map
            }
        }
    }

    pub fn fields(&self) -> HashSet<String> {
        match self {
            TypeDef::Feature(ft) => ft.attributes.keys().cloned().collect(),
            TypeDef::Data(dt) => dt.attributes.keys().cloned().collect(),
            TypeDef::Property(_) => HashSet::new(),
        }
    }
}

pub type Map = IndexMap<String, Attribute, ahash::RandomState>;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DataTypeDef {
    pub attributes: Map,
    #[serde(default, skip_serializing_if = "is_false")]
    pub additional_attributes: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct FeatureTypeDef {
    pub attributes: Map,
    #[serde(default, skip_serializing_if = "is_false")]
    pub additional_attributes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyTypeDef {
    pub members: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attribute {
    #[serde(rename = "ref")]
    pub type_ref: TypeRef,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub min_occurs: u16,
    #[serde(default, skip_serializing_if = "is_some_one")]
    pub max_occurs: Option<u16>,
    pub original_name: Option<String>,
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
            original_name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum TypeRef {
    Unknown,
    String,
    Code,
    Integer,
    NonNegativeInteger,
    Double,
    Boolean,
    /// A string containing a valid JSON. It contains the original attribute definition.
    JsonString(Box<Attribute>),
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
