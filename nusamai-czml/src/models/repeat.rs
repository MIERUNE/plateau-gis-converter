use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, InterpolatableProperty, ReferenceValue, ReferenceValueProperty};

pub type Repeat = RepeatType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum RepeatType {
    Array(Vec<RepeatProperties>),
    Object(RepeatProperties),
}

pub type Cartesian2Value = Vec<f64>;

pub type Cartesian2ValueProperty = Vec<f64>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RepeatProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian2: Option<Cartesian2Value>,

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
    pub cartesian2_value_property: Option<Cartesian2ValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
