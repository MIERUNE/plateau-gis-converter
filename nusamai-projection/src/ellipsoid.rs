/// Ellipsoid parameters
pub trait Ellipsoid {
    /// Semi-major axis
    const A: f64;
    /// Inverse flattening
    const INV_F: f64;

    /// Flattening
    const F: f64 = 1. / Self::INV_F;
    /// Semi-minor axis
    const B: f64 = Self::A * (1. - Self::F);
    /// Eccentricity squared
    const E_SQ: f64 = Self::F * (2. - Self::F);
}

/// WGS84 Eliipsoid
pub struct WGS84 {}
impl Ellipsoid for WGS84 {
    const A: f64 = 6378137.;
    const INV_F: f64 = 298.257223563;
}

/// GRS80 Eliipsoid
pub struct GRS80 {}
impl Ellipsoid for GRS80 {
    const A: f64 = 6378137.;
    const INV_F: f64 = 298.257222101;
}
