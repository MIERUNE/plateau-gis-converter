use serde::{Deserialize, Serialize};

use crate::{
    DeletableProperty, ReferenceValue, ReferenceValueProperty, StringValue, StringValueProperty,
};

#[derive(Serialize, Deserialize)]
pub struct CzmlString {
    #[serde(flatten)]
    pub value: StringValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringValueType {
    Array(Vec<StringProperties>),
    Object(StringProperties),
    String(String),
}

#[derive(Serialize, Deserialize)]
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
