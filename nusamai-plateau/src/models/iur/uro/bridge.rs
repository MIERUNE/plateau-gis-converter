use nusamai_citygml::{citygml_data, citygml_property, CityGMLElement, Code, Measure, Length, Date, GYear};

#[citygml_data(name = "uro:BridgeStructureAttribute")]
pub struct BridgeStructureAttribute {
    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:bridgeType")]
    pub bridge_type: Option<Code>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:weightRestriction")]
    pub weight_restriction: Option<Measure>,

    #[citygml(path = b"uro:heightRestriction")]
    pub height_restriction: Option<Length>,

    #[citygml(path = b"uro:widthRestriction")]
    pub width_restriction: Option<Length>,

    #[citygml(path = b"uro:underGirderHeight")]
    pub under_girder_height: Option<Length>,

    #[citygml(path = b"uro:slopeType")]
    pub slope_type: Option<Code>,

    #[citygml(path = b"uro:escalator")]
    pub escalator: Option<bool>,

}

#[citygml_data(name = "uro:BridgeFunctionalAttribute")]
pub struct BridgeFunctionalAttribute {
    #[citygml(path = b"uro:directionType")]
    pub direction_type: Option<Code>,

    #[citygml(path = b"uro:userType")]
    pub user_type: Option<Code>,

}

#[citygml_data(name = "uro:ConstructionBaseAttribute")]
pub struct ConstructionBaseAttribute {
    #[citygml(path = b"uro:adminType")]
    pub admin_type: Option<Code>,

    #[citygml(path = b"uro:administorator")]
    pub administorator: Option<String>,

    #[citygml(path = b"uro:adminOffice")]
    pub admin_office: Option<String>,

    #[citygml(path = b"uro:operatorType")]
    pub operator_type: Option<Code>,

    #[citygml(path = b"uro:installerType")]
    pub installer_type: Option<Code>,

    #[citygml(path = b"uro:installer")]
    pub installer: Option<String>,

    #[citygml(path = b"uro:structureOrdinance")]
    pub structure_ordinance: Option<String>,

    #[citygml(path = b"uro:specification")]
    pub specification: Option<String>,

    #[citygml(path = b"uro:kana")]
    pub kana: Option<String>,

    #[citygml(path = b"uro:constructionStartYear")]
    pub construction_start_year: Option<GYear>,

    #[citygml(path = b"uro:completionYear")]
    pub completion_year: Option<GYear>,

    #[citygml(path = b"uro:facilityAge")]
    pub facility_age: Option<i64>,

    #[citygml(path = b"uro:update")]
    pub update: Option<Date>,

    #[citygml(path = b"uro:purpose")]
    pub purpose: Option<Code>,

}

#[citygml_data(name = "uro:ConstructionDataQualityAttribute")]
pub struct ConstructionDataQualityAttribute {
    #[citygml(path = b"uro:srcScale")]
    pub src_scale: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc")]
    pub geometry_src_desc: Vec<Code>,

    #[citygml(path = b"uro:thematicSrcDesc")]
    pub thematic_src_desc: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDesc")]
    pub appearance_src_desc: Vec<Code>,

    #[citygml(path = b"uro:dataAcquisition")]
    pub data_acquisition: Option<String>,

    #[citygml(path = b"uro:photoScale")]
    pub photo_scale: Option<i64>,

    #[citygml(path = b"uro:lod1HeightType")]
    pub lod1_height_type: Option<Code>,

    #[citygml(path = b"uro:lodType")]
    pub lod_type: Vec<Code>,

}

#[citygml_property(name = "uro:DisasterRiskAttributeProperty")]
pub enum DisasterRiskAttributeProperty {
    #[citygml(path = b"uro:HighTideRiskAttribute")]
    HighTideRiskAttribute(HighTideRiskAttribute),
    #[citygml(path = b"uro:InlandFloodingRiskAttribute")]
    InlandFloodingRiskAttribute(InlandFloodingRiskAttribute),
    #[citygml(path = b"uro:LandSlideRiskAttribute")]
    LandSlideRiskAttribute(LandSlideRiskAttribute),
    #[citygml(path = b"uro:RiverFloodingRiskAttribute")]
    RiverFloodingRiskAttribute(RiverFloodingRiskAttribute),
    #[citygml(path = b"uro:TsunamiRiskAttribute")]
    TsunamiRiskAttribute(TsunamiRiskAttribute),
}

#[citygml_data(name = "uro:HighTideRiskAttribute")]
pub struct HighTideRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

}

#[citygml_data(name = "uro:InlandFloodingRiskAttribute")]
pub struct InlandFloodingRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

}

#[citygml_data(name = "uro:LandSlideRiskAttribute")]
pub struct LandSlideRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:areaType", required)]
    pub area_type: Option<Code>,

}

#[citygml_data(name = "uro:RiverFloodingRiskAttribute")]
pub struct RiverFloodingRiskAttribute {
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

#[citygml_data(name = "uro:TsunamiRiskAttribute")]
pub struct TsunamiRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

}

#[citygml_data(name = "uro:ConstructionRiskAssessmentAttribute")]
pub struct ConstructionRiskAssessmentAttribute {
    #[citygml(path = b"uro:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"uro:riskType", required)]
    pub risk_type: Option<Code>,

    #[citygml(path = b"uro:status")]
    pub status: Option<Code>,

    #[citygml(path = b"uro:referenceDate", required)]
    pub reference_date: Option<Date>,

}
