use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{CzmlPolygon, CzmlString, Model};

// todo: Modify HashMap and Value
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Packet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CzmlString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    // todo: Implement TimeIntervalCollectionValue.json
    #[serde(
        default = "default_availability",
        skip_serializing_if = "is_default_availability"
    )]
    pub availability: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_from: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billboard: Option<HashMap<String, Value>>,

    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub corridor: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cylinder: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipse: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipsoid: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<CzmlPolygon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline_volume: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tileset: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall: Option<HashMap<String, Value>>,

    #[serde(rename = "agi_conicSensor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agi_conic_sensor: Option<HashMap<String, Value>>,

    #[serde(rename = "agi_customPatternSensor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agi_custom_pattern_sensor: Option<HashMap<String, Value>>,

    #[serde(rename = "agi_rectangularSensor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agi_rectangular_sensor: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agi_fan: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub agi_vector: Option<HashMap<String, Value>>,
}

fn default_availability() -> Value {
    Value::String("0000-00-00T00:00:00Z/9999-12-31T24:00:00Z".to_string())
}
fn is_default_availability(availability: &Value) -> bool {
    *availability == default_availability()
}

impl Default for Packet {
    fn default() -> Self {
        Self {
            id: None,
            delete: None,
            name: None,
            parent: None,
            description: None,
            clock: None,
            version: None,
            availability: default_availability(),
            properties: None,
            position: None,
            orientation: None,
            view_from: None,
            billboard: None,
            box_: None,
            corridor: None,
            cylinder: None,
            ellipse: None,
            ellipsoid: None,
            label: None,
            model: None,
            path: None,
            point: None,
            polygon: None,
            polyline: None,
            polyline_volume: None,
            rectangle: None,
            tileset: None,
            wall: None,
            agi_conic_sensor: None,
            agi_custom_pattern_sensor: None,
            agi_rectangular_sensor: None,
            agi_fan: None,
            agi_vector: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_deserialize() {
        let packet: Packet = serde_json::from_str("{}").unwrap();
        assert_eq!(
            packet.availability,
            Value::String("0000-00-00T00:00:00Z/9999-12-31T24:00:00Z".to_string())
        );
    }

    #[test]
    fn test_default_serialize() {
        let packet = Packet::default();
        let json = serde_json::to_string(&packet).unwrap();
        assert_eq!(json, r#"{}"#);
    }
}
