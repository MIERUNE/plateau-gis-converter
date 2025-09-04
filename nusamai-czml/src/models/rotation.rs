use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, InterpolatableProperty, ReferenceValue, ReferenceValueProperty};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rotation {
    #[serde(flatten)]
    pub value: RotationValueType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RotationValueType {
    Array(Vec<RotationProperties>),
    Object(Box<RotationProperties>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum UnitQuaternionValue {
    Constant([f64; 4]),
    TimeTagged(Vec<UnitQuaternionTimeTagged>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitQuaternionTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitQuaternionValueProperty {
    pub value: Option<UnitQuaternionValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RotationProperties {
    pub unit_quaternion: Option<UnitQuaternionValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub distance_display_condition_value_property: Option<UnitQuaternionValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            value: RotationValueType::Object(Box::new(RotationProperties {
                unit_quaternion: Some(UnitQuaternionValue::Constant([0.0, 0.0, 0.0, 1.0])),
                reference: None,
                interpolatable_property: None,
                deletable_property: None,
                distance_display_condition_value_property: None,
                reference_value_property: None,
            })),
        }
    }
}
