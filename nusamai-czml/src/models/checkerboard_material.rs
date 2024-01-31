use serde::{Deserialize, Serialize};

use crate::{Color, Repeat};

pub type CheckerboardMaterial = CheckerboardMaterialType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum CheckerboardMaterialType {
    Array(Vec<CheckerboardMaterialProperties>),
    Object(CheckerboardMaterialProperties),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CheckerboardMaterialProperties {
    #[serde(default = "default_even_color")]
    pub even_color: Color,

    #[serde(default = "default_odd_color")]
    pub odd_color: Color,

    #[serde(default = "default_repeat")]
    pub repeat: Repeat,
}

fn default_even_color() -> Color {
    Color::from_rgb(1.0, 1.0, 1.0)
}

fn default_odd_color() -> Color {
    Color::from_rgb(0.0, 0.0, 0.0)
}

fn default_repeat() -> Repeat {
    Repeat::Object(Repeat {
        cartesian2: Some(vec![1.0, 1.0]),
        ..Default::default()
    })
}
