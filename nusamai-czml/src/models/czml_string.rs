use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type CzmlString = StringValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StringValueType {
    Array(Vec<StringProperties>),
    Object(StringProperties),
    String(String),
}

pub type StringValue = String;

pub type StringValueProperty = Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StringProperties {
    pub string: Option<StringValue>,

    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    pub string_value_property: Option<StringValueProperty>,

    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
