use serde::{Deserialize, Serialize};

use crate::IntegerValue;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IntegerValueProperty {
    pub value: Option<IntegerValue>,
}
