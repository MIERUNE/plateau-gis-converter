use flatgeom::{
    Coord, LineString, LineString3, MultiLineString, MultiLineString3, MultiPoint, MultiPoint3,
    MultiPolygon, Polygon,
};

/// Create a GeoJSON Polygon from `flatgeom::MultiPolygon`.
pub fn polygon_to_value<const D: usize>(poly: &Polygon<[f64; D]>) -> geojson::Value {
    polygon_to_value_with_mapping(poly, |c| c.to_vec())
}

/// Create a GeoJSON Polygon from vertices and indices.
pub fn indexed_polygon_to_value(vertices: &[[f64; 3]], poly_idx: &Polygon<u32>) -> geojson::Value {
    polygon_to_value_with_mapping(poly_idx, |idx| vertices[idx as usize].to_vec())
}

/// Create a GeoJSON Polygon from `flatgeom::Polygon` with a mapping function.
pub fn polygon_to_value_with_mapping<T: Coord>(
    poly: &Polygon<T>,
    mapping: impl Fn(T) -> Vec<f64>,
) -> geojson::Value {
    let coords: geojson::PolygonType = poly
        .rings()
        .map(|ls| {
            ls.iter_closed()
                .map(&mapping) // Get the actual coord values
                .collect()
        })
        .collect::<Vec<_>>();
    geojson::Value::Polygon(coords)
}

/// Create a GeoJSON MultiPolygon from `flatgeom::MultiPolygon`.
pub fn multipolygon_to_value<const D: usize>(mpoly: &MultiPolygon<[f64; D]>) -> geojson::Value {
    multipolygon_to_value_with_mapping(mpoly, |c| c.to_vec())
}

/// Create a GeoJSON MultiPolygon from vertices and indices.
pub fn indexed_multipolygon_to_value(
    vertices: &[[f64; 3]],
    mpoly_idx: &MultiPolygon<u32>,
) -> geojson::Value {
    multipolygon_to_value_with_mapping(mpoly_idx, |idx| vertices[idx as usize].to_vec())
}

/// Create a GeoJSON MultiPolygon from `flatgeom::MultiPolygon` with a mapping function.
pub fn multipolygon_to_value_with_mapping<T: Coord>(
    mpoly: &MultiPolygon<T>,
    mapping: impl Fn(T) -> Vec<f64>,
) -> geojson::Value {
    let coords: Vec<geojson::PolygonType> = mpoly
        .iter()
        .map(|poly| {
            poly.rings()
                .map(|ls| {
                    ls.iter_closed()
                        .map(&mapping) // Get the actual coord values
                        .collect()
                })
                .collect::<Vec<_>>()
        })
        .collect();
    geojson::Value::MultiPolygon(coords)
}

/// Create a GeoJSON LineString from `flatgeom::LineString`.
pub fn linestring_to_value(ls: &LineString3) -> geojson::Value {
    linestring_to_value_with_mapping(ls, |c| c.to_vec())
}

/// Create a GeoJSON LineString from vertices and indices.
pub fn indexed_linestring_to_value(
    vertices: &[[f64; 3]],
    ls_idx: &LineString<u32>,
) -> geojson::Value {
    linestring_to_value_with_mapping(ls_idx, |idx| vertices[idx as usize].to_vec())
}

/// Create a GeoJSON LineString from `flatgeom::LineString` with a mapping function.
pub fn linestring_to_value_with_mapping<T: Coord>(
    ls: &LineString<T>,
    mapping: impl Fn(T) -> Vec<f64>,
) -> geojson::Value {
    let coords = ls.iter().map(&mapping).collect(); // Get the actual coord values .collect()
    geojson::Value::LineString(coords)
}

/// Create a GeoJSON MultiLineString from `flatgeom::MultiLineString`.
pub fn multilinestring_to_value(mls: &MultiLineString3) -> geojson::Value {
    multilinestring_to_value_with_mapping(mls, |c| c.to_vec())
}

/// Create a GeoJSON MultiLineString from vertices and indices.
pub fn indexed_multilinestring_to_value(
    vertices: &[[f64; 3]],
    mls_idx: &MultiLineString<u32>,
) -> geojson::Value {
    multilinestring_to_value_with_mapping(mls_idx, |idx| vertices[idx as usize].to_vec())
}

