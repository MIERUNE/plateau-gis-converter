use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Tunnel {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tun:class")]
    pub class: Option<String>,
}
