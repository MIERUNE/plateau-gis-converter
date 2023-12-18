mod conversion;

use citygml::object::FeatureOrData;
use citygml::Geometries;

pub struct TopLevelCityObject<'a> {
    pub cityobj: FeatureOrData<'a>,
    pub geometries: Geometries,
}

pub fn toplevel_city_object_to_geojson_feature(tlco: &TopLevelCityObject) -> geojson::Feature {
    let mpoly_geojson_geom = conversion::multipolygon_to_geojson_geometry(
        &tlco.geometries.vertices,
        &tlco.geometries.multipolygon,
    );

    // TODO: Handle `tlco.geometries.multilinestring`

    // TODO: Handle properties (`tlco.cityobj` -> `geojson::Feature.properties`)
    geojson::Feature {
        bbox: None,
        geometry: Some(mpoly_geojson_geom),
        id: None,
        properties: None,
        foreign_members: None,
    }
}
