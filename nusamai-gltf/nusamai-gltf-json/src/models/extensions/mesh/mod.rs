pub use cesiumtiles::{
    gltf_extensions::mesh::ext_structural_metadata,
    models::gltf_extensions::mesh::ext_mesh_features,
};
pub mod khr_materials_variants;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct MeshPrimitive {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "EXT_mesh_features")]
    pub ext_mesh_features: Option<ext_mesh_features::ExtMeshFeatures>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "EXT_structural_metadata")]
    pub ext_structural_metadata: Option<ext_structural_metadata::ExtStructuralMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "KHR_materials_variants")]
    pub khr_materials_variants: Option<khr_materials_variants::KhrMaterialsVariants>,

    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}
