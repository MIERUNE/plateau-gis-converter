use nusamai_citygml::{citygml_data, CityGmlElement, Code};

#[citygml_data(name = "uro:FacilityTypeAttribute")]
pub struct FacilityTypeAttribute {
    #[citygml(path = b"uro:class")]
    pub class: Option<Code>,

    #[citygml(path = b"uro:function")]
    pub function: Vec<Code>,
}
