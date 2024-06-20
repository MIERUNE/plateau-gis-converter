use crate::sink::DataRequirements;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformOptionDetail {
    pub label: String,
    pub requirements: DataRequirements,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformOptions {
    items: IndexMap<String, TransformOptionDetail>,
}

impl TransformOptions {
    pub fn new() -> Self {
        TransformOptions {
            items: IndexMap::new(),
        }
    }

    pub fn insert_option(&mut self, key: String, detail: TransformOptionDetail) {
        self.items.insert(key, detail);
    }

    pub fn get_requirements(&self, key: &str) -> Option<&DataRequirements> {
        self.items.get(key).map(|detail| &detail.requirements)
    }
}
