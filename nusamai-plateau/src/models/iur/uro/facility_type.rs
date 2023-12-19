use citygml::{CityGMLElement, Code};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct FacilityTypeAttribute {
    #[citygml(path = b"uro:class")]
    pub class: Option<Code>,

    #[citygml(path = b"uro:function")]
    pub function: Vec<Code>,
}
