use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct SolitaryVegetationObject {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"veg:class")]
    pub class: Option<String>,
}

#[derive(Default, Debug, CityGMLElement)]
pub struct PlantCover {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"veg:class")]
    pub class: Option<String>,
}
