use nusamai_citygml::{
    citygml_data, citygml_property, CityGmlElement, Code, Date, GYear, Length, Measure,
};

#[citygml_property(name = "uro:FacilityAttributeProperty")]
pub enum FacilityAttributeProperty {
    #[citygml(path = b"uro:CargoHandlingFacility")]
    CargoHandlingFacility(Box<CargoHandlingFacility>),
    #[citygml(path = b"uro:CyberportMarinaAndPBS")]
    CyberportMarinaAndPBS(Box<CyberportMarinaAndPBS>),
    #[citygml(path = b"uro:FishingPortCapacity")]
    FishingPortCapacity(Box<FishingPortCapacity>),
    #[citygml(path = b"uro:FishingPortFacility")]
    FishingPortFacility(Box<FishingPortFacility>),
    #[citygml(path = b"uro:HarborFacility")]
    HarborFacility(Box<HarborFacility>),
    #[citygml(path = b"uro:MaintenanceHistoryAttribute")]
    MaintenanceHistoryAttribute(Box<MaintenanceHistoryAttribute>),
    #[citygml(path = b"uro:MooringFacility")]
    MooringFacility(Box<MooringFacility>),
    #[citygml(path = b"uro:NavigationAssistanceFacility")]
    NavigationAssistanceFacility(Box<NavigationAssistanceFacility>),
    #[citygml(path = b"uro:ParkFacilityLongevityPlanAttribute")]
    ParkFacilityLongevityPlanAttribute(Box<ParkFacilityLongevityPlanAttribute>),
    #[citygml(path = b"uro:PortEnvironmentalImprovementFacility")]
    PortEnvironmentalImprovementFacility(Box<PortEnvironmentalImprovementFacility>),
    #[citygml(path = b"uro:PortManagementFacility")]
    PortManagementFacility(Box<PortManagementFacility>),
    #[citygml(path = b"uro:PortPassengerFacility")]
    PortPassengerFacility(Box<PortPassengerFacility>),
    #[citygml(path = b"uro:PortPollutionControlFacility")]
    PortPollutionControlFacility(Box<PortPollutionControlFacility>),
    #[citygml(path = b"uro:PortProtectiveFacility")]
    PortProtectiveFacility(Box<PortProtectiveFacility>),
    #[citygml(path = b"uro:PortStorageFacility")]
    PortStorageFacility(Box<PortStorageFacility>),
    #[citygml(path = b"uro:PortTransportationFacility")]
    PortTransportationFacility(Box<PortTransportationFacility>),
    #[citygml(path = b"uro:PortWasteTreatmentFacility")]
    PortWasteTreatmentFacility(Box<PortWasteTreatmentFacility>),
    #[citygml(path = b"uro:PortWelfareFacility")]
    PortWelfareFacility(Box<PortWelfareFacility>),
    #[citygml(path = b"uro:ShipServiceFacility")]
    ShipServiceFacility(Box<ShipServiceFacility>),
}

#[citygml_data(name = "uro:CargoHandlingFacility")]
pub struct CargoHandlingFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:mainCargo")]
    pub main_cargo: Option<Code>,

    #[citygml(path = b"uro:mooringFacility")]
    pub mooring_facility: Option<String>,

    #[citygml(path = b"uro:liftableLoad")]
    pub liftable_load: Option<Measure>,

    #[citygml(path = b"uro:ability")]
    pub ability: Option<i64>,

    #[citygml(path = b"uro:packingName")]
    pub packing_name: Option<Code>,

    #[citygml(path = b"uro:acquisitionYear")]
    pub acquisition_year: Option<GYear>,

    #[citygml(path = b"uro:innerTotalFloorArea")]
    pub inner_total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:innerOfSiteArea")]
    pub inner_of_site_area: Option<Measure>,

    #[citygml(path = b"uro:outerOfTotalFloorArea")]
    pub outer_of_total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:outerSiteArea")]
    pub outer_site_area: Option<Measure>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:CyberportMarinaAndPBS")]
pub struct CyberportMarinaAndPBS {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:geologicalType")]
    pub geological_type: Option<Code>,

