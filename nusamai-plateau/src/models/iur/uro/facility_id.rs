use citygml::{CityGMLElement, Code, Measure};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub enum FacilityIdAttributeProperty {
    #[default]
    Unknown,

    #[citygml(path = b"uro:FacilityIdAttribute")]
    FacilityIdAttribute(FacilityIdAttribute),

    #[citygml(path = b"uro:RiverFacilityIdAttribute")]
    RiverFacilityIdAttribute(RiverFacilityIdAttribute),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct FacilityIdAttribute {
    #[citygml(path = b"uro:id")]
    pub id: Option<String>,

    #[citygml(path = b"uro:partId")]
    pub part_id: Option<String>,

    #[citygml(path = b"uro:branchId")]
    pub branch_id: Option<String>,

    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Vec<Code>,

    #[citygml(path = b"uro:city")]
    pub city: Vec<Code>,

    #[citygml(path = b"uro:route")]
    pub route: Option<String>,

    #[citygml(path = b"uro:startPost")]
    pub start_post: Option<String>,

    #[citygml(path = b"uro:endPost")]
    pub end_post: Option<String>,

    #[citygml(path = b"uro:startLat")]
    pub start_lat: Option<f64>,

    #[citygml(path = b"uro:startLong")]
    pub start_long: Option<f64>,

    #[citygml(path = b"uro:alternativeName")]
    pub alternative_name: Vec<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct RiverFacilityIdAttribute {
    #[citygml(path = b"uro:id")]
    pub id: Option<String>,

    #[citygml(path = b"uro:partId")]
    pub part_id: Option<String>,

    #[citygml(path = b"uro:branchId")]
    pub branch_id: Option<String>,

    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Vec<Code>,

    #[citygml(path = b"uro:city")]
    pub city: Vec<Code>,

    #[citygml(path = b"uro:route")]
    pub route: Option<String>,

    #[citygml(path = b"uro:startPost")]
    pub start_post: Option<String>,

    #[citygml(path = b"uro:endPost")]
    pub end_post: Option<String>,

    #[citygml(path = b"uro:startLat")]
    pub start_lat: Option<f64>,

    #[citygml(path = b"uro:startLong")]
    pub start_long: Option<f64>,

    #[citygml(path = b"uro:alternativeName")]
    pub alternative_name: Vec<String>,

    #[citygml(path = b"uro:riverCode")]
    pub river_code: Option<Code>,

    #[citygml(path = b"uro:riverName")]
    pub river_name: Option<String>,

    #[citygml(path = b"uro:sideType")]
    pub side_type: Option<Code>,

    #[citygml(path = b"uro:leftPost")]
    pub left_post: Option<Measure>,

    #[citygml(path = b"uro:leftDistance")]
    pub left_distance: Option<Measure>,

    #[citygml(path = b"uro:rightPost")]
    pub right_post: Option<Measure>,

    #[citygml(path = b"uro:rightDistance")]
    pub right_distance: Option<Measure>,

    #[citygml(path = b"uro:leftStartPost")]
    pub left_start_post: Option<Measure>,

    #[citygml(path = b"uro:leftStartDistance")]
    pub left_start_distance: Option<Measure>,

    #[citygml(path = b"uro:rightStartPost")]
    pub right_start_post: Option<Measure>,

    #[citygml(path = b"uro:rightStartDistance")]
    pub right_start_distance: Option<Measure>,

    #[citygml(path = b"uro:leftEndPost")]
    pub left_end_post: Option<Measure>,

    #[citygml(path = b"uro:leftEndDistance")]
    pub left_end_distance: Option<Measure>,

    #[citygml(path = b"uro:rightEndPost")]
    pub right_end_post: Option<Measure>,

    #[citygml(path = b"uro:rightEndDistance")]
    pub right_end_distance: Option<Measure>,
}
