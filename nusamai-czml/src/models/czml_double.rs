use serde::{Deserialize, Serialize};

use crate::{
    DeletableProperty, DoubleValue, DoubleValueProperty, InterpolatableProperty, ReferenceValue,
    ReferenceValueProperty,
};

pub type CzmlDouble = DoubleValueType;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DoubleValueType {
    Array(Vec<DoubleProperties>),
    Object(DoubleProperties),
    Double(f32),
}

#[derive(Serialize, Deserialize)]
pub struct DoubleProperties {
    pub number: Option<DoubleValue>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub uri_value_property: Option<DoubleValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
