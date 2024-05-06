use nusamai_citygml::{
    citygml_feature, citygml_property, CityGmlElement, Code, GYear, Length, MeasureOrNullList,
};

use super::{core::Address, iur::uro};

#[citygml_feature(name = "bldg:Building")]
pub struct Building {
    #[citygml(path = b"bldg:class")]
    pub class: Option<Code>,

    #[citygml(path = b"bldg:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"bldg:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"bldg:yearOfConstruction")]
    pub year_of_construction: Option<GYear>,

    #[citygml(path = b"bldg:yearOfDemolition")]
    pub year_of_demolition: Option<GYear>,

    #[citygml(path = b"bldg:roofType")]
    pub roof_type: Option<Code>,

    #[citygml(path = b"bldg:measuredHeight")]
    pub measured_height: Option<Length>,

    #[citygml(path = b"bldg:storeysAboveGround")]
    pub storeys_above_ground: Option<u64>,

    #[citygml(path = b"bldg:storeysBelowGround")]
    pub storeys_below_ground: Option<u64>,

    #[citygml(path = b"bldg:storeyHeightsAboveGround")]
    pub storey_heights_above_ground: Option<MeasureOrNullList>,

    #[citygml(path = b"bldg:storeyHeightsBelowGround")]
    pub storey_heights_below_ground: Option<MeasureOrNullList>,

    #[citygml(path = b"bldg:outerBuildingInstallation/bldg:BuildingInstallation")]
    pub outer_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorBuildingInstallation/bldg:IntBuildingInstallation")]
    pub interior_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> bldg:_BoundarySurface

    #[citygml(path = b"bldg:interiorRoom/bldg:Room")]
    pub interior_room: Vec<Room>,

    #[citygml(path = b"bldg:consistsOfBuildingPart/bldg:BuildingPart")]
    pub consists_of_building_part: Vec<BuildingPart>,

    #[citygml(path = b"bldg:address/core:Address")]
    pub address: Vec<Address>,

    #[citygml(path = b"uro:buildingDataQualityAttribute/uro:BuildingDataQualityAttribute")]
    #[citygml(path = b"uro:bldgDataQualityAttribute/uro:DataQualityAttribute")]
    pub bldg_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:buildingDisasterRiskAttribute")]
    #[citygml(path = b"uro:bldgDisasterRiskAttribute")]
    pub bldg_disaster_risk_attribute: Vec<uro::DisasterRiskAttributeProperty>, // -> uro:DisasterRiskAttribute

    #[citygml(path = b"uro:bldgDmAttribute")]
    pub bldg_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:bldgFacilityAttribute")]
    pub bldg_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:bldgFacilityIdAttribute")]
    pub bldg_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:bldgFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub bldg_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    #[citygml(path = b"uro:bldgKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub bldg_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:bldgRealEstateIDAttribute/uro:RealEstateIDAttribute")]
    pub bldg_real_estate_id_attribute: Option<uro::RealEstateIDAttribute>,

    #[citygml(path = b"uro:bldgUsecaseAttribute/uro:BuildingUsecaseAttribute")]
    pub bldg_usecase_attribute: Vec<uro::BuildingUsecaseAttribute>,

    #[citygml(path = b"uro:buildingDetailAttribute/uro:BuildingDetailAttribute")]
    pub building_detail_attribute: Vec<uro::BuildingDetailAttribute>,

    #[citygml(path = b"uro:buildingIDAttribute/uro:BuildingIDAttribute")]
    pub building_id_attribute: Vec<uro::BuildingIDAttribute>,

    #[citygml(path = b"uro:ifcBuildingAttribute")]
    pub ifc_building_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBuildingAttribute")]
    pub indoor_building_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute

    #[citygml(path = b"uro:largeCustomerFacilityAttribute/uro:LargeCustomerFacilityAttribute")]
    pub large_customer_facility_attribute: Vec<uro::LargeCustomerFacilityAttribute>,
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
    pub year_of_construction: Option<GYear>,

