use nusamai_citygml::{citygml_data, CityGMLElement, Code, Date, Length, Measure, Point, URI};

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

#[citygml_data(name = "uro:Occupancy")]
pub struct Occupancy {
    #[citygml(path = b"uro:interval")]
    pub interval: Option<Code>,

    #[citygml(path = b"uro:numberofOccupants", required)]
    pub numberof_occupants: Option<i64>,

    #[citygml(path = b"uro:occupantType")]
    pub occupant_type: Option<Code>,
}

#[citygml_data(name = "uro:Elevation")]
pub struct Elevation {
    #[citygml(path = b"uro:elevationReference", required)]
    pub elevation_reference: Option<Code>,

    #[citygml(path = b"uro:elevationValue", required)]
    pub elevation_value: Option<Point>,
}

type HeightStatusValue = String;

#[citygml_data(name = "uro:Height")]
pub struct Height {
    #[citygml(path = b"uro:highReference", required)]
    pub high_reference: Option<Code>,

    #[citygml(path = b"uro:lowReference", required)]
    pub low_reference: Option<Code>,

    #[citygml(path = b"uro:status", required)]
    pub status: Option<HeightStatusValue>,

    #[citygml(path = b"uro:value", required)]
    pub value: Option<Length>,
}

#[citygml_data(name = "uro:ConstructionEvent")]
pub struct ConstructionEvent {
    #[citygml(path = b"uro:event", required)]
    pub event: Option<Code>,

    #[citygml(path = b"uro:dateOfEvent", required)]
    pub date_of_event: Option<Date>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
}
