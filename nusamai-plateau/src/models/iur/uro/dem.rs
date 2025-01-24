use nusamai_citygml::{citygml_feature, CityGmlElement};

use super::common::DataQualityAttribute;

#[allow(non_camel_case_types)]
#[citygml_feature(name = "uro:demDataQualityAttribute")]
pub struct demDataQualityAttribute {
    #[citygml(path = b"uro:DataQualityAttribute", required)]
    pub data_quality_attribute: Option<DataQualityAttribute>,
}
