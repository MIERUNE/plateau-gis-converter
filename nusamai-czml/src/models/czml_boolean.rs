use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type CzmlBoolean = BooleanValueType;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BooleanValueType {
    Array(Vec<BooleanProperties>),
    Object(BooleanProperties),
    Boolean(bool),
}

pub type BooleanValue = String;

pub type BooleanValueProperty = Value;

#[derive(Serialize, Deserialize)]
pub struct BooleanProperties {
    pub boolean: Option<BooleanValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub boolean_value_property: Option<BooleanValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
