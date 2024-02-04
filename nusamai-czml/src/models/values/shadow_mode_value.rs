use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShadowModeValue {
    Disabled,
    Enabled,
    CastOnly,
    ReceiveOnly,
}
