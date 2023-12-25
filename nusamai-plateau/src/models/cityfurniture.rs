use super::iur::uro;
use citygml::{citygml_feature, CityGMLElement, Code};

#[citygml_feature(name = "frn:CityFurniture")]
pub struct CityFurniture {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<uro::CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    pub city_furniture_data_quality_attribute: Option<uro::CityFurnitureDataQualityAttribute>,

    #[citygml(path = b"uro:frnFacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<uro::FacilityIdAttribute>,

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<uro::FacilityAttributeProperty>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<uro::DmAttributeProperty>,
}
