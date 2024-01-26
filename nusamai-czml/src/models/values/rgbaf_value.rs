use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RgbafValue {
    Constant([f32; 4]),
    TimeTagged(Vec<RgbafTimeTagged>),
}

#[derive(Serialize, Deserialize)]
pub struct RgbafTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}
