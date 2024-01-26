use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    rgba_value_property::RgbaValueProperty, DeletableProperty, InterpolatableProperty,
    ReferenceValue, ReferenceValueProperty, RgbaValue, RgbafValue, RgbafValueProperty,
};

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
