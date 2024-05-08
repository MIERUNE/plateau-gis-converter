use nusamai_citygml::{citygml_feature, CityGmlElement, Code};

use super::iur::uro;

#[citygml_feature(name = "luse:LandUse")]
pub struct LandUse {
    #[citygml(path = b"luse:class")]
    pub class: Option<Code>,

    #[citygml(path = b"luse:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"luse:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:ifcLandUseAttribute")]
    pub ifc_land_use_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:landUseDetailAttribute/uro:LandUseDetailAttribute")]
    pub land_use_detail_attribute: Vec<uro::LandUseDetailAttribute>,

    #[citygml(path = b"uro:luseDataQualityAttribute/uro:LandUseDataQualityAttribute")]
    #[citygml(path = b"uro:luseDataQualityAttribute/uro:DataQualityAttribute")]
    pub luse_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:luseDmAttribute")]
    pub luse_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:luseFacilityAttribute")]
    pub luse_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:luseFacilityIdAttribute")]
    pub luse_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:luseFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub luse_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:luseKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub luse_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,
}
