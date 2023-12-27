use citygml::{citygml_property,citygml_data,CityGMLElement,Code,Measure,Date};

#[citygml_property(name = "uro:FacilityAttributeProperty")]
pub enum FacilityAttributeProperty {
    #[citygml(path = b"uro:RiverFacilityIdAttribute")]
    RiverFacilityIdAttribute(RiverFacilityIdAttribute),

    #[citygml(path = b"uro:CargoHandlingFacility")]
    CargoHandlingFacility(CargoHandlingFacility),
    
    #[citygml(path = b"uro:CyberportMarinaAndPBS")]
    CyberportMarinaAndPBS(CyberportMarinaAndPBS),

    #[citygml(path = b"uro:FishingPortFacilityAttribute")]
    FishingPortFacilityAttribute(FishingPortFacilityAttribute),
    // TODO:
    // #[citygml(path = b"uro:FishingPortCapacity")]
    // FishingPortAttribute(FishingPortAttribute),
    // TODO:
    // #[citygml(path = b"uro:FishingPortFacility")]
    // FishingPortFacility(FishingPortFacility),
    // TODO:
    // #[citygml(path = b"uro:HarborFacility")]
    // HarborFacility(HarborFacility),
    // TODO:
    // #[citygml(path = b"uro:MaintenanceHistoryAttribute")]
    // MaintenanceHistoryAttribute(MaintenanceHistoryAttribute),
    // TODO:
    // #[citygml(path = b"uro:MooringFacility")]
    // MooringFacility(MooringFacility),
    // TODO:
    // #[citygml(path = b"uro:NavigationAssistanceFacility")]
    // NavigationAssistanceFacility(NavigationAssistanceFacility),
    // TODO:
    // #[citygml(path = b"uro:PortAttribute")]
    // PortAttribute(PortAttribute),
    // TODO:
    // #[citygml(path = b"uro:PortEnvironmentalImprovementFacility")]
    // PortEnvironmentalImprovementFacility(PortEnvironmentalImprovementFacility),
    // TODO:
    // #[citygml(path = b"uro:PortManagementFacility")]
    // PortManagementFacility(PortManagementFacility),
    // TODO:
    // #[citygml(path = b"uro:PortPassengerFacility")]
    // PortPassengerFacility(PortPassengerFacility),
    // TODO:
    // #[citygml(path = b"uro:PortPollutionControlFacility")]
    // PortPollutionControlFacility(PortPollutionControlFacility),
    // TODO:
    // #[citygml(path = b"uro:PortProtectiveFacility")]
    // PortProtectiveFacility(PortProtectiveFacility),
    // TODO:
    // #[citygml(path = b"uro:PortStorageFacility")]
    // PortStorageFacility(PortStorageFacility),
    // TODO:
    // #[citygml(path = b"uro:PortTransportationFacility")]
    // PortTransportationFacility(PortTransportationFacility),
    // TODO:
    // #[citygml(path = b"uro:PortWasteTreatmentFacility")]
    // PortWasteTreatmentFacility(PortWasteTreatmentFacility),
    // TODO:
    // #[citygml(path = b"uro:PortWelfareFacility")]
    // PortWelfareFacility(PortWelfareFacility),
    // TODO:
    // #[citygml(path = b"uro:ShipServiceFacility")]
    // ShipServiceFacility(ShipServiceFacility),
}


#[citygml_data(name = "uro:RiverFacilityIdAttribute")]
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
    pub river_code: Code,

    #[citygml(path = b"uro:riverName")]
    pub river_name: Option<String>,

    #[citygml(path = b"uro:sideType")]
    pub side_type: Code,

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

    #[citygml(path = b"uro:leftEndPost")]
    pub left_end_post: Option<Measure>,

    #[citygml(path = b"uro:lefEndDistance")]
    pub lef_end_distance: Option<Measure>,

    #[citygml(path = b"uro:rightStartPost")]
    pub right_start_post: Option<Measure>,

    #[citygml(path = b"uro:rightStartDistance")]
    pub right_start_distance: Option<Measure>,

    #[citygml(path = b"uro:rightEndPost")]
    pub right_end_post: Option<Measure>,

    #[citygml(path = b"uro:rightEndDistance")]
    pub right_end_distance: Option<Measure>,
}

