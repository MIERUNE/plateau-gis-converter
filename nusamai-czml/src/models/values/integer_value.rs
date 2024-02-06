use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum IntegerValue {
    Single(i32),
    Constant([i32; 1]),
    TimeTagged(Vec<IntegerTimeTagged>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IntegerTimeTagged {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub value: i32,
}
