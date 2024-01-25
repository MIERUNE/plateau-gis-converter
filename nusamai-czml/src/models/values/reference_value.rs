use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReferenceValue {
    #[serde(flatten)]
    pub value: String,
}
