use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeletableProperty {
    #[serde(rename = "delete")]
    pub delete_flag: bool,
}