/// Create a GeoJSON MultiLineString from `flatgeom::MultiLineString` with a mapping function.
pub fn multilinestring_to_value_with_mapping<T: Coord>(
    mls: &MultiLineString<T>,
    mapping: impl Fn(T) -> Vec<f64>,
) -> geojson::Value {
    let coords = mls
        .iter()
        .map(|ls_idx| {
            ls_idx
                .iter()
                .map(&mapping) // Get the actual coord values
                .collect()
        })
        .collect();
    geojson::Value::MultiLineString(coords)
}

/// Create a GeoJSON Point from `flatgeom::Point`.
pub fn point_to_value(point: &[f64; 3]) -> geojson::Value {
    point_to_value_with_mapping(*point, |c| c.to_vec())
}

/// Create a GeoJSON Point from vertices and indices.
pub fn indexed_point_to_value(vertices: &[[f64; 3]], point_idx: u32) -> geojson::Value {
    point_to_value_with_mapping(point_idx, |idx| vertices[idx as usize].to_vec())
}

/// Create a GeoJSON Point from `flatgeom::Point` with a mapping function.
pub fn point_to_value_with_mapping<T: Coord>(
    point: T,
    mapping: impl Fn(T) -> Vec<f64>,
) -> geojson::Value {
    geojson::Value::Point(mapping(point))
}

/// Create a GeoJSON MultiPoint from `flatgeom::Point`.
pub fn multipoint_to_value(mpoint: &MultiPoint3) -> geojson::Value {
    multipoint_to_value_with_mapping(mpoint, |c| c.to_vec())
}

/// Create a GeoJSON MultiPoint from vertices and indices.
pub fn indexed_multipoint_to_value(
    vertices: &[[f64; 3]],
    mpoint_idx: &MultiPoint<u32>,
) -> geojson::Value {
    multipoint_to_value_with_mapping(mpoint_idx, |idx| vertices[idx as usize].to_vec())
}

/// Create a GeoJSON MultiPoint from `flatgeom::MultiPoint` with a mapping function.
pub fn multipoint_to_value_with_mapping<T: Coord>(
    mpoint: &MultiPoint<T>,
    mapping: impl Fn(T) -> Vec<f64>,
) -> geojson::Value {
    let coords = mpoint
        .iter()
        .map(&mapping) // Get the actual coord values
        .collect();
    geojson::Value::MultiPoint(coords)
}

#[cfg(test)]
mod tests {
    use flatgeom::{MultiPolygon3, Polygon3};

    use super::*;

    #[test]
    fn test_polygon() {
        let mut poly = Polygon3::new();
        //  polygon
        poly.add_ring([
            [10., 10., 0.],
            [10., 20., 0.],
            [20., 20., 0.],
            [20., 10., 0.], // not closed
        ]);
        poly.add_ring([
            [15., 15., 0.],
            [18., 10., 0.],
            [18., 18., 0.],
            [15., 18., 0.],
        ]);
        let value = polygon_to_value(&poly);
        let geojson::Value::Polygon(poly) = value else {
            panic!("The result is not a GeoJSON Polygon");
        };
        assert_eq!(
            poly,
            vec![
                vec![
                    vec![10., 10., 0.],
                    vec![10., 20., 0.],
                    vec![20., 20., 0.],
                    vec![20., 10., 0.],
                    vec![10., 10., 0.],
                ],
                vec![
                    vec![15., 15., 0.],
                    vec![18., 10., 0.],
                    vec![18., 18., 0.],
                    vec![15., 18., 0.],
                    vec![15., 15., 0.],
                ],
            ],
        );
    }

