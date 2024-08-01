use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub enum MimeType {
    #[default]
    #[serde(rename = "image/jpeg")]
    ImageJpeg,
    #[serde(rename = "image/png")]
    ImagePng,
    #[serde(rename = "image/webp")]
    ImageWebp,
}

/// Image data used to create a texture. Image MAY be referenced by an URI (or IRI) or a buffer view index.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde[rename_all = "camelCase"]]
#[serde(deny_unknown_fields)]
pub struct Image {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The URI (or IRI) of the image.  Relative paths are relative to the current glTF asset.  Instead of referencing an external file, this field MAY contain a `data:`-URI. This field MUST NOT be defined when `buffer_view` is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// The image's media type. This field MUST be defined when `buffer_view` is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<MimeType>,

    /// The index of the bufferView that contains the image. This field MUST NOT be defined when `uri` is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<u32>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<ImageExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImageExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}
