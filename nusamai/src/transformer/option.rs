// use crate::{
//     transformer,
//     transformer::{LodSelection, TransformerConfig},
// };

// pub fn use_lod_config(default_value: &str) -> TransformerConfig {
//     TransformerConfig {
//         key: "use_lod".to_string(),
//         label: "出力LODの選択".to_string(),
//         parameter: transformer::ParameterType::Selection(LodSelection::create_lod_selection(
//             default_value,
//             Some(&["textured_max_lod"]),
//         )),
//     }
// }

// pub fn use_lod_config(default_value: &str) -> TransformerConfig {
//     TransformerConfig {
//         key: "use_lod".to_string(),
//         label: "出力LODの選択".to_string(),
//         parameter: transformer::ParameterType::Selection(LodSelection::create_lod_selection(
//             default_value,
//             None,
//         )),
//     }
// }
