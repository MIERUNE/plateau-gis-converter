use serde::{Deserialize, Serialize};

use crate::models::packet::Packet;

#[derive(Serialize, Deserialize)]
pub struct Czml {
    pub packet: Vec<Packet>,
}
