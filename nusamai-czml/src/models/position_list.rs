use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, ReferenceValueProperty};

pub type PositionList = PositionListType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PositionListType {
    Array(Vec<PositionListProperties>),
    Object(PositionListProperties),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PositionListProperties {
    pub reference_frame: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Cartesian3ListValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_radians: Option<CartographicRadiansListValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_degrees: Option<CartographicDegreesListValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<ReferenceListValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian3_list_value_property: Option<Cartesian3ListValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_radians_list_value_property: Option<CartographicRadiansListValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_degrees_list_value_property: Option<CartographicDegreesListValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
