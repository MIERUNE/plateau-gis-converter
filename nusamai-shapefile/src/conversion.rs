use nusamai_geometry::{CoordNum, MultiPoint, MultiPoint3};
use shapefile::NO_DATA;

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
