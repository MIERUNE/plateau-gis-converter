use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct SedimentDisasterProneArea {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
