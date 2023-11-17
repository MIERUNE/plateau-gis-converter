use serde::{Deserialize, Serialize};

/// The root nodes of a scene.
#[derive(Serialize, Deserialize, Debug)]
#[serde[rename_all = "camelCase"]]
#[serde(deny_unknown_fields)]
pub struct Scene {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The indices of each root node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<u32>>,
}
