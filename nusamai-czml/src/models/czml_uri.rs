use serde::{Deserialize, Serialize};

use crate::{
    DeletableProperty, ReferenceValue, ReferenceValueProperty, UriValue, UriValueProperty,
};

#[derive(Serialize, Deserialize)]
pub struct CzmlUri {
    #[serde(flatten)]
    pub value: UriValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum UriValueType {
    Array(Vec<UriProperties>),
    Object(UriProperties),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub struct UriProperties {
    pub uri: Option<UriValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub uri_value_property: Option<UriValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
