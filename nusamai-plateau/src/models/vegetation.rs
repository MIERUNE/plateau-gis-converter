use nusamai_citygml::{citygml_feature, CityGmlElement, Code, Length};

use super::uro;

#[citygml_feature(name = "veg:SolitaryVegetationObject")]
pub struct SolitaryVegetationObject {
    #[citygml(path = b"uro:vegetationDataQualityAttribute/uro:VegetationDataQualityAttribute")]
    #[citygml(path = b"uro:vegDataQualityAttribute/uro:DataQualityAttribute")]
    pub veg_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:vegDmAttribute")]
    pub veg_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:vegFacilityAttribute")]
    pub veg_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:vegFacilityIdAttribute")]
    pub veg_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:vegFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub veg_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:vegKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub veg_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"veg:class")]
    pub class: Option<Code>,

    #[citygml(path = b"veg:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"veg:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"veg:species")]
    pub species: Option<Code>,

    #[citygml(path = b"veg:height")]
    pub height: Option<Length>,

    #[citygml(path = b"veg:trunkDiameter")]
    pub trunk_diameter: Option<Length>,

    #[citygml(path = b"veg:crownDiameter")]
    pub crown_diameter: Option<Length>,
}

#[citygml_feature(name = "veg:PlantCover")]
pub struct PlantCover {
    #[citygml(path = b"uro:vegetationDataQualityAttribute/uro:VegetationDataQualityAttribute")]
    #[citygml(path = b"uro:vegDataQualityAttribute/uro:DataQualityAttribute")]
    pub veg_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:vegDmAttribute")]
    pub veg_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:vegFacilityAttribute")]
    pub veg_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:vegFacilityIdAttribute")]
    pub veg_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:vegFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub veg_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:vegKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub veg_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"veg:class")]
    pub class: Option<Code>,

    #[citygml(path = b"veg:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"veg:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"veg:averageHeight")]
    pub average_height: Option<Length>,
}
