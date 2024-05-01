use nusamai_citygml::{citygml_feature, CityGmlElement, Code, GYear, Length, MeasureOrNullList};

use crate::models::{building as bldg, core, iur::uro};

#[citygml_feature(name = "uro:UndergroundBuilding")]
pub struct UndergroundBuilding {
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
    pub outer_building_installation: Vec<bldg::BuildingInstallation>,

    #[citygml(path = b"bldg:interiorBuildingInstallation/bldg:IntBuildingInstallation")]
    pub interior_building_installation: Vec<bldg::BuildingInstallation>,

    #[citygml(path = b"bldg:boundedBy")]
    pub bounded_by: Vec<bldg::BoundarySurfaceProperty>, // -> bldg:_BoundarySurface

    #[citygml(path = b"bldg:interiorRoom/bldg:Room")]
    pub interior_room: Vec<bldg::Room>,

    #[citygml(path = b"bldg:consistsOfBuildingPart/bldg:BuildingPart")]
    pub consists_of_building_part: Vec<bldg::BuildingPart>,

    #[citygml(path = b"bldg:address/core:Address")]
    pub address: Vec<core::Address>,

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
