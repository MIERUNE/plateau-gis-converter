use serde::{Deserialize, Serialize};

use crate::DoubleValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct DoubleValueProperty {
    pub value: Option<DoubleValue>,
}
