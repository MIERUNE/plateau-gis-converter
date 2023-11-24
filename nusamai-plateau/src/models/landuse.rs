use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct LandUse {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
