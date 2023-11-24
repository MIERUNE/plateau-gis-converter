use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct Building {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:class")]
    pub class: Option<String>,

    #[citygml(path = b"bldg:measuredHeight")]
    pub measured_height: Option<f64>,

    #[citygml(path = b"uro:buildingIDAttribute/uro:BuildingIDAttribute")]
    pub building_id_attribute: Option<BuildingIDAttribute>,

    #[citygml(path = b"uro:buildingDetailAttribute/uro:BuildingDetailAttribute")]
    pub building_detail_attribute: Option<BuildingDetailAttribute>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurface>,
}

#[derive(Default, Debug, CityGMLElement)]
pub enum BoundingSurface {
    #[default]
    Unknown,
    #[citygml(path = b"bldg:WallSurface")]
    WallSurface(WallSurface),
    #[citygml(path = b"bldg:RoofSurface")]
    RoofSurface(RoofSurface),
    #[citygml(path = b"bldg:GroundSurface")]
    GroundSurface(GroundSurface),
    #[citygml(path = b"bldg:OuterCeilingSurface")]
    OuterCeilingSurface(OuterCeilingSurface),
    #[citygml(path = b"bldg:OuterFloorSurface")]
    OuterFloorSurface(OuterFloorSurface),
    #[citygml(path = b"bldg:ClosureSurface")]
    ClosureSurface(ClosureSurface),
}

#[derive(Default, Debug, CityGMLElement)]
pub struct WallSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement)]
pub struct RoofSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement)]
pub struct GroundSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement)]
pub struct OuterCeilingSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement)]
pub struct OuterFloorSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement)]
pub struct ClosureSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingIDAttribute {
    #[citygml(path = b"uro:buildingID")]
    pub building_id: Option<String>,

    #[citygml(path = b"uro:city")]
    pub city: Option<String>,

    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Option<String>,
}

#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingDetailAttribute {
    #[citygml(path = b"uro:surveyYear")]
    pub survey_year: Option<String>,
}
