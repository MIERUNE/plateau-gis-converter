use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type StripeOrientation = StripeOrientationType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StripeOrientationType {
    Array(Vec<StripeOrientationProperties>),
    Object(StripeOrientationProperties),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StripeOrientationValue {
    Horizontal,
    Vertical,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StripeOrientationValueProperty {
    #[serde(flatten)]
    pub arc_type: StripeOrientationValue,
}

pub type Cartesian2Value = Vec<f64>;

pub type Cartesian2ValueProperty = Vec<f64>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StripeOrientationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_orientation: Option<StripeOrientationValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_orientation_value_property: Option<StripeOrientationValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}