    #[test]
    fn test_indexed_polygon() {
        let vertices: Vec<[f64; 3]> = vec![
            // 1st polygon, exterior (vertex 0~3)
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
            // 1st polygon, interior 1 (vertex 4~7)
            [1., 1., 111.],
            [2., 1., 111.],
            [2., 2., 111.],
            [1., 2., 111.],
            // 1st polygon, interior 2 (vertex 8~11)
            [3., 3., 111.],
            [4., 3., 111.],
            [4., 4., 111.],
            [3., 4., 111.],
        ];

        let mut poly = Polygon::<u32>::new();
        // 1st polygon
        poly.add_ring([0, 1, 2, 3, 0]);
        poly.add_ring([4, 5, 6, 7, 4]);
        poly.add_ring([8, 9, 10, 11, 8]);

        let value = indexed_polygon_to_value(&vertices, &poly);

        let geojson::Value::Polygon(rings) = value else {
            panic!("The result is not a GeoJSON Polygon")
        };

        assert_eq!(
            rings,
            vec![
                vec![
                    [0., 0., 111.],
                    [5., 0., 111.],
                    [5., 5., 111.],
                    [0., 5., 111.],
                    [0., 0., 111.]
                ],
                vec![
                    [1., 1., 111.],
                    [2., 1., 111.],
                    [2., 2., 111.],
                    [1., 2., 111.],
                    [1., 1., 111.]
                ],
                vec![
                    [3., 3., 111.],
                    [4., 3., 111.],
                    [4., 4., 111.],
                    [3., 4., 111.],
                    [3., 3., 111.]
                ]
            ]
        );
    }

    #[test]
    fn test_multipolygon() {
        let mut mpoly = MultiPolygon3::new();
        // 1st polygon
        mpoly.add_exterior([
            [0., 0., 0.],
            [0., 10., 0.],
            [10., 10., 0.],
            [10., 0., 0.],
            [0., 0., 0.], // closed
        ]);
        //  polygon
        mpoly.add_exterior([
            [10., 10., 0.],
            [10., 20., 0.],
            [20., 20., 0.],
            [20., 10., 0.], // not closed
        ]);
        mpoly.add_interior([
            [15., 15., 0.],
            [18., 10., 0.],
            [18., 18., 0.],
            [15., 18., 0.],
        ]);
        let value = multipolygon_to_value(&mpoly);
        let geojson::Value::MultiPolygon(mpoly) = value else {
            panic!("The result is not a GeoJSON MultiPolygon");
        };
        assert_eq!(
            mpoly,
            vec![
                vec![vec![
                    vec![0., 0., 0.],
                    vec![0., 10., 0.],
                    vec![10., 10., 0.],
                    vec![10., 0., 0.],
                    vec![0., 0., 0.],
                ]],
                vec![
                    vec![
                        vec![10., 10., 0.],
                        vec![10., 20., 0.],
                        vec![20., 20., 0.],
                        vec![20., 10., 0.],
                        vec![10., 10., 0.],
                    ],
                    vec![
                        vec![15., 15., 0.],
                        vec![18., 10., 0.],
                        vec![18., 18., 0.],
                        vec![15., 18., 0.],
                        vec![15., 15., 0.],
                    ],
                ],
            ],
        );
    }

