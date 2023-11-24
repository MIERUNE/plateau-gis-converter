use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct ReliefFeature {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"gml:name")]
    name: Option<String>,

    #[citygml(path = b"dem:lod")]
    lod: Option<i32>,
}
