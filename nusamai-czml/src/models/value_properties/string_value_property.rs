use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StringValueProperty {
    #[serde(flatten)]
    pub value: String,
}
