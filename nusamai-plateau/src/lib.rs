pub mod models;

#[derive(Debug)]
pub struct TopLevelCityObject {
    pub root: citygml::object::ObjectValue,
    pub geometries: citygml::geometry::Geometries,
}
