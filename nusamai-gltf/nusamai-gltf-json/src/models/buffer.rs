use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u16)]
pub enum BufferViewTarget {
    #[default]
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

/// A buffer points to binary geometry, animation, or skins.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
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
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BufferExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BufferView {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The index of the buffer.
    pub buffer: u32,

    /// The offset into the buffer in bytes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub byte_offset: u32,

    /// The length of the bufferView in bytes.
    pub byte_length: u32,

    /// The stride, in bytes, between vertex attributes.  When this is not defined, data is tightly packed. When two or more accessors use the same buffer view, this field MUST be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<u8>,

    /// The hint representing the intended GPU buffer type to use with this buffer view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<BufferViewTarget>,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let view: BufferView = serde_json::from_str(r#"{"buffer":2,"byteLength":100}"#).unwrap();
        assert_eq!(view.buffer, 2);
        assert_eq!(view.byte_length, 100);
        assert_eq!(view.byte_offset, 0);
    }
}
