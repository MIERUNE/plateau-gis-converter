use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

#[derive(Serialize, Deserialize)]
pub struct HeightReference {
    #[serde(flatten)]
    pub value: HeightReferenceValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeightReferenceValueType {
    Array(Vec<HeightReferenceProperties>),
    Object(HeightReferenceProperties),
    String(String),
}

pub type HeightReferenceValue = String;

pub type HeightReferenceValueProperty = Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeightReferenceProperties {
    pub height_reference: Option<HeightReferenceValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub string_value_property: Option<HeightReferenceValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
