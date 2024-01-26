use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{CzmlString, Model};

// todo: Modify HashMap
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Packet {
    pub id: Option<String>,
    #[serde(rename = "delete")]
    pub delete: Option<bool>,
    pub name: Option<String>,
    pub parent: Option<String>,
    pub description: Option<CzmlString>,
    pub clock: Option<HashMap<String, Value>>,
    pub version: Option<String>,
    pub availability: Option<HashMap<String, Value>>,
    pub properties: Option<HashMap<String, String>>,
    pub position: Option<HashMap<String, String>>,
    pub orientation: Option<HashMap<String, String>>,
    pub view_from: Option<HashMap<String, String>>,
    pub billboard: Option<HashMap<String, String>>,
    #[serde(rename = "box")]
    pub box_: Option<HashMap<String, String>>,
    pub corridor: Option<HashMap<String, String>>,
    pub cylinder: Option<HashMap<String, String>>,
    pub ellipse: Option<HashMap<String, String>>,
    pub ellipsoid: Option<HashMap<String, String>>,
    pub label: Option<HashMap<String, String>>,
    pub model: Option<Model>,
    // TODO: add more properties
    pub path: Option<HashMap<String, String>>,
    pub point: Option<HashMap<String, String>>,
    pub polygon: Option<HashMap<String, String>>,
    pub polyline: Option<HashMap<String, String>>,
    pub polyline_volume: Option<HashMap<String, String>>,
    pub rectangle: Option<HashMap<String, String>>,
    pub tileset: Option<HashMap<String, String>>,
    pub wall: Option<HashMap<String, String>>,
    pub agi_conic_sensor: Option<HashMap<String, String>>,
    pub agi_custom_pattern_sensor: Option<HashMap<String, String>>,
    pub agi_rectangular_sensor: Option<HashMap<String, String>>,
    pub agi_fan: Option<HashMap<String, String>>,
    pub agi_vector: Option<HashMap<String, String>>,
}
