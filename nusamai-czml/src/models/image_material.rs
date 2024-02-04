use serde::{Deserialize, Serialize};

use crate::{Color, CzmlBoolean, CzmlUri, Repeat};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImageMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CzmlUri>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat: Option<Repeat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent: Option<CzmlBoolean>,
}