    #[citygml(path = b"uro:obstructingStructures")]
    pub obstructing_structures: Option<String>,

    #[citygml(path = b"uro:mainPartLength")]
    pub main_part_length: Option<Length>,

    #[citygml(path = b"uro:totalLength")]
    pub total_length: Option<Length>,

    #[citygml(path = b"uro:waveDissipatorLength")]
    pub wave_dissipator_length: Option<Length>,

    #[citygml(path = b"uro:facilityWidth")]
    pub facility_width: Option<Length>,

    #[citygml(path = b"uro:apronWidth")]
    pub apron_width: Option<Length>,

    #[citygml(path = b"uro:restrictionStructure")]
    pub restriction_structure: Option<String>,

    #[citygml(path = b"uro:plannedDepth")]
    pub planned_depth: Option<Length>,

    #[citygml(path = b"uro:currentDepth")]
    pub current_depth: Option<Length>,

    #[citygml(path = b"uro:innerTotalFloorArea")]
    pub inner_total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:innerOfSiteArea")]
    pub inner_of_site_area: Option<Measure>,

    #[citygml(path = b"uro:outerOfTotalFloorArea")]
    pub outer_of_total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:outerSiteArea")]
    pub outer_site_area: Option<Measure>,

    #[citygml(path = b"uro:ceilingHeight")]
    pub ceiling_height: Option<Length>,

    #[citygml(path = b"uro:gravityResistant")]
    pub gravity_resistant: Option<Measure>,

    #[citygml(path = b"uro:form")]
    pub form: Option<Code>,

    #[citygml(path = b"uro:areaType")]
    pub area_type: Option<Code>,

    #[citygml(path = b"uro:mainVessels")]
    pub main_vessels: Option<Code>,

    #[citygml(path = b"uro:isDredged")]
    pub is_dredged: Option<bool>,

    #[citygml(path = b"uro:mooringPostWeight")]
    pub mooring_post_weight: Option<Measure>,

    #[citygml(path = b"uro:numberOfMooringPosts")]
    pub number_of_mooring_posts: Option<i64>,

    #[citygml(path = b"uro:resistantMaterial")]
    pub resistant_material: Option<i64>,

    #[citygml(path = b"uro:lighting")]
    pub lighting: Option<i64>,

    #[citygml(path = b"uro:stairs")]
    pub stairs: Option<i64>,

    #[citygml(path = b"uro:lifesaving")]
    pub lifesaving: Option<String>,

    #[citygml(path = b"uro:lifesavingNumber")]
    pub lifesaving_number: Option<i64>,

    #[citygml(path = b"uro:bumper")]
    pub bumper: Option<Length>,

    #[citygml(path = b"uro:numberOfVehicleBoardings")]
    pub number_of_vehicle_boardings: Option<i64>,

    #[citygml(path = b"uro:vehicleBoardingWidth")]
    pub vehicle_boarding_width: Option<Length>,

    #[citygml(path = b"uro:shipType")]
    pub ship_type: Option<String>,

    #[citygml(path = b"uro:numberOfSeats")]
    pub number_of_seats: Option<i64>,

    #[citygml(path = b"uro:mainCargo")]
    pub main_cargo: Option<Code>,

    #[citygml(path = b"uro:storageCapacity")]
    pub storage_capacity: Option<i64>,

    #[citygml(path = b"uro:storageCapacityUnit")]
    pub storage_capacity_unit: Option<Code>,

    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:FishingPortCapacity")]
pub struct FishingPortCapacity {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:capacity")]
    pub capacity: Option<String>,

    #[citygml(path = b"uro:weightCapacity")]
    pub weight_capacity: Option<Measure>,

    #[citygml(path = b"uro:hullForm")]
    pub hull_form: Option<i64>,

    #[citygml(path = b"uro:shipNumber")]
    pub ship_number: Option<i64>,

    #[citygml(path = b"uro:waterDepth-2m")]
    pub water_depth_2m: Option<Measure>,

    #[citygml(path = b"uro:waterDepth2-3m")]
    pub water_depth2_3m: Option<Measure>,

    #[citygml(path = b"uro:waterDepth3-6m")]
    pub water_depth3_6m: Option<Measure>,

