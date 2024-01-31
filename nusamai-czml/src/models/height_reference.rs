use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type HeightReference = HeightReferenceValueType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum HeightReferenceValueType {
    Array(Vec<HeightReferenceProperties>),
    Object(HeightReferenceProperties),
    String(String),
}

pub type HeightReferenceValue = String;

pub type HeightReferenceValueProperty = Value;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeightReferenceProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_reference: Option<HeightReferenceValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value_property: Option<HeightReferenceValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
