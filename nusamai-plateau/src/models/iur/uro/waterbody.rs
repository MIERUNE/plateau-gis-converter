use nusamai_citygml::{
    citygml_data, citygml_property, CityGmlElement, Code, GYearMonth, Length, Measure,
};

#[citygml_property(name = "uro:WaterBodyFloodingRiskAttributeProperty")]
pub enum WaterBodyFloodingRiskAttributeProperty {
    #[citygml(path = b"uro:WaterBodyHighTideRiskAttribute")]
    WaterBodyHighTideRiskAttribute(WaterBodyHighTideRiskAttribute),
    #[citygml(path = b"uro:WaterBodyInlandFloodingRiskAttribute")]
    WaterBodyInlandFloodingRiskAttribute(WaterBodyInlandFloodingRiskAttribute),
    #[citygml(path = b"uro:WaterBodyRiverFloodingRiskAttribute")]
    WaterBodyRiverFloodingRiskAttribute(WaterBodyRiverFloodingRiskAttribute),
    #[citygml(path = b"uro:WaterBodyTsunamiRiskAttribute")]
    WaterBodyTsunamiRiskAttribute(WaterBodyTsunamiRiskAttribute),
}

#[citygml_data(name = "uro:WaterBodyHighTideRiskAttribute")]
pub struct WaterBodyHighTideRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,
}

#[citygml_data(name = "uro:WaterBodyInlandFloodingRiskAttribute")]
pub struct WaterBodyInlandFloodingRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,
}

#[citygml_data(name = "uro:WaterBodyRiverFloodingRiskAttribute")]
pub struct WaterBodyRiverFloodingRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:adminType", required)]
    pub admin_type: Option<Code>,

    #[citygml(path = b"uro:scale", required)]
    pub scale: Option<Code>,

    #[citygml(path = b"uro:duration")]
    pub duration: Option<Measure>,
}

#[citygml_data(name = "uro:WaterBodyTsunamiRiskAttribute")]
pub struct WaterBodyTsunamiRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,
}

#[citygml_data(name = "uro:WaterBodyDetailAttribute")]
pub struct WaterBodyDetailAttribute {
    #[citygml(path = b"uro:kana")]
    pub kana: Option<String>,

    #[citygml(path = b"uro:waterSystemCode")]
    pub water_system_code: Option<Code>,

    #[citygml(path = b"uro:riverCode")]
    pub river_code: Option<Code>,

    #[citygml(path = b"uro:adminType")]
    pub admin_type: Option<Code>,

    #[citygml(path = b"uro:flowDirection")]
    pub flow_direction: Option<bool>,

    #[citygml(path = b"uro:maximumDepth")]
    pub maximum_depth: Option<Length>,

    #[citygml(path = b"uro:waterSurfaceElevation")]
    pub water_surface_elevation: Option<Length>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:measurementYearMonth")]
    pub measurement_year_month: Option<GYearMonth>,

    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Vec<Code>,

    #[citygml(path = b"uro:city")]
    pub city: Vec<Code>,
}
