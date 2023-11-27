use citygml::{CityGMLElement, GeometryRef};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct ReliefFeature {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"gml:name")]
    name: Option<String>,

    #[citygml(path = b"dem:lod")]
    lod: Option<i32>,

    #[citygml(path = b"dem:reliefComponent")]
    relief_component: Vec<ReliefComponent>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub enum ReliefComponent {
    #[default]
    Unknown,
    #[citygml(path = b"dem:BreaklineRelief")]
    BreaklineRelief(BreaklineRelief),
    #[citygml(path = b"dem:MassPointRelief")]
    MassPointRelief(MassPointRelief),
    #[citygml(path = b"dem:RasterRelief")]
    RasterRelief(RasterRelief),
    #[citygml(path = b"dem:TINRelief")]
    TINRelief(TINRelief),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct TINRelief {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"gml:name")]
    name: Option<String>,

    #[citygml(auto_geom = b"dem")]
    geometries: GeometryRef,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct BreaklineRelief {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct MassPointRelief {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct RasterRelief {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