#[citygml_data(name = "uro:CargoHandlingFacility")]
pub struct CargoHandlingFacility {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:portFacilityDetailsType")]
    pub port_facility_details_type: Code,

    #[citygml(path = b"uro:portName")]
    pub port_name: String,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<u64>,

    #[citygml(path = b"uro:mainCargo")]
    pub main_cargo: Option<Code>,

    #[citygml(path = b"uro:mooringFacility")]
    pub mooring_facility: Option<String>,

    #[citygml(path = b"uro:liftableLoad")]
    pub liftable_load: Option<Measure>,

    #[citygml(path = b"uro:ability")]
    pub ability: Option<u64>,

    #[citygml(path = b"uro:packingName")]
    pub packing_name: Option<Code>,

    #[citygml(path = b"uro:acquisitionYear")]
    pub acquisition_year: Option<String>,

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
    pub total_cost: Option<u64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:CyberportMarinaAndPBS")]
pub struct CyberportMarinaAndPBS{
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:facilityDetailType")]
    pub facility_details_type: Code,

    #[citygml(path = b"uro:portName")]
    pub port_name: String,

    #[citygml(path = b"uro:portStatus")]
    pub port_status: Option<Code>,

    #[citygml(path = b"uro:district")]
    pub district: Option<String>,

    #[citygml(path = b"uro:grantType")]
    pub grant_type: Option<Code>,

    #[citygml(path = b"uro:isDesignated")]
    pub is_designated: Option<bool>,

    #[citygml(path = b"uro:degradationLevel")]
    pub degradation_level: Option<u64>,

    #[citygml(path = b"uro:geologicalType")]
    pub geological_type: Option<Code>,

    #[citygml(path = b"uro:obstructingStructures")]
    pub obstructing_structures: Option<String>,

    #[citygml(path = b"uro:mainPartLength")]
    pub main_part_length: Option<Measure>,

    #[citygml(path = b"uro:totalLength")]
    pub total_length: Option<Measure>,

    #[citygml(path = b"uro:waveDissipatorLength")]
    pub wave_dissipator_length: Option<Measure>,

    #[citygml(path = b"uro:facilityWidth")]
    pub facility_width: Option<Measure>,

    #[citygml(path = b"uro:apronWidth")]
    pub apron_width: Option<Measure>,

    #[citygml(path = b"uro:restrictionStructure")]
    pub restriction_structure: Option<String>,

    #[citygml(path = b"uro:plannedDepth")]
    pub planned_depth: Option<Measure>,

    #[citygml(path = b"uro:currentDepth")]
    pub current_depth: Option<Measure>,

    #[citygml(path = b"uro:innerTotalFloorArea")]
    pub inner_total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:innerOfSiteArea")]
    pub inner_of_site_area: Option<Measure>,

    #[citygml(path = b"uro:outerOfTotalFloorArea")]
    pub outer_of_total_floor_area: Option<Measure>,

    #[citygml(path = b"uro:outerSiteArea")]
    pub outer_site_area: Option<Measure>,

    #[citygml(path = b"uro:ceilingHeight")]
    pub ceiling_height: Option<Measure>,

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
    pub number_of_mooring_posts: Option<u64>,

    #[citygml(path = b"uro:resistantMaterial")]
    pub resistant_material: Option<u64>,

    #[citygml(path = b"uro:lighting")]
    pub lighting: Option<u64>,

    #[citygml(path = b"uro:stairs")]
    pub stairs: Option<u64>,

    #[citygml(path = b"uro:lifesaving")]
    pub lifesaving: Option<String>,

    #[citygml(path = b"uro:lifesavingNumber")]
    pub lifesaving_number: Option<u64>,

    #[citygml(path = b"uro:bumper")]
    pub bumper: Option<Measure>,

    #[citygml(path = b"uro:numberOfVehicleBoardings")]
    pub number_of_vehicle_boardings: Option<u64>,

