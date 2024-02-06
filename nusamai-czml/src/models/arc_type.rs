use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type ArcType = ArcTypeType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ArcTypeType {
    Array(Vec<ArcTypeProperties>),
    Object(ArcTypeProperties),
    String(ArcTypeValue),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArcTypeValue {
    None,
    Geodesic,
    Rhumb,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ArcTypeValueProperty {
    #[serde(flatten)]
    pub arc_type: ArcTypeValue,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArcTypeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_type: Option<ArcTypeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_type_value_property: Option<ArcTypeValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
