use nusamai_citygml::{citygml_feature, CityGmlElement, Code};

use super::iur::uro;

#[citygml_feature(name = "frn:CityFurniture")]
pub struct CityFurniture {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<uro::CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,
}
