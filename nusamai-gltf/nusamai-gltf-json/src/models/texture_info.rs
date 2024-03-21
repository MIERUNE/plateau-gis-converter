use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Reference to a texture.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde[rename_all = "camelCase"]]
#[serde(deny_unknown_fields)]
pub struct TextureInfo {
    /// The index of the texture.
    pub index: u32,

    /// This integer value is used to construct a string in the format `TEXCOORD_<set index>` which is a reference to a key in `mesh.primitives.attributes` (e.g. a value of `0` corresponds to `TEXCOORD_0`). A mesh primitive **MUST** have the corresponding texture coordinate attributes for the material to be applicable to it.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tex_coord: u32,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<TextureInfoExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextureInfoExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
