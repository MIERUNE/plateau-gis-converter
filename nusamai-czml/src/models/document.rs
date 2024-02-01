use serde::{Deserialize, Serialize};

use crate::models::packet::Packet;

#[derive(Serialize, Deserialize)]
pub struct Document {
    #[serde(flatten)]
    pub packets: Vec<Packet>,
}
