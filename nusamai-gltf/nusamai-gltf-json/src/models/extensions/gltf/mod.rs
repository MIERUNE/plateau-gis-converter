use std::collections::HashMap;

pub use cesiumtiles::models::gltf_extensions::gltf::ext_structural_metadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Gltf {
    #[serde(rename = "EXT_structural_metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_structural_metadata: Option<ext_structural_metadata::ExtStructuralMetadata>,

    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}
