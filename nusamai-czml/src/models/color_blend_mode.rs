use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

#[derive(Serialize, Deserialize)]
pub struct ColorBlendMode {
    #[serde(flatten)]
    pub value: ColorBlendModeValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorBlendModeValueType {
    Array(Vec<ColorBlendModeProperties>),
    Object(ColorBlendModeProperties),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub enum ColorBlendModeValue {
    #[serde(rename = "HIGHLIGHT")]
    Highlight,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "MIX")]
    Mix,
}

pub type ColorBlendModeValueProperty = Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorBlendModeProperties {
    pub color_blend_mode: Option<ColorBlendModeValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub string_value_property: Option<ColorBlendModeValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
