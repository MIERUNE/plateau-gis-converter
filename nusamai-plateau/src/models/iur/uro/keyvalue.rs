use nusamai_citygml::{citygml_data, CityGmlElement, Code};

#[citygml_data(name = "uro:KeyValuePairAttribute")]
pub struct KeyValuePairAttribute {
    #[citygml(path = b"uro:key", required)]
    pub key: Option<Code>,
    #[citygml(path = b"uro:codeValue")]
    pub code_value: Option<Code>,
}
