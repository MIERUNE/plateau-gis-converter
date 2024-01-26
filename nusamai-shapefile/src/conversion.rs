use nusamai_geometry::{CoordNum, MultiLineString, MultiLineString3, MultiPoint, MultiPoint3};
use shapefile::NO_DATA;

/// Create a Shapefile PolylineZ from `nusamai_geometry::MultiLineString`.
pub fn multilinestring_to_shape(mls: &MultiLineString3) -> shapefile::PolylineZ {
    multilinestring_to_shape_with_mapping(mls, |c| c)
}

/// Create a Shapefile PolylineZ from vertices and indices.
pub fn indexed_multilinestring_to_shape(
    vertices: &[[f64; 3]],
    mls_idx: &MultiLineString<1, u32>,
) -> shapefile::PolylineZ {
    multilinestring_to_shape_with_mapping(mls_idx, |idx| vertices[idx[0] as usize])
}

/// Create a Shapefile PolylineZ from `nusamai_geometry::MultiLineString` with a mapping function.
pub fn multilinestring_to_shape_with_mapping<const D: usize, T: CoordNum>(
    mls: &MultiLineString<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
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

/// Create a Shapefile MultiPointZ from `nusamai_geometry::MultiPoint`.
pub fn multipoint_to_shape(mpoint: &MultiPoint3) -> shapefile::MultipointZ {
    multipoint_to_shape_with_mapping(mpoint, |c| c)
}

/// Create a Shapefile MultiPointZ from vertices and indices.
pub fn indexed_multipoint_to_shape(
    vertices: &[[f64; 3]],
    mpoint_idx: &MultiPoint<1, u32>,
) -> shapefile::MultipointZ {
    multipoint_to_shape_with_mapping(mpoint_idx, |idx| vertices[idx[0] as usize])
}

/// Create a Shapefile MultiPointZ from `nusamai_geometry::MultiPoint` with a mapping function.
pub fn multipoint_to_shape_with_mapping<const D: usize, T: CoordNum>(
    mpoint: &MultiPoint<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
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
    fn test_multilinestring_to_shape() {
        let mut mls = MultiLineString::<3>::new();
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

        let mut mls = MultiLineString::<1, u32>::new();
        mls.add_linestring([[0], [1]]);
        mls.add_linestring([[2], [3]]);
        mls.add_linestring([[4], [5], [6]]);

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
        let mut mpoint = MultiPoint::<3>::new();
        mpoint.push(&[11., 12., 13.]);
        mpoint.push(&[21., 22., 23.]);
        mpoint.push(&[31., 32., 33.]);

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
        let mut mpoint = MultiPoint::<1, u32>::new();
        mpoint.push(&[0]);
        mpoint.push(&[1]);
        mpoint.push(&[2]);

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
