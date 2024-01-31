use serde::{Deserialize, Serialize};

use crate::{DeletableProperty, ReferenceValue, ReferenceValueProperty};

pub type ClassificationType = ClassificationTypeValueType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ClassificationTypeValueType {
    Array(Vec<ClassificationTypeProperties>),
    Object(ClassificationTypeProperties),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClassificationTypeValue {
    Terrain,
    Cesium3dTile,
    Both,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassificationTypeValueProperty {
    #[serde(flatten)]
    pub classification_type: ClassificationTypeValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationTypeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_type: Option<ClassificationTypeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceValue>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable_property: Option<DeletableProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_type_value_property: Option<ClassificationTypeValueProperty>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_value_property: Option<ReferenceValueProperty>,
}
