use flatgeom::{
    Coord, MultiLineString, MultiLineString3, MultiPoint, MultiPoint3, MultiPolygon, MultiPolygon3,
    Polygon, Polygon3,
};
use shapefile::NO_DATA;

/// Create a Shapefile Polygon from `flatgeom::Polygon`.
pub fn polygon_to_shape(poly: &Polygon3) -> shapefile::PolygonZ {
    polygon_to_shape_with_mapping(poly, |c| c)
}

/// Create a Shapefile Polygon from vertices and indices.
pub fn indexed_polygon_to_shape(
    vertices: &[[f64; 3]],
    poly_idx: &Polygon<u32>,
) -> shapefile::PolygonZ {
    polygon_to_shape_with_mapping(poly_idx, |idx| vertices[idx as usize])
}

/// Create a Shapefile Polygon from `flatgeom::Polygon` with a mapping function.
pub fn polygon_to_shape_with_mapping<T: Coord>(
    poly: &Polygon<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> shapefile::PolygonZ {
    let all_rings = polygon_to_shape_rings_with_mapping(poly, &mapping);
    shapefile::PolygonZ::with_rings(all_rings)
}

/// Create a Shapefile Polygon from `flatgeom::MultiPolygon`.
pub fn multipolygon_to_shape(mpoly: &MultiPolygon3) -> shapefile::PolygonZ {
    multipolygon_to_shape_with_mapping(mpoly, |c| c)
}

/// Create a Shapefile Polygon from vertices and indices.
pub fn indexed_multipolygon_to_shape(
    vertices: &[[f64; 3]],
    mpoly_idx: &MultiPolygon<u32>,
) -> shapefile::PolygonZ {
    multipolygon_to_shape_with_mapping(mpoly_idx, |idx| vertices[idx as usize])
}

/// Create a Shapefile Polygon from `flatgeom::MultiPolygon` with a mapping function.
pub fn multipolygon_to_shape_with_mapping<T: Coord>(
    mpoly: &MultiPolygon<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> shapefile::PolygonZ {
    let all_rings = mpoly
        .iter()
        .flat_map(|poly| polygon_to_shape_rings_with_mapping(&poly, &mapping))
        .collect::<Vec<shapefile::PolygonRing<shapefile::PointZ>>>();

    shapefile::PolygonZ::with_rings(all_rings)
}

fn polygon_to_shape_rings_with_mapping<T: Coord>(
    poly: &Polygon<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> Vec<shapefile::PolygonRing<shapefile::PointZ>> {
    let outer_points = poly
        .exterior()
        .iter_closed()
        .map(&mapping)
        .map(|coords| shapefile::PointZ::new(coords[0], coords[1], coords[2], NO_DATA))
        .collect::<Vec<shapefile::PointZ>>();
    let outer_ring = shapefile::PolygonRing::Outer(outer_points);

    let inner_rings = poly
        .interiors()
        .map(|ring| {
            ring.iter_closed()
                .map(&mapping)
                .map(|coords| shapefile::PointZ::new(coords[0], coords[1], coords[2], NO_DATA))
                .collect::<Vec<shapefile::PointZ>>()
        })
        .map(shapefile::PolygonRing::Inner)
        .collect::<Vec<shapefile::PolygonRing<shapefile::PointZ>>>();

    let mut all_rings = vec![outer_ring];
    all_rings.extend(inner_rings);
    all_rings
}

/// Create a Shapefile PolylineZ from `flatgeom::MultiLineString`.
pub fn multilinestring_to_shape(mls: &MultiLineString3) -> shapefile::PolylineZ {
    multilinestring_to_shape_with_mapping(mls, |c| c)
}

/// Create a Shapefile PolylineZ from vertices and indices.
pub fn indexed_multilinestring_to_shape(
    vertices: &[[f64; 3]],
    mls_idx: &MultiLineString<u32>,
) -> shapefile::PolylineZ {
    multilinestring_to_shape_with_mapping(mls_idx, |idx| vertices[idx as usize])
}

/// Create a Shapefile PolylineZ from `flatgeom::MultiLineString` with a mapping function.
pub fn multilinestring_to_shape_with_mapping<T: Coord>(
    mls: &MultiLineString<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> shapefile::PolylineZ {
    let parts = mls
        .iter()
        .map(|ls| {
            ls.iter()
                .map(&mapping)
                .map(|coords| shapefile::PointZ::new(coords[0], coords[1], coords[2], NO_DATA))
                .collect()
        })
        .collect::<Vec<Vec<shapefile::PointZ>>>();

    shapefile::PolylineZ::with_parts(parts)
}

/// Create a Shapefile MultiPointZ from `flatgeom::MultiPoint`.
pub fn multipoint_to_shape(mpoint: &MultiPoint3) -> shapefile::MultipointZ {
    multipoint_to_shape_with_mapping(mpoint, |c| c)
}

/// Create a Shapefile MultiPointZ from vertices and indices.
pub fn indexed_multipoint_to_shape(
    vertices: &[[f64; 3]],
    mpoint_idx: &MultiPoint<u32>,
) -> shapefile::MultipointZ {
    multipoint_to_shape_with_mapping(mpoint_idx, |idx| vertices[idx as usize])
}

/// Create a Shapefile MultiPointZ from `flatgeom::MultiPoint` with a mapping function.
pub fn multipoint_to_shape_with_mapping<T: Coord>(
    mpoint: &MultiPoint<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> shapefile::MultipointZ {
    let shape_points = mpoint
        .iter()
        .map(&mapping)
        .map(|coords| shapefile::PointZ::new(coords[0], coords[1], coords[2], NO_DATA))
        .collect::<Vec<_>>();

    shapefile::MultipointZ::from(shape_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_to_shape() {
        let mut poly = Polygon3::new();
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

        let shape = polygon_to_shape(&poly);

        assert_eq!(shape.rings().len(), poly.rings().collect::<Vec<_>>().len());
        assert_eq!(
            shape.ring(0).unwrap(),
            &shapefile::PolygonRing::Outer(vec![
                shapefile::PointZ::new(10., 10., 0., NO_DATA),
                shapefile::PointZ::new(10., 20., 0., NO_DATA),
                shapefile::PointZ::new(20., 20., 0., NO_DATA),
                shapefile::PointZ::new(20., 10., 0., NO_DATA),
                shapefile::PointZ::new(10., 10., 0., NO_DATA), // automatically closed
            ])
        );
        assert_eq!(
            shape.ring(1).unwrap(),
            &shapefile::PolygonRing::Inner(vec![
                shapefile::PointZ::new(15., 15., 0., NO_DATA),
                shapefile::PointZ::new(18., 10., 0., NO_DATA),
                shapefile::PointZ::new(18., 18., 0., NO_DATA),
                shapefile::PointZ::new(15., 18., 0., NO_DATA),
                shapefile::PointZ::new(15., 15., 0., NO_DATA),
            ])
        );
    }

    #[test]
    fn test_indexed_polygon_to_shape() {
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

        let shape = indexed_polygon_to_shape(&vertices, &poly);

        assert_eq!(shape.rings().len(), poly.rings().collect::<Vec<_>>().len());
        assert_eq!(
            shape.ring(0).unwrap(),
            &shapefile::PolygonRing::Outer(vec![
                // Outer ring: re-ordered to clockwise
                shapefile::PointZ::new(0., 0., 111., NO_DATA),
                shapefile::PointZ::new(0., 5., 111., NO_DATA),
                shapefile::PointZ::new(5., 5., 111., NO_DATA),
                shapefile::PointZ::new(5., 0., 111., NO_DATA),
                shapefile::PointZ::new(0., 0., 111., NO_DATA),
            ])
        );
        assert_eq!(
            shape.ring(1).unwrap(),
            &shapefile::PolygonRing::Inner(vec![
                shapefile::PointZ::new(1., 1., 111., NO_DATA),
                shapefile::PointZ::new(2., 1., 111., NO_DATA),
                shapefile::PointZ::new(2., 2., 111., NO_DATA),
                shapefile::PointZ::new(1., 2., 111., NO_DATA),
                shapefile::PointZ::new(1., 1., 111., NO_DATA),
            ])
        );
        assert_eq!(
            shape.ring(2).unwrap(),
            &shapefile::PolygonRing::Inner(vec![
                shapefile::PointZ::new(3., 3., 111., NO_DATA),
                shapefile::PointZ::new(4., 3., 111., NO_DATA),
                shapefile::PointZ::new(4., 4., 111., NO_DATA),
                shapefile::PointZ::new(3., 4., 111., NO_DATA),
                shapefile::PointZ::new(3., 3., 111., NO_DATA),
            ])
        );
    }

    #[test]
    fn test_multipolygon_to_shape() {
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
            [15., 18., 0.], // not closed
        ]);

        let shape = multipolygon_to_shape(&mpoly);

        assert_eq!(
            shape.rings().len(),
            mpoly
                .iter()
                .map(|poly| poly.rings().collect::<Vec<_>>().len())
                .sum()
        );
        assert_eq!(
            shape.ring(0).unwrap(),
            &shapefile::PolygonRing::Outer(vec![
                shapefile::PointZ::new(0., 0., 0., NO_DATA),
                shapefile::PointZ::new(0., 10., 0., NO_DATA),
                shapefile::PointZ::new(10., 10., 0., NO_DATA),
                shapefile::PointZ::new(10., 0., 0., NO_DATA),
                shapefile::PointZ::new(0., 0., 0., NO_DATA),
            ])
        );
        assert_eq!(
            shape.ring(1).unwrap(),
            &shapefile::PolygonRing::Outer(vec![
                shapefile::PointZ::new(10., 10., 0., NO_DATA),
                shapefile::PointZ::new(10., 20., 0., NO_DATA),
                shapefile::PointZ::new(20., 20., 0., NO_DATA),
                shapefile::PointZ::new(20., 10., 0., NO_DATA),
                shapefile::PointZ::new(10., 10., 0., NO_DATA), // closed
            ])
        );
        assert_eq!(
            shape.ring(2).unwrap(),
            &shapefile::PolygonRing::Inner(vec![
                shapefile::PointZ::new(15., 15., 0., NO_DATA),
                shapefile::PointZ::new(18., 10., 0., NO_DATA),
                shapefile::PointZ::new(18., 18., 0., NO_DATA),
                shapefile::PointZ::new(15., 18., 0., NO_DATA),
                shapefile::PointZ::new(15., 15., 0., NO_DATA), // closed
            ])
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

        let shape = indexed_multipolygon_to_shape(&vertices, &mpoly);

        assert_eq!(
            shape.rings().len(),
            mpoly
                .iter()
                .map(|poly| poly.rings().collect::<Vec<_>>().len())
                .sum()
        );
        assert_eq!(
            shape.ring(0).unwrap(),
            &shapefile::PolygonRing::Outer(vec![
                // Outer ring: re-ordered to clockwise
                shapefile::PointZ::new(0., 0., 111., NO_DATA),
                shapefile::PointZ::new(0., 5., 111., NO_DATA),
                shapefile::PointZ::new(5., 5., 111., NO_DATA),
                shapefile::PointZ::new(5., 0., 111., NO_DATA),
                shapefile::PointZ::new(0., 0., 111., NO_DATA),
            ])
        );
        assert_eq!(
            shape.ring(1).unwrap(),
            &shapefile::PolygonRing::Inner(vec![
                shapefile::PointZ::new(1., 1., 111., NO_DATA),
                shapefile::PointZ::new(2., 1., 111., NO_DATA),
                shapefile::PointZ::new(2., 2., 111., NO_DATA),
                shapefile::PointZ::new(1., 2., 111., NO_DATA),
                shapefile::PointZ::new(1., 1., 111., NO_DATA),
            ])
        );
        assert_eq!(
            shape.ring(2).unwrap(),
            &shapefile::PolygonRing::Inner(vec![
                shapefile::PointZ::new(3., 3., 111., NO_DATA),
                shapefile::PointZ::new(4., 3., 111., NO_DATA),
                shapefile::PointZ::new(4., 4., 111., NO_DATA),
                shapefile::PointZ::new(3., 4., 111., NO_DATA),
                shapefile::PointZ::new(3., 3., 111., NO_DATA),
            ])
        );
        assert_eq!(
            shape.ring(3).unwrap(),
            &shapefile::PolygonRing::Outer(vec![
                // Outer ring: re-ordered to clockwise
                shapefile::PointZ::new(4., 0., 222., NO_DATA),
                shapefile::PointZ::new(4., 3., 222., NO_DATA),
                shapefile::PointZ::new(7., 3., 222., NO_DATA),
                shapefile::PointZ::new(7., 0., 222., NO_DATA),
                shapefile::PointZ::new(4., 0., 222., NO_DATA),
            ])
        );
        assert_eq!(
            shape.ring(4).unwrap(),
            &shapefile::PolygonRing::Inner(vec![
                shapefile::PointZ::new(5., 1., 222., NO_DATA),
                shapefile::PointZ::new(6., 1., 222., NO_DATA),
                shapefile::PointZ::new(6., 2., 222., NO_DATA),
                shapefile::PointZ::new(5., 2., 222., NO_DATA),
                shapefile::PointZ::new(5., 1., 222., NO_DATA),
            ])
        );
        assert_eq!(
            shape.ring(5).unwrap(),
            &shapefile::PolygonRing::Outer(vec![
                // Outer ring: re-ordered to clockwise
                shapefile::PointZ::new(4., 0., 333., NO_DATA),
                shapefile::PointZ::new(4., 3., 333., NO_DATA),
                shapefile::PointZ::new(7., 3., 333., NO_DATA),
                shapefile::PointZ::new(7., 0., 333., NO_DATA),
                shapefile::PointZ::new(4., 0., 333., NO_DATA),
            ])
        );
    }

    #[test]
    fn test_multilinestring_to_shape() {
        let mut mls = MultiLineString3::new();
        mls.add_linestring([[11., 12., 13.], [21., 22., 23.], [31., 32., 33.]]);
        mls.add_linestring([[111., 112., 113.], [121., 122., 123.], [131., 132., 133.]]);

        let shape = multilinestring_to_shape(&mls);

        assert_eq!(shape.parts().len(), mls.len());
        assert_eq!(
            shape.part(0).unwrap(),
            &vec![
                shapefile::PointZ::new(11., 12., 13., NO_DATA),
                shapefile::PointZ::new(21., 22., 23., NO_DATA),
                shapefile::PointZ::new(31., 32., 33., NO_DATA),
            ]
        );
        assert_eq!(
            shape.part(1).unwrap(),
            &vec![
                shapefile::PointZ::new(111., 112., 113., NO_DATA),
                shapefile::PointZ::new(121., 122., 123., NO_DATA),
                shapefile::PointZ::new(131., 132., 133., NO_DATA),
            ]
        )
    }

    #[test]
    fn test_indexed_multilinestring_to_shape() {
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

        let shape = indexed_multilinestring_to_shape(&vertices, &mls);

        assert_eq!(shape.parts().len(), mls.len());
        assert_eq!(
            shape.part(0).unwrap(),
            &vec![
                shapefile::PointZ::new(0., 0., 111., NO_DATA),
                shapefile::PointZ::new(1., 1., 111., NO_DATA),
            ]
        );
        assert_eq!(
            shape.part(1).unwrap(),
            &vec![
                shapefile::PointZ::new(2., 3., 222., NO_DATA),
                shapefile::PointZ::new(4., 5., 222., NO_DATA),
            ]
        );
        assert_eq!(
            shape.part(2).unwrap(),
            &vec![
                shapefile::PointZ::new(6., 7., 333., NO_DATA),
                shapefile::PointZ::new(8., 9., 333., NO_DATA),
                shapefile::PointZ::new(10., 11., 333., NO_DATA),
            ]
        )
    }

    #[test]
    fn test_multipoint_to_shape() {
        let mut mpoint = MultiPoint3::new();
        mpoint.push([11., 12., 13.]);
        mpoint.push([21., 22., 23.]);
        mpoint.push([31., 32., 33.]);

        let shape = multipoint_to_shape(&mpoint);

        assert_eq!(shape.points().len(), mpoint.len());
        assert_eq!(
            shape.point(0).unwrap(),
            &shapefile::PointZ::new(11., 12., 13., NO_DATA)
        );
        assert_eq!(
            shape.point(1).unwrap(),
            &shapefile::PointZ::new(21., 22., 23., NO_DATA)
        );
        assert_eq!(
            shape.point(2).unwrap(),
            &shapefile::PointZ::new(31., 32., 33., NO_DATA)
        );
    }

    #[test]
    fn test_indexed_multipoint() {
        let vertices = vec![[11., 12., 13.], [21., 22., 23.], [31., 32., 33.]];
        let mut mpoint = MultiPoint::<u32>::new();
        mpoint.push(0);
        mpoint.push(1);
        mpoint.push(2);

        let shape = indexed_multipoint_to_shape(&vertices, &mpoint);

        assert_eq!(shape.points().len(), mpoint.len());
        assert_eq!(
            shape.point(0).unwrap(),
            &shapefile::PointZ::new(11., 12., 13., NO_DATA)
        );
        assert_eq!(
            shape.point(1).unwrap(),
            &shapefile::PointZ::new(21., 22., 23., NO_DATA)
        );
        assert_eq!(
            shape.point(2).unwrap(),
            &shapefile::PointZ::new(31., 32., 33., NO_DATA)
        );
    }
}
