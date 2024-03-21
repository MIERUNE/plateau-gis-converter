use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Metadata about the glTF asset.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    /// Tool that generated this glTF model.  Useful for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,

    /// The glTF version in the form of `<major>.<minor>` that this asset targets.
    pub version: String,

    /// The minimum glTF version in the form of `<major>.<minor>` that this asset targets. This property **MUST NOT** be greater than the asset version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<AssetExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssetExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: Default::default(),
            generator: Some("nusamai-gltf".to_string()),
            version: "2.0".into(),
            min_version: Default::default(),
            extensions: Default::default(),
            extras: Default::default(),
        }
    }
}
