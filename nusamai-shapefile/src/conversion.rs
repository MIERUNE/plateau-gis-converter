use nusamai_geometry::{CoordNum, MultiPoint};
use shapefile::NO_DATA;

pub fn multipoint3_to_shape(mpoint: &MultiPoint<3>) -> shapefile::MultipointZ {
    multipoint3_to_shape_with_mapping(mpoint, |c| c.to_vec())
}

pub fn multipoint3_to_shape_with_mapping<T: CoordNum>(
    mpoint: &MultiPoint<3, T>,
    mapping: impl Fn([T; 3]) -> Vec<f64>,
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

        let shape = multipoint3_to_shape(&mpoint);

        assert_eq!(shape.points().len(), 3);
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
