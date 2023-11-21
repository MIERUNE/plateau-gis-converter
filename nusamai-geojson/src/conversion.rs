use nusamai_geometry::{CoordNum, Geometry, MultiPolygon, Polygon};

/// A wrapper to convert an arbitrary "nusamai geometry" to a "geojson geometry"
pub fn nusamai_to_geojson_geometry<const D: usize, T: CoordNum>(
    geometry: &Geometry<D, T>,
) -> geojson::Geometry {
    match geometry {
        Geometry::MultiPoint(_) => unimplemented!(),
        Geometry::LineString(_) => unimplemented!(),
        Geometry::MultiLineString(_) => unimplemented!(),
        Geometry::Polygon(poly) => polygon_to_geojson_geometry(poly),
        Geometry::MultiPolygon(mpoly) => multi_polygon_to_geojson_geometry(mpoly),
    }
}

fn polygon_to_geojson_geometry<const D: usize, T: CoordNum>(
    poly: &Polygon<D, T>,
) -> geojson::Geometry {
    let rings = polygon_to_rings(poly);
    geojson::Geometry::new(geojson::Value::Polygon(rings))
}

fn multi_polygon_to_geojson_geometry<const D: usize, T: CoordNum>(
    mpoly: &MultiPolygon<D, T>,
) -> geojson::Geometry {
    let ring_list: Vec<geojson::PolygonType> =
        mpoly.iter().map(|poly| polygon_to_rings(&poly)).collect();
    geojson::Geometry::new(geojson::Value::MultiPolygon(ring_list))
}

fn polygon_to_rings<const D: usize, T: CoordNum>(poly: &Polygon<D, T>) -> geojson::PolygonType {
    let mut rings = Vec::new();

    let exterior = poly.exterior();
    let exterior_positions = exterior
        .iter_closed()
        .map(|slice| slice.iter().map(|&t| t.to_f64().unwrap()).collect())
        .collect::<Vec<Vec<f64>>>();

    rings.push(exterior_positions);

    for interior in poly.interiors() {
        let interior_positions = interior
            .iter_closed()
            .map(|slice| slice.iter().map(|&t| t.to_f64().unwrap()).collect())
            .collect::<Vec<Vec<f64>>>();
        rings.push(interior_positions);
    }

    rings
}

#[cfg(test)]
mod tests {
    use super::*;
    use nusamai_geometry::{MultiPolygon2, Polygon2};

    #[test]
    fn test_polygon_basic() {
        let mut poly = Polygon2::new();
        poly.add_ring([[0., 0.], [5., 0.], [5., 5.], [0., 5.]]);
        poly.add_ring([[1., 1.], [2., 1.], [2., 2.], [1., 2.]]);
        poly.add_ring([[3., 3.], [4., 3.], [4., 4.], [3., 4.]]);

        let geojson_geometry = polygon_to_geojson_geometry(&poly);

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());
        if let geojson::Value::Polygon(rings) = geojson_geometry.value {
            assert_eq!(rings.len(), 3);
            assert_eq!(rings[0].len(), 5);
            assert_eq!(rings[1].len(), 5);
            assert_eq!(rings[2].len(), 5);
            assert_eq!(
                rings[0],
                vec![[0., 0.], [5., 0.], [5., 5.], [0., 5.], [0., 0.]]
            );
            assert_eq!(
                rings[1],
                vec![[1., 1.], [2., 1.], [2., 2.], [1., 2.], [1., 1.]]
            );
            assert_eq!(
                rings[2],
                vec![[3., 3.], [4., 3.], [4., 4.], [3., 4.], [3., 3.]]
            );
        } else {
            unreachable!("The result is not a GeoJSON Polygon");
        };
    }

    #[test]
    fn test_multi_polygon_basic() {
        let mut mpoly = MultiPolygon2::new();

        // 1st polygon
        mpoly.add_exterior([[0., 0.], [5., 0.], [5., 5.], [0., 5.], [0., 0.]]);
        mpoly.add_interior([[1., 1.], [2., 1.], [2., 2.], [1., 2.], [1., 1.]]);
        mpoly.add_interior([[3., 3.], [4., 3.], [4., 4.], [3., 4.], [3., 3.]]);

        // 2nd polygon
        mpoly.add_exterior([[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]);
        mpoly.add_interior([[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]);

        // 3rd polygon
        mpoly.add_exterior([[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]);
        mpoly.add_interior([[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]);

        let geojson_geometry = multi_polygon_to_geojson_geometry(&mpoly);
    }
}
