use nusamai_geometry::{MultiLineString, MultiPolygon, Polygon};

pub fn multipolygon_to_geojson_geometry(
    vertices: &[[f64; 3]],
    mpoly: &MultiPolygon<1, u32>,
) -> geojson::Geometry {
    let ring_list: Vec<geojson::PolygonType> = mpoly
        .iter()
        .map(|poly| polygon_to_rings(vertices, &poly))
        .collect();

    geojson::Geometry::new(geojson::Value::MultiPolygon(ring_list))
}

fn polygon_to_rings(vertices: &[[f64; 3]], poly: &Polygon<1, u32>) -> geojson::PolygonType {
    let linestrings = std::iter::once(poly.exterior()).chain(poly.interiors());

    let rings: Vec<_> = linestrings
        .map(|ls| {
            let coords: Vec<_> = ls
                .iter_closed()
                .map(|idx| vertices[idx[0] as usize].to_vec()) // Get the actual coord values
                .collect();
            coords
        })
        .collect();

    rings
}

pub fn multilinestring_to_geojson_geometry(
    vertices: &[[f64; 3]],
    mls: &MultiLineString<1, u32>,
) -> geojson::Geometry {
    let mls_coords: Vec<geojson::LineStringType> = mls
        .iter()
        .map(|ls| {
            let coords: Vec<_> = ls
                .iter()
                .map(|idx| vertices[idx[0] as usize].to_vec()) // Get the actual coord values;
                .collect();
            coords
        })
        .collect();
    geojson::Geometry::new(geojson::Value::MultiLineString(mls_coords))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multipolygon() {
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

        let mut mpoly = MultiPolygon::<'_, 1, u32>::new();
        // 1st polygon
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        mpoly.add_interior([[4], [5], [6], [7], [4]]);
        mpoly.add_interior([[8], [9], [10], [11], [8]]);
        // 2nd polygon
        mpoly.add_exterior([[12], [13], [14], [15], [12]]);
        mpoly.add_interior([[16], [17], [18], [19], [16]]);
        // 3rd polygon
        mpoly.add_exterior([[20], [21], [22], [23], [20]]);

        let geojson_geometry = multipolygon_to_geojson_geometry(&vertices, &mpoly);

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

        let geojson_geometry = multilinestring_to_geojson_geometry(&vertices, &mls);

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
}
