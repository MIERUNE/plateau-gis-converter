use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct EXTTextureWebP {
    pub source: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]

pub struct TextureExtensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "EXT_texture_webp")]
    pub ext_texture_webp: Option<EXTTextureWebP>,

    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}
