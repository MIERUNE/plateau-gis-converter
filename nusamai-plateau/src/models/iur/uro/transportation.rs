use nusamai_citygml::{
    citygml_data, citygml_property, CityGmlElement, Code, Date, GYear, Length, Measure, Point,
};

#[citygml_data(name = "uro:RoadStructureAttribute")]
pub struct RoadStructureAttribute {
    #[citygml(path = b"uro:widthType")]
    pub width_type: Option<Code>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:numberOfLanes")]
    pub number_of_lanes: Option<i64>,

    #[citygml(path = b"uro:sectionType")]
    pub section_type: Option<Code>,
}

#[citygml_data(name = "uro:TrafficVolumeAttribute")]
pub struct TrafficVolumeAttribute {
    #[citygml(path = b"uro:sectionID")]
    pub section_id: Option<String>,

    #[citygml(path = b"uro:routeName")]
    pub route_name: Option<String>,

    #[citygml(path = b"uro:weekday12hourTrafficVolume")]
    pub weekday12hour_traffic_volume: Option<i64>,

    #[citygml(path = b"uro:weekday24hourTrafficVolume")]
    pub weekday24hour_traffic_volume: Option<i64>,

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
    pub survey_year: Option<GYear>,
}

#[citygml_data(name = "uro:RoadType")]
pub struct RoadType {
    #[citygml(path = b"uro:id")]
    pub id: Option<String>,

    #[citygml(path = b"uro:creationDate")]
    pub creation_date: Option<Date>,

    #[citygml(path = b"uro:isTemporary")]
    pub is_temporary: Option<bool>,

    #[citygml(path = b"uro:roadType")]
    pub road_type: Option<Code>,

    #[citygml(path = b"uro:widthType")]
    pub width_type: Option<Code>,

    #[citygml(path = b"uro:isTollRoad")]
    pub is_toll_road: Option<bool>,

    #[citygml(path = b"uro:separator")]
    pub separator: Option<Length>,

    #[citygml(path = b"uro:isHighWay")]
    pub is_high_way: Option<bool>,
}

#[citygml_data(name = "uro:RailwayTrackAttribute")]
pub struct RailwayTrackAttribute {
    #[citygml(geom = b"uro")]
    geometries: nusamai_citygml::GeometryRefs,

    #[citygml(path = b"uro:routeName")]
    pub route_name: Option<String>,

    #[citygml(path = b"uro:directionType")]
    pub direction_type: Option<Code>,

    #[citygml(path = b"uro:trackType")]
    pub track_type: Option<Code>,

    #[citygml(path = b"uro:startPost")]
    pub start_post: Option<String>,

    #[citygml(path = b"uro:endPost")]
    pub end_post: Option<String>,

    #[citygml(path = b"uro:alignmentType")]
    pub alignment_type: Option<Code>,

    #[citygml(path = b"uro:controlPoint/uro:ControlPoint")]
    pub control_point: Vec<ControlPoint>,
}

#[citygml_data(name = "uro:TrafficAreaStructureAttribute")]
pub struct TrafficAreaStructureAttribute {
    #[citygml(path = b"uro:numberOfLanes")]
    pub number_of_lanes: Option<i64>,
}

#[citygml_data(name = "uro:ControlPoint")]
pub struct ControlPoint {
    #[citygml(path = b"uro:startPost")]
    pub start_post: Option<String>,

    #[citygml(path = b"uro:endPost")]
    pub end_post: Option<String>,

    #[citygml(path = b"uro:function", required)]
    pub function: Option<Code>,

    #[citygml(path = b"uro:parameter", required)]
    pub parameter: Option<ControlPointType>,

    #[citygml(path = b"uro:startPoint/gml:Point")]
    pub start_point: Option<Point>,

    #[citygml(path = b"uro:endPoint/gml:Point")]
    pub end_point: Option<Point>,
}

#[citygml_property(name = "uro:ControlPointType")]
pub enum ControlPointType {
    #[citygml(path = b"uro:circularCurve/uro:CircularCurveType")]
    CircularCurveType(CircularCurveType),
    #[citygml(path = b"uro:transitionCurve/uro:TransitionCurveType")]
    TransitionCurveType(TransitionCurveType),
    #[citygml(path = b"uro:slopeType/uro:SlopeType")]
    SlopeType(SlopeType),
    #[citygml(path = b"uro:verticalCurve/uro:VerticalCurveType")]
    VerticalCurveType(VerticalCurveType),
}

#[citygml_data(name = "uro:CircularCurveType")]
pub struct CircularCurveType {
    #[citygml(path = b"uro:radius", required)]
    pub radius: Option<Length>,

    #[citygml(path = b"uro:intersection", required)]
    pub intersection: Option<f64>,

    #[citygml(path = b"uro:cutLength", required)]
    pub cut_length: Option<Length>,

