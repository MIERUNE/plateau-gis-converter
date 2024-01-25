use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StringValue {
    #[serde(flatten)]
    pub value: String,
}
