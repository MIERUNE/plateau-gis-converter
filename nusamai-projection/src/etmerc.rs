//! Extended Transverse Mercator projection
//!
//! Ported from PROJ (<https://github.com/OSGeo/PROJ/blob/master/src/projections/tmerc.cpp>).

use crate::{ellipsoid::Ellipsoid, error::TransformError};

/* Constant for "exact" transverse mercator */
const ETMERC_ORDER: usize = 6;

type Coeffs = [f64; ETMERC_ORDER];

#[derive(Debug, Clone)]
struct PoderEngsager {
    /// Merid. quad., scaled to the projection
    pub q_n: f64,
    /// Radius vector in polar coord. systems
    pub z_b: f64,
    /// Constants for Gauss -> Geo lat
    pub cgb: Coeffs,
    /// Constants for Geo lat -> Gauss
    pub cbg: Coeffs,
    /// Constants for transv. merc. -> geo
    pub utg: Coeffs,
    /// Constants for geo -> transv. merc.
    pub gtu: Coeffs,
}

#[derive(Debug, Clone)]
pub struct ExtendedTransverseMercatorProjection {
    q: PoderEngsager,
    /// false longitude (radians)
    lam0: f64,
    /// semi-major axis
    a: f64,
}

impl ExtendedTransverseMercatorProjection {
    pub fn new(lng0: f64, lat0: f64, k: f64, ellips: &Ellipsoid) -> Self {
        let q = setup_exact(lat0.to_radians(), k, ellips);
        Self {
            q,
            lam0: lng0.to_radians(),
            a: ellips.a(),
        }
    }

    pub fn project_forward(
        &self,
        lng: f64,
        lat: f64,
        z: f64,
    ) -> Result<(f64, f64, f64), TransformError> {
        self.project_forward_radians(lng.to_radians(), lat.to_radians(), z)
    }

