use citygml::*;

struct TopLevelCityObject {
    cityobj: citygml::object::FeatureOrData<'static>,
    geometries: citygml::Geometries,
}
