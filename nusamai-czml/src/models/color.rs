use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, InterpolatableProperty, ReferenceValue, ReferenceValueProperty};

pub type Color = ColorValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ColorValueType {
    Array(Vec<ColorProperties>),
    Object(Box<ColorProperties>),
}

impl Default for ColorValueType {
    fn default() -> Self {
        ColorValueType::Object(Box::default())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum RgbaValue {
    Constant([u8; 4]),
    RgbaTimeTagged(Vec<RgbaTimeTagged>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum RgbafValue {
    Constant([f32; 4]),
    TimeTagged(Vec<RgbafTimeTagged>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RgbafTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RgbaTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RgbaValueProperty {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RgbafValueProperty {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ColorProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgba: Option<RgbaValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgbaf: Option<RgbafValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolatable_property: Option<InterpolatableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgba_value_property: Option<RgbaValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgbaf_value_property: Option<RgbafValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