    pub fn project_inverse(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<(f64, f64, f64), TransformError> {
        let (lam, phi, z) = self.project_inverse_radians(x, y, z)?;
        Ok((lam.to_degrees(), phi.to_degrees(), z))
    }

    fn project_forward_radians(
        &self,
        lam: f64,
        phi: f64,
        z: f64,
    ) -> Result<(f64, f64, f64), TransformError> {
        let lam = lam - self.lam0;
        let q = &self.q;

        /* ell. LAT, LNG -> Gaussian LAT, LNG */
        let cn = gatg(&q.cbg, phi, (2. * phi).cos(), (2. * phi).sin());
        /* Gaussian LAT, LNG -> compl. sph. LAT */
        let sin_cn = cn.sin();
        let cos_cn = cn.cos();
        let sin_ce = lam.sin();
        let cos_ce = lam.cos();

        let cos_cn_cos_ce = cos_cn * cos_ce;
        let mut cn: f64 = f64::atan2(sin_cn, cos_cn_cos_ce);

        let inv_denom_tan_ce = 1. / f64::hypot(sin_cn, cos_cn_cos_ce);
        let tan_ce = sin_ce * cos_cn * inv_denom_tan_ce;

        // compl. sph. N, E -> ell. norm. N, E
        let mut ce = tan_ce.asinh(); // Replaces: ce = log(tan(FORTPI + ce*0.5));

        /*
         *  Non-optimized version:
         *  let  sin_arg_r  = sin(2*cn);
         *  let  cos_arg_r  = cos(2*cn);
         *
         *  Given:
         *      sin(2 * cn) = 2 sin(cn) cos(cn)
         *          sin(atan(y)) = y / sqrt(1 + y^2)
         *          cos(atan(y)) = 1 / sqrt(1 + y^2)
         *      ==> sin(2 * cn) = 2 tan_cn / (1 + tan_cn^2)
         *
         *      cos(2 * cn) = 2cos^2(cn) - 1
         *                  = 2 / (1 + tan_cn^2) - 1
         */
        let two_inv_denom_tan_ce = 2. * inv_denom_tan_ce;
        let two_inv_denom_tan_ce_square = two_inv_denom_tan_ce * inv_denom_tan_ce;
        let tmp_r = cos_cn_cos_ce * two_inv_denom_tan_ce_square;
        let sin_arg_r = sin_cn * tmp_r;
        let cos_arg_r = cos_cn_cos_ce * tmp_r - 1.;

        /*
         *  Non-optimized version:
         *  let  sinh_arg_i = sinh(2*ce);
         *  let  cosh_arg_i = cosh(2*ce);
         *
         *  Given
         *      sinh(2 * ce) = 2 sinh(ce) cosh(ce)
         *          sinh(asinh(y)) = y
         *          cosh(asinh(y)) = sqrt(1 + y^2)
         *      ==> sinh(2 * ce) = 2 tan_ce sqrt(1 + tan_ce^2)
         *
         *      cosh(2 * ce) = 2cosh^2(ce) - 1
         *                   = 2 * (1 + tan_ce^2) - 1
         *
         * and 1+tan_ce^2 = 1 + sin_ce^2 * cos_cn^2 / (sin_cn^2 + cos_cn^2 *
         * cos_ce^2) = (sin_cn^2 + cos_cn^2 * cos_ce^2 + sin_ce^2 * cos_cn^2) /
         * (sin_cn^2 + cos_cn^2 * cos_ce^2) = 1. / (sin_cn^2 + cos_cn^2 * cos_ce^2)
         * = inv_denom_tan_ce^2
         *
         */
        let sinh_arg_i = tan_ce * two_inv_denom_tan_ce;
        let cosh_arg_i = two_inv_denom_tan_ce_square - 1.;

        let (dcn, dce) = clen_s(&q.gtu, sin_arg_r, cos_arg_r, sinh_arg_i, cosh_arg_i);
        cn += dcn;
        ce += dce;

        if ce.abs() <= 2.623395162778 {
            let y = q.q_n * cn + q.z_b; /* Northing */
            let x = q.q_n * ce; /* Easting  */
            Ok((x * self.a, y * self.a, z))
        } else {
            Err(TransformError::OutsideProjectionDomain)
        }
    }

    fn project_inverse_radians(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> Result<(f64, f64, f64), TransformError> {
        let q = &self.q;
        let x = x / self.a;
        let y = y / self.a;

        // normalize N, E
        let mut cn = (y - q.z_b) / q.q_n;
        let mut ce = x / q.q_n;

        if ce.abs() <= 2.623395162778 {
            // 150 degrees
            // norm. N, E -> compl. sph. LAT, LNG
            let sin_arg_r = (2. * cn).sin();
            let cos_arg_r = (2. * cn).cos();

            // let sinh_arg_i = sinh(2*ce);
            // let cosh_arg_i = cosh(2*ce);
            let exp_2_ce = (2. * ce).exp();
            let half_inv_exp_2_ce = 0.5 / exp_2_ce;
            let sinh_arg_i = 0.5 * exp_2_ce - half_inv_exp_2_ce;
            let cosh_arg_i = 0.5 * exp_2_ce + half_inv_exp_2_ce;

            let (dcn, dce) = clen_s(&q.utg, sin_arg_r, cos_arg_r, sinh_arg_i, cosh_arg_i);
            cn += dcn;
            ce += dce;

            // compl. sph. LAT -> Gaussian LAT, LNG
            let sin_cn = cn.sin();
            let cos_cn = cn.cos();

            /*
             *      One can divide both member of ce = atan2(...) by cos_ce, which
             * gives: ce     = atan2 (tan_ce, cos_cn) = atan2(sinh(ce), cos_cn)
             *
             *      and the same for cn = atan2(...)
             *      cn     = atan2 (sin_cn, hypot (sin_ce, cos_ce*cos_cn)/cos_ce)
             *             = atan2 (sin_cn, hypot (sin_ce/cos_ce, cos_cn))
             *             = atan2 (sin_cn, hypot (tan_ce, cos_cn))
             *             = atan2 (sin_cn, hypot (sinhce, cos_cn))
             */
            let sinhce = (ce).sinh();
            let ce = f64::atan2(sinhce, cos_cn);
            let modulus_ce = f64::hypot(sinhce, cos_cn);
            let cn = f64::atan2(sin_cn, modulus_ce);

            // Gaussian LAT, LNG -> ell. LAT, LNG

            // Optimization of the computation of cos(2*cn) and sin(2*cn)
            let tmp = 2. * modulus_ce / (sinhce * sinhce + 1.);
            let sin_2_cn = sin_cn * tmp;
            let cos_2_cn = tmp * modulus_ce - 1.;
            // let cos_2_cn = cos(2 * cn);
            // let sin_2_cn = sin(2 * cn);

            let phi = gatg(&q.cgb, cn, cos_2_cn, sin_2_cn);
            let lam = ce;
            Ok((lam + self.lam0, phi, z))
        } else {
            Err(TransformError::OutsideProjectionDomain)
        }
    }
}

fn setup_exact(phi0: f64, k: f64, ellips: &Ellipsoid) -> PoderEngsager {
    let mut q = PoderEngsager {
        q_n: 0.,
        z_b: 0.,
        cgb: [0.; 6],
        cbg: [0.; 6],
        utg: [0.; 6],
        gtu: [0.; 6],
    };

    // third flattening
    let n: f64 = ellips.f() / (2. - ellips.f());

    let mut np: f64 = n;

    q.cgb[0] = n
        * (2.
            + n * (-2. / 3.0
                + n * (-2. + n * (116. / 45. + n * (26. / 45. + n * (-2854. / 675.))))));
    q.cbg[0] = n
        * (-2.
            + n * (2. / 3.0
                + n * (4. / 3. + n * (-82. / 45. + n * (32. / 45. + n * (4642. / 4725.))))));

    np *= n;
    q.cgb[1] = np
        * (7. / 3.0 + n * (-8. / 5. + n * (-227. / 45. + n * (2704. / 315. + n * (2323. / 945.)))));
    q.cbg[1] = np
        * (5. / 3.0 + n * (-16. / 15. + n * (-13. / 9. + n * (904. / 315. + n * (-1522. / 945.)))));

    np *= n;
    // n^5 coeff corrected from 1262/105 -> -1262/105
    q.cgb[2] = np * (56. / 15. + n * (-136. / 35. + n * (-1262. / 105. + n * (73814. / 2835.))));

    q.cbg[2] = np * (-26. / 15. + n * (34. / 21. + n * (8. / 5. + n * (-12686. / 2835.))));

    np *= n;
    // n^5 coeff corrected from 322/35 -> 332/35
    q.cgb[3] = np * (4279. / 630. + n * (-332. / 35. + n * (-399572. / 14175.)));
    q.cbg[3] = np * (1237. / 630. + n * (-12. / 5. + n * (-24832. / 14175.)));

    np *= n;
    q.cgb[4] = np * (4174. / 315. + n * (-144838. / 6237.));
    q.cbg[4] = np * (-734. / 315. + n * (109598. / 31185.));

    np *= n;
    q.cgb[5] = np * (601676. / 22275.);
    q.cbg[5] = np * (444337. / 155925.);

    // Constants of the projections
    // Transverse Mercator (UTM, ITM, etc)
    np = n * n;

    // Norm. mer. quad, K&W p.50 (96), p.19 (38b), p.5 (2)
    q.q_n = k / (1. + n) * (1. + np * (1. / 4. + np * (1. / 64. + np / 256.)));

    // coef of trig series
    // utg := ell. N, E -> sph. N, E,  KW p194 (65)
    // gtu := sph. N, E -> ell. N, E,  KW p196 (69)
    q.utg[0] = n
        * (-0.5
            + n * (2. / 3.0
                + n * (-37. / 96.0
                    + n * (1. / 360. + n * (81. / 512. + n * (-96199. / 604800.))))));
    q.gtu[0] = n
        * (0.5
            + n * (-2. / 3.0
                + n * (5. / 16.0 + n * (41. / 180. + n * (-127. / 288. + n * (7891. / 37800.))))));
    q.utg[1] = np
        * (-1. / 48.0
            + n * (-1. / 15.0
                + n * (437. / 1440. + n * (-46. / 105. + n * (1118711. / 3870720.)))));
    q.gtu[1] = np
        * (13. / 48.0
            + n * (-3. / 5.0
                + n * (557. / 1440. + n * (281. / 630. + n * (-1983433. / 1935360.)))));

    np *= n;
    q.utg[2] = np * (-17. / 480. + n * (37. / 840. + n * (209. / 4480. + n * (-5569. / 90720.))));
    q.gtu[2] =
        np * (61. / 240. + n * (-103. / 140. + n * (15061. / 26880. + n * (167603. / 181440.))));

    np *= n;
    q.utg[3] = np * (-4397. / 161280. + n * (11. / 504. + n * (830251. / 7257600.)));
    q.gtu[3] = np * (49561. / 161280. + n * (-179. / 168. + n * (6601661. / 7257600.)));

    np *= n;
    q.utg[4] = np * (-4583. / 161280. + n * (108847. / 3991680.));
    q.gtu[4] = np * (34729. / 80640. + n * (-3418889. / 1995840.));

    np *= n;
    q.utg[5] = np * (-20648693. / 638668800.);
    q.gtu[5] = np * (212378941. / 319334400.);

    // Gaussian latitude value of the origin latitude */
    let z: f64 = gatg(&q.cbg, phi0, (2. * phi0).cos(), (2. * phi0).sin());

    // Origin northing minus true northing at the origin latitude
    // i.e. true northing = N - P->z_b
    q.z_b = -q.q_n * (z + clens(&q.gtu, 2. * z));

    q
}

// Helper functions for "exact" transverse mercator
fn gatg(p1: &Coeffs, b: f64, cos_2b: f64, sin_2b: f64) -> f64 {
    let mut h2 = 0f64;
    let two_cos_2b: f64 = 2. * cos_2b;
    let mut h = p1[p1.len() - 1];
    for v in p1[..p1.len() - 1].iter().rev() {
        (h, h2) = (-h2 + two_cos_2b * h + v, h);
    }
    b + h * sin_2b
}

// Real Clenshaw summation
fn clens(a: &Coeffs, arg_r: f64) -> f64 {
    let cos_arg_r = arg_r.cos();
    let r = 2. * cos_arg_r;

    /* summation loop */
    let mut hr1 = 0.;
    let mut hr = a[a.len() - 1];
    for v in a[..a.len() - 1].iter().rev() {
        (hr, hr1) = (-hr1 + r * hr + v, hr);
    }
    arg_r.sin() * hr
}

/* Complex Clenshaw summation */
fn clen_s(
    a: &Coeffs,
    sin_arg_r: f64,
    cos_arg_r: f64,
    sinh_arg_i: f64,
    cosh_arg_i: f64,
) -> (f64, f64) {
    /* arguments */
    let r = 2. * cos_arg_r * cosh_arg_i;
    let i = -2. * sin_arg_r * sinh_arg_i;

    /* summation loop */
    let (mut hi1, mut hr1) = (0., 0.);
    let mut hi = 0.;
    let mut hr = a[a.len() - 1];
    for v in a[..a.len() - 1].iter().rev() {
        let (hr2, hi2) = (hr1, hi1);
        (hr1, hi1) = (hr, hi);
        hr = -hr2 + r * hr1 - i * hi1 + v;
        hi = -hi2 + i * hr1 + r * hi1;
    }

    let r = sin_arg_r * cosh_arg_i;
    let i = cos_arg_r * sinh_arg_i;
    (r * hr - i * hi, r * hi + i * hr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ellipsoid::grs80;

    #[test]
    fn round_trip() {
        // Japan Plane Rectangular CS VIII
        // $ cs2cs epsg:6668 epsg:6676
        // 36.65209371778363 138.19318970050347 0
        // 72396.23	-27430.91 0.00

        let lat0 = 36.0;
        let lng0 = 138.5;
        let k = 0.9999;

        let lng = 138.19318970050347;
        let lat = 36.65209371778363;

        let ellips = grs80();
        let tmerc = ExtendedTransverseMercatorProjection::new(lng0, lat0, k, &ellips);

        let (x, y, _z) = tmerc.project_forward(lng, lat, 0.).unwrap();
        assert!((x - -27430.911753676937).abs() < 1e-9);
        assert!((y - 72396.2255270589).abs() < 1e-9);

        let (lng2, lat2, _z) = tmerc.project_inverse(x, y, 0.).unwrap();
        assert!((lng - lng2).abs() < 1e-10);
        assert!((lat - lat2).abs() < 1e-10);
    }
}
