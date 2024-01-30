use serde::{Deserialize, Serialize};

use crate::Cartesian3Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cartesian3ValueProperty {
    pub value: Option<Cartesian3Value>,
}
