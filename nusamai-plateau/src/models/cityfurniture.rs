use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct CityFurniture {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"frn:class")]
    pub class: Option<String>,
}
