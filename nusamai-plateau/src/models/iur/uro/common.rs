use nusamai_citygml::{citygml_data, CityGMLElement, Code, Date, Measure, URI};

#[citygml_data(name = "uro:UserDefinedValue")]
pub struct UserDefinedValue {
    #[citygml(path = b"uro:stringValue")]
    pub string_value: Option<String>,

    #[citygml(path = b"uro:intValue")]
    pub int_value: Option<i64>,

    #[citygml(path = b"uro:doubleValue")]
    pub double_value: Option<f64>,

    #[citygml(path = b"uro:codeValue")]
    pub code_value: Option<Code>,

    #[citygml(path = b"uro:dateValue")]
    pub date_value: Option<Date>,

    #[citygml(path = b"uro:uriValue")]
    pub uri_value: Option<URI>,

    #[citygml(path = b"uro:measuredValue")]
    pub measured_value: Option<Measure>,
}
