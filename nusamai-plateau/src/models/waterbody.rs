use nusamai_citygml::{citygml_feature, citygml_property, CityGmlElement, Code};

use super::iur::uro;

#[citygml_feature(name = "wtr:WaterBody")]
pub struct WaterBody {
    #[citygml(path = b"wtr:class")]
    pub class: Option<Code>,

    #[citygml(path = b"wtr:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"wtr:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"wtr:boundedBy")]
    pub bounded_by: Vec<WaterBoundarySurfaceProperty>, // -> wtr:_WaterBoundarySurface

    #[citygml(path = b"uro:floodingRiskAttribute")]
    pub flooding_risk_attribute: Vec<uro::FloodingRiskAttributeProperty>, // -> uro:FloodingRiskAttribute

    #[citygml(path = b"uro:waterBodyDetailAttribute/uro:WaterBodyDetailAttribute")]
    pub water_body_detail_attribute: Vec<uro::WaterBodyDetailAttribute>,

    #[citygml(path = b"uro:wtrDataQualityAttribute/uro:DataQualityAttribute")]
    pub wtr_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:wtrDmAttribute")]
    pub wtr_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:wtrFacilityAttribute")]
    pub wtr_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:wtrFacilityIdAttribute")]
    pub wtr_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:wtrFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub wtr_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:wtrKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub wtr_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,
}

#[citygml_property(name = "wtr:_WaterBoundarySurfaceProperty")]
pub enum WaterBoundarySurfaceProperty {
    #[citygml(path = b"wtr:WaterClosureSurface")]
    WaterClosureSurface(WaterClosureSurface),
    #[citygml(path = b"wtr:WaterGroundSurface")]
    WaterGroundSurface(WaterGroundSurface),
    #[citygml(path = b"wtr:WaterSurface")]
    WaterSurface(WaterSurface),
}

#[citygml_feature(name = "wtr:WaterClosureSurface")]
pub struct WaterClosureSurface {}

#[citygml_feature(name = "wtr:WaterGroundSurface")]
pub struct WaterGroundSurface {}

#[citygml_feature(name = "wtr:WaterSurface")]
pub struct WaterSurface {
    #[citygml(path = b"wtr:waterLevel")]
    pub water_level: Option<Code>,
}