    #[test]
    fn test_indexed_multipolygon() {
        let vertices: Vec<[f64; 3]> = vec![
            // 1st polygon, exterior (vertex 0~3)
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
            // 1st polygon, interior 1 (vertex 4~7)
            [1., 1., 111.],
            [2., 1., 111.],
            [2., 2., 111.],
            [1., 2., 111.],
            // 1st polygon, interior 2 (vertex 8~11)
            [3., 3., 111.],
            [4., 3., 111.],
            [4., 4., 111.],
            [3., 4., 111.],
            // 2nd polygon, exterior (vertex 12~15)
            [4., 0., 222.],
            [7., 0., 222.],
            [7., 3., 222.],
            [4., 3., 222.],
            // 2nd polygon, interior (vertex 16~19)
            [5., 1., 222.],
            [6., 1., 222.],
            [6., 2., 222.],
            [5., 2., 222.],
            // 3rd polygon, exterior (vertex 20~23)
            [4., 0., 333.],
            [7., 0., 333.],
            [7., 3., 333.],
            [4., 3., 333.],
        ];

        let mut mpoly = MultiPolygon::<u32>::new();
        // 1st polygon
        mpoly.add_exterior([0, 1, 2, 3, 0]);
        mpoly.add_interior([4, 5, 6, 7, 4]);
        mpoly.add_interior([8, 9, 10, 11, 8]);
        // 2nd polygon
        mpoly.add_exterior([12, 13, 14, 15, 12]);
        mpoly.add_interior([16, 17, 18, 19, 16]);
        // 3rd polygon
        mpoly.add_exterior([20, 21, 22, 23, 20]);

        let value = indexed_multipolygon_to_value(&vertices, &mpoly);

        if let geojson::Value::MultiPolygon(rings_list) = value {
            for (i, rings) in rings_list.iter().enumerate() {
                match i {
                    0 => {
                        assert_eq!(rings.len(), 3);
                        assert_eq!(rings[0].len(), 5);
                        assert_eq!(rings[1].len(), 5);
                        assert_eq!(rings[2].len(), 5);
                        assert_eq!(
                            rings[0],
                            vec![
                                [0., 0., 111.],
                                [5., 0., 111.],
                                [5., 5., 111.],
                                [0., 5., 111.],
                                [0., 0., 111.]
                            ]
                        );
                        assert_eq!(
                            rings[1],
                            vec![
                                [1., 1., 111.],
                                [2., 1., 111.],
                                [2., 2., 111.],
                                [1., 2., 111.],
                                [1., 1., 111.]
                            ]
                        );
                        assert_eq!(
                            rings[2],
                            vec![
                                [3., 3., 111.],
                                [4., 3., 111.],
                                [4., 4., 111.],
                                [3., 4., 111.],
                                [3., 3., 111.]
                            ]
                        );
                    }
                    1 => {
                        assert_eq!(rings.len(), 2);
                        assert_eq!(rings[0].len(), 5);
                        assert_eq!(rings[1].len(), 5);
                        assert_eq!(
                            rings[0],
                            vec![
                                [4., 0., 222.],
                                [7., 0., 222.],
                                [7., 3., 222.],
                                [4., 3., 222.],
                                [4., 0., 222.]
                            ]
                        );
                        assert_eq!(
                            rings[1],
                            vec![
                                [5., 1., 222.],
                                [6., 1., 222.],
                                [6., 2., 222.],
                                [5., 2., 222.],
                                [5., 1., 222.]
                            ]
                        );
                    }
                    2 => {
                        assert_eq!(rings.len(), 1);
                        assert_eq!(rings[0].len(), 5);
                        assert_eq!(
                            rings[0],
                            vec![
                                [4., 0., 333.],
                                [7., 0., 333.],
                                [7., 3., 333.],
                                [4., 3., 333.],
                                [4., 0., 333.]
                            ]
                        );
                    }
                    _ => unreachable!("Unexpected number of polygons"),
                }
            }
        } else {
            unreachable!("The result is not a GeoJSON MultiPolygon");
        };
    }

    #[test]
    fn test_linestring() {
        let mut ls = LineString3::new();
        ls.push([11., 12., 13.]);
        ls.push([21., 22., 23.]);
        ls.push([31., 32., 33.]);

        let value = linestring_to_value(&ls);
        let geojson::Value::LineString(ls) = value else {
            panic!("The result is not a GeoJSON LineString");
        };
        assert_eq!(
            ls,
            vec![
                vec![11., 12., 13.],
                vec![21., 22., 23.],
                vec![31., 32., 33.],
            ],
        );
    }

    #[test]
    fn test_indexed_linestring() {
        let vertices = vec![[0., 0., 111.], [1., 1., 111.]];

        let mut ls = LineString::<u32>::new();
        ls.push(0);
        ls.push(1);

        let value = indexed_linestring_to_value(&vertices, &ls);
        let geojson::Value::LineString(ls) = value else {
            unreachable!();
        };
        assert_eq!(ls, vec![vec![0., 0., 111.], vec![1., 1., 111.]]);
    }

    #[test]
    fn test_multilinestring() {
        let mut mls = MultiLineString3::new();
        mls.add_linestring([[11., 12., 13.], [21., 22., 23.], [31., 32., 33.]]);
        mls.add_linestring([[111., 112., 113.], [121., 122., 123.], [131., 132., 133.]]);

        let value = multilinestring_to_value(&mls);
        let geojson::Value::MultiLineString(mls) = value else {
            panic!("The result is not a GeoJSON MultiPolygon");
        };
        assert_eq!(
            mls,
            vec![
                vec![
                    vec![11., 12., 13.],
                    vec![21., 22., 23.],
                    vec![31., 32., 33.],
                ],
                vec![
                    vec![111., 112., 113.],
                    vec![121., 122., 123.],
                    vec![131., 132., 133.],
                ],
            ]
        );
    }

