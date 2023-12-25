use citygml::{citygml_feature, CityGMLElement};

#[citygml_feature(name = "brid:Bridge")]
pub struct Bridge {
    #[citygml(path = b"brid:class")]
    pub class: Option<String>,
}

// TODO: Building と類似の構造を持つ
