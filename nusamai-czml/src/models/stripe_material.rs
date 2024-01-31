use serde::{Deserialize, Serialize};

use crate::{Color, CzmlDouble, StripeOrientation};

pub type StripeMaterial = StripeMaterialType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StripeMaterialType {
    Array(Vec<StripeMaterialProperties>),
    Object(StripeMaterialProperties),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StripeMaterialProperties {
    #[serde(default = "default_orientation")]
    pub orientation: StripeOrientation,

    #[serde(default = "default_even_color")]
    pub even_color: Color,

    #[serde(default = "default_odd_color")]
    pub odd_color: Color,

    #[serde(default = "default_offset")]
    pub offset: CzmlDouble,

    #[serde(default = "default_repeat")]
    pub repeat: CzmlDouble,
}

fn default_orientation() -> StripeOrientation {
    StripeOrientation::Horizontal
}

fn default_even_color() -> Color {
    Color::from_rgb(1.0, 1.0, 1.0)
}

fn default_odd_color() -> Color {
    Color::from_rgb(0.0, 0.0, 0.0)
}

fn default_offset() -> CzmlDouble {
    CzmlDouble::Double(0.0)
}

fn default_repeat() -> CzmlDouble {
    CzmlDouble::Double(1.0)
}
