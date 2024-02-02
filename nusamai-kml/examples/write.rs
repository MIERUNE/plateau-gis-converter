use kml::{
    types::{Geometry, Kml, Placemark},
    KmlWriter,
};
use nusamai_geometry::MultiPoint;
use nusamai_kml::conversion::multipoint_to_kml;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let mut mpoint = MultiPoint::<3>::new();
    mpoint.push(&[11., 12., 13.]);
    mpoint.push(&[21., 22., 23.]);
    mpoint.push(&[31., 32., 33.]);

    let multi_geom = multipoint_to_kml(&mpoint);

    let placemark = Placemark {
        geometry: Some(Geometry::MultiGeometry(multi_geom)),
        ..Default::default()
    };

    let folder = Kml::Folder {
        attrs: HashMap::new(),
        elements: vec![Kml::Placemark(placemark)],
    };

    let file_path = "output.kml";
    let file = File::create(file_path).expect("Failed to create file");

    let mut buf_writer = BufWriter::new(file);

    writeln!(buf_writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)
        .expect("Failed to write XML declaration");
    writeln!(
        buf_writer,
        r#"<kml xmlns="http://www.opengis.net/kml/2.2">"#
    ) // Add <kml> tag here
    .expect("Failed to write <kml> tag");
    let mut kml_writer = KmlWriter::from_writer(&mut buf_writer);
    kml_writer.write(&folder).expect("Failed to write KML data");
    writeln!(buf_writer, "</kml>") // Add </kml> tag here
        .expect("Failed to write </kml> tag");
}
