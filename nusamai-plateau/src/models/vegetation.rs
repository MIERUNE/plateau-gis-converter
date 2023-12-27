use nusamai_citygml::{citygml_feature, CityGMLElement};

#[citygml_feature(name = "veg:SolitaryVegetationObject")]
pub struct SolitaryVegetationObject {
    #[citygml(path = b"veg:class")]
    pub class: Option<String>,
}

#[citygml_feature(name = "veg:PlantCover")]
pub struct PlantCover {
    #[citygml(path = b"veg:class")]
    pub class: Option<String>,
}
