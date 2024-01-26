use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type ColorBlendMode = ColorBlendModeValueType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ColorBlendModeValueType {
    Array(Vec<ColorBlendModeProperties>),
    Object(ColorBlendModeProperties),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColorBlendModeValue {
    #[serde(rename = "HIGHLIGHT")]
    Highlight,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "MIX")]
    Mix,
}

pub type ColorBlendModeValueProperty = Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColorBlendModeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_blend_mode: Option<ColorBlendModeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value_property: Option<ColorBlendModeValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
