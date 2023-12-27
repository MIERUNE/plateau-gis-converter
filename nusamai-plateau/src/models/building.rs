use nusamai_citygml::{
    citygml_data, citygml_feature, citygml_property, CityGMLElement, Code, Measure,
};

#[citygml_feature(name = "bldg:Building")]
pub struct Building {
    #[citygml(path = b"bldg:class")]
    pub class: Option<Code>,

    #[citygml(path = b"bldg:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"bldg:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"bldg:yearOfConstruction")]
    pub year_of_construction: Option<u64>,

    #[citygml(path = b"bldg:yearOfDemolition")]
    pub year_of_demolition: Option<u64>,

    #[citygml(path = b"bldg:roofType")]
    pub roof_type: Vec<Code>,

    #[citygml(path = b"bldg:measuredHeight")]
    pub measured_height: Option<Measure>,

    #[citygml(path = b"bldg:storeysAboveGround")]
    pub storeys_above_ground: Option<u64>,

    #[citygml(path = b"bldg:storeysBelowGround")]
    pub storeys_below_ground: Option<u64>,

    #[citygml(path = b"uro:buildingDisasterRiskAttribute")]
    pub building_disaster_risk_attribute: Vec<BuildingDisasterRiskAttributeProperty>,

    #[citygml(path = b"uro:buildingIDAttribute/uro:BuildingIDAttribute")]
    pub building_id_attribute: Option<BuildingIDAttribute>,

    #[citygml(path = b"uro:buildingDetailAttribute/uro:BuildingDetailAttribute")]
    pub building_detail_attribute: Option<BuildingDetailAttribute>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurfaceProperty>,

    #[citygml(path = b"bldg:interiorRoom/bldg:Room")]
    pub interior_room: Vec<Room>,

    #[citygml(path = b"bldg:outerBuildingInstallation/bldg:BuildingInstallation")]
    pub outer_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorBuildingInstallation/bldg:IntBuildingInstallation")]
    pub interior_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:consistsOfBuildingPart/bldg:BuildingPart")]
    pub consists_of_building_part: Vec<BuildingPart>,
}

#[citygml_feature(name = "bldg:BuildingPart")]
pub struct BuildingPart {
    #[citygml(path = b"bldg:class")]
    pub class: Option<Code>,

    #[citygml(path = b"bldg:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"bldg:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"bldg:yearOfConstruction")]
    pub year_of_construction: Option<u64>,

    #[citygml(path = b"bldg:yearOfDemolition")]
    pub year_of_demolition: Option<u64>,

    #[citygml(path = b"bldg:roofType")]
    pub roof_type: Vec<Code>,

    #[citygml(path = b"bldg:measuredHeight")]
    pub measured_height: Option<Measure>,

    #[citygml(path = b"bldg:storeysAboveGround")]
    pub storeys_above_ground: Option<u64>,

    #[citygml(path = b"bldg:storeysBelowGround")]
    pub storeys_below_ground: Option<u64>,

    #[citygml(path = b"uro:buildingIDAttribute/uro:BuildingIDAttribute")]
    pub building_id_attribute: Option<BuildingIDAttribute>,

    #[citygml(path = b"uro:buildingDetailAttribute/uro:BuildingDetailAttribute")]
    pub building_detail_attribute: Option<BuildingDetailAttribute>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurfaceProperty>,

    #[citygml(path = b"bldg:interiorRoom/bldg:Room")]
    pub interior_room: Vec<Room>,

    #[citygml(path = b"bldg:outerBuildingInstallation/bldg:BuildingInstallation")]
    pub outer_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorBuildingInstallation/bldg:IntBuildingInstallation")]
    pub interior_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:consistsOfBuildingPart/bldg:BuildingPart")]
    pub consists_of_building_part: Vec<BuildingPart>,
}

