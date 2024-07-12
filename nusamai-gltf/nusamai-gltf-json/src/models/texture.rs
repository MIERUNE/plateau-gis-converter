use crate::extensions;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A texture and its sampler.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde[rename_all = "camelCase"]]
#[serde(deny_unknown_fields)]
pub struct Texture {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The index of the sampler used by this texture. When undefined, a sampler with repeat wrapping and auto filtering **SHOULD** be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<u32>,

    /// The index of the image used by this texture. When undefined, an extension or other mechanism **SHOULD** supply an alternate texture source, otherwise behavior is undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<u32>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::texture::TextureExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}
