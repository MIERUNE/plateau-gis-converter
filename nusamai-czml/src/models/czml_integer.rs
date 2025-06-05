use serde::{Deserialize, Serialize};

use crate::{
    DeletableProperty, IntegerValue, IntegerValueProperty, InterpolatableProperty, ReferenceValue,
    ReferenceValueProperty,
};

pub type CzmlInteger = IntegerValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum IntegerValueType {
    Array(Vec<IntegerProperties>),
    Object(Box<IntegerProperties>),
    Integer(i32),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntegerProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<IntegerValue>,

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
    pub uri_value_property: Option<IntegerValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
