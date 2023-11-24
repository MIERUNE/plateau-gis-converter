//! uro:OtherConstruction (PLATEAU, CityGML 2.x)
//! con:OtherConstruction (CityGML 3.x)

use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct OtherConstruction {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
