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
    pub delete: Option<bool>,
    pub name: Option<String>,
    pub parent: Option<String>,
    pub description: Option<CzmlString>,
    pub clock: Option<HashMap<String, Value>>,
    pub version: Option<String>,
    pub availability: Option<Value>,
    pub properties: Option<HashMap<String, Value>>,
    pub position: Option<HashMap<String, Value>>,
    pub orientation: Option<HashMap<String, Value>>,
    pub view_from: Option<HashMap<String, Value>>,
    pub billboard: Option<HashMap<String, Value>>,
    #[serde(rename = "box")]
    pub box_: Option<HashMap<String, Value>>,
    pub corridor: Option<HashMap<String, Value>>,
    pub cylinder: Option<HashMap<String, Value>>,
    pub ellipse: Option<HashMap<String, Value>>,
    pub ellipsoid: Option<HashMap<String, Value>>,
    pub label: Option<HashMap<String, Value>>,
    pub model: Option<Model>,
    pub path: Option<HashMap<String, Value>>,
    pub point: Option<HashMap<String, Value>>,
    pub polygon: Option<HashMap<String, Value>>,
    pub polyline: Option<HashMap<String, Value>>,
    pub polyline_volume: Option<HashMap<String, Value>>,
    pub rectangle: Option<HashMap<String, Value>>,
    pub tileset: Option<HashMap<String, Value>>,
    pub wall: Option<HashMap<String, Value>>,
    #[serde(rename = "agi_conicSensor")]
    pub agi_conic_sensor: Option<HashMap<String, Value>>,
    #[serde(rename = "agi_customPatternSensor")]
    pub agi_custom_pattern_sensor: Option<HashMap<String, Value>>,
    #[serde(rename = "agi_rectangularSensor")]
    pub agi_rectangular_sensor: Option<HashMap<String, Value>>,
    pub agi_fan: Option<HashMap<String, Value>>,
    pub agi_vector: Option<HashMap<String, Value>>,
}