    #[citygml(path = b"uro:waterDepth6-m")]
    pub water_depth6_m: Option<Measure>,

    #[citygml(path = b"uro:heightAboveAWL")]
    pub height_above_a_w_l: Option<Length>,

    #[citygml(path = b"uro:heightOnFoundations")]
    pub height_on_foundations: Option<Length>,

    #[citygml(path = b"uro:luminousRange")]
    pub luminous_range: Option<Length>,

    #[citygml(path = b"uro:luminousColor")]
    pub luminous_color: Option<String>,

    #[citygml(path = b"uro:candlePower")]
    pub candle_power: Option<i64>,

    #[citygml(path = b"uro:lightType")]
    pub light_type: Option<String>,

    #[citygml(path = b"uro:period")]
    pub period: Option<String>,

    #[citygml(path = b"uro:maximumGroundingWeight")]
    pub maximum_grounding_weight: Option<i64>,

    #[citygml(path = b"uro:handleablePower")]
    pub handleable_power: Option<i64>,

    #[citygml(path = b"uro:maximumWaterSupply")]
    pub maximum_water_supply: Option<i64>,

    #[citygml(path = b"uro:maximumRefueling")]
    pub maximum_refueling: Option<String>,

    #[citygml(path = b"uro:people")]
    pub people: Option<i64>,

    #[citygml(path = b"uro:other")]
    pub other: Option<String>,
}

#[citygml_data(name = "uro:FishingPortFacility")]
pub struct FishingPortFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:facilityDetailsType", required)]
    pub facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portType", required)]
    pub port_type: Option<Code>,

    #[citygml(path = b"uro:address", required)]
    pub address: Option<String>,

    #[citygml(path = b"uro:designatedArea", required)]
    pub designated_area: Option<String>,

    #[citygml(path = b"uro:designation")]
    pub designation: Vec<String>,

    #[citygml(path = b"uro:designatedAdministrator")]
    pub designated_administrator: Vec<String>,

    #[citygml(path = b"uro:referenceNumber")]
    pub reference_number: Vec<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<String>,

    #[citygml(path = b"uro:facilityManager")]
    pub facility_manager: Option<String>,

    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:otherStructure")]
    pub other_structure: Option<String>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:ceilingHeight")]
    pub ceiling_height: Option<Length>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:otherSizeDescription")]
    pub other_size_description: Option<String>,

    #[citygml(path = b"uro:dateOfConstructionOrAcquisition")]
    pub date_of_construction_or_acquisition: Option<Date>,

    #[citygml(path = b"uro:cost")]
    pub cost: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:HarborFacility")]
pub struct HarborFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:geologicalType")]
    pub geological_type: Option<Code>,

    #[citygml(path = b"uro:obstructingStructures")]
    pub obstructing_structures: Option<String>,

    #[citygml(path = b"uro:structuralLimitations")]
    pub structural_limitations: Option<Length>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:minimumWidth")]
    pub minimum_width: Option<Length>,

    #[citygml(path = b"uro:maximumWidth")]
    pub maximum_width: Option<Length>,

    #[citygml(path = b"uro:plannedDepth")]
    pub planned_depth: Option<Length>,

    #[citygml(path = b"uro:currentDepth")]
    pub current_depth: Option<Length>,

    #[citygml(path = b"uro:isDredged")]
    pub is_dredged: Option<bool>,

    #[citygml(path = b"uro:areaType")]
    pub area_type: Option<Code>,

    #[citygml(path = b"uro:innerArea")]
    pub inner_area: Option<Measure>,

    #[citygml(path = b"uro:outerArea")]
    pub outer_area: Option<Measure>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Vec<String>,
}

#[citygml_data(name = "uro:MaintenanceHistoryAttribute")]
pub struct MaintenanceHistoryAttribute {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:maintenanceType", required)]
    pub maintenance_type: Option<Code>,

    #[citygml(path = b"uro:maintenanceFiscalYear")]
    pub maintenance_fiscal_year: Option<GYear>,

    #[citygml(path = b"uro:maintenanceYear")]
    pub maintenance_year: Option<GYear>,

