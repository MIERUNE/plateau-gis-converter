use nusamai_citygml::{citygml_feature, citygml_property, CityGMLElement};

#[citygml_feature(name = "dem:ReliefFeature")]
pub struct ReliefFeature {
    #[citygml(path = b"dem:lod")]
    lod: Option<u64>,

    #[citygml(path = b"dem:reliefComponent")]
    relief_component: Vec<ReliefComponent>,
}

#[citygml_property(name = "dem:ReliefComponentProperty")]
pub enum ReliefComponent {
    #[citygml(path = b"dem:BreaklineRelief")]
    BreaklineRelief(BreaklineRelief),
    #[citygml(path = b"dem:MassPointRelief")]
    MassPointRelief(MassPointRelief),
    #[citygml(path = b"dem:RasterRelief")]
    RasterRelief(RasterRelief),
    #[citygml(path = b"dem:TINRelief")]
    TINRelief(TINRelief),
}

#[citygml_feature(name = "dem:TinRelief")]
pub struct TINRelief {
    // ...
}

#[citygml_feature(name = "dem:BreaklineRelief")]
pub struct BreaklineRelief {
    // ...
}

#[citygml_feature(name = "dem:MassPointRelief")]
pub struct MassPointRelief {
    // ...
}

#[citygml_feature(name = "dem:RasterRelief")]
pub struct RasterRelief {
    // ...
}
