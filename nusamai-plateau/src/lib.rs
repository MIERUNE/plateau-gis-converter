pub mod codelist;
pub mod models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TopLevelCityObject {
    pub root: citygml::object::ObjectValue,
    pub geometries: citygml::geometry::Geometries,
}

// 仮実装

impl TopLevelCityObject {
    pub fn to_feature(&self) {}
}

#[derive(Debug, Clone, Default)]
struct Feature {}
