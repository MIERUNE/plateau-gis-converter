use nusamai_citygml::{citygml_data, CityGmlElement, Code, GYear, Measure};

#[citygml_data(name = "uro:LandUseDetailAttribute")]
pub struct LandUseDetailAttribute {
    #[citygml(path = b"uro:id")]
    pub id: Option<String>,

    #[citygml(path = b"uro:orgLandUse")]
    pub org_land_use: Option<Code>,

    #[citygml(path = b"uro:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"uro:ownerType")]
    pub owner_type: Option<Code>,

    #[citygml(path = b"uro:owner")]
    pub owner: Option<String>,

    #[citygml(path = b"uro:areaInSquareMeter")]
    pub area_in_square_meter: Option<Measure>,

    #[citygml(path = b"uro:areaInHa")]
    pub area_in_ha: Option<Measure>,

    #[citygml(path = b"uro:buildingCoverageRate")]
    pub building_coverage_rate: Option<f64>,

    #[citygml(path = b"uro:floorAreaRate")]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"uro:specifiedBuildingCoverageRate")]
    pub specified_building_coverage_rate: Option<f64>,

    #[citygml(path = b"uro:specifiedFloorAreaRate")]
    pub specified_floor_area_rate: Option<f64>,

    #[citygml(path = b"uro:standardFloorAreaRate")]
    pub standard_floor_area_rate: Option<f64>,

    #[citygml(path = b"uro:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"uro:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"uro:districtsAndZonesType")]
    pub districts_and_zones_type: Vec<Code>,

    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"uro:city")]
    pub city: Option<Code>,

    #[citygml(path = b"uro:reference")]
    pub reference: Option<String>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,

    #[citygml(path = b"uro:surveyYear")]
    pub survey_year: Option<GYear>,
}
