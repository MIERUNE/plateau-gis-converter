use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type HeightReference = HeightReferenceValueType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum HeightReferenceValueType {
    Array(Vec<HeightReferenceProperties>),
    Object(HeightReferenceProperties),
    String(HeightReferenceValue),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HeightReferenceValue {
    None,
    ClampToGround,
    RelativeToGround,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct HeightReferenceValueProperty {
    #[serde(flatten)]
    pub height_reference_value: HeightReferenceValue,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HeightReferenceProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_reference: Option<HeightReferenceValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value_property: Option<HeightReferenceValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