#[citygml_property(name = "bldg:BoundarySurfaceProperty")]
pub enum BoundingSurfaceProperty {
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

#[citygml_feature(name = "bldg:WallSurface")]
pub struct WallSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_feature(name = "bldg:RoofSurface")]
pub struct RoofSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_feature(name = "bldg:GroundSurface")]
pub struct GroundSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_feature(name = "bldg:OuterCeilingSurface")]
pub struct OuterCeilingSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_feature(name = "bldg:OuterFloorSurface")]
pub struct OuterFloorSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_feature(name = "bldg:ClosureSurface")]
pub struct ClosureSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_feature(name = "bldg:CeilingSurface")]
pub struct CeilingSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_feature(name = "bldg:FloorSurface")]
#[citygml(name = "bldg:FloorSurface")]
pub struct FloorSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_feature(name = "bldg:InteriorWallSurface")]
pub struct InteriorWallSurface {
    #[citygml(path = b"bldg:opening")]
    opening: Vec<OpeningProperty>,
}

#[citygml_property(name = "bldg:OpeningProperty")]
pub enum OpeningProperty {
    #[citygml(path = b"bldg:Window")]
    Window(Window),
    #[citygml(path = b"bldg:Door")]
    Door(Door),
}

#[citygml_feature(name = "bldg:Window")]
pub struct Window {
    // ...
}

#[citygml_feature(name = "bldg:Door")]
pub struct Door {
    // ...
}

#[citygml_feature(name = "bldg:Room")]
pub struct Room {
    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurfaceProperty>,

    #[citygml(path = b"bldg:roomInstallation/bldg:IntBuildingInstallation")]
    pub room_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorFurniture/bldg:BuildingFurniture")]
    pub interior_furniture: Vec<BuildingFurniture>,
}

#[citygml_feature(name = "bldg:BuildingInstallation")]
pub struct BuildingInstallation {
    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurfaceProperty>,
}

#[citygml_feature(name = "bldg:BuildingFurniture")]
pub struct BuildingFurniture {
    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurfaceProperty>,
}

#[citygml_data(name = "uro:BuildingIDAttribute")]
pub struct BuildingIDAttribute {
    #[citygml(path = b"uro:buildingID")]
    pub building_id: Option<String>,

    #[citygml(path = b"uro:branchID")]
    pub branch_id: Option<i64>,

    #[citygml(path = b"uro:partID")]
    pub part_id: Option<i64>,

    #[citygml(path = b"uro:city")]
    pub city: Option<Code>,

    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Option<Code>,
}

#[citygml_data(name = "uro:BuildingDetailAttribute")]
pub struct BuildingDetailAttribute {
    #[citygml(path = b"uro:surveyYear")]
    pub survey_year: Option<String>,
}

#[citygml_property(name = "uro:BuildingDisasterRiskAttributeProperty")]
pub enum BuildingDisasterRiskAttributeProperty {
    #[citygml(path = b"uro:BuildingLandSlideRiskAttribute")]
    BuildingLandSlideRiskAttribute(BuildingLandSlideRiskAttribute),
    #[citygml(path = b"uro:BuildingTsunamiRiskAttribute")]
    BuildingTsunamiRiskAttribute(BuildingTsunamiRiskAttribute),
    #[citygml(path = b"uro:BuildingRiverFloodingRiskAttribute")]
    BuildingRiverFloodingRiskAttribute(BuildingRiverFloodingRiskAttribute),
}

#[citygml_data(name = "uro:BuildingLandSlideRiskAttribute")]
pub struct BuildingLandSlideRiskAttribute {
    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
    #[citygml(path = b"uro:areaType")]
    pub area_type: Option<String>,
}

#[citygml_data(name = "uro:BuildingTsunamiRiskAttribute")]
pub struct BuildingTsunamiRiskAttribute {
    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
    #[citygml(path = b"uro:areaType")]
    pub area_type: Option<String>,
}

#[citygml_data(name = "uro:BuildingRiverFloodingRiskAttribute")]
pub struct BuildingRiverFloodingRiskAttribute {
    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
    #[citygml(path = b"uro:areaType")]
    pub area_type: Option<String>,
}
