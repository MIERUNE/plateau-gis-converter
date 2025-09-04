use serde::{Deserialize, Serialize};

use crate::Color;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SolidColorMaterial {
    #[serde(default = "default_color")]
    pub color: Color,
}

fn default_color() -> Color {
    Color::Object(Box::default())
}
