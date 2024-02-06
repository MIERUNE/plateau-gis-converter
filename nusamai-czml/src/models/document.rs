use serde::{Deserialize, Serialize};

use crate::models::packet::Packet;

#[derive(Serialize, Deserialize)]
pub struct Document {
    // In Cesium, it is necessary to always place a Packet like "{"id": "document", "version": "1.0"}" at the beginning.
    pub packets: Vec<Packet>,
}
