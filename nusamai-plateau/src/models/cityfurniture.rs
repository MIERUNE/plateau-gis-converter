use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct CityFurniture {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"frn:class")]
    pub class: Option<String>,
}
