use serde::{Deserialize, Serialize};

use crate::{sink::DataRequirements, transformer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectionOptions {
    pub label: String,
    pub value: String,
}

impl SelectionOptions {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Selection {
    pub options: Vec<SelectionOptions>,
    pub selected_value: String,
}

impl Selection {
    pub fn new(options: Vec<(&str, &str)>, selected_value: &str) -> Self {
        let options: Vec<SelectionOptions> = options
            .into_iter()
            .map(|(label, value)| SelectionOptions::new(label, value))
            .collect();

        let valid_value = options.iter().any(|opt| opt.value == selected_value);
        if !valid_value {
            panic!("selected_value must be one of the options");
        }

        Self {
            options,
            selected_value: selected_value.to_string(),
        }
    }

    pub fn set_selected_value(&mut self, value: &str) -> Result<(), String> {
        if self.options.iter().any(|opt| opt.value == value) {
            self.selected_value = value.to_string();
            Ok(())
        } else {
            Err("Invalid value".to_string())
        }
    }

    pub fn get_options(&self) -> Vec<SelectionOptions> {
        self.options.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LodSelection;

impl LodSelection {
    pub fn get_lod_selection_options() -> Vec<(&'static str, &'static str)> {
        vec![
            ("最大LOD", "max_lod"),
            ("最小LOD", "min_lod"),
            ("テクスチャ付き最大LOD", "textured_max_lod"),
            // This option will be used in 3dtiles sink.
            ("すべてのLOD", "all_lod"),
        ]
    }

    pub fn lod_selection_with_texture(default_value: &str) -> Selection {
        Selection::new(Self::get_lod_selection_options(), default_value)
    }

    pub fn lod_selection_without_texture(default_value: &str) -> Selection {
        let options = Self::get_lod_selection_options()
            .into_iter()
            .filter(|&(_, value)| value != "textured_max_lod")
            .collect::<Vec<_>>();
        Selection::new(options, default_value)
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
pub struct TransformerRegistry {
    pub configs: Vec<TransformerConfig>,
}

impl TransformerRegistry {
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
