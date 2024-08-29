use rayon::vec;
use serde::{Deserialize, Serialize};

use crate::{pipeline::PipelineError, sink::DataRequirements, transformer};

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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Selection {
    pub options: Vec<SelectionOptions>,
    pub selected_value: String,
}

impl Selection {
    pub fn new(options: Vec<SelectionOptions>, selected_value: &str) -> Self {
        let valid_value = options.iter().any(|opt| opt.value == selected_value);
        if !valid_value {
            panic!("selectedValue must be one of the options");
        }

        Self {
            options,
            selected_value: selected_value.to_string(),
        }
    }

    pub fn set_selected_value(&mut self, value: &str) {
        let valid_value = self.options.iter().any(|opt| opt.value == value);
        if valid_value {
            self.selected_value = value.to_string();
        } else {
            panic!("Invalid value");
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LodSelection {
    MaxLod,
    MinLod,
    Lod0,
    Lod1,
    Lod2,
    Lod3,
    Lod4,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Requirement {
    UseAppearance,
    NotUseAppearance,
    UseLod(LodSelection),
    // ...
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParameterType {
    String(String),
    Boolean(bool),
    Integer(i32),
    Selection(Selection),
    // and so on ...
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformerConfig {
    pub key: String,
    pub label: String,
    pub parameter: ParameterType,
    pub requirements: Vec<Requirement>,
}

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

    pub fn build(&self, default_requirements: DataRequirements) -> DataRequirements {
        let mut data_requirements = default_requirements;

        for config in &self.configs {
            // NOTE:configのparameterによって処理を分岐
            match &config.parameter {
                ParameterType::String(_value) => {
                    // TODO: Processing for String types.
                }
                ParameterType::Boolean(value) => {
                    if *value && config.key == "use_texture" {
                        for req in &config.requirements {
                            match req {
                                Requirement::UseAppearance => {
                                    data_requirements.set_appearance(true);
                                }
                                Requirement::NotUseAppearance => {
                                    data_requirements.set_appearance(false);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                ParameterType::Integer(_value) => {
                    // TODO: Processing for Integer types.
                }
                ParameterType::Selection(value) => {
                    if config.key == "use_lod" {
                        println!("{}", value.selected_value);
                        match value.selected_value.as_str() {
                            "max_lod" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Highest,
                                    ..Default::default()
                                })
                            }
                            "min_lod" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Lowest,
                                    ..Default::default()
                                })
                            }
                            "lod0" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Lod0,
                                    ..Default::default()
                                })
                            }
                            "lod1" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Lod1,
                                    ..Default::default()
                                })
                            }
                            "lod2" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Lod2,
                                    ..Default::default()
                                })
                            }
                            "lod3" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Lod3,
                                    ..Default::default()
                                })
                            }
                            "lod4" => {
                                data_requirements.set_lod_filter(transformer::LodFilterSpec {
                                    mode: transformer::LodFilterMode::Lod4,
                                    ..Default::default()
                                })
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
