use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type CzmlBoolean = BooleanValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum BooleanValueType {
    Array(Vec<BooleanProperties>),
    Object(BooleanProperties),
    Boolean(bool),
}

pub type BooleanValue = String;

pub type BooleanValueProperty = Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BooleanProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<BooleanValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value_property: Option<BooleanValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
