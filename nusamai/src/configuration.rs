use indexmap::map::Entry;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    items: IndexMap<String, ConfigItem>,
}

impl Config {
    pub fn add(&mut self, item: ConfigItem) {
        match self.items.entry(item.name.clone()) {
            Entry::Occupied(entry) => {
                panic!("Configuration item {} already exists", entry.key())
            }
            Entry::Vacant(entry) => {
                entry.insert(item);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigItem {
    pub name: String,
    pub description: String,
    // pub value: ...
}
