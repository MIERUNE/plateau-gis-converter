use serde::{Deserialize, Serialize};

use crate::ShadowModeValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct ShadowModeValueProperty {
    pub value: Option<ShadowModeValue>,
}
