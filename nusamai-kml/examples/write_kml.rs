use kml::{
    types::{Geometry, Kml, MultiGeometry, Placemark},
    KmlWriter,
};
use nusamai_geometry::{MultiPoint, Polygon3};
use nusamai_kml::conversion::{multipoint_to_kml, polygon_to_kml};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

fn poly_to_multigeom() -> MultiGeometry {
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

    polygon_to_kml(&poly)
}
fn point_to_multigeom() -> MultiGeometry {
    let mut mpoint = MultiPoint::<3>::new();
    mpoint.push(&[11., 12., 13.]);
    mpoint.push(&[21., 22., 23.]);
    mpoint.push(&[31., 32., 33.]);

    multipoint_to_kml(&mpoint)
}

fn multigeometry_to_file(multi_geom: MultiGeometry, filename: String) {
    let placemark = Placemark {
        geometry: Some(Geometry::MultiGeometry(multi_geom)),
        ..Default::default()
    };

    let folder = Kml::Folder {
        attrs: HashMap::new(),
        elements: vec![Kml::Placemark(placemark)],
    };

    let file_path = filename;
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

fn main() {
    multigeometry_to_file(point_to_multigeom(), "point.kml".to_string());

    multigeometry_to_file(poly_to_multigeom(), "poly.kml".to_string());
}
