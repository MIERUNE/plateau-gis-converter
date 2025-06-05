use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, InterpolatableProperty, ReferenceValue, ReferenceValueProperty};

pub type DistanceDisplayCondition = DistanceDisplayConditionValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum DistanceDisplayConditionValueType {
    Array(Vec<DistanceDisplayConditionProperties>),
    Object(Box<DistanceDisplayConditionProperties>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum DistanceDisplayConditionValue {
    Constant([f64; 2]),
    TimeTagged(Vec<DistanceDisplayConditionTimeTagged>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DistanceDisplayConditionTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub near_distance: f64,
    pub far_distance: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DistanceDisplayConditionValueProperty {
    pub near_distance: f64,
    pub far_distance: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DistanceDisplayConditionProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_display_condition: Option<DistanceDisplayConditionValue>,

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
    pub distance_display_condition_value_property: Option<DistanceDisplayConditionValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
