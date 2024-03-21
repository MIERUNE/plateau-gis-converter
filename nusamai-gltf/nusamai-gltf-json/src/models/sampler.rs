use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u16)]
pub enum MagFilter {
    #[default]
    Nearest = 9728,
    Linear = 9729,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u16)]
pub enum MinFilter {
    #[default]
    Nearest = 9728,
    Linear = 9729,
    NearestMipmapNearest = 9984,
    LinearMipmapNearest = 9985,
    NearestMipmapLinear = 9986,
    LinearMipmapLinear = 9987,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u16)]
pub enum WrappingMode {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    #[default]
    Repeat = 10497,
}

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Sampler {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Magnification filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag_filter: Option<MagFilter>,

    /// Minification filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_filter: Option<MinFilter>,

    /// S (U) wrapping mode.  All valid values correspond to WebGL enums.
    #[serde(skip_serializing_if = "is_default")]
    pub wrap_s: WrappingMode,

    /// T (V) wrapping mode.
    #[serde(skip_serializing_if = "is_default")]
    pub wrap_t: WrappingMode,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<SamplerExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SamplerExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
