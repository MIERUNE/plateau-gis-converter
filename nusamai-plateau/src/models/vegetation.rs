use citygml::{CityGMLElement, GeometryReference};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct SolitaryVegetationObject {
    #[citygml(auto_geom = b"veg")]
    pub geometries: GeometryReference,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"veg:class")]
    pub class: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct PlantCover {
    #[citygml(auto_geom = b"veg")]
    pub geometries: GeometryReference,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"veg:class")]
    pub class: Option<String>,
}
