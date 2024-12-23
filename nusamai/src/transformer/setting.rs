use serde::{Deserialize, Serialize};

use crate::transformer::selection::{LodSelection, Selection};
use crate::{sink::DataRequirements, transformer};

pub fn use_lod_config(default_value: &str, exclude: Option<&[&str]>) -> TransformerConfig {
    TransformerConfig {
        key: "use_lod".to_string(),
        label: "出力LODの選択".to_string(),
        parameter: transformer::ParameterType::Selection(LodSelection::create_lod_selection(
            default_value,
            exclude,
        )),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParameterType {
    String(String),
    Boolean(bool),
    Integer(i32),
    Selection(Selection),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformerConfig {
    pub key: String,
    pub label: String,
    pub parameter: ParameterType,
}

// FIXME: 設計を見直す
// FIXME: 意味のある名前に変える
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransformerSettings {
    pub configs: Vec<TransformerConfig>,
}

impl TransformerSettings {
    pub fn new() -> Self {
        Self { configs: vec![] }
    }

    pub fn insert(&mut self, def: TransformerConfig) {
        self.configs.push(def);
    }

    pub fn update_transformer(&mut self, config: TransformerConfig) {
        self.configs = self
            .configs
            .iter()
            .map(|c| {
                if c.key == config.key {
                    config.clone()
                } else {
                    c.clone()
                }
            })
            .collect();
    }

    pub fn initialize_valid_keys(&self) -> Vec<String> {
        self.configs
            .iter()
            .map(|config| config.key.clone())
            .collect()
    }

    pub fn build(&self, default_requirements: DataRequirements) -> DataRequirements {
        let mut data_requirements = default_requirements;

        for config in &self.configs {
            // Branch the processing based on the parameter type of the config
            match &config.parameter {
                ParameterType::String(_value) => {
                    // TODO: Processing for String types.
                }
                ParameterType::Boolean(_value) => {
                    // TODO: Processing for Boolean types.
                }
                ParameterType::Integer(_value) => {
                    // TODO: Processing for Integer types.
                }
                ParameterType::Selection(value) => {
                    if config.key == "use_lod" {
                        match value.selected_value.as_str() {
                            "max_lod" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Highest,
                                    ..Default::default()
                                });
                            }
                            "min_lod" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Lowest,
                                    ..Default::default()
                                })
                            }
                            "textured_max_lod" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::TexturedHighest,
                                    ..Default::default()
                                });
                                data_requirements.set_appearance(true);
                            }
                            "all_lod" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::All,
                                    ..Default::default()
                                });
                                data_requirements.set_appearance(true);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        data_requirements
    }
}
