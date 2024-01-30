use kml::{
    types::{Geometry, KmlDocument, Placemark, Point},
    Kml,
};
use nusamai_geometry::{CoordNum, MultiPoint};
use std::collections::HashMap;

pub fn multipoint_to_kml_with_mapping<const D: usize, T: CoordNum>(
    mpoint: &MultiPoint<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> Vec<Kml> {
    let points = mpoint
        .iter()
        .map(&mapping)
        .map(|coords| Point::new(coords[0], coords[1], Some(coords[2])))
        .collect::<Vec<_>>();

    points
        .into_iter()
        .map(|pt| {
            Kml::Placemark(Placemark {
                geometry: Some(Geometry::Point(pt)),
                ..Default::default()
            })
        })
        .collect()
}

pub fn multipoint_to_kml(mpoint: &MultiPoint<3>) -> Kml {
    let placemarks = multipoint_to_kml_with_mapping(mpoint, |c| c);
    let folder = Kml::Folder {
        attrs: HashMap::new(),
        elements: placemarks,
    };

    let document = KmlDocument {
        elements: vec![folder],
        ..Default::default()
    };

    Kml::KmlDocument(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multipoint_to_kml() {
        let mut mpoint = MultiPoint::<3>::new();
        mpoint.push(&[11., 12., 13.]);
        mpoint.push(&[21., 22., 23.]);
        mpoint.push(&[31., 32., 33.]);

        let kml = multipoint_to_kml(&mpoint);

        match kml {
            Kml::KmlDocument(doc) => {
                assert_eq!(doc.elements.len(), 1);
                match doc.elements[0] {
                    Kml::Folder { ref elements, .. } => {
                        assert_eq!(elements.len(), 3);
                        match &elements[0] {
                            Kml::Placemark(Placemark {
                                geometry: Some(Geometry::Point(Point { coord: coords, .. })),
                                ..
                            }) => {
                                assert_eq!(coords.x, 11.);
                                assert_eq!(coords.y, 12.);
                                assert_eq!(coords.z, Some(13.));
                            }
                            _ => panic!("Unexpected element type"),
                        }

                        match &elements[1] {
                            Kml::Placemark(Placemark {
                                geometry: Some(Geometry::Point(Point { coord: coords, .. })),
                                ..
                            }) => {
                                assert_eq!(coords.x, 21.);
                                assert_eq!(coords.y, 22.);
                                assert_eq!(coords.z, Some(23.));
                            }
                            _ => panic!("Unexpected element type"),
                        }

                        match &elements[2] {
                            Kml::Placemark(Placemark {
                                geometry: Some(Geometry::Point(Point { coord: coords, .. })),
                                ..
                            }) => {
                                assert_eq!(coords.x, 31.);
                                assert_eq!(coords.y, 32.);
                                assert_eq!(coords.z, Some(33.));
                            }
                            _ => panic!("Unexpected element type"),
                        }
                    }
                    _ => panic!("Unexpected element type"),
                }
            }
            _ => panic!("Unexpected element type"),
        }
    }
}
