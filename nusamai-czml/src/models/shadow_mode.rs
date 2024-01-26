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
    pub shadow_mode: Option<ShadowModeValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub string_value_property: Option<ShadowModeValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
