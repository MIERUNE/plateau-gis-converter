/// Ellipsoid parameters
pub struct Ellipsoid {
    /// Semi-major axis
    a: f64,
    /// Inverse flattening
    inv_f: f64,

    /// Flattening
    f: f64,
    /// Semi-minor axis
    b: f64,
    /// Eccentricity squared
    e_sq: f64,
}

impl Ellipsoid {
    #[inline]
    pub fn new(a: f64, inv_f: f64) -> Self {
        let f = 1. / inv_f;
        Self {
            a,
            inv_f,
            f,
            b: a * (1. - f),
            e_sq: f * (2. - f),
        }
    }

    /// Semi-major axis
    #[inline]
    pub fn a(&self) -> f64 {
        self.a
    }

    /// Inverse flattening
    #[inline]
    pub fn inv_f(&self) -> f64 {
        self.inv_f
    }

    /// Flattening
    #[inline]
    pub fn b(&self) -> f64 {
        self.b
    }

    /// Semi-minor axis
    #[inline]
    pub fn f(&self) -> f64 {
        self.f
    }

    /// Eccentricity squared
    #[inline]
    pub fn e_sq(&self) -> f64 {
        self.e_sq
    }
}

/// WGS84 Ellipsoid
#[inline]
pub fn wgs84() -> Ellipsoid {
    Ellipsoid::new(6378137., 298.257223563)
}

/// GRS80 Ellipsoid
#[inline]
pub fn grs80() -> Ellipsoid {
    Ellipsoid::new(6378137., 298.257222101)
}
