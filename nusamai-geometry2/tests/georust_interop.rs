//! Testing mutual conversion between geo_types and our MultiPolygon
//!
//! Efficiency of the conversion is not considered.
//!
//! TODO: Implement these conversions as the 'geo-interop' feature.

use geo_types::{Coord, LineString, MultiPolygon, Polygon};
use geojson::GeoJson;
use nusamai_geometry2::{LineString2, MultiPolygon2};

/// Convert GeoRust MultiPolygon to MultiPolygon
fn georust_to_compact(multipolygon: &MultiPolygon) -> MultiPolygon2 {
    let MultiPolygon(multipolygon) = &multipolygon;
    let mut mpoly = MultiPolygon2::new();

    for polygon in multipolygon {
        let LineString(exterior) = &polygon.exterior();
        mpoly.add_exterior(exterior.iter().map(|c| [c.x, c.y]));
        for LineString(interior) in polygon.interiors() {
            mpoly.add_interior(interior.iter().map(|c| [c.x, c.y]));
        }
    }
    mpoly
}

/// Convert MultiPolygon to GeoRust MultiPolygon
fn compact_to_georust(mpoly: &MultiPolygon2) -> MultiPolygon {
    fn _coords_to_linestring(coords: &LineString2) -> LineString<f64> {
        LineString::new(
            coords
                .iter_closed()
                .map(|a| Coord { x: a[0], y: a[1] })
                .collect(),
        )
    }

    let polygons = mpoly
        .iter()
        .map(|poly| {
            let exterior = _coords_to_linestring(&poly.exterior());
            let interiors = poly
                .interiors()
                .map(|interior| _coords_to_linestring(&interior))
                .collect();
            Polygon::new(exterior, interiors)
        })
        .collect();

    MultiPolygon(polygons)
}

#[test]
fn test_georust_multipolygon_interop() {
    let geojson_str = r#"
    {
        "type": "MultiPolygon",
        "coordinates": [
            [
                [[102.0, 2.0], [103.0, 2.0], [103.0, 3.0], [102.0, 2.0]]
            ],
            [
                [[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]],
                [[100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.8], [100.2, 0.2]]
            ],
            [
                [[102.0, 0.0], [103.0, 0.0], [103.0, 1.0], [102.0, 1.0], [102.0, 0.0]],
                [[102.2, 0.2], [102.4, 0.2], [102.4, 0.4], [102.2, 0.4], [102.2, 0.2]],
                [[102.6, 0.6], [102.8, 0.6], [102.8, 0.8], [102.6, 0.8], [102.6, 0.6]]
            ],
            [
                [[101.0, 1.0], [102.0, 1.0], [102.0, 2.0], [101.0, 2.0], [101.0, 1.0]],
                [[101.2, 1.2], [101.4, 1.2], [101.4, 1.4], [101.2, 1.4], [101.2, 1.2]],
                [[101.6, 1.6], [101.8, 1.6], [101.8, 1.8], [101.6, 1.6]]
            ]
        ]
    }
    "#;

    // Load GeoJSON
    let Ok(GeoJson::Geometry(geometry)) = geojson_str.parse::<GeoJson>() else {
        panic!("failed to parse GeoJSON");
    };

    // GeoJSON -> GeoRust MultiPolygon
    let Ok(mpoly): Result<MultiPolygon, _> = geometry.value.try_into() else {
        panic!("failed to convert GeoJSON to MultiPolygon");
    };

    // GeoRust MultiPolygon -> MultiPolygon
    let compact_mpoly = georust_to_compact(&mpoly);

    // MultiPolygon -> GeoRust MultiPolygon
    let mpoly_again = compact_to_georust(&compact_mpoly);

    // Check equality
    assert_eq!(mpoly, mpoly_again);
}
