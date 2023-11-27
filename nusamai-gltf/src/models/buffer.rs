use serde::{Deserialize, Serialize};
use serde_repr::*;

use std::collections::HashMap;

use serde_json::Value;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum BufferViewTarget {
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

/// A buffer points to binary geometry, animation, or skins.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Buffer {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The URI (or IRI) of the buffer.  Relative paths are relative to the current glTF asset.  Instead of referencing an external file, this field MAY contain a data:-URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// The length of the buffer in bytes.
    pub byte_length: u32,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<BufferExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BufferExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

impl Buffer {
    pub fn new() -> Self {
        Default::default()
    }
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BufferView {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The index of the buffer.
    pub buffer: u32,

    /// The offset into the buffer in bytes.
    #[serde(default)]
    pub byte_offset: u32,

    /// The length of the bufferView in bytes.
    pub byte_length: u32,

    /// The stride, in bytes, between vertex attributes.  When this is not defined, data is tightly packed. When two or more accessors use the same buffer view, this field MUST be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<u32>,

    /// The hint representing the intended GPU buffer type to use with this buffer view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<BufferViewTarget>,
}

impl BufferView {
    pub fn new() -> Self {
        Default::default()
    }
}
