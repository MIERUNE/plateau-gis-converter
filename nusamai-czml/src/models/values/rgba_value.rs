use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RgbaValue {
    Constant([u8; 4]),
    RgbaTimeTagged(Vec<RgbaTimeTagged>),
}

#[derive(Serialize, Deserialize)]
pub struct RgbaTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
