mod conversion;

use citygml::object::FeatureOrData;
use citygml::Geometries;

pub struct TopLevelCityObject<'a> {
    pub cityobj: FeatureOrData<'a>,
    pub geometries: Geometries,
}

// TODO: Handle properties (`tlco.cityobj` -> `geojson::Feature.properties`)
pub fn toplevel_city_object_to_geojson_features(
    tlco: &TopLevelCityObject,
) -> Vec<geojson::Feature> {
    let mpoly_geojson_geom = conversion::multipolygon_to_geojson_geometry(
        &tlco.geometries.vertices,
        &tlco.geometries.multipolygon,
    );
    let mpoly_geojson_feat = geojson::Feature {
        bbox: None,
        geometry: Some(mpoly_geojson_geom),
        id: None,
        properties: None, // TODO: from `tlco.cityobj`
        foreign_members: None,
    };

    let mls_geojson_geom = conversion::multilinestring_to_geojson_geometry(
        &tlco.geometries.vertices,
        &tlco.geometries.multilinestring,
    );
    // TODO: Handle properties (`tlco.cityobj` -> `geojson::Feature.properties`)
    let mls_geojson_feat = geojson::Feature {
        bbox: None,
        geometry: Some(mls_geojson_geom),
        id: None,
        properties: None, // TODO: from `tlco.cityobj`
        foreign_members: None,
    };

    vec![mpoly_geojson_feat, mls_geojson_feat]
}
