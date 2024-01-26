use serde::{Deserialize, Serialize};

use crate::DoubleValue;

#[derive(Serialize, Deserialize)]
pub struct DoubleValueProperty {
    pub value: Option<DoubleValue>,
}
