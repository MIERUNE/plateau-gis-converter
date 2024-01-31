use serde::{Deserialize, Serialize};

pub type Material = MaterialType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MaterialType {
    Array(Vec<MaterialProperties>),
    Object(MaterialProperties),
    Material(Material),
}

#[derive(Serialize, Deserialize, Debug)]
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
