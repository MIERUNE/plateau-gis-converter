
use super::iur::uro;
use citygml::{CityGMLElement, Date, GeometryRef, Code};


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Road {
    #[citygml(geom = b"tran")]
    geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"core:creationDate")]
    creation_date: Option<Date>,

    #[citygml(path = b"core:terminationDate")]
    termination_date: Option<Date>,

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

    #[citygml(path = b"uro:tranDmAttribute")]
    pub tran_dm_attribute: Vec<uro::DmAttributeProperty>,

    #[citygml(path = b"uro:tranFacilityTypeAttribute")]
    pub tran_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:tranFacilityIdAttribute")]
    pub tran_facility_id_attribute: Option<uro::FacilityIdAttribute>,

    #[citygml(path = b"uro:tranFacilityAttribute")]
    pub tran_facility_attribute: Vec<uro::FacilityAttributeProperty>,

    #[citygml(path = b"uro:tranDataQualityAttribute/uro:TranDataQualityAttribute")]
    pub tran_data_quality_attribute: Option<uro::TransportationDataQualityAttribute>,

    #[citygml(path = b"uro:roadStructureAttribute/uro:RoadStructureAttribute")]
    pub road_structure_attribute: Vec<uro::RoadStructureAttribute>,

    #[citygml(path = b"uro:trafficVolumeAttribute/uro:TrafficVolumeAttribute")]
    pub traffic_volume_attribute: Vec<uro::TrafficVolumeAttribute>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Railway {
#[citygml(geom = b"tran")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Track {
#[citygml(geom = b"tran")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Square {
#[citygml(geom = b"tran")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

/// uro:Waterway (PLATEAU, CityGML 2.x)
/// tran:Waterway (CityGML 3.x)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct Waterway {
#[citygml(geom = b"tran")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tran:trafficArea/tran:TrafficArea")]
    traffic_area: Vec<TrafficArea>,

    #[citygml(path = b"tran:auxiliaryTrafficArea/tran:AuxiliaryTrafficArea")]
    auxiliary_traffic_area: Vec<AuxiliaryTrafficArea>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct TrafficArea {
    #[citygml(geom = b"tran")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"gml:name")]
    name: Option<String>,

    #[citygml(path = b"core:creationDate")]
    creation_date: Option<Date>,

    #[citygml(path = b"core:terminationDate")]
    termination_date: Option<Date>,

    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tran:surfaceMaterial")]
    pub surface_material: Option<Code>,

    #[citygml(path = b"uro:tranDmAttribute")]
    pub tran_dm_attribute: Vec<uro::DmAttributeProperty>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct AuxiliaryTrafficArea {
    #[citygml(geom = b"tran")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"gml:name")]
    name: Option<String>,

    #[citygml(path = b"core:creationDate")]
    creation_date: Option<Date>,

    #[citygml(path = b"core:terminationDate")]
    termination_date: Option<Date>,

    #[citygml(path = b"tran:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tran:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"uro:tranDmAttribute")]
    pub tran_dm_attribute: Vec<uro::DmAttributeProperty>,
}
