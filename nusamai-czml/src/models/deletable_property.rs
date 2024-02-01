use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletableProperty {
    pub delete: bool,
}
