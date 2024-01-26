use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RgbafValueProperty {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
