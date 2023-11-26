use citygml::{CityGMLElement, GeometryReference};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct GenericCityObject {
    #[citygml(auto_geom = b"gen")]
    pub geometries: GeometryReference,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
