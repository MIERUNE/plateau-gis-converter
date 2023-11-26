use citygml::{CityGMLElement, GeometryReference};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct WaterBody {
    #[citygml(auto_geom = b"wtr")]
    pub geometries: GeometryReference,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
