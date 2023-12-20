use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
    pub extras: Option<HashMap<String, Value>>,
}

/// Feature ID Texture in EXT_mesh_features
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FeatureIdTexture {
    /// Texture channels containing feature IDs, identified by index. Feature IDs may be packed into multiple channels if a single channel does not have sufficient bit depth to represent all feature ID values. The values are packed in little-endian order.
    #[serde(default = "default_channels")]
    pub channels: Vec<u32>,

    /// The index of the texture.
    pub index: u32,

    /// This integer value is used to construct a string in the format `TEXCOORD_<set index>` which is a reference to a key in `mesh.primitives.attributes` (e.g. a value of `0` corresponds to `TEXCOORD_0`). A mesh primitive **MUST** have the corresponding texture coordinate attributes for the material to be applicable to it.
    #[serde(default)]
    pub tex_coord: u32,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<HashMap<String, Value>>,
}

fn default_channels() -> Vec<u32> {
    vec![0]
}

/// EXT_mesh_features glTF Mesh Primitive extension
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Primitive {
    #[serde(skip_serializing_if = "Option::is_none", rename = "EXT_mesh_features")]
    pub ext_mesh_features: Option<ExtMeshFeatures>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "EXT_structural_metadata"
    )]
    pub ext_structural_metadata: Option<ExtStructuralMetadata>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "KHR_materials_variants"
    )]
    pub khr_materials_variants: Option<KhrMaterialsVariants>,
}

// TODO: Implement KHR_materials_variants
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KhrMaterialsVariants {
    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
    pub extras: Option<HashMap<String, Value>>,
}

/// EXT_structural_metadata glTF Mesh Primitive extension
#[derive(Serialize, Deserialize, Debug, Default)]
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
    pub extras: Option<HashMap<String, Value>>,
}
