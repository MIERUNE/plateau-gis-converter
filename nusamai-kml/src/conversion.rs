use nusamai_geometry::{CoordNum, MultiPoint};
use kml::{types::{Point}};


pub fn multipoint_to_kml_with_mapping<const D: usize, T: CoordNum>(
    mpoint: &MultiPoint<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> Vec<Point> {
    let kml_points = mpoint
        .iter()
        .map(&mapping)
        .map(|coords| Point::new(coords[0], coords[1], Some(coords[2])))
        .collect::<Vec<_>>();

    kml_points
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multipoint_to_kml() {

        use kml::{Kml, KmlWriter, types::{Placemark, KmlDocument, Geometry}};
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::collections::HashMap;

        let mut mpoint = MultiPoint::<3>::new();
        mpoint.push(&[11., 12., 13.]);
        mpoint.push(&[21., 22., 23.]);
        mpoint.push(&[31., 32., 33.]);

        let kml_points = multipoint_to_kml_with_mapping(&mpoint, |c| c);

        
        let placemarks: Vec<Kml<_>> = kml_points.into_iter()
        .map(|pt| Kml::Placemark(Placemark {
            geometry: Some(Geometry::Point(pt)),
            ..Default::default()
        }))
        .collect();

        let folder = Kml::Folder {
            attrs: HashMap::new(),
            elements: placemarks,
        };

        let document = KmlDocument {
            elements: vec![folder],
            ..Default::default()
        };


        let kml = Kml::KmlDocument(document);

        let file_path = "output.kml";
        let file = File::create(file_path).expect("Failed to create file");

        let mut buf_writer = BufWriter::new(file);


        writeln!(buf_writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#).expect("Failed to write XML declaration");
        let mut kml_writer = KmlWriter::from_writer(&mut buf_writer);
        kml_writer.write(&kml).expect("Failed to write KML data");
            
    }
}