    #[citygml(path = b"uro:maintenanceDate")]
    pub maintenance_date: Option<Date>,

    #[citygml(path = b"uro:status")]
    pub status: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
}

#[citygml_data(name = "uro:MooringFacility")]
pub struct MooringFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:mainPartLength")]
    pub main_part_length: Option<Length>,

    #[citygml(path = b"uro:totalLength")]
    pub total_length: Option<Length>,

    #[citygml(path = b"uro:facilityWidth")]
    pub facility_width: Option<Length>,

    #[citygml(path = b"uro:apronWidth")]
    pub apron_width: Option<Length>,

    #[citygml(path = b"uro:plannedDepth")]
    pub planned_depth: Option<Length>,

    #[citygml(path = b"uro:currentDepth")]
    pub current_depth: Option<Length>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:ceilingHeight")]
    pub ceiling_height: Option<Length>,

    #[citygml(path = b"uro:gravityResistant")]
    pub gravity_resistant: Option<Measure>,

    #[citygml(path = b"uro:form")]
    pub form: Option<Code>,

    #[citygml(path = b"uro:mainVessels")]
    pub main_vessels: Option<Code>,

    #[citygml(path = b"uro:mooringPostWeight")]
    pub mooring_post_weight: Option<Measure>,

    #[citygml(path = b"uro:numberOfMooringPosts")]
    pub number_of_mooring_posts: Option<i64>,

    #[citygml(path = b"uro:resistantMaterial")]
    pub resistant_material: Option<i64>,

    #[citygml(path = b"uro:lighting")]
    pub lighting: Option<i64>,

    #[citygml(path = b"uro:stairs")]
    pub stairs: Option<i64>,

    #[citygml(path = b"uro:lifesavingAppliances")]
    pub lifesaving_appliances: Option<String>,

    #[citygml(path = b"uro:numberOfLifesavingAppliances")]
    pub number_of_lifesaving_appliances: Option<i64>,

    #[citygml(path = b"uro:bumper")]
    pub bumper: Option<Length>,

    #[citygml(path = b"uro:numberOfVehicleBoardings")]
    pub number_of_vehicle_boardings: Option<i64>,

    #[citygml(path = b"uro:vehicleBoardingWidth")]
    pub vehicle_boarding_width: Option<Length>,

    #[citygml(path = b"uro:shipType")]
    pub ship_type: Option<String>,

    #[citygml(path = b"uro:numberOfSeats")]
    pub number_of_seats: Option<i64>,

    #[citygml(path = b"uro:mainCargo")]
    pub main_cargo: Option<Code>,

    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:NavigationAssistanceFacility")]
pub struct NavigationAssistanceFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<String>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:ParkFacilityLongevityPlanAttribute")]
pub struct ParkFacilityLongevityPlanAttribute {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:parkCode", required)]
    pub park_code: Option<Code>,

    #[citygml(path = b"uro:parkName", required)]
    pub park_name: Option<Code>,

    #[citygml(path = b"uro:parkType", required)]
    pub park_type: Option<Code>,

    #[citygml(path = b"uro:facilityName")]
    pub facility_name: Option<Code>,

    #[citygml(path = b"uro:facilityNameOptional")]
    pub facility_name_optional: Option<String>,

    #[citygml(path = b"uro:specificFacilityName", required)]
    pub specific_facility_name: Option<String>,

    #[citygml(path = b"uro:numberOfFacilities/uro:NumberOfFacilities")]
    pub number_of_facilities: Option<NumberOfFacilities>,

    #[citygml(path = b"uro:size")]
    pub size: Option<String>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:mainMaterialOptional")]
    pub main_material_optional: Option<String>,

    #[citygml(path = b"uro:installationYear", required)]
    pub installation_year: Option<GYear>,

    #[citygml(path = b"uro:disposalLimitPeriod")]
    pub disposal_limit_period: Option<i64>,

    #[citygml(path = b"uro:expectedUsagePeriod")]
    pub expected_usage_period: Option<i64>,

