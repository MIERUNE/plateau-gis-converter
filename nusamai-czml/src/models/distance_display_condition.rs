use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, InterpolatableProperty, ReferenceValue, ReferenceValueProperty};

#[derive(Serialize, Deserialize)]
pub struct DistanceDisplayCondition {
    #[serde(flatten)]
    pub value: DistanceDisplayConditionValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DistanceDisplayConditionValueType {
    Array(Vec<DistanceDisplayConditionProperties>),
    Object(DistanceDisplayConditionProperties),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DistanceDisplayConditionValue {
    Constant([f64; 2]),
    TimeTagged(Vec<DistanceDisplayConditionTimeTagged>),
}

#[derive(Serialize, Deserialize)]
pub struct DistanceDisplayConditionTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub near_distance: f64,
    pub far_distance: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DistanceDisplayConditionValueProperty {
    pub near_distance: f64,
    pub far_distance: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistanceDisplayConditionProperties {
    pub distance_display_condition: Option<DistanceDisplayConditionValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub distance_display_condition_value_property: Option<DistanceDisplayConditionValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
