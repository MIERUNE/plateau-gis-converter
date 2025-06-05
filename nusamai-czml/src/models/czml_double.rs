use serde::{Deserialize, Serialize};

use crate::{
    DeletableProperty, DoubleValue, DoubleValueProperty, InterpolatableProperty, ReferenceValue,
    ReferenceValueProperty,
};

pub type CzmlDouble = DoubleValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum DoubleValueType {
    Array(Vec<DoubleProperties>),
    Object(Box<DoubleProperties>),
    Double(f32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoubleProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<DoubleValue>,

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
    pub uri_value_property: Option<DoubleValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
