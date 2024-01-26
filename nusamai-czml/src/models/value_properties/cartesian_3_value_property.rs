use serde::{Deserialize, Serialize};

use crate::Cartesian3Value;

#[derive(Serialize, Deserialize)]
pub struct Cartesian3ValueProperty {
    pub value: Option<Cartesian3Value>,
}
