use nusamai_citygml::{citygml_feature, CityGmlElement};

use super::common::DataQualityAttribute;

#[citygml_feature(name = "uro:demDataQualityAttribute")]
pub struct DemDataQualityAttribute {
    #[citygml(path = b"uro:DataQualityAttribute", required)]
    pub data_quality_attribute: Option<DataQualityAttribute>,
}
