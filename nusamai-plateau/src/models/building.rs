use citygml::{CityGMLElement, Code, GeometryRef, Measure, NaiveDate};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Building {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"core:creationDate")]
    pub creation_date: Option<NaiveDate>,

    #[citygml(path = b"core:terminationDate")]
    pub termination_date: Option<NaiveDate>,

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
    pub building_disaster_risk_attribute: Vec<BuildingDisasterRiskAttribute>,

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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingPart {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"core:creationDate")]
    pub creation_date: Option<NaiveDate>,

    #[citygml(path = b"core:terminationDate")]
    pub termination_date: Option<NaiveDate>,

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

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
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
    #[citygml(path = b"bldg:CeilingSurface")]
    CeilingSurface(CeilingSurface),
    #[citygml(path = b"bldg:FloorSurface")]
    FloorSurface(FloorSurface),
    #[citygml(path = b"bldg:InteriorWallSurface")]
    InteriorWallSurface(InteriorWallSurface),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct WallSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct RoofSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct GroundSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct OuterCeilingSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct OuterFloorSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct ClosureSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct CeilingSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct FloorSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct InteriorWallSurface {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:opening")]
    opening: Vec<Opening>,
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
#[derive(Default, Debug, CityGMLElement)]
pub enum Opening {
    #[default]
    Unknown,
    #[citygml(path = b"bldg:Window")]
    Window(Window),
    #[citygml(path = b"bldg:Door")]
    Door(Door),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Window {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Door {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Room {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurface>,

    #[citygml(path = b"bldg:roomInstallation/bldg:IntBuildingInstallation")]
    pub room_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorFurniture/bldg:BuildingFurniture")]
    pub interior_furniture: Vec<BuildingFurniture>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingInstallation {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurface>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingFurniture {
    #[citygml(geom = b"bldg")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundingSurface>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingDetailAttribute {
    #[citygml(path = b"uro:surveyYear")]
    pub survey_year: Option<String>,
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
#[derive(Default, Debug, CityGMLElement)]
pub enum BuildingDisasterRiskAttribute {
    #[default]
    Unknown,
    #[citygml(path = b"uro:BuildingLandSlideRiskAttribute")]
    BuildingLandSlideRiskAttribute(BuildingLandSlideRiskAttribute),
    #[citygml(path = b"uro:BuildingTsunamiRiskAttribute")]
    BuildingTsunamiRiskAttribute(BuildingTsunamiRiskAttribute),
    #[citygml(path = b"uro:BuildingRiverFloodingRiskAttribute")]
    BuildingRiverFloodingRiskAttribute(BuildingRiverFloodingRiskAttribute),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingLandSlideRiskAttribute {
    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
    #[citygml(path = b"uro:areaType")]
    pub area_type: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingTsunamiRiskAttribute {
    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
    #[citygml(path = b"uro:areaType")]
    pub area_type: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct BuildingRiverFloodingRiskAttribute {
    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
    #[citygml(path = b"uro:areaType")]
    pub area_type: Option<String>,
}
