use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Packet {
    id: Option<String>,
    #[serde(rename = "delete")]
    delete_flag: Option<bool>,
    name: Option<String>,
    parent: Option<String>,
    description: Option<String>, // String.json
    version: Option<String>,
    tileset: Option<String>, // Tileset.json
    model: Option<String>,   // Model.json
}
