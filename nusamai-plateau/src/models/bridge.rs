use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct Bridge {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"brid:class")]
    pub class: Option<String>,
}
