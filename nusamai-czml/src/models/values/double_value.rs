use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum DoubleValue {
    Single(f64),
    Constant([f64; 1]),
    TimeTagged(Vec<DoubleTimeTagged>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DoubleTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub value: f64,
}
