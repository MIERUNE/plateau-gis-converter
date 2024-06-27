use serde::{Deserialize, Serialize};

use crate::{sink::DataRequirements, transformer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Requirements {
    UseAppearance,
    NotUseAppearance,
    UseMaxLod,
    // ...
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformerDefinition {
    pub key: String,
    pub label: String,
    pub enabled: bool,
    pub requirements: Vec<Requirements>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransformerSettings {
    pub definition: Vec<TransformerDefinition>,
}

impl TransformerSettings {
    pub fn new() -> Self {
        Self { definition: vec![] }
    }

    pub fn insert(&mut self, def: TransformerDefinition) {
        self.definition.push(def);
    }

    pub fn update_transformer(&mut self, key: &str, enabled: bool) {
        for def in &mut self.definition {
            // Ignored if key does not exist
            if def.key == key {
                def.enabled = enabled;
            }
        }
    }

    pub fn build(&self, default_requirements: DataRequirements) -> DataRequirements {
        let mut data_requirements = default_requirements;
        let settings = TransformerSettings::new();

        for def in &settings.definition {
            for req in def.requirements.clone() {
                match req {
                    Requirements::UseAppearance => data_requirements.set_appearance(true),
                    Requirements::NotUseAppearance => todo!(),
                    Requirements::UseMaxLod => {
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
pub struct TransformerSwitchOption {
    pub key: String,
    pub enabled: bool,
}
