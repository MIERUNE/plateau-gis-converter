use serde::{Deserialize, Serialize};

use crate::{pipeline::PipelineError, sink::DataRequirements, transformer};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Selection {
    Bool(bool),        // オン・オフの設定
    Lod(LodSelection), // LODの選択
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
    UseMaxLod,
    UseMinLod,
    // ...
}

// NOTE:test
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Requirement2 {
    UseAppearance,
    NotUseAppearance,
    UseMaxLod,
    UseMinLod,
    UseLod(LodSelection),
    // ...
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformerConfig {
    pub key: String,
    pub label: String,
    pub is_enabled: bool,
    pub requirements: Vec<Requirement>,
}

// NOTE:test
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformerConfig2 {
    pub key: String,
    pub label: String,
    pub selection: Option<Selection>, // 汎用的な設定項目
    pub requirements: Vec<Requirement2>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransformerRegistry {
    pub configs: Vec<TransformerConfig>,
}

// NOTE:test
pub struct TransformerRegistry2 {
    pub configs: Vec<TransformerConfig2>,
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

        for config in &self.configs {
            if !config.is_enabled {
                continue;
            }
            for req in config.requirements.clone() {
                match req {
                    Requirement::UseAppearance => data_requirements.set_appearance(true),
                    Requirement::NotUseAppearance => data_requirements.set_appearance(false),
                    Requirement::UseMaxLod => {
                        data_requirements.set_lod_filter(transformer::LodFilterSpec {
                            mode: transformer::LodFilterMode::Highest,
                            ..Default::default()
                        })
                    }
                    Requirement::UseMinLod => {
                        data_requirements.set_lod_filter(transformer::LodFilterSpec {
                            mode: transformer::LodFilterMode::Lowest,
                            ..Default::default()
                        })
                    }
                }
            }
        }

        data_requirements
    }
}

// NOTE:test
impl TransformerRegistry2 {
    pub fn new() -> Self {
        Self { configs: vec![] }
    }

    pub fn insert(&mut self, def: TransformerConfig2) {
        self.configs.push(def);
    }

    pub fn update_transformer(
        &mut self,
        selection: Option<Selection>,
    ) -> Result<(), PipelineError> {
        for def in &mut self.configs {
            // Ignored if key does not exist
            // if def.selection == selection {
            //     // def.is_enabled = is_enabled;
            // }
        }

        Ok(())
    }

    pub fn build(&self, default_requirements: DataRequirements) -> DataRequirements {
        let mut data_requirements = default_requirements;

        for config in &self.configs {
            if !config.selection.is_none() {
                continue;
            }
            for req in config.requirements.clone() {
                match req {
                    Requirement2::UseAppearance => data_requirements.set_appearance(true),
                    Requirement2::NotUseAppearance => data_requirements.set_appearance(false),
                    Requirement2::UseMaxLod => {
                        data_requirements.set_lod_filter(transformer::LodFilterSpec {
                            mode: transformer::LodFilterMode::Highest,
                            ..Default::default()
                        })
                    }
                    Requirement2::UseMinLod => {
                        data_requirements.set_lod_filter(transformer::LodFilterSpec {
                            mode: transformer::LodFilterMode::Lowest,
                            ..Default::default()
                        })
                    }
                    Requirement2::UseLod(selection) => {
                        let mode = match selection {
                            LodSelection::MaxLod => transformer::LodFilterMode::Highest,
                            LodSelection::MinLod => transformer::LodFilterMode::Lowest,
                            LodSelection::Lod0 => transformer::LodFilterMode::Lod0,
                            LodSelection::Lod1 => transformer::LodFilterMode::Lod1,
                            LodSelection::Lod2 => transformer::LodFilterMode::Lod2,
                            LodSelection::Lod3 => transformer::LodFilterMode::Lod3,
                            LodSelection::Lod4 => transformer::LodFilterMode::Lod4,
                        };
                        data_requirements.set_lod_filter(transformer::LodFilterSpec {
                            mode,
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

// NOTE:test
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformerOption2 {
    pub key: String,
    pub is_enabled: bool,
}
