//! uro:OtherConstruction (PLATEAU, CityGML 2.x)
//! con:OtherConstruction (CityGML 3.x)

use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct OtherConstruction {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
