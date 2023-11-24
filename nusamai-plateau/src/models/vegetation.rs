use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct SolitaryVegetationObject {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"veg:class")]
    pub class: Option<String>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct PlantCover {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"veg:class")]
    pub class: Option<String>,
}
