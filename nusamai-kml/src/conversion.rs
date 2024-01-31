use kml::{
    types::{Geometry, KmlDocument, Placemark, Point},
    Kml,
};
use nusamai_geometry::{CoordNum, MultiPoint};
use std::collections::HashMap;

pub fn multipoint_to_kml_with_mapping<const D: usize, T: CoordNum>(
    mpoint: &MultiPoint<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> Kml {
    let points = mpoint
        .iter()
        .map(&mapping)
        .map(|coords| Point::new(coords[0], coords[1], Some(coords[2])))
        .collect::<Vec<_>>();

    let placemarks = points
        .into_iter()
        .map(|pt| {
            Kml::Placemark(Placemark {
                geometry: Some(Geometry::Point(pt)),
                ..Default::default()
            })
        })
        .collect();
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

pub fn multipoint_to_kml(mpoint: &MultiPoint<3>) -> Kml {
    multipoint_to_kml_with_mapping(mpoint, |c| c)
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

        if let Kml::KmlDocument(doc) = kml {
            assert_eq!(doc.elements.len(), 1);

            if let Kml::Folder { elements, .. } = &doc.elements[0] {
                assert_eq!(elements.len(), 3);

                if let Kml::Placemark(Placemark {
                    geometry: Some(Geometry::Point(Point { coord: coords, .. })),
                    ..
                }) = &elements[0]
                {
                    assert_eq!(coords.x, 11.);
                    assert_eq!(coords.y, 12.);
                    assert_eq!(coords.z, Some(13.));
                } else {
                    panic!("Unexpected element type");
                }

                if let Kml::Placemark(Placemark {
                    geometry: Some(Geometry::Point(Point { coord: coords, .. })),
                    ..
                }) = &elements[1]
                {
                    assert_eq!(coords.x, 21.);
                    assert_eq!(coords.y, 22.);
                    assert_eq!(coords.z, Some(23.));
                } else {
                    panic!("Unexpected element type");
                }

                if let Kml::Placemark(Placemark {
                    geometry: Some(Geometry::Point(Point { coord: coords, .. })),
                    ..
                }) = &elements[2]
                {
                    assert_eq!(coords.x, 31.);
                    assert_eq!(coords.y, 32.);
                    assert_eq!(coords.z, Some(33.));
                } else {
                    panic!("Unexpected element type");
                }
            } else {
                panic!("Unexpected element type");
            }
        }
    }
}
