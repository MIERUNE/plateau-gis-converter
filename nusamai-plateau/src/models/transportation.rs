use citygml::{citygml_feature, CityGMLElement};

#[citygml_feature(name = "tran:Road")]
pub struct Road {
    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[citygml_feature(name = "tran:Railway")]
pub struct Railway {
    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[citygml_feature(name = "tran:Track")]
pub struct Track {
    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[citygml_feature(name = "tran:Square")]
pub struct Square {
    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

/// uro:Waterway (PLATEAU, CityGML 2.x)
/// tran:Waterway (CityGML 3.x)
#[citygml_feature(name = "uro:Waterway")]
pub struct Waterway {
    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[citygml_feature(name = "tran:TrafficArea")]
pub struct TrafficArea {
    // ...
}

#[citygml_feature(name = "tran:AuxiliaryTrafficArea")]
pub struct AuxiliaryTrafficArea {
    // ...
}
