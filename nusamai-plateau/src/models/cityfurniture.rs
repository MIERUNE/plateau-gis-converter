use citygml::{CityGMLElement, Code, GeometryRef};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct CityFurniture {
    #[citygml(geom = b"frn")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,
}
