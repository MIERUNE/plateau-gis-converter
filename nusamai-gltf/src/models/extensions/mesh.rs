use crate::models::texture_info::TextureInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// glTFProperty (Placeholder, update with actual structure)
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GlTFProperty {
    // Structure fields go here
}

/// Feature ID Attribute in EXT_mesh_features
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FeatureIdAttribute {
    /// An integer value used to construct a string in the format `_FEATURE_ID_<set index>`
    #[serde(rename = "_feature_id")]
    pub set_index: u32,
}

impl FeatureIdAttribute {
    /// Converts the feature ID to the corresponding string format.
    pub fn to_string_format(&self) -> String {
        format!("_FEATURE_ID_{}", self.set_index)
    }
}

/// Feature ID in EXT_mesh_features
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FeatureId {
    /// The number of unique features in the attribute or texture.
    pub feature_count: u32,

    /// A value that indicates that no feature is associated with this vertex or texel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_feature_id: Option<u32>,

    /// A label assigned to this feature ID set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// An attribute containing feature IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<FeatureIdAttribute>,

    /// A texture containing feature IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<FeatureIdTexture>,

    /// The index of the property table containing per-feature property values.
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
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FeatureIdTexture {
    /// Texture channels containing feature IDs.
    #[serde(default = "default_channels")]
    pub channels: Vec<u32>,

    /// Refer to textureInfo.schema.json
    #[serde(flatten)]
    pub texture_info: TextureInfo,

    /// Additional properties (details not provided in the schema)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

fn default_channels() -> Vec<u32> {
    vec![0]
}

/// EXT_mesh_features glTF Mesh Primitive extension
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Primitive {
    #[serde(rename = "EXT_mesh_features")]
    pub ext_mesh_features: ExtMeshFeatures,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtMeshFeatures {
    /// An array of feature ID sets.
    pub feature_ids: Vec<FeatureId>,

    /// Refer to glTFProperty.schema.json
    #[serde(flatten)]
    pub gltf_property: GlTFProperty,

    /// Additional properties (details not provided in the schema)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}
