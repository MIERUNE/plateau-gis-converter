use kml::types::{
    AltitudeMode, Coord, Geometry, LinearRing, MultiGeometry, Point, Polygon as KmlPolygon,
};
use nusamai_geometry::{CoordNum, MultiPoint, MultiPolygon, Polygon};
use std::{collections::HashMap, vec};

const EXTRUDE: bool = false;
const ALTITUDE_MODE: AltitudeMode = AltitudeMode::RelativeToGround;

pub fn multipolygon_to_kml(mpoly: &MultiPolygon<3>) -> Vec<KmlPolygon> {
    multipolygon_to_kml_with_mapping(mpoly, |c| c)
}

pub fn indexed_multipolygon_to_kml(
    vertices: &[[f64; 3]],
    mpoly_idx: &MultiPolygon<1, u32>,
) -> Vec<KmlPolygon> {
    multipolygon_to_kml_with_mapping(mpoly_idx, |idx| vertices[idx[0] as usize])
}

fn multipolygon_to_kml_with_mapping<const D: usize, T: CoordNum>(
    mpoly: &MultiPolygon<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> Vec<KmlPolygon> {
    mpoly
        .iter()
        .flat_map(|poly| polygon_to_kml_with_mapping(&poly, &mapping)) // Flatten the vector of vectors
        .collect()
}

fn polygon_to_kml_polygon_with_mapping<const D: usize, T: CoordNum>(
    poly: &Polygon<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> KmlPolygon {
    KmlPolygon {
        outer: polygon_to_kml_outer_boundary_with_mapping(poly, &mapping),
        inner: polygon_to_kml_inner_boundary_with_mapping(poly, &mapping),
        extrude: EXTRUDE,
        tessellate: false,
        altitude_mode: ALTITUDE_MODE,
        attrs: HashMap::new(),
    }
}

fn polygon_to_kml_outer_boundary_with_mapping<const D: usize, T: CoordNum>(
    poly: &Polygon<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> LinearRing {
    let outer_coords: Vec<Coord> = poly
        .exterior()
        .iter_closed()
        .map(&mapping)
        .map(|[x, y, z]| Coord { x, y, z: Some(z) })
        .collect();

    LinearRing {
        coords: outer_coords,
        extrude: EXTRUDE,
        tessellate: false,
        altitude_mode: ALTITUDE_MODE,
        attrs: HashMap::new(),
    }
}

fn polygon_to_kml_inner_boundary_with_mapping<const D: usize, T: CoordNum>(
    poly: &Polygon<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> Vec<LinearRing> {
    poly.interiors()
        .map(|ring| {
            ring.iter_closed()
                .map(&mapping)
                .map(|[x, y, z]| Coord { x, y, z: Some(z) })
                .collect()
        })
        .map(|coords| LinearRing {
            coords,
            extrude: EXTRUDE,
            tessellate: false,
            altitude_mode: ALTITUDE_MODE,
            attrs: HashMap::new(),
        })
        .collect()
}

/// Create a kml::MultiGeometry with Polygon from `nusamai_geometry::MultiPoint` with a mapping function.
pub fn polygon_to_kml_with_mapping<const D: usize, T: CoordNum>(
    poly: &Polygon<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> Vec<KmlPolygon> {
    vec![polygon_to_kml_polygon_with_mapping(poly, mapping)]
}

/// Create a kml::MultiGeometry from a nusamai_geometry::MultiPolygon
pub fn polygon_to_kml(poly: &Polygon<3>) -> Vec<KmlPolygon> {
    polygon_to_kml_with_mapping(poly, |c| c)
}

/// Create a kml::MultiGeometry with Polygon vertices and indices.
pub fn indexed_polygon_to_kml(
    vertices: &[[f64; 3]],
    poly_idx: &Polygon<1, u32>,
) -> Vec<KmlPolygon> {
    polygon_to_kml_with_mapping(poly_idx, |idx| vertices[idx[0] as usize])
}

/// Create a kml::MultiGeometry with Points from `nusamai_geometry::MultiPoint` with a mapping function.
pub fn multipoint_to_kml_with_mapping<const D: usize, T: CoordNum>(
    mpoint: &MultiPoint<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> MultiGeometry {
    MultiGeometry {
        geometries: mpoint
            .iter()
            .map(&mapping)
            .map(|coord| Geometry::Point(Point::new(coord[0], coord[1], Some(coord[2]))))
            .collect(),
        attrs: HashMap::new(),
    }
}

/// Create a kml::MultiGeometry with Points vertices and indices.
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
    use nusamai_geometry::Polygon3;

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

    #[test]
    fn test_polygon_to_kml() {
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

        let polygons = polygon_to_kml(&poly);

        assert_eq!(polygons[0].outer.coords.len(), 5);
        assert_eq!(
            polygons[0].outer.coords[0],
            Coord {
                x: 10.,
                y: 10.,
                z: Some(0.)
            }
        );
        assert_eq!(
            polygons[0].outer.coords[1],
            Coord {
                x: 10.,
                y: 20.,
                z: Some(0.)
            }
        );
        assert_eq!(
            polygons[0].outer.coords[2],
            Coord {
                x: 20.,
                y: 20.,
                z: Some(0.)
            }
        );
        assert_eq!(
            polygons[0].outer.coords[3],
            Coord {
                x: 20.,
                y: 10.,
                z: Some(0.)
            }
        );
        assert_eq!(
            polygons[0].outer.coords[4],
            Coord {
                x: 10.,
                y: 10.,
                z: Some(0.)
            }
        );

        assert_eq!(polygons[0].inner[0].coords.len(), 5);
        assert_eq!(
            polygons[0].inner[0].coords[0],
            Coord {
                x: 15.,
                y: 15.,
                z: Some(0.)
            }
        );
        assert_eq!(
            polygons[0].inner[0].coords[1],
            Coord {
                x: 18.,
                y: 10.,
                z: Some(0.)
            }
        );
        assert_eq!(
            polygons[0].inner[0].coords[2],
            Coord {
                x: 18.,
                y: 18.,
                z: Some(0.)
            }
        );
        assert_eq!(
            polygons[0].inner[0].coords[3],
            Coord {
                x: 15.,
                y: 18.,
                z: Some(0.)
            }
        );
        assert_eq!(
            polygons[0].inner[0].coords[4],
            Coord {
                x: 15.,
                y: 15.,
                z: Some(0.)
            }
        );
    }

    #[test]
    fn test_indexed_polygon_to_kml() {
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

        let mut poly = Polygon::<1, u32>::new();
        poly.add_ring([[0], [1], [2], [3], [0]]);
        poly.add_ring([[4], [5], [6], [7], [4]]);
        poly.add_ring([[8], [9], [10], [11], [8]]);

        let polygons = indexed_polygon_to_kml(&vertices, &poly);

        assert_eq!(polygons.len(), 1);

        assert_eq!(polygons[0].outer.coords.len(), 5);
        assert_eq!(
            polygons[0].outer.coords[0],
            Coord {
                x: 0.,
                y: 0.,
                z: Some(111.)
            }
        );
        assert_eq!(
            polygons[0].outer.coords[1],
            Coord {
                x: 5.,
                y: 0.,
                z: Some(111.)
            }
        );
        assert_eq!(
            polygons[0].outer.coords[2],
            Coord {
                x: 5.,
                y: 5.,
                z: Some(111.)
            }
        );
        assert_eq!(
            polygons[0].outer.coords[3],
            Coord {
                x: 0.,
                y: 5.,
                z: Some(111.)
            }
        );
        assert_eq!(
            polygons[0].outer.coords[4],
            Coord {
                x: 0.,
                y: 0.,
                z: Some(111.)
            }
        );

        assert_eq!(polygons[0].inner[0].coords.len(), 5);
        assert_eq!(
            polygons[0].inner[0].coords[0],
            Coord {
                x: 1.,
                y: 1.,
                z: Some(111.)
            }
        );
        assert_eq!(
            polygons[0].inner[0].coords[1],
            Coord {
                x: 2.,
                y: 1.,
                z: Some(111.)
            }
        );
        assert_eq!(
            polygons[0].inner[0].coords[2],
            Coord {
                x: 2.,
                y: 2.,
                z: Some(111.)
            }
        );
        assert_eq!(
            polygons[0].inner[0].coords[3],
            Coord {
                x: 1.,
                y: 2.,
                z: Some(111.)
            }
        );
        assert_eq!(
            polygons[0].inner[0].coords[4],
            Coord {
                x: 1.,
                y: 1.,
                z: Some(111.)
            }
        );
    }
}
