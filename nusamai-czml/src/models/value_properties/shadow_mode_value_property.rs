use serde::{Deserialize, Serialize};

use crate::ShadowModeValue;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ShadowModeValueProperty {
    pub value: Option<ShadowModeValue>,
}
