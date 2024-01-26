use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeletableProperty {
    pub delete: bool,
}
