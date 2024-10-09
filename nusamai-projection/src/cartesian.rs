//! Conversion between geodetic and geocentric (cartesian) coordinate systems.

use std::f64::consts::FRAC_PI_6;

use crate::ellipsoid::Ellipsoid;

/// Convert from geodetic to geocentric coordinate system.
pub fn geodetic_to_geocentric(
    ellips: &Ellipsoid,
    lng: f64,
    lat: f64,
    height: f64,
) -> (f64, f64, f64) {
    let (lam, phi) = (lng.to_radians(), lat.to_radians());
    let n = if ellips.e_sq() == 0.0 {
        ellips.a() // optimization for sphere (e=0)
    } else {
        ellips.a() / (1. - ellips.e_sq() * phi.sin() * phi.sin()).sqrt()
    };
    let x = (n + height) * phi.cos() * lam.cos();
    let y = (n + height) * phi.cos() * lam.sin();
    let z = (n * (1. - ellips.e_sq()) + height) * phi.sin();
    (x, y, z)
}

/// Convert from geocentric to geodetic coordinate system.
///
/// We uses Hugues Vermeille's *[An analytical method to transform geocentric into geodetic coordinates](https://DOI.org/10.1007/s00190-010-0419-x)*, J. Geodesy (2011) 85, page 105-117.
pub fn geocentric_to_geodetic(ellips: &Ellipsoid, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let a = ellips.a();
    let e_sq = ellips.e_sq();
    let e_quad = e_sq * e_sq;

    let p = (x * x + y * y) / (a * a);
    let q = (1. - e_sq) * z * z / (a * a);
    let r = (p + q - e_quad) / 6.;

    let evol = 8. * r.powi(3) + e_quad * p * q;

    let (phi, h) = if evol > 0. || q != 0. {
        let u = if evol > 0. {
            // Outside the evolute
            let l = (evol.sqrt() + (e_quad * p * q).sqrt()).cbrt();
            (3. * r * r) / (2. * l * l) + 0.5 * (l + r / l) * (l + r / l)
        } else {
            // On or inside the evolute and not in the singular disc
            let t = 2. / 3.
                * ((e_quad * p * q).sqrt()).atan2((-evol).sqrt() + (-8. * r * r * r).sqrt());
            -4. * r * (t).sin() * (FRAC_PI_6 + t).cos()
        };
        let v = (u * u + e_quad * q).sqrt();
        let w = e_sq * (u + v - q) / (2. * v);
        let k = (u + v) / ((w * w + u + v).sqrt() + w);
        let d = k * (x * x + y * y).sqrt() / (k + e_sq);
        let h = (k + e_sq - 1.) / k * (d * d + z * z).sqrt();
        let phi = 2. * z.atan2(d + (d * d + z * z).sqrt());
        (phi, h)
    } else {
        // In the singular disc
        let h = -a * ((1. - e_sq) * (e_sq - p) / e_sq).sqrt();
        let phi =
            2. * ((e_quad - p).sqrt()).atan2((e_sq * (e_sq - p)).sqrt() + ((1. - e_sq) * p).sqrt());
        (phi, h)
    };

    let lam = f64::atan2(y, x);

    (lam.to_degrees(), phi.to_degrees(), h)
}

