use nusamai_citygml::{citygml_data, CityGMLElement, Code};

#[citygml_data(name = "uro:TransportationDataQualityAttribute")]
pub struct TransportationDataQualityAttribute {
    #[citygml(path = b"uro:srcScale")]
    pub src_scale: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc")]
    pub geometry_src_desc: Vec<Code>,

    #[citygml(path = b"uro:thematicSrcDesc")]
    pub thematic_src_desc: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDesc")]
    pub appearance_src_desc: Vec<Code>,

    #[citygml(path = b"uro:lodType")]
    pub lod_type: Option<String>, // TODO: uro:CityFurnitureLODType(enumerations)
}
#[citygml_data(name = "uro:RoadStructureAttribute")]
pub struct RoadStructureAttribute {
    #[citygml(path = b"uro:widthType")]
    pub width_type: Option<Code>,

    #[citygml(path = b"uro:width")]
    pub width: Option<f64>,

    #[citygml(path = b"uro:numberOfLanes")]
    pub number_of_lanes: Option<u64>,

    #[citygml(path = b"uro:sectionType")]
    pub section_type: Option<Code>,
}

#[citygml_data(name = "uro:TrafficVolumeAttribute")]
pub struct TrafficVolumeAttribute {
    #[citygml(path = b"uro:sectionID")]
    pub section_type: Option<String>,

    #[citygml(path = b"uro:routeName")]
    pub route_name: Option<String>,

    #[citygml(path = b"uro:weekday12hourTrafficVolume")]
    pub weekday12hour_traffic_volume: Option<u64>,

    #[citygml(path = b"uro:weekday24hourTrafficVolume")]
    pub weekday24hour_traffic_volume: Option<u64>,

    #[citygml(path = b"uro:largeVehicleRate")]
    pub large_vehicle_rate: Option<f64>,

    #[citygml(path = b"uro:congestionRate")]
    pub congestion_rate: Option<f64>,

    #[citygml(path = b"uro:averageTravelSpeedInCongestion")]
    pub average_travel_speed_in_congestion: Option<f64>,

    #[citygml(path = b"uro:averageInboundTravelSpeedInCongestion")]
    pub average_inbound_travel_speed_in_congestion: Option<f64>,

    #[citygml(path = b"uro:averageOutboundTravelSpeedInCongestion")]
    pub average_outbound_travel_speed_in_congestion: Option<f64>,

    #[citygml(path = b"uro:averageInboundTravelSpeedNotCongestion")]
    pub average_inbound_travel_speed_not_congestion: Option<f64>,

    #[citygml(path = b"uro:averageOutboundTravelSpeedNotCongestion")]
    pub average_outbound_travel_speed_not_congestion: Option<f64>,

    #[citygml(path = b"uro:observationPointName")]
    pub observation_point_name: Option<String>,

    #[citygml(path = b"uro:reference")]
    pub reference: Option<String>,

    #[citygml(path = b"uro:surveyYear")]
    pub survey_year: Option<String>, // TODO: xs:gYear
}
