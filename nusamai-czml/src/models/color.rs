use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, InterpolatableProperty, ReferenceValue, ReferenceValueProperty};

#[derive(Serialize, Deserialize)]
pub struct Color {
    #[serde(flatten)]
    pub value: ColorValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorValueType {
    Array(Vec<ColorProperties>),
    Object(ColorProperties),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RgbaValue {
    Constant([u8; 4]),
    RgbaTimeTagged(Vec<RgbaTimeTagged>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RgbafValue {
    Constant([f32; 4]),
    TimeTagged(Vec<RgbafTimeTagged>),
}

#[derive(Serialize, Deserialize)]
pub struct RgbafTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(Serialize, Deserialize)]
pub struct RgbaTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Serialize, Deserialize)]
pub struct RgbaValueProperty {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Serialize, Deserialize)]
pub struct RgbafValueProperty {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorProperties {
    pub rgba: Option<RgbaValue>,
    pub rgbaf: Option<RgbafValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub rgba_value_property: Option<RgbaValueProperty>,
    #[serde(flatten)]
    pub rgbaf_value_property: Option<RgbafValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
