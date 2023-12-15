//! Japan Plane Reculangular Coordinate Systems

use crate::{ellipsoid::grs80, etmerc::ExtendedTransverseMercatorProjection};

pub enum JPRZone {
    Zone1,
    Zone2,
    Zone3,
    Zone4,
    Zone5,
    Zone6,
    Zone7,
    Zone8,
    Zone9,
    Zone10,
    Zone11,
    Zone12,
    Zone13,
    Zone14,
    Zone15,
    Zone16,
    Zone17,
    Zone18,
    Zone19,
}

pub struct JPRParams {
    /// Central longitude
    lng0: f64,
    /// Central latitude
    lat0: f64,
    /// Zone number
    zone: u8,
}

impl JPRParams {
    pub fn lng0(&self) -> f64 {
        self.lng0
    }
    pub fn lat0(&self) -> f64 {
        self.lat0
    }
    pub fn zone(&self) -> usize {
        self.zone as usize
    }
}

impl JPRZone {
    /// Gets the transverse mercator projection for the zone.
    pub fn projection(&self) -> ExtendedTransverseMercatorProjection {
        let params = self.params();
        ExtendedTransverseMercatorProjection::new(params.lng0(), params.lat0(), 0.9999, &grs80())
    }

    pub fn params(&self) -> JPRParams {
        match self {
            JPRZone::Zone1 => JPRParams {
                lng0: 129.5,
                lat0: 33.0,
                zone: 1,
            },
            JPRZone::Zone2 => JPRParams {
                lng0: 131.0,
                lat0: 33.0,
                zone: 2,
            },
            JPRZone::Zone3 => JPRParams {
                lng0: 132.166_666_666_666_67,
                lat0: 36.0,
                zone: 3,
            },
            JPRZone::Zone4 => JPRParams {
                lng0: 133.5,
                lat0: 36.0,
                zone: 4,
            },
            JPRZone::Zone5 => JPRParams {
                lng0: 134.333_333_333_333_33,
                lat0: 36.0,
                zone: 5,
            },
            JPRZone::Zone6 => JPRParams {
                lng0: 136.5,
                lat0: 36.0,
                zone: 6,
            },
            JPRZone::Zone7 => JPRParams {
                lng0: 137.133_333_333_333_33,
                lat0: 36.0,
                zone: 7,
            },
            JPRZone::Zone8 => JPRParams {
                lng0: 138.5,
                lat0: 36.0,
                zone: 8,
            },
            JPRZone::Zone9 => JPRParams {
                lng0: 139.833_333_333_333_33,
                lat0: 36.0,
                zone: 9,
            },
            JPRZone::Zone10 => JPRParams {
                lng0: 140.833_333_333_333_33,
                lat0: 40.0,
                zone: 10,
            },
            JPRZone::Zone11 => JPRParams {
                lng0: 140.25,
                lat0: 44.0,
                zone: 11,
            },
            JPRZone::Zone12 => JPRParams {
                lng0: 142.25,
                lat0: 44.0,
                zone: 12,
            },
            JPRZone::Zone13 => JPRParams {
                lng0: 144.25,
                lat0: 44.0,
                zone: 13,
            },
            JPRZone::Zone14 => JPRParams {
                lng0: 142.0,
                lat0: 26.0,
                zone: 14,
            },
            JPRZone::Zone15 => JPRParams {
                lng0: 127.5,
                lat0: 26.0,
                zone: 15,
            },
            JPRZone::Zone16 => JPRParams {
                lng0: 124.0,
                lat0: 26.0,
                zone: 16,
            },
            JPRZone::Zone17 => JPRParams {
                lng0: 131.5,
                lat0: 26.0,
                zone: 17,
            },
            JPRZone::Zone18 => JPRParams {
                lng0: 136.5,
                lat0: 20.0,
                zone: 18,
            },
            JPRZone::Zone19 => JPRParams {
                lng0: 154.5,
                lat0: 26.0,
                zone: 19,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        // Japan Plane Rectangular CS VIII
        // $ cs2cs epsg:6668 epsg:6676
        // 36.65209371778363 138.19318970050347 0
        // 72396.23	-27430.91 0.00

        let tmerc = JPRZone::Zone8.projection();
        let lng = 138.19318970050347;
        let lat = 36.65209371778363;

        let Ok((x, y)) = tmerc.project_forward(lng, lat) else {
            panic!("error");
        };
        assert!((x - -27430.911753676937).abs() < 1e-9);
        assert!((y - 72396.2255270589).abs() < 1e-9);

        let Ok((lng2, lat2)) = tmerc.project_inverse(x, y) else {
            panic!("error");
        };
        assert!((lng - lng2).abs() < 1e-10);
        assert!((lat - lat2).abs() < 1e-10);
    }
}
