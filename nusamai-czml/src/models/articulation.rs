use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::{
    DeletableProperty, DoubleValue, DoubleValueProperty, InterpolatableProperty, ReferenceValue,
    ReferenceValueProperty,
};

pub type Articulation = ArticulationValueType;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArticulationValueType {
    Array(Vec<ArticulationProperties>),
    Object(ArticulationProperties),
    Number(Number),
}

#[derive(Serialize, Deserialize)]
pub struct ArticulationProperties {
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
