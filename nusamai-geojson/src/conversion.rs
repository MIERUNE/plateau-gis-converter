use nusamai_geometry::{CoordNum, Geometry, MultiPolygon, Polygon};

/// A wrapper to convert an arbitrary "nusamai geometry" to a "geojson geometry"
// TODO: implementations for all geometry variants
pub fn nusamai_to_geojson_geometry<const D: usize, T: CoordNum>(
    geometry: &Geometry<D, T>,
) -> geojson::Geometry {
    match geometry {
        Geometry::MultiPoint(mpoint) => multi_point_to_geojson_geometry(mpoint),
        Geometry::LineString(_) => unimplemented!(),
        Geometry::MultiLineString(_) => unimplemented!(),
        Geometry::Polygon(poly) => polygon_to_geojson_geometry(poly),
        Geometry::MultiPolygon(mpoly) => multi_polygon_to_geojson_geometry(mpoly),
    }
}

fn multi_point_to_geojson_geometry<const D: usize, T: CoordNum>(
    mpoint: &nusamai_geometry::MultiPoint<D, T>,
) -> geojson::Geometry {
    let point_list: Vec<geojson::PointType> = mpoint
        .iter()
        .map(|point| point.iter().map(|&t| t.to_f64().unwrap()).collect())
        .collect();
    geojson::Geometry::new(geojson::Value::MultiPoint(point_list))
}

fn line_string_to_geojson_geometry<const D: usize, T: CoordNum>(
    line_string: &nusamai_geometry::LineString<D, T>,
) -> geojson::Geometry {
    let point_list: geojson::LineStringType = line_string
        .iter()
        .map(|point| point.iter().map(|&t| t.to_f64().unwrap()).collect())
        .collect();
    geojson::Geometry::new(geojson::Value::LineString(point_list))
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
        .map(|line_string| {
            line_string
                .iter_closed()
                .map(|slice| slice.iter().map(|&t| t.to_f64().unwrap()).collect())
                .collect()
        })
        .collect();
    rings
}

#[cfg(test)]
mod tests {
    use super::*;
    use nusamai_geometry::{MultiPoint2, MultiPolygon2, Polygon2, Polygon3};

    #[test]
    fn test_multi_point_basic() {
        let mut mpoint = MultiPoint2::new();
        mpoint.push(&[0., 0.]);
        mpoint.push(&[1., 1.]);
        mpoint.push(&[2., 2.]);

        let geojson_geometry = multi_point_to_geojson_geometry(&mpoint);

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());

        if let geojson::Value::MultiPoint(points) = geojson_geometry.value {
            assert_eq!(points.len(), mpoint.len());
            assert_eq!(points[0], vec![0., 0.]);
            assert_eq!(points[1], vec![1., 1.]);
            assert_eq!(points[2], vec![2., 2.]);
        } else {
            unreachable!("The result is not a GeoJSON MultiPoint");
        };
    }

    #[test]
    fn test_line_string_basic() {
        let mut line_string = nusamai_geometry::LineString2::new();
        line_string.push(&[0., 0.]);
        line_string.push(&[1., 1.]);
        line_string.push(&[2., 2.]);

        let geojson_geometry = line_string_to_geojson_geometry(&line_string);

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());

        if let geojson::Value::LineString(points) = geojson_geometry.value {
            assert_eq!(points.len(), line_string.len());
            assert_eq!(points[0], vec![0., 0.]);
            assert_eq!(points[1], vec![1., 1.]);
            assert_eq!(points[2], vec![2., 2.]);
        } else {
            unreachable!("The result is not a GeoJSON LineString");
        };
    }

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
    fn test_polygon_basic_3d() {
        let mut poly = Polygon3::new();
        poly.add_ring([[0., 0., 99.], [5., 0., 99.], [5., 5., 99.], [0., 5., 99.]]);
        poly.add_ring([[1., 1., 99.], [2., 1., 99.], [2., 2., 99.], [1., 2., 99.]]);
        poly.add_ring([[3., 3., 99.], [4., 3., 99.], [4., 4., 99.], [3., 4., 99.]]);

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
                    [5., 0., 99.],
                    [5., 5., 99.],
                    [0., 5., 99.],
                    [0., 0., 99.]
                ]
            );
            assert_eq!(
                rings[1],
                vec![
                    [1., 1., 99.],
                    [2., 1., 99.],
                    [2., 2., 99.],
                    [1., 2., 99.],
                    [1., 1., 99.]
                ]
            );
            assert_eq!(
                rings[2],
                vec![
                    [3., 3., 99.],
                    [4., 3., 99.],
                    [4., 4., 99.],
                    [3., 4., 99.],
                    [3., 3., 99.]
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
                    }
                    1 => {
                        assert_eq!(rings.len(), 2);
                        assert_eq!(rings[0].len(), 5);
                        assert_eq!(rings[1].len(), 5);
                        assert_eq!(
                            rings[0],
                            vec![[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]
                        );
                        assert_eq!(
                            rings[1],
                            vec![[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]
                        );
                    }
                    2 => {
                        assert_eq!(rings.len(), 2);
                        assert_eq!(rings[0].len(), 5);
                        assert_eq!(rings[1].len(), 5);
                        assert_eq!(
                            rings[0],
                            vec![[4., 0.], [7., 0.], [7., 3.], [4., 3.], [4., 0.]]
                        );
                        assert_eq!(
                            rings[1],
                            vec![[5., 1.], [6., 1.], [6., 2.], [5., 2.], [5., 1.]]
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
