use nusamai_citygml::{citygml_feature, CityGmlElement, Code};

use super::iur::uro;

#[citygml_feature(name = "tran:Road")]
pub struct Road {
    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tran:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    pub traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    pub auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,

    #[citygml(path = b"uro:tranDataQualityAttribute/uro:TransportationDataQualityAttribute")]
    #[citygml(path = b"uro:roadDataQualityAttribute/uro:RoadDataQualityAttribute")]
    #[citygml(path = b"uro:tranDataQualityAttribute/uro:DataQualityAttribute")]
    pub tran_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:tranDmAttribute")]
    pub tran_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:tranFacilityAttribute")]
    pub tran_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:tranFacilityIdAttribute")]
    pub tran_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:tranFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub tran_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:tranKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub tran_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:tranUsecaseAttribute/uro:TrafficObjectUsecaseAttribute")]
    pub tran_usecase_attribute: Vec<uro::TrafficObjectUsecaseAttribute>,

    #[citygml(path = b"uro:roadStatus/uro:RoadType")]
    pub road_status: Vec<uro::RoadType>,

    #[citygml(path = b"uro:roadStructureAttribute/uro:RoadStructureAttribute")]
    pub road_structure_attribute: Vec<uro::RoadStructureAttribute>,

    #[citygml(path = b"uro:trafficVolumeAttribute/uro:TrafficVolumeAttribute")]
    pub traffic_volume_attribute: Vec<uro::TrafficVolumeAttribute>,
}

#[citygml_feature(name = "tran:Railway")]
pub struct Railway {
    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tran:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    pub traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    pub auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,

    #[citygml(path = b"uro:tranDataQualityAttribute/uro:TransportationDataQualityAttribute")]
    #[citygml(path = b"uro:tranDataQualityAttribute/uro:DataQualityAttribute")]
    pub tran_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:tranDmAttribute")]
    pub tran_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:tranFacilityAttribute")]
    pub tran_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:tranFacilityIdAttribute")]
    pub tran_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:tranFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub tran_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:tranKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub tran_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:tranUsecaseAttribute/uro:TrafficObjectUsecaseAttribute")]
    pub tran_usecase_attribute: Vec<uro::TrafficObjectUsecaseAttribute>,

    #[citygml(path = b"uro:railwayRouteAttribute/uro:RailwayRouteAttribute")]
    pub railway_route_attribute: Vec<uro::RailwayRouteAttribute>,
}

#[citygml_feature(name = "tran:Track")]
pub struct Track {
    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tran:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    pub traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    pub auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,

    #[citygml(path = b"uro:tranDataQualityAttribute/uro:TransportationDataQualityAttribute")]
    #[citygml(path = b"uro:tranDataQualityAttribute/uro:DataQualityAttribute")]
    pub tran_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:tranDmAttribute")]
    pub tran_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:tranFacilityAttribute")]
    pub tran_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:tranFacilityIdAttribute")]
    pub tran_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:tranFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub tran_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:tranKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub tran_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:tranUsecaseAttribute/uro:TrafficObjectUsecaseAttribute")]
    pub tran_usecase_attribute: Vec<uro::TrafficObjectUsecaseAttribute>,

    #[citygml(path = b"uro:trackAttribute/uro:TrackAttribute")]
    pub track_attribute: Vec<uro::TrackAttribute>,
}

#[citygml_feature(name = "tran:Square")]
pub struct Square {
    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tran:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    pub traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    pub auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,

    #[citygml(path = b"uro:tranDataQualityAttribute/uro:TransportationDataQualityAttribute")]
    #[citygml(path = b"uro:tranDataQualityAttribute/uro:DataQualityAttribute")]
    pub tran_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:tranDmAttribute")]
    pub tran_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:tranFacilityAttribute")]
    pub tran_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:tranFacilityIdAttribute")]
    pub tran_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:tranFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub tran_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:tranKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub tran_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:tranUsecaseAttribute/uro:TrafficObjectUsecaseAttribute")]
    pub tran_usecase_attribute: Vec<uro::TrafficObjectUsecaseAttribute>,

    #[citygml(path = b"uro:squareUrbanPlanAttribute")]
    pub square_urban_plan_attribute: Vec<uro::SquareUrbanPlanAttributeProperty>, // -> uro:SquareUrbanPlanAttribute
}

/// uro:Waterway (PLATEAU, CityGML 2.x)
/// tran:Waterway (CityGML 3.x)
#[citygml_feature(name = "uro:Waterway")]
pub struct Waterway {
    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tran:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    pub traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    pub auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,

    #[citygml(path = b"uro:tranDataQualityAttribute/uro:TransportationDataQualityAttribute")]
    #[citygml(path = b"uro:tranDataQualityAttribute/uro:DataQualityAttribute")]
    pub tran_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:tranDmAttribute")]
    pub tran_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:tranFacilityAttribute")]
    pub tran_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:tranFacilityIdAttribute")]
    pub tran_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:tranFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub tran_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:tranKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub tran_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:tranUsecaseAttribute/uro:TrafficObjectUsecaseAttribute")]
    pub tran_usecase_attribute: Vec<uro::TrafficObjectUsecaseAttribute>,

    #[citygml(path = b"uro:waterwayDetailAttribute/uro:WaterwayDetailAttribute")]
    pub waterway_detail_attribute: Option<uro::WaterwayDetailAttribute>,
}

#[citygml_feature(name = "tran:TrafficArea")]
pub struct TrafficArea {
    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tran:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tran:surfaceMaterial")]
    pub surface_material: Option<Code>,

    #[citygml(path = b"uro:railwayTrackAttribute/uro:RailwayTrackAttribute")]
    pub railway_track_attribute: Vec<uro::RailwayTrackAttribute>,

    #[citygml(path = b"uro:trafficAreaStructureAttribute/uro:TrafficAreaStructureAttribute")]
    pub traffic_area_structure_attribute: Vec<uro::TrafficAreaStructureAttribute>,
}

#[citygml_feature(name = "tran:AuxiliaryTrafficArea")]
pub struct AuxiliaryTrafficArea {
    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tran:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tran:surfaceMaterial")]
    pub surface_material: Option<Code>,
}
