use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Building {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:class")]
    pub class: Option<String>,

    #[citygml(path = b"bldg:function")]
    pub function: Vec<String>,

    #[citygml(path = b"bldg:measuredHeight")]
    pub measured_height: Option<f64>,

    #[citygml(path = b"uro:buildingIDAttribute/uro:BuildingIDAttribute")]
    pub building_id_attribute: Option<BuildingIDAttribute>,

    #[citygml(path = b"uro:buildingDetailAttribute/uro:BuildingDetailAttribute")]
    pub building_detail_attribute: Option<BuildingDetailAttribute>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurface>,

    #[citygml(path = b"bldg:interiorRoom/bldg:Room")]
    pub interior_room: Vec<Room>,

    #[citygml(path = b"bldg:outerBuildingInstallation/bldg:BuildingInstallation")]
    pub outer_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorBuildingInstallation/bldg:IntBuildingInstallation")]
    pub interior_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:consistsOfBuildingPart/bldg:BuildingPart")]
    pub consists_of_building_part: Vec<BuildingPart>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct BuildingPart {
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

    #[citygml(path = b"bldg:interiorRoom/bldg:Room")]
    pub interior_room: Vec<Room>,

    #[citygml(path = b"bldg:outerBuildingInstallation/bldg:BuildingInstallation")]
    pub outer_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorBuildingInstallation/bldg:IntBuildingInstallation")]
    pub interior_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:consistsOfBuildingPart/bldg:BuildingPart")]
    pub consists_of_building_part: Vec<BuildingPart>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
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
    #[citygml(path = b"bldg:CeilingSurface")]
    CeilingSurface(CeilingSurface),
    #[citygml(path = b"bldg:FloorSurface")]
    FloorSurface(FloorSurface),
    #[citygml(path = b"bldg:InteriorWallSurface")]
    InteriorWallSurface(InteriorWallSurface),
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct WallSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct RoofSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct GroundSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct OuterCeilingSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct OuterFloorSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct ClosureSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct CeilingSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}
#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct FloorSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}
#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct InteriorWallSurface {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub enum Opening {
    #[default]
    Unknown,
    #[citygml(path = b"bldg:Window")]
    Window(Window),
    #[citygml(path = b"bldg:Door")]
    Door(Door),
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Window {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Door {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Room {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurface>,

    #[citygml(path = b"bldg:roomInstallation/bldg:IntBuildingInstallation")]
    pub room_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorFurniture/bldg:BuildingFurniture")]
    pub interior_furniture: Vec<BuildingFurniture>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct BuildingInstallation {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurface>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct BuildingFurniture {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurface>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct BuildingIDAttribute {
    #[citygml(path = b"uro:buildingID")]
    pub building_id: Option<String>,

    #[citygml(path = b"uro:city")]
    pub city: Option<String>,

    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Option<String>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct BuildingDetailAttribute {
    #[citygml(path = b"uro:surveyYear")]
    pub survey_year: Option<String>,
}
