use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RgbaValueProperty {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
