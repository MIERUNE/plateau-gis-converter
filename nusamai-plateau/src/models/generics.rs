use citygml::{CityGMLElement, GeometryRef};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct GenericCityObject {
    #[citygml(auto_geom = b"gen")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
