use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub key: String,
    pub label: String,
    pub value: bool,
    pub requirements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformOptions {
    pub categories: Vec<Category>,
}

impl TransformOptions {
    pub fn new() -> Self {
        Self { categories: vec![] }
    }

    pub fn insert_option(&mut self, category: Category) {
        self.categories.push(category);
    }
}
