use serde::{Deserialize, Serialize};

use crate::{
    Color, ColorProperties, CzmlDouble, LineCount, LineCountProperties, LineOffset,
    LineOffsetProperties, LineThickness, LineThicknessProperties, RgbaValue,
};

pub type GridMaterial = GridMaterialType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum GridMaterialType {
    Array(Vec<GridMaterialProperties>),
    Object(Box<GridMaterialProperties>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GridMaterialProperties {
    #[serde(default = "default_color")]
    pub color: Color,

    #[serde(default = "default_cell_alpha")]
    pub cell_alpha: CzmlDouble,

    #[serde(default = "default_line_count")]
    pub line_count: LineCount,

    #[serde(default = "default_line_thickness")]
    pub line_thickness: LineThickness,

    #[serde(default = "default_line_offset")]
    pub line_offset: LineOffset,
}

fn default_color() -> Color {
    Color::Object(Box::new(ColorProperties {
        rgba: Some(RgbaValue::Constant([0, 0, 0, 0])),
        ..Default::default()
    }))
}

fn default_cell_alpha() -> CzmlDouble {
    CzmlDouble::Double(0.5)
}

fn default_line_count() -> LineCount {
    LineCount::Object(Box::new(LineCountProperties {
        cartesian2: Some(vec![8.0, 8.0]),
        ..Default::default()
    }))
}

fn default_line_thickness() -> LineThickness {
    LineThickness::Object(Box::new(LineThicknessProperties {
        cartesian2: Some(vec![1.0, 1.0]),
        ..Default::default()
    }))
}

fn default_line_offset() -> LineOffset {
    LineOffset::Object(Box::new(LineOffsetProperties {
        cartesian2: Some(vec![1.0, 1.0]),
        ..Default::default()
    }))
}
