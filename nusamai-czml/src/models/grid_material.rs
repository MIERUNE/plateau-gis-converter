use serde::{Deserialize, Serialize};

use crate::{
    Color, CzmlDouble, LineCount, LineCountProperties, LineOffset, LineThickness,
    LineThicknessProperties,
};

pub type GridMaterial = GridMaterialType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum GridMaterialType {
    Array(Vec<GridMaterialProperties>),
    Object(GridMaterialProperties),
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
    Color::new(1.0, 1.0, 1.0, 1.0)
}

fn default_cell_alpha() -> CzmlDouble {
    CzmlDouble::Double(0.5)
}

fn default_line_count() -> LineCount {
    LineCount::Object(LineCountProperties {
        cartesian2: Some(vec![8, 8]),
        reference: None,
        interpolatable_property: None,
        deletable_property: None,
        cartesian2_value_property: None,
        reference_value_property: None,
    })
}

fn default_line_thickness() -> LineThickness {
    LineThickness::Object(LineThicknessProperties {
        cartesian2: Some(vec![1.0, 1.0]),
        reference: None,
        interpolatable_property: None,
        deletable_property: None,
        cartesian2_value_property: None,
        reference_value_property: None,
    })
}

fn default_line_offset() -> LineOffset {
    LineOffset::Object(LineOffset {
        cartesian2: Some(vec![1.0, 1.0]),
        reference: None,
        interpolatable_property: None,
        deletable_property: None,
        cartesian2_value_property: None,
        reference_value_property: None,
    })
}