    #[test]
    fn test_indexed_multilinestring() {
        let vertices = vec![
            // 1st linestring
            [0., 0., 111.],
            [1., 1., 111.],
            // 2nd linestring
            [2., 3., 222.],
            [4., 5., 222.],
            // 3rd linestring
            [6., 7., 333.],
            [8., 9., 333.],
            [10., 11., 333.],
        ];

        let mut mls = MultiLineString::<u32>::new();
        mls.add_linestring([0, 1]);
        mls.add_linestring([2, 3]);
        mls.add_linestring([4, 5, 6]);

        let value = indexed_multilinestring_to_value(&vertices, &mls);

        if let geojson::Value::MultiLineString(lines) = value {
            assert_eq!(lines.len(), mls.len());
            for (i, li) in lines.iter().enumerate() {
                match i {
                    0 => {
                        assert_eq!(li.len(), 2);
                        assert_eq!(li[0], [0., 0., 111.]);
                        assert_eq!(li[1], [1., 1., 111.]);
                    }
                    1 => {
                        assert_eq!(li.len(), 2);
                        assert_eq!(li[0], [2., 3., 222.]);
                        assert_eq!(li[1], [4., 5., 222.]);
                    }
                    2 => {
                        assert_eq!(li.len(), 3);
                        assert_eq!(li[0], [6., 7., 333.]);
                        assert_eq!(li[1], [8., 9., 333.]);
                        assert_eq!(li[2], [10., 11., 333.]);
                    }
                    _ => unreachable!("Unexpected number of lines"),
                }
            }
        } else {
            unreachable!("The result is not a GeoJSON MultiLineString");
        }
    }

    #[test]
    fn test_point() {
        let point = [11., 12., 13.];
        let value = point_to_value(&point);
        let geojson::Value::Point(point) = value else {
            panic!("The result is not a GeoJSON MultiPolygon");
        };
        assert_eq!(point, vec![11., 12., 13.],);
    }

    #[test]
    fn test_indexed_point() {
        let vertices = vec![[0., 0., 111.], [1., 2., 222.], [3., 4., 333.]];
        let point_idx = 1;

        let value = indexed_point_to_value(&vertices, point_idx);

        let geojson::Value::Point(point) = value else {
            unreachable!();
        };
        assert_eq!(point, [1., 2., 222.]);
    }

    #[test]
    fn test_multipoint() {
        let mut mpoint = MultiPoint3::new();
        mpoint.push([11., 12., 13.]);
        mpoint.push([21., 22., 23.]);
        mpoint.push([31., 32., 33.]);

        let value = multipoint_to_value(&mpoint);
        let geojson::Value::MultiPoint(mpoint) = value else {
            panic!("The result is not a GeoJSON MultiPolygon");
        };
        assert_eq!(
            mpoint,
            vec![
                vec![11., 12., 13.],
                vec![21., 22., 23.],
                vec![31., 32., 33.],
            ]
        );
    }

    #[test]
    fn test_indexed_multipoint() {
        let vertices = vec![[0., 0., 111.], [1., 2., 222.], [3., 4., 333.]];
        let mut mpoint = MultiPoint::<u32>::new();
        mpoint.push(0);
        mpoint.push(1);
        mpoint.push(2);

        let value = indexed_multipoint_to_value(&vertices, &mpoint);

        if let geojson::Value::MultiPoint(point_list) = value {
            assert_eq!(point_list.len(), mpoint.len());
            for (i, point) in point_list.iter().enumerate() {
                match i {
                    0 => {
                        assert_eq!(*point, vec![0., 0., 111.]);
                    }
                    1 => {
                        assert_eq!(*point, vec![1., 2., 222.]);
                    }
                    2 => {
                        assert_eq!(*point, vec![3., 4., 333.]);
                    }
                    _ => unreachable!("Unexpected number of points"),
                }
            }
        } else {
            unreachable!("The result is not a GeoJSON MultiPoint");
        }
    }
}
