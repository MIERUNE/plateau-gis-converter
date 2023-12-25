pub mod codelist;
pub mod models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TopLevelCityObject {
    pub root: citygml::object::ObjectValue,
    pub geometries: citygml::geometry::Geometries,
}
