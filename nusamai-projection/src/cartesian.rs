//! Conversion between geographic and geocentric (cartesian) coordinate systems.

use crate::ellipsoid::Ellipsoid;

/// Convert from geographic to geocentric coordinate system.
pub fn geographic_to_geocentric<E: Ellipsoid>(lng: f64, lat: f64, height: f64) -> (f64, f64, f64) {
    let (lng_rad, lat_rad) = (lng.to_radians(), lat.to_radians());
    let tan_psi = (1. - E::E_SQ) * lat_rad.tan();
    let z = E::A * (1. / (1. / (tan_psi * tan_psi) + 1. / ((1. - E::F) * (1. - E::F)))).sqrt();
    let r = E::A * (1. / (1. + (tan_psi * tan_psi) / ((1. - E::F) * (1. - E::F)))).sqrt();
    let x = r * lng_rad.cos();
    let y = r * lng_rad.sin();
    let dhz = lat_rad.sin();
    let dhx = lat_rad.cos() * lng_rad.cos();
    let dhy = lat_rad.cos() * lng_rad.sin();
    (x + dhx * height, y + dhy * height, z + dhz * height)
}

/// Convert from geocentric to geographic coordinate system.
pub fn geocentric_to_geographic<E: Ellipsoid>(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    println!("not implemented: {:?}", (x, y, z));
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::WGS84;

    #[test]
    fn fixtures() {
        {
            let (x, y, z) = geographic_to_geocentric::<WGS84>(140., 37., 50.);
            assert!((x - -3906851.9770472576).abs() < 1e-9);
            assert!((y - 3278238.0530045824).abs() < 1e-9);
            assert!((z - 3817423.251099322).abs() < 1e-9);
        }

        // north pole
        {
            let height = 150.;
            let (x, y, z) = geographic_to_geocentric::<WGS84>(123., 90., 150.);
            assert!((x - 0.).abs() < 1e-9);
            assert!((y - 0.).abs() < 1e-9);
            assert!((z - (WGS84::B + height)).abs() < 1e-9);
        }

        // null island
        {
            let height = 100.;
            let (x, y, z) = geographic_to_geocentric::<WGS84>(0., 0., height);
            assert!((x - (WGS84::A + height)).abs() < 1e-9);
            assert!((y - 0.).abs() < 1e-9);
            assert!((z - 0.).abs() < 1e-9);
        }
    }
}
