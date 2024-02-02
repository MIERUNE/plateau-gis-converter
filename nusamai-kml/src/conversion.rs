use kml::types::{Geometry, MultiGeometry, Point};
use nusamai_geometry::{CoordNum, MultiPoint};
use std::collections::HashMap;

/// Create a kml::MultiGeometry from `nusamai_geometry::MultiPoint` with a mapping function.
pub fn multipoint_to_kml_with_mapping<const D: usize, T: CoordNum>(
    mpoint: &MultiPoint<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> MultiGeometry {
    let points = mpoint
        .iter()
        .map(&mapping)
        .map(|coords| Point::new(coords[0], coords[1], Some(coords[2])))
        .collect::<Vec<_>>();
    MultiGeometry {
        geometries: points.into_iter().map(|pt| Geometry::Point(pt)).collect(),
        attrs: HashMap::new(),
    }
}

/// Create a kml::MultiGeometry from vertices and indices.
pub fn indexed_multipoint_to_kml(
    vertices: &[[f64; 3]],
    mpoint_idx: &MultiPoint<1, u32>,
) -> MultiGeometry {
    multipoint_to_kml_with_mapping(mpoint_idx, |idx| vertices[idx[0] as usize])
}

/// Create a kml::MultiGeometry from a nusamai_geometry::MultiPoint
pub fn multipoint_to_kml(mpoint: &MultiPoint<3>) -> MultiGeometry {
    multipoint_to_kml_with_mapping(mpoint, |c| c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kml::types::{Geometry, Point};

    #[test]
    fn test_multipoint_to_kml() {
        let mut mpoint = MultiPoint::<3>::new();
        mpoint.push(&[11., 12., 13.]);
        mpoint.push(&[21., 22., 23.]);
        mpoint.push(&[31., 32., 33.]);

        let multi_geom = multipoint_to_kml(&mpoint);

        assert_eq!(&multi_geom.geometries.len(), &3);

        assert_eq!(
            &multi_geom.geometries,
            &vec![
                Geometry::Point(Point::new(11., 12., Some(13.))),
                Geometry::Point(Point::new(21., 22., Some(23.))),
                Geometry::Point(Point::new(31., 32., Some(33.)))
            ]
        );
    }

    #[test]
    fn test_indexed_multipoint_to_kml() {
        let vertices = vec![[11., 12., 13.], [21., 22., 23.], [31., 32., 33.]];
        let mut mpoint_idx = MultiPoint::<1, u32>::new();
        mpoint_idx.push(&[0]);
        mpoint_idx.push(&[1]);
        mpoint_idx.push(&[2]);

        let multi_geom = indexed_multipoint_to_kml(&vertices, &mpoint_idx);

        assert_eq!(&multi_geom.geometries.len(), &3);

        assert_eq!(
            &multi_geom.geometries,
            &vec![
                Geometry::Point(Point::new(11., 12., Some(13.))),
                Geometry::Point(Point::new(21., 22., Some(23.))),
                Geometry::Point(Point::new(31., 32., Some(33.)))
            ]
        );
    }
}