    #[citygml(path = b"bldg:yearOfDemolition")]
    pub year_of_demolition: Option<GYear>,

    #[citygml(path = b"bldg:roofType")]
    pub roof_type: Option<Code>,

    #[citygml(path = b"bldg:measuredHeight")]
    pub measured_height: Option<Length>,

    #[citygml(path = b"bldg:storeysAboveGround")]
    pub storeys_above_ground: Option<u64>,

    #[citygml(path = b"bldg:storeysBelowGround")]
    pub storeys_below_ground: Option<u64>,

    #[citygml(path = b"bldg:storeyHeightsAboveGround")]
    pub storey_heights_above_ground: Option<MeasureOrNullList>,

    #[citygml(path = b"bldg:storeyHeightsBelowGround")]
    pub storey_heights_below_ground: Option<MeasureOrNullList>,

    #[citygml(path = b"bldg:outerBuildingInstallation/bldg:BuildingInstallation")]
    pub outer_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:interiorBuildingInstallation/bldg:IntBuildingInstallation")]
    pub interior_building_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> bldg:_BoundarySurface

    #[citygml(path = b"bldg:interiorRoom/bldg:Room")]
    pub interior_room: Vec<Room>,

    #[citygml(path = b"bldg:consistsOfBuildingPart/bldg:BuildingPart")]
    pub consists_of_building_part: Vec<BuildingPart>,

    #[citygml(path = b"bldg:address/core:Address")]
    pub address: Vec<Address>,

    #[citygml(path = b"uro:buildingDataQualityAttribute/uro:BuildingDataQualityAttribute")]
    #[citygml(path = b"uro:bldgDataQualityAttribute/uro:DataQualityAttribute")]
    pub bldg_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:buildingDisasterRiskAttribute")]
    #[citygml(path = b"uro:bldgDisasterRiskAttribute")]
    pub bldg_disaster_risk_attribute: Vec<uro::DisasterRiskAttributeProperty>, // -> uro:DisasterRiskAttribute

    #[citygml(path = b"uro:bldgDmAttribute")]
    pub bldg_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:bldgFacilityAttribute")]
    pub bldg_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:bldgFacilityIdAttribute")]
    pub bldg_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:bldgFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub bldg_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    #[citygml(path = b"uro:bldgKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub bldg_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:bldgRealEstateIDAttribute/uro:RealEstateIDAttribute")]
    pub bldg_real_estate_id_attribute: Option<uro::RealEstateIDAttribute>,

    #[citygml(path = b"uro:bldgUsecaseAttribute/uro:BuildingUsecaseAttribute")]
    pub bldg_usecase_attribute: Vec<uro::BuildingUsecaseAttribute>,

    #[citygml(path = b"uro:buildingDetailAttribute/uro:BuildingDetailAttribute")]
    pub building_detail_attribute: Vec<uro::BuildingDetailAttribute>,

    #[citygml(path = b"uro:buildingIDAttribute/uro:BuildingIDAttribute")]
    pub building_id_attribute: Vec<uro::BuildingIDAttribute>,

    #[citygml(path = b"uro:ifcBuildingAttribute")]
    pub ifc_building_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBuildingAttribute")]
    pub indoor_building_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute

    #[citygml(path = b"uro:largeCustomerFacilityAttribute/uro:LargeCustomerFacilityAttribute")]
    pub large_customer_facility_attribute: Vec<uro::LargeCustomerFacilityAttribute>,
}

