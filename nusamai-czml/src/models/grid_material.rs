use serde::{Deserialize, Serialize};

use crate::{Color, CzmlDouble};

pub type GridMaterial = GridMaterialType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum GridMaterialType {
    Array(Vec<GridMaterialProperties>),
    Object(GridMaterialProperties),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GridMaterialProperties {
    pub color: Color,

    pub cell_alpha: CzmlDouble,

    pub line_count: LineCount,

    pub line_thickness: LineThickness,

    pub line_offset: LineOffset,
}
