use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Bridge {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"brid:class")]
    pub class: Option<String>,
}
