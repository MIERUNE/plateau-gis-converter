//! EXT_structural_metadata
//!
//! https://github.com/CesiumGS/glTF/tree/3d-tiles-next/extensions/2.0/Vendor/EXT_structural_metadata

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// EXT_structural_metadata glTF Mesh Primitive extension
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct ExtStructuralMetadata {
    /// An array of indexes of property textures in the root `EXT_structural_metadata` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_textures: Option<Vec<u32>>,

    /// An array of indexes of property attributes in the root `EXT_structural_metadata` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_attributes: Option<Vec<u32>>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}