    #[citygml(
        path = b"uro:repairsBeforeParkHealthAssessment/uro:RepairsBeforeParkHealthAssessment"
    )]
    pub repairs_before_park_health_assessment: Option<RepairsBeforeParkHealthAssessment>,

    #[citygml(path = b"uro:parkHealthAssessment/uro:ParkHealthAssessment")]
    pub park_health_assessment: Vec<ParkHealthAssessment>,

    #[citygml(path = b"uro:managementType")]
    pub management_type: Option<Code>,

    #[citygml(path = b"uro:expectedRenewalYearWithMeasures")]
    pub expected_renewal_year_with_measures: Option<GYear>,

    #[citygml(path = b"uro:longevityMeasures/uro:LongevityMeasures")]
    pub longevity_measures: Vec<LongevityMeasures>,

    #[citygml(path = b"uro:noteForLongevity")]
    pub note_for_longevity: Option<String>,
}

#[citygml_data(name = "uro:PortEnvironmentalImprovementFacility")]
pub struct PortEnvironmentalImprovementFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:usage")]
    pub usage: Option<String>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:totalFoorArea")]
    pub total_foor_area: Option<Measure>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:PortManagementFacility")]
pub struct PortManagementFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:totalFloorArea")]
    pub total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:numberOfShipTypes")]
    pub number_of_ship_types: Option<i64>,

    #[citygml(path = b"uro:unitOfShipType")]
    pub unit_of_ship_type: Option<Code>,

    #[citygml(path = b"uro:loadingCapacity")]
    pub loading_capacity: Option<i64>,

    #[citygml(path = b"uro:acquisitionYear")]
    pub acquisition_year: Option<GYear>,

    #[citygml(path = b"uro:usage")]
    pub usage: Option<String>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:PortPassengerFacility")]
pub struct PortPassengerFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:totalFloorArea")]
    pub total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:acquisitionYear")]
    pub acquisition_year: Option<GYear>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:PortPollutionControlFacility")]
pub struct PortPollutionControlFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:crossSectionalArea")]
    pub cross_sectional_area: Option<Measure>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:height")]
    pub height: Option<Length>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:PortProtectiveFacility")]
pub struct PortProtectiveFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Vec<String>,
}

#[citygml_data(name = "uro:PortStorageFacility")]
pub struct PortStorageFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:innerTotalFloorArea")]
    pub inner_total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:innerOfSiteArea")]
    pub inner_of_site_area: Option<Measure>,

    #[citygml(path = b"uro:outerOfTotalFloorArea")]
    pub outer_of_total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:outerSiteArea")]
    pub outer_site_area: Option<Measure>,

    #[citygml(path = b"uro:mainCargo")]
    pub main_cargo: Option<Code>,

    #[citygml(path = b"uro:storageCapacity")]
    pub storage_capacity: Option<i64>,

    #[citygml(path = b"uro:storageCapacityUnit")]
    pub storage_capacity_unit: Option<Code>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:PortTransportationFacility")]
pub struct PortTransportationFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:startingPoint")]
    pub starting_point: Option<String>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:beddingWidth")]
    pub bedding_width: Option<Length>,

    #[citygml(path = b"uro:numberOfLanes")]
    pub number_of_lanes: Option<i64>,

    #[citygml(path = b"uro:parkingLotCapacityOfBus")]
    pub parking_lot_capacity_of_bus: Option<i64>,

    #[citygml(path = b"uro:parkingLotCapacityOfCars")]
    pub parking_lot_capacity_of_cars: Option<i64>,

    #[citygml(path = b"uro:routeType")]
    pub route_type: Option<Code>,

    #[citygml(path = b"uro:heightToDigit")]
    pub height_to_digit: Option<Length>,

    #[citygml(path = b"uro:heightLimit")]
    pub height_limit: Option<Length>,

    #[citygml(path = b"uro:minimumWidth")]
    pub minimum_width: Option<Length>,

    #[citygml(path = b"uro:minimumDepth")]
    pub minimum_depth: Option<Length>,

    #[citygml(path = b"uro:numberOfAircraftParkingSpaces")]
    pub number_of_aircraft_parking_spaces: Option<i64>,

    #[citygml(path = b"uro:pavementType")]
    pub pavement_type: Option<Code>,

    #[citygml(path = b"uro:mainCargo")]
    pub main_cargo: Option<Code>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:PortWasteTreatmentFacility")]
