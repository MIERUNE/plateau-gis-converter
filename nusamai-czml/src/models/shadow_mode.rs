use serde::{Deserialize, Serialize};

use crate::{
    DeletableProperty, ReferenceValue, ReferenceValueProperty, ShadowModeValue,
    ShadowModeValueProperty,
};

#[derive(Serialize, Deserialize)]
pub struct ShadowMode {
    #[serde(flatten)]
    pub value: ShadowModeValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShadowModeValueType {
    Array(Vec<ShadowModeProperties>),
    Object(ShadowModeProperties),
    String(String),
}

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
