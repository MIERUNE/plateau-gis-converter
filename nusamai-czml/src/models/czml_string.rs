use serde::{Deserialize, Serialize};

use crate::models::deletable_property::DeletableProperty;

#[derive(Serialize, Deserialize)]
pub struct CzmlString {
    #[serde(flatten)]
    pub value: StringValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringValueType {
    Array(Vec<StringValue>),
    Object(StringValue),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct StringValue {
    // string.json自体に定義されているproperties
    pub string: Option<StringValueItem>,
    pub reference: Option<StringValueItem>,
    // allOfに定義されているproperties
    pub deletable_property: Option<DeletableProperty>,
    pub string_value_property: Option<StringValueProperty>,
    pub reference_value_property: Option<StringValueProperty>,
}
