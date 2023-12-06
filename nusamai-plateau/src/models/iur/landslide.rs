use citygml::{CityGMLElement, GeometryRef};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct SedimentDisasterProneArea {
    #[citygml(geom = b"urf")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
