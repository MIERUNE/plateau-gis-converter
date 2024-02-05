use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type CzmlUri = UriValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum UriValueType {
    Array(Vec<UriProperties>),
    Object(UriProperties),
    String(String),
}

pub type UriValue = String;

pub type UriValueProperty = Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UriProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<UriValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_value_property: Option<UriValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
