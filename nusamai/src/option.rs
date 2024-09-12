use crate::{
    transformer,
    transformer::{LodSelection, TransformerConfig},
};

pub fn use_lod_config(default_value: &str) -> TransformerConfig {
    TransformerConfig {
        key: "use_lod".to_string(),
        label: "出力LODの選択".to_string(),
        parameter: transformer::ParameterType::Selection(LodSelection::create_lod_selection(
            default_value,
        )),
    }
}

pub fn use_texture_config(default_value: bool) -> TransformerConfig {
    TransformerConfig {
        key: "use_texture".to_string(),
        label: "テクスチャの使用".to_string(),
        parameter: transformer::ParameterType::Boolean(default_value),
    }
}
