mod conversion;

pub use conversion::nusamai_to_geojson_geometry;
use nusamai_geometry::{CoordNum, Geometry};

/// Convert a vector of "nusamai geometries" to a "geojson feature collection"
// TODO: Handle properties
pub fn geometries_to_geojson<const D: usize, T: CoordNum>(
    geometries: &[Geometry<D, T>],
) -> geojson::GeoJson {
    let geojson_features = geometries
        .iter()
        .map(nusamai_to_geojson_geometry)
        .map(geojson_geometry_to_feature);

    let geojson_feature_collection = geojson::FeatureCollection {
        bbox: None,
        features: geojson_features.collect(),
        foreign_members: None,
    };
    geojson::GeoJson::from(geojson_feature_collection)
}

/// An intermediate function to create a "geojson feature" from a "geojson geometry"
// TODO: Handle properties
pub fn geojson_geometry_to_feature(geojson_geom: geojson::Geometry) -> geojson::Feature {
    geojson::Feature {
        bbox: None,
        geometry: Some(geojson_geom),
        id: None,
        properties: None,
        foreign_members: None,
    }
}
