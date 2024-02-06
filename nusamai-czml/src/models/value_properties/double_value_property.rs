use serde::{Deserialize, Serialize};

use crate::DoubleValue;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DoubleValueProperty {
    pub value: Option<DoubleValue>,
}
