use nusamai_citygml::{citygml_data, CityGmlElement, Code, Date, Length, Measure, Point, Uri};

#[citygml_data(name = "uro:DataQualityAttribute")]
#[citygml(allow_extra)]
pub struct DataQualityAttribute {
    // PLATEAU 3.x compatibility
    #[citygml(path = b"uro:srcScale")]
    pub src_scale: Vec<Code>,

    // PLATEAU 3.x compatibility
    #[citygml(path = b"uro:geometrySrcDesc")]
    pub geometry_src_desc: Vec<Code>,

    // PLATEAU 3.x compatibility
    #[citygml(path = b"uro:appearanceSrcDesc")]
    pub appearance_src_desc: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc0")]
    pub geometry_src_desc0: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc1")]
    pub geometry_src_desc1: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc2")]
    pub geometry_src_desc2: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc3")]
    pub geometry_src_desc3: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc4")]
    pub geometry_src_desc4: Vec<Code>,

    #[citygml(path = b"uro:thematicSrcDesc")]
    pub thematic_src_desc: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDescLod0")]
    pub appearance_src_desc_lod0: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDescLod1")]
    pub appearance_src_desc_lod1: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDescLod2")]
    pub appearance_src_desc_lod2: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDescLod3")]
    pub appearance_src_desc_lod3: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDescLod4")]
    pub appearance_src_desc_lod4: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDescLod0")]
    pub geometry_src_desc_lod0: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDescLod1")]
    pub geometry_src_desc_lod1: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDescLod2")]
    pub geometry_src_desc_lod2: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDescLod3")]
    pub geometry_src_desc_lod3: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDescLod4")]
    pub geometry_src_desc_lod4: Vec<Code>,

    #[citygml(path = b"uro:lodType")]
    pub lod_type: Vec<Code>,

    #[citygml(path = b"uro:lod1HeightType")]
    pub lod1_height_type: Option<Code>,

    #[citygml(path = b"uro:dataAcquisition")]
    pub data_acquisition: Option<String>,

    #[citygml(path = b"uro:tranDataAcquisition")]
    pub tran_data_acquisition: Option<String>,

    #[citygml(path = b"uro:publicSurveyDataQualityAttribute/uro:PublicSurveyDataQualityAttribute")]
    pub public_survey_data_quality_attribute: Option<PublicSurveyDataQualityAttribute>,
}

#[citygml_data(name = "uro:PublicSurveyDataQualityAttribute")]
pub struct PublicSurveyDataQualityAttribute {
    #[citygml(path = b"uro:srcScaleLod0")]
    pub src_scale_lod0: Option<Code>,

    #[citygml(path = b"uro:srcScaleLod1")]
    pub src_scale_lod1: Option<Code>,

    #[citygml(path = b"uro:srcScaleLod2")]
    pub src_scale_lod2: Option<Code>,

    #[citygml(path = b"uro:srcScaleLod3")]
    pub src_scale_lod3: Option<Code>,

    #[citygml(path = b"uro:srcScaleLod4")]
    pub src_scale_lod4: Option<Code>,

    #[citygml(path = b"uro:publicSurveySrcDescLod0")]
    pub public_survey_src_desc_lod0: Vec<Code>,

    #[citygml(path = b"uro:publicSurveySrcDescLod1")]
    pub public_survey_src_desc_lod1: Vec<Code>,

    #[citygml(path = b"uro:publicSurveySrcDescLod2")]
    pub public_survey_src_desc_lod2: Vec<Code>,

    #[citygml(path = b"uro:publicSurveySrcDescLod3")]
    pub public_survey_src_desc_lod3: Vec<Code>,

    #[citygml(path = b"uro:publicSurveySrcDescLod4")]
    pub public_survey_src_desc_lod4: Vec<Code>,
}

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
    pub uri_value: Option<Uri>,

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
