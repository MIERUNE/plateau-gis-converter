use nusamai_geometry::{CoordNum, MultiLineString, MultiPoint, MultiPolygon};

/// Create a GeoJSON MultiPolygon from `nusamai_geometry::MultiPolygon`.
pub fn multipolygon_to_geometry(mpoly: &MultiPolygon<3>) -> geojson::Geometry {
    multipolygon_to_geometry_with_mapping(mpoly, |c| c.to_vec())
}

/// Create a GeoJSON MultiPolygon from vertices and indices.
pub fn indexed_multipolygon_to_geometry(
    vertices: &[[f64; 3]],
    mpoly_idx: &MultiPolygon<1, u32>,
) -> geojson::Geometry {
    multipolygon_to_geometry_with_mapping(mpoly_idx, |idx| vertices[idx[0] as usize].to_vec())
}

/// Create a GeoJSON MultiPolygon from `nusamai_geometry::MultiPolygon` with a mapping function.
pub fn multipolygon_to_geometry_with_mapping<const D: usize, T: CoordNum>(
    mpoly: &MultiPolygon<D, T>,
    mapping: impl Fn(&[T]) -> Vec<f64>,
) -> geojson::Geometry {
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
    geojson::Value::MultiPolygon(coords).into()
}

/// Create a GeoJSON MultiLineString from `nusamai_geometry::MultiLineString`.
pub fn multilinestring_to_geometry(mls: &MultiLineString<3>) -> geojson::Geometry {
    multilinestring_to_geometry_with_mapping(mls, |c| c.to_vec())
}

/// Create a GeoJSON MultiLineString from vertices and indices.
pub fn indexed_multilinestring_to_geometry(
    vertices: &[[f64; 3]],
    mls_idx: &MultiLineString<1, u32>,
) -> geojson::Geometry {
    multilinestring_to_geometry_with_mapping(mls_idx, |idx| vertices[idx[0] as usize].to_vec())
}

/// Create a GeoJSON MultiLineString from `nusamai_geometry::MultiPolygon` with a mapping function.
pub fn multilinestring_to_geometry_with_mapping<const D: usize, T: CoordNum>(
    mls: &MultiLineString<D, T>,
    mapping: impl Fn(&[T]) -> Vec<f64>,
) -> geojson::Geometry {
    let coords = mls
        .iter()
        .map(|ls_idx| {
            ls_idx
                .iter()
                .map(&mapping) // Get the actual coord values
                .collect()
        })
        .collect();
    geojson::Value::MultiLineString(coords).into()
}

/// Create a GeoJSON MultiPoint from `nusamai_geometry::MultiPoint`.
pub fn multipoint_to_geometry(mpoint: &MultiPoint<3>) -> geojson::Geometry {
    multipoint_to_geometry_with_mapping(mpoint, |c| c.to_vec())
}

/// Create a GeoJSON MultiPoint from vertices and indices.
pub fn indexed_multipoint_to_geometry(
    vertices: &[[f64; 3]],
    mpoint_idx: &MultiPoint<1, u32>,
) -> geojson::Geometry {
    multipoint_to_geometry_with_mapping(mpoint_idx, |idx| vertices[idx[0] as usize].to_vec())
}

/// Create a GeoJSON MultiPoint from `nusamai_geometry::MultiPoint` with a mapping function.
pub fn multipoint_to_geometry_with_mapping<const D: usize, T: CoordNum>(
    mpoint: &MultiPoint<D, T>,
    mapping: impl Fn(&[T]) -> Vec<f64>,
) -> geojson::Geometry {
    let coords = mpoint
        .iter()
        .map(&mapping) // Get the actual coord values
        .collect();
    geojson::Value::MultiPoint(coords).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multipolygon() {
        let mut mpoly = MultiPolygon::<3>::new();
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
        let geom = multipolygon_to_geometry(&mpoly);
        let geojson::Value::MultiPolygon(mpoly) = geom.value else {
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

        let mut mpoly = MultiPolygon::<1, u32>::new();
        // 1st polygon
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        mpoly.add_interior([[4], [5], [6], [7], [4]]);
        mpoly.add_interior([[8], [9], [10], [11], [8]]);
        // 2nd polygon
        mpoly.add_exterior([[12], [13], [14], [15], [12]]);
        mpoly.add_interior([[16], [17], [18], [19], [16]]);
        // 3rd polygon
        mpoly.add_exterior([[20], [21], [22], [23], [20]]);

        let geojson_geometry = indexed_multipolygon_to_geometry(&vertices, &mpoly);

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
    fn test_multilinestring() {
        let mut mls = MultiLineString::<3>::new();
        mls.add_linestring([[11., 12., 13.], [21., 22., 23.], [31., 32., 33.]]);
        mls.add_linestring([[111., 112., 113.], [121., 122., 123.], [131., 132., 133.]]);

        let geom = multilinestring_to_geometry(&mls);
        let geojson::Value::MultiLineString(mls) = geom.value else {
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

        let mut mls = MultiLineString::<1, u32>::new();
        mls.add_linestring([[0], [1]]);
        mls.add_linestring([[2], [3]]);
        mls.add_linestring([[4], [5], [6]]);

        let geojson_geometry = indexed_multilinestring_to_geometry(&vertices, &mls);

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());
        if let geojson::Value::MultiLineString(lines) = geojson_geometry.value {
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
    fn test_multipoint() {
        let mut mpoint = MultiPoint::<3>::new();
        mpoint.push(&[11., 12., 13.]);
        mpoint.push(&[21., 22., 23.]);
        mpoint.push(&[31., 32., 33.]);

        let geom = multipoint_to_geometry(&mpoint);
        let geojson::Value::MultiPoint(mpoint) = geom.value else {
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
        let mut mpoint = MultiPoint::<1, u32>::new();
        mpoint.push(&[0]);
        mpoint.push(&[1]);
        mpoint.push(&[2]);

        let geojson_geometry = indexed_multipoint_to_geometry(&vertices, &mpoint);

        assert!(geojson_geometry.bbox.is_none());
        assert!(geojson_geometry.foreign_members.is_none());
        if let geojson::Value::MultiPoint(point_list) = geojson_geometry.value {
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
