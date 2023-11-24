use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct ReliefFeature {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
