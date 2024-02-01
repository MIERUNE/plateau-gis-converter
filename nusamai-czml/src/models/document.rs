use serde::{Deserialize, Serialize};

use crate::models::packet::Packet;

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub packets: Vec<Packet>,
}
