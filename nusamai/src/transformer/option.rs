use crate::{
    transformer,
    transformer::{LodSelection, TransformerConfig},
};

pub fn use_lod_config(default_value: &str) -> TransformerConfig {
    TransformerConfig {
        key: "use_lod".to_string(),
        label: "出力LODの選択".to_string(),
        parameter: transformer::ParameterType::Selection(
            LodSelection::lod_selection_without_texture(default_value),
        ),
    }
}

pub fn use_textured_lod_config(default_value: &str) -> TransformerConfig {
    TransformerConfig {
        key: "use_lod".to_string(),
        label: "出力LODの選択".to_string(),
        parameter: transformer::ParameterType::Selection(LodSelection::lod_selection_with_texture(
            default_value,
        )),
    }
}
