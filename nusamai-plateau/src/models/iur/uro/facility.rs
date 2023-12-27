use citygml::{citygml_property,citygml_data,CityGMLElement,Code,Measure};

#[citygml_property(name = "uro:FacilityAttributeProperty")]
pub enum FacilityAttributeProperty {
    #[citygml(path = b"uro:RiverFacilityIdAttribute")]
    RiverFacilityIdAttribute(RiverFacilityIdAttribute),

    #[citygml(path = b"uro:CargoHandlingFacility")]
    CargoHandlingFacility(CargoHandlingFacility),
    // TODO:
    // #[citygml(path = b"uro:CyberportMarinaAndPBS")]
    // CyberportMarinaAndPBS(CyberportMarinaAndPBS),
    // TODO:
    // #[citygml(path = b"uro:FishingPortAttribute")]
    // FishingPortAttribute(FishingPortAttribute),
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