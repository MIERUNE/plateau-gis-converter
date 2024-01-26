use nusamai_geometry::{CoordNum, MultiPoint};
use kml::{Kml, KmlWriter, types::{Point}};


pub fn multipoint_to_kml_with_mapping<const D: usize, T: CoordNum>(
    mpoint: &MultiPoint<D, T>,
    mapping: impl Fn([T; D]) -> [f64; 3],
) -> Vec<Kml> {
    let mut kml_point = mpoint
        .iter()
        .map(&mapping)
        .map(|coords| Point::new(coords[0], coords[1], Some(coords[2])))
        .collect::<Vec<_>>();

    let mut kml_points: Vec<Kml> = Vec::new();

    for point in kml_point {
        kml_points.push(Kml::Point(point));
    }

    kml_points
    
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

        let kml_points = multipoint_to_kml_with_mapping(&mpoint, |c| c);
        
        let mut buf = Vec::new();
        let mut writer = KmlWriter::from_writer(&mut buf);
        for kml_point in kml_points {
            writer.write(&kml_point).unwrap();
        }

        let kml_point_count = String::from_utf8(buf).unwrap().matches("<Point>").count();
        assert_eq!(kml_point_count, mpoint.len());

        
    }
}