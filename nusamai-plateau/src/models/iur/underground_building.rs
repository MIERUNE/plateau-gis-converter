use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct UndergroundBuilding {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
