use serde::{Deserialize, Serialize};

use crate::{
    CheckerboardMaterial, GridMaterial, ImageMaterial, SolidColorMaterial, StripeMaterial,
};

pub type Material = MaterialType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum MaterialType {
    Array(Vec<MaterialProperties>),
    Object(Box<MaterialProperties>),
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MaterialProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_color: Option<SolidColorMaterial>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageMaterial>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<GridMaterial>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe: Option<StripeMaterial>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkerboard: Option<CheckerboardMaterial>,
}
