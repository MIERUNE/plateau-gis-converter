use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// glTFProperty (Placeholder, update with actual structure)
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GlTFProperty {
    // Structure fields go here
}

/// Class (Placeholder, update with actual structure)
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Class {
    // Structure fields go here
}

/// Enum (Placeholder, update with actual structure)
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Enum {
    // Structure fields go here
}

/// Schema in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    /// Unique identifier for the schema.
    pub id: String,

    /// The name of the schema, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Application-specific version of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// A dictionary, where each key is a class ID and each value is an object defining the class.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub classes: HashMap<String, Class>,

    /// A dictionary, where each key is an enum ID and each value is an object defining the values for the enum.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub enums: HashMap<String, Enum>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}