#[citygml_property(name = "bldg:_BoundarySurfaceProperty")]
pub enum BoundarySurfaceProperty {
    #[citygml(path = b"bldg:CeilingSurface")]
    CeilingSurface(CeilingSurface),
    #[citygml(path = b"bldg:ClosureSurface")]
    ClosureSurface(ClosureSurface),
    #[citygml(path = b"bldg:FloorSurface")]
    FloorSurface(FloorSurface),
    #[citygml(path = b"bldg:GroundSurface")]
    GroundSurface(GroundSurface),
    #[citygml(path = b"bldg:InteriorWallSurface")]
    InteriorWallSurface(InteriorWallSurface),
    #[citygml(path = b"bldg:OuterCeilingSurface")]
    OuterCeilingSurface(OuterCeilingSurface),
    #[citygml(path = b"bldg:OuterFloorSurface")]
    OuterFloorSurface(OuterFloorSurface),
    #[citygml(path = b"bldg:RoofSurface")]
    RoofSurface(RoofSurface),
    #[citygml(path = b"bldg:WallSurface")]
    WallSurface(WallSurface),
}
#[citygml_feature(name = "bldg:CeilingSurface")]
pub struct CeilingSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
#[citygml_feature(name = "bldg:ClosureSurface")]
pub struct ClosureSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
#[citygml_feature(name = "bldg:FloorSurface")]
pub struct FloorSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
#[citygml_feature(name = "bldg:GroundSurface")]
pub struct GroundSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
#[citygml_feature(name = "bldg:InteriorWallSurface")]
pub struct InteriorWallSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
#[citygml_feature(name = "bldg:OuterCeilingSurface")]
pub struct OuterCeilingSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
#[citygml_feature(name = "bldg:OuterFloorSurface")]
pub struct OuterFloorSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
#[citygml_feature(name = "bldg:RoofSurface")]
pub struct RoofSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
#[citygml_feature(name = "bldg:WallSurface")]
pub struct WallSurface {
    #[citygml(path = b"bldg:opening")]
    pub opening: Vec<OpeningProperty>, // -> bldg:_Opening

    #[citygml(path = b"uro:ifcBoundarySurfaceAttribute")]
    pub ifc_boundary_surface_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorBoundarySurfaceAttribute")]
    pub indoor_boundary_surface_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}

#[citygml_property(name = "bldg:_OpeningProperty")]
pub enum OpeningProperty {
    #[citygml(path = b"bldg:Door")]
    Door(Door),
    #[citygml(path = b"bldg:Window")]
    Window(Window),
}
#[citygml_feature(name = "bldg:Door")]
pub struct Door {
    #[citygml(path = b"uro:ifcOpeningAttribute")]
    pub ifc_opening_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorOpeningAttribute")]
    pub indoor_opening_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute

    #[citygml(path = b"bldg:address/core:Address")]
    pub address: Vec<Address>,
}
#[citygml_feature(name = "bldg:Window")]
pub struct Window {
    #[citygml(path = b"uro:ifcOpeningAttribute")]
    pub ifc_opening_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorOpeningAttribute")]
    pub indoor_opening_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}

#[citygml_feature(name = "bldg:Room")]
pub struct Room {
    #[citygml(path = b"bldg:class")]
    pub class: Option<Code>,

    #[citygml(path = b"bldg:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"bldg:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> bldg:_BoundarySurface

    #[citygml(path = b"bldg:interiorFurniture/bldg:BuildingFurniture")]
    pub interior_furniture: Vec<BuildingFurniture>,

    #[citygml(path = b"bldg:roomInstallation/bldg:IntBuildingInstallation")]
    pub room_installation: Vec<BuildingInstallation>,

    #[citygml(path = b"uro:ifcRoomAttribute")]
    pub ifc_room_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorRoomAttribute")]
    pub indoor_room_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}

#[citygml_feature(name = "bldg:BuildingInstallation")]
pub struct BuildingInstallation {
    #[citygml(path = b"bldg:class")]
    pub class: Option<Code>,

    #[citygml(path = b"bldg:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"bldg:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> bldg:_BoundarySurface

    #[citygml(path = b"uro:ifcBuildingInstallationAttribute")]
    pub ifc_building_installation_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute
}

#[citygml_feature(name = "bldg:BuildingFurniture")]
pub struct BuildingFurniture {
    #[citygml(path = b"bldg:class")]
    pub class: Option<Code>,

    #[citygml(path = b"bldg:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"bldg:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:ifcBuildingFurnitureAttribute")]
    pub ifc_building_furniture_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorFutnitureAttribute")]
    pub indoor_futniture_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute
}