pub struct PortWasteTreatmentFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:perimeter")]
    pub perimeter: Option<Length>,

    #[citygml(path = b"uro:mainPartLength")]
    pub main_part_length: Option<Length>,

    #[citygml(path = b"uro:innerShoreLength")]
    pub inner_shore_length: Option<Length>,

    #[citygml(path = b"uro:ceilingHeight")]
    pub ceiling_height: Option<Length>,

    #[citygml(path = b"uro:waveDissipatorLength")]
    pub wave_dissipator_length: Option<Length>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:wasteType")]
    pub waste_type: Option<Code>,

    #[citygml(path = b"uro:plannedDisposalArea")]
    pub planned_disposal_area: Option<Measure>,

    #[citygml(path = b"uro:plannedDisposalAmount")]
    pub planned_disposal_amount: Option<i64>,

    #[citygml(path = b"uro:receivingCapacity")]
    pub receiving_capacity: Option<i64>,

    #[citygml(path = b"uro:shipType")]
    pub ship_type: Option<String>,

    #[citygml(path = b"uro:unitOfReceivingCapacity")]
    pub unit_of_receiving_capacity: Option<Code>,

    #[citygml(path = b"uro:acquisitionYear")]
    pub acquisition_year: Option<GYear>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:PortWelfareFacility")]
pub struct PortWelfareFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:totalFloorArea")]
    pub total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:ShipServiceFacility")]
pub struct ShipServiceFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType", required)]
    pub port_facility_details_type: Option<Code>,

    #[citygml(path = b"uro:portName", required)]
    pub port_name: Option<String>,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<i64>,

    #[citygml(path = b"uro:shipType")]
    pub ship_type: Option<String>,

    #[citygml(path = b"uro:supplyAbility")]
    pub supply_ability: Option<i64>,

    #[citygml(path = b"uro:supplyAbilityUnit")]
    pub supply_ability_unit: Option<Code>,

    #[citygml(path = b"uro:mooringPlace")]
    pub mooring_place: Option<String>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:acquisitionYear")]
    pub acquisition_year: Option<GYear>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<i64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:NumberOfFacilities")]
pub struct NumberOfFacilities {
    #[citygml(path = b"uro:quantity", required)]
    pub quantity: Option<i64>,

    #[citygml(path = b"uro:quantityUnit", required)]
    pub quantity_unit: Option<Code>,
}

#[citygml_data(name = "uro:LongevityMeasures")]
pub struct LongevityMeasures {
    #[citygml(path = b"uro:fiscalYearForCountermeasures", required)]
    pub fiscal_year_for_countermeasures: Option<GYear>,

    #[citygml(path = b"uro:countermeasuresCost/uro:CountermeasuresCost")]
    pub countermeasures_cost: Option<CountermeasuresCost>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
}

#[citygml_data(name = "uro:RepairsBeforeParkHealthAssessment")]
pub struct RepairsBeforeParkHealthAssessment {
    #[citygml(path = b"uro:repair", required)]
    pub repair: Option<Code>,

    #[citygml(path = b"uro:repairFiscalYear")]
    pub repair_fiscal_year: Option<GYear>,
}

#[citygml_data(name = "uro:ParkHealthAssessment")]
pub struct ParkHealthAssessment {
    #[citygml(path = b"uro:assessmentFiscalYear", required)]
    pub assessment_fiscal_year: Option<GYear>,

    #[citygml(path = b"uro:deteriorationStatus")]
    pub deterioration_status: Option<String>,

    #[citygml(path = b"uro:condition")]
    pub condition: Option<Code>,

    #[citygml(path = b"uro:urgency")]
    pub urgency: Option<Code>,
}

#[citygml_data(name = "uro:CountermeasuresCost")]
pub struct CountermeasuresCost {
    #[citygml(path = b"uro:cost")]
    pub cost: Option<i64>,

    #[citygml(path = b"uro:costUnit", required)]
    pub cost_unit: Option<String>,
}
