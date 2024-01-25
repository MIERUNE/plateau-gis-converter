use serde::{Deserialize, Serialize};

use crate::{CzmlString, Model};

#[derive(Serialize, Deserialize)]
pub struct Packet {
    id: Option<String>,
    #[serde(rename = "delete")]
    delete_flag: Option<bool>,
    name: Option<String>,
    parent: Option<String>,
    description: Option<CzmlString>,
    version: Option<String>,
    model: Option<Model>,
}
