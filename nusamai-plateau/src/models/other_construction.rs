//! uro:OtherConstruction (PLATEAU, CityGML 2.x)
//! con:OtherConstruction (CityGML 3.x)

use citygml::{CityGMLElement, GeometryReference};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct OtherConstruction {
    #[citygml(auto_geom = b"uro")]
    pub geometries: GeometryReference,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
