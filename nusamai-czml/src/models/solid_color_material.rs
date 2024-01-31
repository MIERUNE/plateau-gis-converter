use serde::{Deserialize, Serialize};

use crate::Color;

#[derive(Serialize, Deserialize, Debug)]
pub struct SolidColorMaterial {
    #[serde(default = "default_color")]
    pub color: Color,
}

fn default_color() -> Color {
    Color::from_rgb(1.0, 1.0, 1.0)
}