    #[citygml(path = b"uro:curveLength", required)]
    pub curve_length: Option<Length>,
}

#[citygml_data(name = "uro:TransitionCurveType")]
pub struct TransitionCurveType {
    #[citygml(path = b"uro:intersection", required)]
    pub intersection: Option<Length>,

    #[citygml(path = b"uro:distance", required)]
    pub distance: Option<Length>,

    #[citygml(path = b"uro:curveLength", required)]
    pub curve_length: Option<Length>,
}

#[citygml_data(name = "uro:SlopeType")]
pub struct SlopeType {
    #[citygml(path = b"uro:angle", required)]
    pub angle: Option<f64>,

    #[citygml(path = b"uro:elevation", required)]
    pub elevation: Option<Length>,
}

#[citygml_data(name = "uro:VerticalCurveType")]
pub struct VerticalCurveType {
    #[citygml(path = b"uro:length", required)]
    pub length: Option<Length>,

    #[citygml(path = b"uro:verticalDistance", required)]
    pub vertical_distance: Option<Length>,
}

#[citygml_data(name = "uro:TrackAttribute")]
pub struct TrackAttribute {
    #[citygml(path = b"uro:alternativeName")]
    pub alternative_name: Vec<String>,

    #[citygml(path = b"uro:adminType")]
    pub admin_type: Option<Code>,

    #[citygml(path = b"uro:relativeLevel")]
    pub relative_level: Option<i64>,

    #[citygml(path = b"uro:widthType")]
    pub width_type: Option<Code>,

    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:isTollRoad")]
    pub is_toll_road: Option<bool>,

    #[citygml(path = b"uro:separator")]
    pub separator: Option<Length>,
}

#[citygml_data(name = "uro:RailwayRouteAttribute")]
pub struct RailwayRouteAttribute {
    #[citygml(path = b"uro:operatorType", required)]
    pub operator_type: Option<Code>,

    #[citygml(path = b"uro:operator", required)]
    pub operator: Option<String>,

    #[citygml(path = b"uro:alternativeName")]
    pub alternative_name: Vec<String>,

    #[citygml(path = b"uro:railwayType", required)]
    pub railway_type: Option<Code>,

    #[citygml(path = b"uro:startStation", required)]
    pub start_station: Option<String>,

    #[citygml(path = b"uro:endStation", required)]
    pub end_station: Option<String>,
}

#[citygml_property(name = "uro:SquareUrbanPlanAttributeProperty")]
pub enum SquareUrbanPlanAttributeProperty {
    #[citygml(path = b"uro:SquareUrbanPlanAttribute")]
    SquareUrbanPlanAttribute(SquareUrbanPlanAttribute),
    #[citygml(path = b"uro:StationSquareAttribute")]
    StationSquareAttribute(StationSquareAttribute),
    #[citygml(path = b"uro:TerminalAttribute")]
    TerminalAttribute(TerminalAttribute),
}

#[citygml_data(name = "uro:SquareUrbanPlanAttribute")]
pub struct SquareUrbanPlanAttribute {
    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"uro:city")]
    pub city: Option<Code>,

    #[citygml(path = b"uro:urbanPlanningAreaName")]
    pub urban_planning_area_name: Option<String>,

    #[citygml(path = b"uro:enforcer")]
    pub enforcer: Vec<String>,

    #[citygml(path = b"uro:dateOfDecision")]
    pub date_of_decision: Option<Date>,

    #[citygml(path = b"uro:dateOfRevision")]
    pub date_of_revision: Vec<Date>,

    #[citygml(path = b"uro:areaPlanned")]
    pub area_planned: Option<Measure>,

    #[citygml(path = b"uro:areaInService")]
    pub area_in_service: Option<Measure>,

    #[citygml(path = b"uro:remarks")]
    pub remarks: Option<String>,

    #[citygml(path = b"uro:status")]
    pub status: Option<Code>,

    #[citygml(path = b"uro:areaImproved")]
    pub area_improved: Option<Measure>,

    #[citygml(path = b"uro:areaCompleted")]
    pub area_completed: Option<Measure>,

    #[citygml(path = b"uro:projectStartDate")]
    pub project_start_date: Option<Date>,

    #[citygml(path = b"uro:projectEndDate")]
    pub project_end_date: Option<Date>,

    #[citygml(path = b"uro:isCompleted")]
    pub is_completed: Option<bool>,

    #[citygml(path = b"uro:isAuthorized")]
    pub is_authorized: Option<bool>,

    #[citygml(path = b"uro:purpose")]
    pub purpose: Option<String>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,
}

