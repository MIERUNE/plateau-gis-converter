use citygml::CityGMLElement;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Road {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Railway {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Track {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Square {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

/// uro:Waterway (PLATEAU, CityGML 2.x)
/// tran:Waterway (CityGML 3.x)
#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct Waterway {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct TrafficArea {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}

#[derive(Default, Debug, CityGMLElement, Deserialize, Serialize)]
pub struct AuxiliaryTrafficArea {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
