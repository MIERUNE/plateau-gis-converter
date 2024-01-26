use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type ShadowMode = ShadowModeValueType;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShadowModeValueType {
    Array(Vec<ShadowModeProperties>),
    Object(ShadowModeProperties),
    String(String),
}

pub type ShadowModeValue = String;

pub type ShadowModeValueProperty = Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShadowModeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_mode: Option<ShadowModeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value_property: Option<ShadowModeValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
