use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Cartesian3Value {
    Constant([f64; 3]),
    TimeTagged(Vec<Cartesian3TimeTagged>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cartesian3TimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
