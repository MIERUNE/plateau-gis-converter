use super::iur::uro;
use nusamai_citygml::{citygml_feature, citygml_property, CityGMLElement, LODType};

#[citygml_feature(name = "dem:ReliefFeature")]
pub struct ReliefFeature {
    #[citygml(path = b"dem:lod", required)]
    pub lod: Option<LODType>,

    #[citygml(path = b"dem:reliefComponent", required)]
    pub relief_component: Vec<ReliefComponentProperty>, // -> dem:_ReliefComponent
}

#[citygml_property(name = "dem:_ReliefComponentProperty")]
pub enum ReliefComponentProperty {
    #[citygml(path = b"dem:BreaklineRelief")]
    BreaklineRelief(BreaklineRelief),
    #[citygml(path = b"dem:MassPointRelief")]
    MassPointRelief(MassPointRelief),
    #[citygml(path = b"dem:RasterRelief")]
    RasterRelief(RasterRelief),
    #[citygml(path = b"dem:TINRelief")]
    TINRelief(TINRelief),
}

#[citygml_feature(name = "dem:BreaklineRelief")]
pub struct BreaklineRelief {
    #[citygml(path = b"dem:lod", required)]
    pub lod: Option<LODType>,

    /* TODO:
    #[citygml(path = b"dem:extent/gml:Polygon")]
    pub extent: Option<Polygon>,
    */
    #[citygml(path = b"uro:demDmAttribute")]
    pub dem_dm_attribute: Vec<uro::DmAttributeProperty>,
    /* TODO:
    #[citygml(path = b"dem:ridgeOrValleyLines/gml:MultiCurve")]
    pub ridge_or_valley_lines: Option<MultiCurve>,

    #[citygml(path = b"dem:breaklines/gml:MultiCurve")]
    pub breaklines: Option<MultiCurve>,
    */
}

#[citygml_feature(name = "dem:MassPointRelief")]
pub struct MassPointRelief {
    #[citygml(path = b"dem:lod", required)]
    pub lod: Option<LODType>,

    // #[citygml(path = b"dem:extent/gml:Polygon")]
    // pub extent: Option<Polygon>,
    //
    #[citygml(path = b"uro:demDmAttribute")]
    pub dem_dm_attribute: Vec<uro::DmAttributeProperty>,
    /* TODO:
    #[citygml(path = b"dem:reliefPoints/gml:MultiPoint", required)]
    pub relief_points: Option<MultiPoint>,
    */
}

#[citygml_feature(name = "dem:RasterRelief")]
pub struct RasterRelief {
    #[citygml(path = b"dem:lod", required)]
    pub lod: Option<LODType>,

    /* TODO:
    #[citygml(path = b"dem:extent/gml:Polygon")]
    pub extent: Option<Polygon>,
    */
    #[citygml(path = b"uro:demDmAttribute")]
    pub dem_dm_attribute: Vec<uro::DmAttributeProperty>,
    /* TODO:
    #[citygml(path = b"dem:grid/gml:RectifiedGridCoverage", required)]
    pub grid: Option<RectifiedGridCoverage>,
    */
}

#[citygml_feature(name = "dem:TINRelief")]
pub struct TINRelief {
    #[citygml(path = b"dem:lod", required)]
    pub lod: Option<LODType>,

    /* TODO:
    #[citygml(path = b"dem:extent/gml:Polygon")]
    pub extent: Option<Polygon>,
    */
    #[citygml(path = b"uro:demDmAttribute")]
    pub dem_dm_attribute: Vec<uro::DmAttributeProperty>,
    /* TODO:
    #[citygml(path = b"dem:tin", required)]
    pub tin: Option<tinProperty>, // -> gml:TriangulatedSurface
    */
}
