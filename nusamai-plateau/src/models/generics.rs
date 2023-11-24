use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct GenericCityObject {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
