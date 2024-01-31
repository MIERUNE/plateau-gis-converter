use serde::{Deserialize, Serialize};

use crate::{Color, CzmlBoolean, CzmlUri, GridMaterial, Repeat};

pub type Material = MaterialType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MaterialType {
    Array(Vec<MaterialProperties>),
    Object(MaterialProperties),
    Material(Material),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolidColorMaterial {
    #[serde(default = "default_color")]
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CzmlUri>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<Repeat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent: Option<CzmlBoolean>,
}

#[derive(Serialize, Deserialize, Debug)]
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
