use citygml::CityGMLElement;

#[derive(Default, Debug, CityGMLElement)]
pub struct Tunnel {
    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"tun:class")]
    pub class: Option<String>,
}
