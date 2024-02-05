use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The root nodes of a scene.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde[rename_all = "camelCase"]]
#[serde(deny_unknown_fields)]
pub struct Scene {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The indices of each root node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<u32>>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<SceneExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SceneExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}
