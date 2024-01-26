use nusamai_citygml::{citygml_feature, CityGmlElement, Code};

#[citygml_feature(name = "gen:GenericCityObject")]
pub struct GenericCityObject {
    #[citygml(path = b"gen:class")]
    pub class: Option<Code>,

    #[citygml(path = b"gen:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"gen:usage")]
    pub usage: Vec<Code>,
}
