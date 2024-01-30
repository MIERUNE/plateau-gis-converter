//! EXT_mesh_features
//!
//! https://github.com/CesiumGS/glTF/tree/3d-tiles-next/extensions/2.0/Vendor/EXT_mesh_features

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// EXT_mesh_features glTF Mesh Primitive extension
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExtMeshFeatures {
    /// An array of feature ID sets.
    pub feature_ids: Vec<FeatureId>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// Feature ID in EXT_mesh_features
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FeatureId {
    /// The number of unique features in the attribute or texture.
    pub feature_count: u32,

    /// A value that indicates that no feature is associated with this vertex or texel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_feature_id: Option<u32>,

    /// A label assigned to this feature ID set. Labels must be alphanumeric identifiers matching the regular expression `^[a-zA-Z_][a-zA-Z0-9_]*$`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// An attribute containing feature IDs. When `attribute` and `texture` are omitted the feature IDs are assigned to vertices by their index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<u32>,

    /// A texture containing feature IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<FeatureIdTexture>,

    /// The index of the property table containing per-feature property values. Only applicable when using the `EXT_structural_metadata` extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_table: Option<u32>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// Feature ID Texture in EXT_mesh_features
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FeatureIdTexture {
    /// Texture channels containing feature IDs, identified by index. Feature IDs may be packed into multiple channels if a single channel does not have sufficient bit depth to represent all feature ID values. The values are packed in little-endian order.
    #[serde(
        default = "default_channels",
        skip_serializing_if = "is_default_channels"
    )]
    pub channels: Vec<u32>,

    /// The index of the texture.
    pub index: u32,

    /// This integer value is used to construct a string in the format `TEXCOORD_<set index>` which is a reference to a key in `mesh.primitives.attributes` (e.g. a value of `0` corresponds to `TEXCOORD_0`). A mesh primitive **MUST** have the corresponding texture coordinate attributes for the material to be applicable to it.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tex_coord: u32,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

fn default_channels() -> Vec<u32> {
    vec![0]
}

fn is_default_channels(v: &Vec<u32>) -> bool {
    *v == vec![0]
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
