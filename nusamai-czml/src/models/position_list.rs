use serde::{Deserialize, Serialize};

use crate::DeletableProperty;

pub type PositionList = PositionListType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PositionListType {
    Array(Vec<PositionListProperties>),
    Object(PositionListProperties),
}

pub type Cartesian3ListValue = Vec<f64>;
pub type CartographicRadiansListValue = Vec<f64>;
pub type CartographicDegreesListValue = Vec<f64>;
pub type ReferenceListValue = Vec<String>;

pub type Cartesian3ListValueProperty = Vec<f64>;
pub type CartographicRadiansListValueProperty = Vec<f64>;
pub type CartographicDegreesListValueProperty = Vec<f64>;
pub type ReferenceListValueProperty = Vec<String>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PositionListProperties {
    #[serde(
        default = "default_reference_frame",
        skip_serializing_if = "is_default_reference_frame"
    )]
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
    pub reference_value_property: Option<ReferenceListValueProperty>,
}

fn default_reference_frame() -> String {
    String::from("FIXED")
}

fn is_default_reference_frame(reference_frame: &String) -> bool {
    *reference_frame == "FIXED"
}

impl Default for PositionListProperties {
    fn default() -> Self {
        Self {
            reference_frame: String::from("FIXED"),
            cartesian: None,
            cartographic_radians: None,
            cartographic_degrees: None,
            references: None,
            deletable_property: None,
            cartesian3_list_value_property: None,
            cartographic_radians_list_value_property: None,
            cartographic_degrees_list_value_property: None,
            reference_value_property: None,
        }
    }
}
