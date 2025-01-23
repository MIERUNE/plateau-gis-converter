use nusamai_citygml::{
    citygml_data, citygml_feature, citygml_property, CityGmlElement, LODType, Point, Vector,
};

use super::iur::uro;

#[citygml_feature(name = "dem:ReliefFeature")]
pub struct ReliefFeature {
    #[citygml(path = b"dem:lod", required)]
    pub lod: Option<LODType>,

    #[citygml(path = b"dem:reliefComponent", required)]
    pub relief_component: Vec<ReliefComponentProperty>, // -> dem:_ReliefComponent

    #[citygml(path = b"uro:demDataQualityAttribute", required)]
    pub dem_data_quality_attribute: Option<uro::DemDataQualityAttribute>,
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

    #[citygml(path = b"dem:grid/gml:RectifiedGridCoverage", required)]
    pub grid: Option<RectifiedGridCoverage>,
}

#[citygml_data(name = "gml:RectifiedGridCoverage")]
pub struct RectifiedGridCoverage {
    #[citygml(path = b"@gml:id", required)]
    pub id: Option<String>,

    #[citygml(path = b"gml:rectifiedGridDomain", required)]
    pub rectified_grid_domain: Option<RectifiedGridDomain>,
    // TODO:
    // #[citygml(path = b"gml:rangeSet", required)]
    // pub range_set: Option<RangeSet>,
    #[citygml(path = b"gml:coverageFunction")]
    pub coverage_function: Option<CoverageFunction>,
}

#[citygml_data(name = "gml:RectifiedGridDomain")]
pub struct RectifiedGridDomain {
    #[citygml(path = b"gml:RectifiedGrid", required)]
    pub rectified_grid: Option<RectifiedGrid>,
}

#[citygml_data(name = "gml:CoverageFunction")]
pub struct CoverageFunction {
    #[citygml(path = b"gml:MappingRule", required)]
    pub mapping_rule: Option<String>,

    #[citygml(path = b"gml:GridFunction", required)]
    pub grid_function: Option<GridFunction>,
}

#[citygml_data(name = "gml:RectifiedGrid")]
pub struct RectifiedGrid {
    #[citygml(path = b"@gml:id", required)]
    pub id: Option<String>,

    #[citygml(path = b"gml:limits/gml:GridEnvelope", required)]
    pub limits: Option<GridEnvelope>,

    #[citygml(path = b"gml:axisName", required)]
    pub axis_name: Vec<String>,

    #[citygml(path = b"gml:origin/gml:Point", required)]
    pub origin: Option<Point>,

    #[citygml(path = b"gml:offsetVector", required)]
    pub offset_vector: Vec<Vector>,
}

#[citygml_data(name = "gml:GridEnvelope")]
pub struct GridEnvelope {
    // TODO:
    // #[citygml(path = b"gml:low", required)]
    // pub low: Vec<IntegerList>,
    // TODO:
    // #[citygml(path = b"gml:high", required)]
    // pub low: Vec<IntegerList>,
}

#[citygml_data(name = "gml:GridFunction")]
pub struct GridFunction {
    // TODO:
    // #[citygml(path = b"gml:sequenceRule", required)]
    // pub sequence_rule: Vec<SequenceRule>,
    // TODO:
    //#[citygml(path = b"gml:startPoint")]
    //pub start_point: Optional<IntegerList>,
}