// /// Convert from geocentric to geodetic coordinate system.
// pub fn geocentric_to_geodetic(ellips: &Ellipsoid, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
//     // Ported from OJ
//
//     let (lam, p_div_a, z_div_a) = {
//         let ra = 1. / ellips.a();
//         let x_div_a = x * ra;
//         let y_div_a = y * ra;
//         let z_div_a = z * ra;
//         let lam = f64::atan2(y_div_a, x_div_a);
//
//         // Perpendicular distance from point to Z-axis (HM eq. 5-28)
//         let p_div_a = (x_div_a * x_div_a + y_div_a * y_div_a).sqrt();
//
//         (lam, p_div_a, z_div_a)
//     };
//
//     // Non-optimized version:
//     // theta = atan2(z * ellips.a(), p * ellips.b());
//     // c = cos(theta);
//     // s = sin(theta);
//     let b_div_a = 1. - ellips.f();
//     let (c, s) = {
//         let p_div_a_b_div_a = p_div_a * b_div_a;
//         let norm = (z_div_a * z_div_a + p_div_a_b_div_a * p_div_a_b_div_a).sqrt();
//         if norm != 0.0 {
//             let inv_norm = 1.0 / norm;
//             let c = p_div_a_b_div_a * inv_norm;
//             let s = z_div_a * inv_norm;
//             (c, s)
//         } else {
//             (1., 0.)
//         }
//     };
//
//     let e2s = ellips.e_sq() / (1.0 - ellips.e_sq());
//     let y_phi = z_div_a + e2s * b_div_a * s * s * s;
//     let x_phi = p_div_a - ellips.e_sq() * c * c * c;
//     let norm_phi = (y_phi * y_phi + x_phi * x_phi).sqrt();
//
//     let (mut cosphi, mut sinphi) = if norm_phi != 0. {
//         let inv_norm_phi = 1.0 / norm_phi;
//         (x_phi * inv_norm_phi, y_phi * inv_norm_phi)
//     } else {
//         (1., 0.)
//     };
//     let phi = if x_phi <= 0. {
//         // this happen on non-sphere ellipsoid when x,y,z is very close to 0
//         // there is no single solution to the cart->geodetic conversion in
//         // that case, clamp to -90/90 deg and avoid a discontinuous boundary
//         // near the poles
//         cosphi = 0.;
//         sinphi = if z >= 0. { 1. } else { -1. };
//         if z >= 0. {
//             FRAC_2_PI
//         } else {
//             -FRAC_2_PI
//         }
//     } else {
//         f64::atan2(y_phi, x_phi)
//     };
//
//     fn geocentric_radius(a: f64, b_div_a: f64, cosphi: f64, sinphi: f64) -> f64 {
//         // Non-optimized version:
//         // const double b = a * b_div_a;
//         // return hypot(a * a * cosphi, b * b * sinphi) /
//         //        hypot(a * cosphi, b * sinphi);
//         let cosphi_sq = cosphi * cosphi;
//         let sinphi_sq = sinphi * sinphi;
//         let b_div_a_sq = b_div_a * b_div_a;
//         let b_div_a_sq_mul_sinphi_sq = b_div_a_sq * sinphi_sq;
//         a * ((cosphi_sq + b_div_a_sq * b_div_a_sq_mul_sinphi_sq)
//             / (cosphi_sq + b_div_a_sq_mul_sinphi_sq))
//             .sqrt()
//     }
//
//     let height = if cosphi < 1e-6 {
//         // poleward of 89.99994 deg, we avoid division by zero
//         // by computing the height as the cartesian z value
//         // minus the geocentric radius of the Earth at the given latitude
//         z.abs() - geocentric_radius(ellips.a(), b_div_a, cosphi, sinphi)
//     } else {
//         let n = if ellips.e_sq() == 0.0 {
//             ellips.a() // optimization for sphere (e=0)
//         } else {
//             ellips.a() / (1. - ellips.e_sq() * sinphi * sinphi).sqrt()
//         };
//         ellips.a() * p_div_a / cosphi - n
//     };
//
//     (lam.to_degrees(), phi.to_degrees(), height)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let wgs84 = crate::ellipsoid::wgs84();

        // Outside the evolute
        {
            let (lng, lat, height) = (140., 37., 50.);
            let (x, y, z) = geodetic_to_geocentric(&wgs84, lng, lat, height);
            let (lng2, lat2, height2) = geocentric_to_geodetic(&wgs84, x, y, z);
            assert!((lng - lng2).abs() < 1e-10);
            assert!((lat - lat2).abs() < 1e-10);
            assert!((height - height2).abs() < 1e-7);
        }

        // On or inside the evolute and not in the singular disc
        {
            let (lng, lat, height) = (45., 74.58501644931525, -6344866.234164982);
            let (x, y, z) = geodetic_to_geocentric(&wgs84, lng, lat, height);
            let (lng2, lat2, height2) = geocentric_to_geodetic(&wgs84, x, y, z);
            assert!((lng - lng2).abs() < 1e-10);
            assert!((lat - lat2).abs() < 1e-10);
            assert!((height - height2).abs() < 1e-7);
        }

        // In the singular disc
        {
            let (lng, lat, height) = (120., 88.10828645, -6356728.972246517);
            let (x, y, _) = geodetic_to_geocentric(&wgs84, lng, lat, height);
            let z = 0.;
            let (lng2, lat2, height2) = geocentric_to_geodetic(&wgs84, x, y, z);
            assert!((lng - lng2).abs() < 1e-10);
            assert!((lat - lat2).abs() < 1e-10);
            assert!((height - height2).abs() < 30.);
        }

        // Earth's center
        {
            let (lng, lat, height) = (0., 90., wgs84.b());
            let (x, y, z) = geodetic_to_geocentric(&wgs84, lng, lat, height);
            let (lng2, lat2, height2) = geocentric_to_geodetic(&wgs84, x, y, z);
            assert!((lng - lng2).abs() < 1e-9);
            assert!((lat - lat2).abs() < 1e-9);
            assert!((height - height2).abs() < 1e-7);
        }
    }

    #[test]
    fn to_geocentric() {
        let wgs84 = crate::ellipsoid::wgs84();

        {
            let (x, y, z) = geodetic_to_geocentric(&wgs84, 140., 37., 50.);
            assert!((x - -3906851.9770472576).abs() < 1e-10);
            assert!((y - 3278238.0530045824).abs() < 1e-10);
            assert!((z - 3817423.251099322).abs() < 1e-10);
        }

        // north pole
        {
            let height = 150.;
            let (x, y, z) = geodetic_to_geocentric(&wgs84, 123., 90., 150.);
            assert!((x - 0.).abs() < 1e-9);
            assert!((y - 0.).abs() < 1e-9);
            assert!((z - (wgs84.b() + height)).abs() < 1e-9);
        }

        // null island
        {
            let height = 100.;
            let (x, y, z) = geodetic_to_geocentric(&wgs84, 0., 0., height);
            assert!((x - (wgs84.a() + height)).abs() < 1e-9);
            assert!((y - 0.).abs() < 1e-9);
            assert!((z - 0.).abs() < 1e-9);
        }
    }
}
