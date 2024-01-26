use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, InterpolatableProperty, ReferenceValue, ReferenceValueProperty};

#[derive(Serialize, Deserialize)]
pub struct CzmlDouble {
    #[serde(flatten)]
    pub value: DoubleValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DoubleValueType {
    Array(Vec<DoubleProperties>),
    Object(DoubleProperties),
    String(String),
}

pub type DoubleValue = String;

pub type DoubleValueProperty = Value;

#[derive(Serialize, Deserialize)]
pub struct DoubleProperties {
    pub number: Option<DoubleValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub uri_value_property: Option<DoubleValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
