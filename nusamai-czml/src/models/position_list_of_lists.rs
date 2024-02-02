use serde::{Deserialize, Serialize};

use crate::DeletableProperty;

pub type PositionListOfLists = PositionListOfListsType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PositionListOfListsType {
    Array(Vec<PositionListOfListsProperties>),
    Object(PositionListOfListsProperties),
}

pub type Cartesian3ListOfListsValue = Vec<Vec<f64>>;
pub type CartographicRadiansListOfListsValue = Vec<Vec<f64>>;
pub type CartographicDegreesListOfListsValue = Vec<Vec<f64>>;
pub type ReferenceListOfListsValue = Vec<Vec<String>>;

pub type Cartesian3ListOfListsValueProperty = Vec<Vec<f64>>;
pub type CartographicRadiansListOfListsValueProperty = Vec<Vec<f64>>;
pub type CartographicDegreesListOfListsValueProperty = Vec<Vec<f64>>;
pub type ReferenceListOfListsValueProperty = Vec<Vec<String>>;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PositionListOfListsProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian: Option<Cartesian3ListOfListsValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_radians: Option<CartographicRadiansListOfListsValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_degrees: Option<CartographicDegreesListOfListsValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<ReferenceListOfListsValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartesian3_list_of_lists_value_property: Option<Cartesian3ListOfListsValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_radians_list_of_lists_value_property:
        Option<CartographicRadiansListOfListsValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartographic_degrees_list_of_lists_value_property:
        Option<CartographicDegreesListOfListsValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_list_of_lists_value_property: Option<ReferenceListOfListsValueProperty>,
}
