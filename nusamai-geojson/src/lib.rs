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
    let mut geojson_features: Vec<geojson::Feature> = vec![];

    if !tlco.geometries.multipolygon.is_empty() {
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
        geojson_features.push(mpoly_geojson_feat);
    }

    if !tlco.geometries.multilinestring.is_empty() {
        let mls_geojson_geom: geojson::Geometry = conversion::multilinestring_to_geojson_geometry(
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
        geojson_features.push(mls_geojson_feat);
    }

    geojson_features
}
