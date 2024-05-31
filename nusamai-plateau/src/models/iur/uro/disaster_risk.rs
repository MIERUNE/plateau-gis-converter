use nusamai_citygml::{citygml_data, citygml_property, CityGmlElement, Code, Length, Measure};

#[citygml_property(name = "uro:DisasterRiskAttributeProperty")]
pub enum DisasterRiskAttributeProperty {
    #[citygml(path = b"uro:BuildingHighTideRiskAttribute")] // for backward compatibility
    #[citygml(path = b"uro:HighTideRiskAttribute")]
    HighTideRiskAttribute(HighTideRiskAttribute),
    #[citygml(path = b"uro:BuildingInlandFloodingRiskAttribute")] // for backward compatibility
    #[citygml(path = b"uro:InlandFloodingRiskAttribute")]
    InlandFloodingRiskAttribute(InlandFloodingRiskAttribute),
    #[citygml(path = b"uro:BuildingLandSlideRiskAttribute")] // for backward compatibility
    #[citygml(path = b"uro:LandSlideRiskAttribute")]
    LandSlideRiskAttribute(LandSlideRiskAttribute),
    #[citygml(path = b"uro:ReservoirFloodingRiskAttribute")]
    ReservoirFloodingRiskAttribute(ReservoirFloodingRiskAttribute),
    #[citygml(path = b"uro:BuildingRiverFloodingRiskAttribute")] // for backward compatibility
    #[citygml(path = b"uro:RiverFloodingRiskAttribute")]
    RiverFloodingRiskAttribute(RiverFloodingRiskAttribute),
    #[citygml(path = b"uro:BuildingTsunamiRiskAttribute")] // for backward compatibility
    #[citygml(path = b"uro:TsunamiRiskAttribute")]
    TsunamiRiskAttribute(TsunamiRiskAttribute),
}

#[citygml_property(name = "uro:FloodingRiskAttributeProperty")]
pub enum FloodingRiskAttributeProperty {
    #[citygml(path = b"uro:WaterBodyHighTideRiskAttribute")]
    #[citygml(path = b"uro:HighTideRiskAttribute")]
    HighTideRiskAttribute(HighTideRiskAttribute),

    #[citygml(path = b"uro:WaterBodyInlandFloodingRiskAttribute")]
    #[citygml(path = b"uro:InlandFloodingRiskAttribute")]
    InlandFloodingRiskAttribute(InlandFloodingRiskAttribute),

    #[citygml(path = b"uro:ReservoirFloodingRiskAttribute")]
    ReservoirFloodingRiskAttribute(ReservoirFloodingRiskAttribute),

    #[citygml(path = b"uro:WaterBodyRiverFloodingRiskAttribute")]
    #[citygml(path = b"uro:RiverFloodingRiskAttribute")]
    RiverFloodingRiskAttribute(RiverFloodingRiskAttribute),

    #[citygml(path = b"uro:WaterBodyTsunamiRiskAttribute")]
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

#[citygml_data(name = "uro:ReservoirFloodingRiskAttribute")]
pub struct ReservoirFloodingRiskAttribute {
    #[citygml(path = b"uro:description", required)]
    pub description: Option<Code>,

    #[citygml(path = b"uro:rank")]
    pub rank: Option<Code>,

    #[citygml(path = b"uro:rankOrg")]
    pub rank_org: Option<Code>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,
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
