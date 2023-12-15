use nusamai_geometry::{CoordNum, Geometry, MultiPolygon, Polygon};

/// A wrapper to convert an arbitrary "nusamai geometry" to a "geojson geometry"
// TODO: implementations for all geometry variants
pub fn nusamai_to_geojson_geometry<const D: usize, T: CoordNum>(
    geometry: &Geometry<D, T>,
) -> geojson::Geometry {
    match geometry {
        Geometry::MultiPoint(geom) => multi_point_to_geojson_geometry(geom),
        Geometry::LineString(geom) => linestring_to_geojson_geometry(geom),
        Geometry::MultiLineString(geom) => multi_linestring_to_geojson_geometry(geom),
        Geometry::Polygon(geom) => polygon_to_geojson_geometry(geom),
        Geometry::MultiPolygon(geom) => multi_polygon_to_geojson_geometry(geom),
    }
}

/// Swap the order of latitude and longitude:
/// - PLATEAU data holds the data in the order of "latitude, longitude, altitude"
/// - GeoJSON expects the order to be "longitude, latitude, altitude"
fn swap_lat_lon<T: CoordNum>(point: &[T]) -> Vec<T> {
    let mut swapped = point.to_vec();
    swapped.swap(0, 1); // Expect the length of the point to be 2 or more
    swapped
}

fn multi_point_to_geojson_geometry<const D: usize, T: CoordNum>(
    mpoint: &nusamai_geometry::MultiPoint<D, T>,
) -> geojson::Geometry {
    let point_list: Vec<geojson::PointType> = mpoint
        .iter()
        .map(swap_lat_lon)
        .map(|point| point.iter().map(|&t| t.to_f64().unwrap()).collect())
        .collect();
    geojson::Geometry::new(geojson::Value::MultiPoint(point_list))
}

fn linestring_to_geojson_geometry<const D: usize, T: CoordNum>(
    linestring: &nusamai_geometry::LineString<D, T>,
) -> geojson::Geometry {
    let point_list: geojson::LineStringType = linestring
        .iter()
        .map(swap_lat_lon)
        .map(|point| point.iter().map(|&t| t.to_f64().unwrap()).collect())
        .collect();
    geojson::Geometry::new(geojson::Value::LineString(point_list))
}

fn multi_linestring_to_geojson_geometry<const D: usize, T: CoordNum>(
    mlinestring: &nusamai_geometry::MultiLineString<D, T>,
) -> geojson::Geometry {
    let line_list: Vec<geojson::LineStringType> = mlinestring
        .iter()
        .map(|linestring| {
            linestring
                .iter()
                .map(swap_lat_lon)
                .map(|point| point.iter().map(|&t| t.to_f64().unwrap()).collect())
                .collect()
        })
        .collect();
    geojson::Geometry::new(geojson::Value::MultiLineString(line_list))
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
    let rings = std::iter::once(poly.exterior())
        .chain(poly.interiors())
        .map(|linestring| {
            linestring
                .iter_closed()
                .map(swap_lat_lon)
                .map(|slice| slice.iter().map(|&t| t.to_f64().unwrap()).collect())
                .collect()
        })
        .collect();
    rings
}

#[cfg(test)]
mod tests {
    use super::*;
    use nusamai_geometry::{MultiLineString2, MultiPoint2, MultiPolygon2, Polygon2, Polygon3};

    #[test]
    fn test_multi_point_basic() {
        let mut mpoint = MultiPoint2::new();
        mpoint.push(&[0., 0.]);
        mpoint.push(&[1., 2.]);
        mpoint.push(&[3., 4.]);

        let geojson_geometry = multi_point_to_geojson_geometry(&mpoint);

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());

