pub mod codelist;
pub mod models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TopLevelCityObject {
    pub root: nusamai_citygml::object::ObjectValue,
    pub geometries: nusamai_citygml::geometry::Geometries,
}
