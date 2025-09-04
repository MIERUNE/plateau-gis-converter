use serde::{Deserialize, Serialize};

use crate::{
    Cartesian2Value, Cartesian2ValueProperty, DeletableProperty, InterpolatableProperty,
    ReferenceValue, ReferenceValueProperty,
};

pub type LineOffset = LineOffsetType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum LineOffsetType {
    Array(Vec<LineOffsetProperties>),
    Object(Box<LineOffsetProperties>),
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LineOffsetProperties {
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
