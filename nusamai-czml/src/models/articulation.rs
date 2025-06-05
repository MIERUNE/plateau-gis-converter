use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::{
    DeletableProperty, DoubleValue, DoubleValueProperty, InterpolatableProperty, ReferenceValue,
    ReferenceValueProperty,
};

pub type Articulation = ArticulationValueType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ArticulationValueType {
    Array(Vec<ArticulationProperties>),
    Object(Box<ArticulationProperties>),
    Number(Number),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticulationProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<DoubleValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolatable_property: Option<InterpolatableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri_value_property: Option<DoubleValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Articulations {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articulations: Option<HashMap<String, Articulation>>,
}
