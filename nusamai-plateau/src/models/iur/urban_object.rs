use citygml::{CityGMLElement, Code};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct FacilityTypeAttribute{
    #[citygml(path = b"uro:class")]
    pub class: Option<Code>,

    #[citygml(path = b"uro:function")]
    pub function: Vec<Code>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct FacilityIdAttribute{
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