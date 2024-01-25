use serde::{Deserialize, Serialize};

use crate::{
    DeletableProperty, ReferenceValue, ReferenceValueProperty, StringValue, StringValueProperty,
};

#[derive(Serialize, Deserialize)]
pub struct CzmlString {
    #[serde(flatten)]
    pub value: ValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueType {
    Array(Vec<StringProperties>),
    Object(StringProperties),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct StringProperties {
    // string.json自体に定義されているproperties
    pub string: Option<StringValue>,
    pub reference: Option<ReferenceValue>,
    // allOfに定義されているproperties
    pub deletable_property: Option<DeletableProperty>,
    pub string_value_property: Option<StringValueProperty>,
    pub reference_value_property: Option<ReferenceValueProperty>,
}
