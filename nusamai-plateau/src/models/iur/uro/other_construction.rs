//! uro:OtherConstruction (PLATEAU, CityGML 2.x)
//! con:OtherConstruction (CityGML 3.x)

use citygml::{CityGMLElement, GeometryRef};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct OtherConstruction {
    #[citygml(geom = b"uro")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

// TODO: Building と類似の構造を持つ
