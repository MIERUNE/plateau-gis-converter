use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, InterpolatableProperty, ReferenceValue, ReferenceValueProperty};

#[derive(Serialize, Deserialize)]
pub struct Translation {
    #[serde(flatten)]
    pub value: TranslationValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranslationValueType {
    Array(Vec<TranslationProperties>),
    Object(TranslationProperties),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Cartesian3Value {
    Constant([f64; 3]),
    TimeTagged(Vec<Cartesian3TimeTagged>),
}

#[derive(Serialize, Deserialize)]
pub struct Cartesian3TimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Cartesian3ValueProperty {
    pub value: Option<Cartesian3Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationProperties {
    pub cartesian: Option<Cartesian3Value>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub distance_display_condition_value_property: Option<Cartesian3ValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