#[citygml_data(name = "uro:StationSquareAttribute")]
pub struct StationSquareAttribute {
    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"uro:city")]
    pub city: Option<Code>,

    #[citygml(path = b"uro:urbanPlanningAreaName")]
    pub urban_planning_area_name: Option<String>,

    #[citygml(path = b"uro:enforcer")]
    pub enforcer: Vec<String>,

    #[citygml(path = b"uro:dateOfDecision")]
    pub date_of_decision: Option<Date>,

    #[citygml(path = b"uro:dateOfRevision")]
    pub date_of_revision: Vec<Date>,

    #[citygml(path = b"uro:areaPlanned")]
    pub area_planned: Option<Measure>,

    #[citygml(path = b"uro:areaInService")]
    pub area_in_service: Option<Measure>,

    #[citygml(path = b"uro:remarks")]
    pub remarks: Option<String>,

    #[citygml(path = b"uro:status")]
    pub status: Option<Code>,

    #[citygml(path = b"uro:areaImproved")]
    pub area_improved: Option<Measure>,

    #[citygml(path = b"uro:areaCompleted")]
    pub area_completed: Option<Measure>,

    #[citygml(path = b"uro:projectStartDate")]
    pub project_start_date: Option<Date>,

    #[citygml(path = b"uro:projectEndDate")]
    pub project_end_date: Option<Date>,

    #[citygml(path = b"uro:isCompleted")]
    pub is_completed: Option<bool>,

    #[citygml(path = b"uro:isAuthorized")]
    pub is_authorized: Option<bool>,

    #[citygml(path = b"uro:purpose")]
    pub purpose: Option<String>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,

    #[citygml(path = b"uro:station")]
    pub station: Vec<String>,

    #[citygml(path = b"uro:route")]
    pub route: Vec<String>,

    #[citygml(path = b"uro:railwayType")]
    pub railway_type: Vec<Code>,
}

#[citygml_data(name = "uro:TerminalAttribute")]
pub struct TerminalAttribute {
    #[citygml(path = b"uro:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"uro:city")]
    pub city: Option<Code>,

    #[citygml(path = b"uro:urbanPlanningAreaName")]
    pub urban_planning_area_name: Option<String>,

    #[citygml(path = b"uro:enforcer")]
    pub enforcer: Vec<String>,

    #[citygml(path = b"uro:dateOfDecision")]
    pub date_of_decision: Option<Date>,

    #[citygml(path = b"uro:dateOfRevision")]
    pub date_of_revision: Vec<Date>,

    #[citygml(path = b"uro:areaPlanned")]
    pub area_planned: Option<Measure>,

    #[citygml(path = b"uro:areaInService")]
    pub area_in_service: Option<Measure>,

    #[citygml(path = b"uro:remarks")]
    pub remarks: Option<String>,

    #[citygml(path = b"uro:status")]
    pub status: Option<Code>,

    #[citygml(path = b"uro:areaImproved")]
    pub area_improved: Option<Measure>,

    #[citygml(path = b"uro:areaCompleted")]
    pub area_completed: Option<Measure>,

    #[citygml(path = b"uro:projectStartDate")]
    pub project_start_date: Option<Date>,

    #[citygml(path = b"uro:projectEndDate")]
    pub project_end_date: Option<Date>,

    #[citygml(path = b"uro:isCompleted")]
    pub is_completed: Option<bool>,

    #[citygml(path = b"uro:isAuthorized")]
    pub is_authorized: Option<bool>,

    #[citygml(path = b"uro:purpose")]
    pub purpose: Option<String>,

    #[citygml(path = b"uro:note")]
    pub note: Option<String>,

    #[citygml(path = b"uro:terminalType")]
    pub terminal_type: Option<Code>,

    #[citygml(path = b"uro:structure")]
    pub structure: Option<String>,

    #[citygml(path = b"uro:numberOfBerthsPlanned")]
    pub number_of_berths_planned: Option<i64>,

    #[citygml(path = b"uro:numberOfBerthsInService")]
    pub number_of_berths_in_service: Option<i64>,

    #[citygml(path = b"uro:userType")]
    pub user_type: Option<Code>,
}

#[citygml_data(name = "uro:WaterwayDetailAttribute")]
pub struct WaterwayDetailAttribute {
    #[citygml(path = b"uro:routeId", required)]
    pub route_id: Option<String>,

    #[citygml(path = b"uro:routeDirection")]
    pub route_direction: Option<Code>,

    #[citygml(path = b"uro:minimumWidth")]
    pub minimum_width: Option<Length>,

    #[citygml(path = b"uro:maximumWidth")]
    pub maximum_width: Option<Length>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:navigation")]
    pub navigation: Option<String>,

    #[citygml(path = b"uro:plannedDepth")]
    pub planned_depth: Option<Length>,

    #[citygml(path = b"uro:speedLimit")]
    pub speed_limit: Option<Measure>,

    #[citygml(path = b"uro:targetShipType")]
    pub target_ship_type: Vec<String>,
}

#[citygml_data(name = "uro:TrafficObjectUsecaseAttribute")]
pub struct TrafficObjectUsecaseAttribute {
    #[citygml(path = b"uro:minWidth")]
    pub min_width: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,
}
