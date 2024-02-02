use serde::{Deserialize, Serialize};

use crate::{
    DeletableProperty, ReferenceValue, ReferenceValueProperty, ShadowModeValue,
    ShadowModeValueProperty,
};

pub type ShadowMode = ShadowModeValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ShadowModeValueType {
    Array(Vec<ShadowModeProperties>),
    Object(ShadowModeProperties),
    String(ShadowModeValue),
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
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