    #[citygml(path = b"uro:vehicleBoardingWidth")]
    pub vehicle_boarding_width: Option<Measure>,

    #[citygml(path = b"uro:shipType")]
    pub ship_type: Option<String>,

    #[citygml(path = b"uro:numberOfSeats")]
    pub number_of_seats: Option<u64>,

    #[citygml(path = b"uro:mainCargo")]
    pub main_cargo: Option<Code>,

    #[citygml(path = b"uro:storageCapacity")]
    pub storage_capacity: Option<u64>,

    #[citygml(path = b"uro:storageCapacityUnit")]
    pub storage_capacity_unit: Option<Code>,

    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:mainMaterial")]
    pub main_material: Option<Code>,

    #[citygml(path = b"uro:totalCost")]
    pub total_cost: Option<u64>,

    #[citygml(path = b"uro:subsidy")]
    pub subsidy: Option<u64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:FishingPortFacilityAttribute")]
pub struct FishingPortFacilityAttribute {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:facilityDetailsType")]
    pub facility_details_type: Code,

    #[citygml(path = b"uro:portName")]
    pub port_name: String,

    #[citygml(path = b"uro:portTpye")]
    pub port_type: Code,

    #[citygml(path = b"uro:address")]
    pub address: String,

    #[citygml(path = b"uro:designatedArea")]
    pub designated_area: String,

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
    pub length: Option<Measure>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Measure>,

    #[citygml(path = b"uro:ceilingHeight")]
    pub ceiling_height: Option<Measure>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Measure>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:otherSizeDescription")]
    pub other_size_description: Option<String>,

    #[citygml(path = b"uro:dateOfConstructionOrAcquisition")]
    pub date_of_construction_or_acquisition: Option<Date>,

    #[citygml(path = b"uro:cost")]
    pub cost: Option<u64>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:FishingPortCapacityAttribute")]
pub struct FishingPortCapacityAttribute {
    #[citygml(path = b"uro:facilityId")]
    pub facility_id: Option<String>,

    #[citygml(path = b"uro:capacity")]
    pub capacity: Option<String>,

    #[citygml(path = b"uro:weightCapacity")]
    pub weight_capacity: Option<Measure>,

    #[citygml(path = b"uro:hullForm")]
    pub hull_form: Option<u64>,

    #[citygml(path = b"uro:shipNumber")]
    pub ship_number: Option<u64>,

    #[citygml(path = b"uro:waterDepth-2m")]
    pub water_depth_2m: Option<Measure>,

    #[citygml(path = b"uro:waterDepth2-3m")]
    pub water_depth_2_3m: Option<Measure>,

    #[citygml(path = b"uro:waterDepth3-6m")]
    pub water_depth_3_6m: Option<Measure>,

    #[citygml(path = b"uro:waterDepth6-m")]
    pub water_depth_6_m: Option<Measure>,

    #[citygml(path = b"uro:heightAboveAWL")]
    pub height_above_awl: Option<Measure>,

    #[citygml(path = b"uro:heightOnFoundations")]
    pub height_on_foundations: Option<Measure>,

    #[citygml(path = b"uro:luminousRange")]
    pub luminous_range: Option<Measure>,

    #[citygml(path = b"uro:luminousColor")]
    pub luminous_color: Option<String>,

    #[citygml(path = b"uro:candlePower")]
    pub candle_power: Option<u64>,

    #[citygml(path = b"uro:lightType")]
    pub light_type: Option<String>,

    #[citygml(path = b"uro:period")]
    pub period: Option<String>,

    #[citygml(path = b"uro:maximumGroundingWeight")]
    pub maximum_grounding_weight: Option<u64>,

    #[citygml(path = b"uro:handleablePower")]
    pub handleable_power: Option<u64>,

    #[citygml(path = b"uro:maximumWaterSupply")]
    pub maximum_water_supply: Option<u64>,

    #[citygml(path = b"uro:maximumRefueling")]
    pub maximum_refueling: Option<String>,

    #[citygml(path = b"uro:people")]
    pub people: Option<u64>,

    #[citygml(path = b"uro:other")]
    pub other: Option<String>,

}