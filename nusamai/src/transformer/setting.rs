use serde::{Deserialize, Serialize};

use crate::{pipeline::PipelineError, sink::DataRequirements, transformer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Requirement {
    UseAppearance,
    NotUseAppearance,
    UseMaxLod,
    // ...
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformerConfig {
    pub key: String,
    pub label: String,
    pub is_enabled: bool,
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

    pub fn update_transformer(&mut self, key: &str, is_enabled: bool) -> Result<(), PipelineError> {
        for def in &mut self.configs {
            // Ignored if key does not exist
            if def.key == key {
                def.is_enabled = is_enabled;
            }
        }
        Ok(())
    }

    pub fn build(&self, default_requirements: DataRequirements) -> DataRequirements {
        let mut data_requirements = default_requirements;
        let settings = TransformerRegistry::new();

        for def in &settings.configs {
            for req in def.requirements.clone() {
                match req {
                    Requirement::UseAppearance => data_requirements.set_appearance(true),
                    Requirement::NotUseAppearance => todo!(),
                    Requirement::UseMaxLod => {
                        data_requirements.set_lod_filter(transformer::LodFilterSpec {
                            mode: transformer::LodFilterMode::Highest,
                            ..Default::default()
                        })
                    }
                }
            }
        }

        data_requirements
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformerOption {
    pub key: String,
    pub is_enabled: bool,
}
