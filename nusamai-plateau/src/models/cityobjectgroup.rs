use citygml::CityGMLElement;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct CityObjectGroup {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,
}