        if let geojson::Value::MultiPoint(points) = geojson_geometry.value {
            assert_eq!(points.len(), mpoint.len());
            assert_eq!(points[0], vec![0., 0.]);
            assert_eq!(points[1], vec![2., 1.]); // lon, lat swapped
            assert_eq!(points[2], vec![4., 3.]);
        } else {
            unreachable!("The result is not a GeoJSON MultiPoint");
        };
    }

    #[test]
    fn test_linestring_basic() {
        let mut linestring = nusamai_geometry::LineString2::new();
        linestring.push(&[0., 0.]);
        linestring.push(&[1., 2.]);
        linestring.push(&[3., 4.]);

        let geojson_geometry = linestring_to_geojson_geometry(&linestring);

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());

        if let geojson::Value::LineString(points) = geojson_geometry.value {
            assert_eq!(points.len(), linestring.len());
            assert_eq!(points[0], vec![0., 0.]);
            assert_eq!(points[1], vec![2., 1.]);
            assert_eq!(points[2], vec![4., 3.]);
        } else {
            unreachable!("The result is not a GeoJSON LineString");
        };
    }

    #[test]
    fn test_multi_linestring_basic() {
        let mut mls = MultiLineString2::new();
        mls.add_linestring(vec![[0., 0.], [1., 2.]]);
        mls.add_linestring(vec![[3., 4.], [5., 6.]]);
        mls.add_linestring(vec![[7., 8.], [9., 10.], [11., 12.]]);

        let geojson_geometry = multi_linestring_to_geojson_geometry(&mls);

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());

        if let geojson::Value::MultiLineString(lines) = geojson_geometry.value {
            assert_eq!(lines.len(), mls.len());
            assert_eq!(lines[0], vec![[0., 0.], [2., 1.]]);
            assert_eq!(lines[1], vec![[4., 3.], [6., 5.]]);
            assert_eq!(lines[2], vec![[8., 7.], [10., 9.], [12., 11.]]);
        } else {
            unreachable!("The result is not a GeoJSON MultiLineString");
        };
    }

    #[test]
    fn test_polygon_basic() {
        let mut poly = Polygon2::new();
        poly.add_ring([[0., 0.], [51., 0.], [50., 50.], [0., 52.]]);
        poly.add_ring([[10., 10.], [21., 10.], [20., 20.], [10., 22.]]);
        poly.add_ring([[30., 30.], [41., 30.], [40., 40.], [30., 42.]]);

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
                vec![[0., 0.], [0., 51.], [50., 50.], [52., 0.], [0., 0.]] // lon, lat swapped
            );
            assert_eq!(
                rings[1],
                vec![[10., 10.], [10., 21.], [20., 20.], [22., 10.], [10., 10.]]
            );
            assert_eq!(
                rings[2],
                vec![[30., 30.], [30., 41.], [40., 40.], [42., 30.], [30., 30.]]
            );
        } else {
            unreachable!("The result is not a GeoJSON Polygon");
        };
    }

    #[test]
    fn test_polygon_basic_3d() {
        let mut poly = Polygon3::new();
        poly.add_ring([
            [0., 0., 99.],
            [51., 0., 99.],
            [50., 50., 99.],
            [0., 52., 99.],
        ]);
        poly.add_ring([
            [10., 10., 99.],
            [21., 10., 99.],
            [20., 20., 99.],
            [10., 22., 99.],
        ]);
        poly.add_ring([
            [30., 30., 99.],
            [41., 30., 99.],
            [40., 40., 99.],
            [30., 42., 99.],
        ]);

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
                vec![
                    [0., 0., 99.],
                    [0., 51., 99.],
                    [50., 50., 99.],
                    [52., 0., 99.],
                    [0., 0., 99.]
                ]
            );
            assert_eq!(
                rings[1],
                vec![
                    [10., 10., 99.],
                    [10., 21., 99.],
                    [20., 20., 99.],
                    [22., 10., 99.],
                    [10., 10., 99.]
                ]
            );
            assert_eq!(
                rings[2],
                vec![
                    [30., 30., 99.],
                    [30., 41., 99.],
                    [40., 40., 99.],
                    [42., 30., 99.],
                    [30., 30., 99.]
                ]
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

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());
        if let geojson::Value::MultiPolygon(rings_list) = geojson_geometry.value {
            for (i, rings) in rings_list.iter().enumerate() {
                match i {
                    0 => {
                        assert_eq!(rings.len(), 3);
                        assert_eq!(rings[0].len(), 5);
                        assert_eq!(rings[1].len(), 5);
                        assert_eq!(rings[2].len(), 5);
                        assert_eq!(
                            rings[0],
                            vec![[0., 0.], [0., 5.], [5., 5.], [5., 0.], [0., 0.]] // lon, lat swapped
                        );
                        assert_eq!(
                            rings[1],
                            vec![[1., 1.], [1., 2.], [2., 2.], [2., 1.], [1., 1.]]
                        );
                        assert_eq!(
                            rings[2],
                            vec![[3., 3.], [3., 4.], [4., 4.], [4., 3.], [3., 3.]]
                        );
                    }
                    1 => {
                        assert_eq!(rings.len(), 2);
                        assert_eq!(rings[0].len(), 5);
                        assert_eq!(rings[1].len(), 5);
                        assert_eq!(
                            rings[0],
                            vec![[0., 4.], [0., 7.], [3., 7.], [3., 4.], [0., 4.]]
                        );
                        assert_eq!(
                            rings[1],
                            vec![[1., 5.], [1., 6.], [2., 6.], [2., 5.], [1., 5.]]
                        );
                    }
                    2 => {
                        assert_eq!(rings.len(), 2);
                        assert_eq!(rings[0].len(), 5);
                        assert_eq!(rings[1].len(), 5);
                        assert_eq!(
                            rings[0],
                            vec![[0., 4.], [0., 7.], [3., 7.], [3., 4.], [0., 4.]]
                        );
                        assert_eq!(
                            rings[1],
                            vec![[1., 5.], [1., 6.], [2., 6.], [2., 5.], [1., 5.]]
                        );
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            unreachable!("The result is not a GeoJSON MultiPolygon");
        };
    }
}
