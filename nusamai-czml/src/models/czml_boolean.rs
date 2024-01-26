use serde::{Deserialize, Serialize};

use crate::{
    BooleanValue, BooleanValueProperty, DeletableProperty, ReferenceValue, ReferenceValueProperty,
};

#[derive(Serialize, Deserialize)]
pub struct CzmlBoolean {
    #[serde(flatten)]
    pub value: BooleanValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BooleanValueType {
    Array(Vec<BooleanProperties>),
    Object(BooleanProperties),
    Boolean(bool),
}

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
