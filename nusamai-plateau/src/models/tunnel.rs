use citygml::{citygml_feature, CityGMLElement};

#[citygml_feature(name = "tun:Tunnel")]
pub struct Tunnel {
    #[citygml(path = b"tun:class")]
    pub class: Option<String>,
}

// TODO: Building と類似の構造を持つ
