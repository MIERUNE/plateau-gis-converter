use kml::KmlWriter;
use nusamai_geometry::MultiPoint;
use nusamai_kml::conversion::multipoint_to_kml;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let mut mpoint = MultiPoint::<3>::new();
    mpoint.push(&[11., 12., 13.]);
    mpoint.push(&[21., 22., 23.]);
    mpoint.push(&[31., 32., 33.]);

    let kml = multipoint_to_kml(&mpoint);

    let file_path = "output.kml";
    let file = File::create(file_path).expect("Failed to create file");

    let mut buf_writer = BufWriter::new(file);

    writeln!(buf_writer, r#"<?xml version="1.0" encoding="UTF-8"?>"#)
        .expect("Failed to write XML declaration");
    let mut kml_writer = KmlWriter::from_writer(&mut buf_writer);
    kml_writer.write(&kml).expect("Failed to write KML data");
}
