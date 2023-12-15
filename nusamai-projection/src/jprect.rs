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

const JPR_K: f64 = 0.9999;

pub struct JPRZoneParams {
    /// Central longitude
    lng0: f64,
    /// Central latitude
    lat0: f64,
}

impl JPRZoneParams {
    /// Central longitude
    pub fn lng0(&self) -> f64 {
        self.lng0
    }
    /// Central latitude
    pub fn lat0(&self) -> f64 {
        self.lat0
    }
}

impl JPRZone {
    /// Gets the transverse mercator projection for the zone.
    pub fn projection(&self) -> ExtendedTransverseMercatorProjection {
        let params = self.params();
        ExtendedTransverseMercatorProjection::new(params.lng0(), params.lat0(), JPR_K, &grs80())
    }

    /// Get the zone from the zone number.
    pub const fn from_number(no: usize) -> Self {
        match no {
            1 => JPRZone::Zone1,
            2 => JPRZone::Zone2,
            3 => JPRZone::Zone3,
            4 => JPRZone::Zone4,
            5 => JPRZone::Zone5,
            6 => JPRZone::Zone6,
            7 => JPRZone::Zone7,
            8 => JPRZone::Zone8,
            9 => JPRZone::Zone9,
            10 => JPRZone::Zone10,
            11 => JPRZone::Zone11,
            12 => JPRZone::Zone12,
            13 => JPRZone::Zone13,
            14 => JPRZone::Zone14,
            15 => JPRZone::Zone15,
            16 => JPRZone::Zone16,
            17 => JPRZone::Zone17,
            18 => JPRZone::Zone18,
            19 => JPRZone::Zone19,
            _ => panic!("invalid zone number"),
        }
    }

    /// Gets the zone number.
    pub const fn zone_number(&self) -> usize {
        match self {
            JPRZone::Zone1 => 1,
            JPRZone::Zone2 => 2,
            JPRZone::Zone3 => 3,
            JPRZone::Zone4 => 4,
            JPRZone::Zone5 => 5,
            JPRZone::Zone6 => 6,
            JPRZone::Zone7 => 7,
            JPRZone::Zone8 => 8,
            JPRZone::Zone9 => 9,
            JPRZone::Zone10 => 10,
            JPRZone::Zone11 => 11,
            JPRZone::Zone12 => 12,
            JPRZone::Zone13 => 13,
            JPRZone::Zone14 => 14,
            JPRZone::Zone15 => 15,
            JPRZone::Zone16 => 16,
            JPRZone::Zone17 => 17,
            JPRZone::Zone18 => 18,
            JPRZone::Zone19 => 19,
        }
    }

    /// Gets the roman number of the zone.
    pub const fn zone_roman(&self) -> &str {
        match self {
            JPRZone::Zone1 => "I",
            JPRZone::Zone2 => "II",
            JPRZone::Zone3 => "III",
            JPRZone::Zone4 => "IV",
            JPRZone::Zone5 => "V",
            JPRZone::Zone6 => "VI",
            JPRZone::Zone7 => "VII",
            JPRZone::Zone8 => "VIII",
            JPRZone::Zone9 => "IX",
            JPRZone::Zone10 => "X",
            JPRZone::Zone11 => "XI",
            JPRZone::Zone12 => "XII",
            JPRZone::Zone13 => "XIII",
            JPRZone::Zone14 => "XIV",
            JPRZone::Zone15 => "XV",
            JPRZone::Zone16 => "XVI",
            JPRZone::Zone17 => "XVII",
            JPRZone::Zone18 => "XVIII",
            JPRZone::Zone19 => "XIX",
        }
    }

    pub const fn params(&self) -> JPRZoneParams {
        match self {
            JPRZone::Zone1 => JPRZoneParams {
                lng0: 129.5,
                lat0: 33.0,
            },
            JPRZone::Zone2 => JPRZoneParams {
                lng0: 131.0,
                lat0: 33.0,
            },
            JPRZone::Zone3 => JPRZoneParams {
                lng0: 132.166_666_666_666_67,
                lat0: 36.0,
            },
            JPRZone::Zone4 => JPRZoneParams {
                lng0: 133.5,
                lat0: 36.0,
            },
            JPRZone::Zone5 => JPRZoneParams {
                lng0: 134.333_333_333_333_33,
                lat0: 36.0,
            },
            JPRZone::Zone6 => JPRZoneParams {
                lng0: 136.5,
                lat0: 36.0,
            },
            JPRZone::Zone7 => JPRZoneParams {
                lng0: 137.133_333_333_333_33,
                lat0: 36.0,
            },
            JPRZone::Zone8 => JPRZoneParams {
                lng0: 138.5,
                lat0: 36.0,
            },
            JPRZone::Zone9 => JPRZoneParams {
                lng0: 139.833_333_333_333_33,
                lat0: 36.0,
            },
            JPRZone::Zone10 => JPRZoneParams {
                lng0: 140.833_333_333_333_33,
                lat0: 40.0,
            },
            JPRZone::Zone11 => JPRZoneParams {
                lng0: 140.25,
                lat0: 44.0,
            },
            JPRZone::Zone12 => JPRZoneParams {
                lng0: 142.25,
                lat0: 44.0,
            },
            JPRZone::Zone13 => JPRZoneParams {
                lng0: 144.25,
                lat0: 44.0,
            },
            JPRZone::Zone14 => JPRZoneParams {
                lng0: 142.0,
                lat0: 26.0,
            },
            JPRZone::Zone15 => JPRZoneParams {
                lng0: 127.5,
                lat0: 26.0,
            },
            JPRZone::Zone16 => JPRZoneParams {
                lng0: 124.0,
                lat0: 26.0,
            },
            JPRZone::Zone17 => JPRZoneParams {
                lng0: 131.0,
                lat0: 26.0,
            },
            JPRZone::Zone18 => JPRZoneParams {
                lng0: 136.0,
                lat0: 20.0,
            },
            JPRZone::Zone19 => JPRZoneParams {
                lng0: 154.0,
                lat0: 26.0,
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

        let zone = JPRZone::Zone8;
        assert_eq!(zone.zone_number(), 8);
        assert_eq!(zone.zone_roman(), "VIII");

        let tmerc = zone.projection();
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

    #[test]
    fn zones() {
        for i in 1..=19 {
            let zone = JPRZone::from_number(i);
            assert_eq!(zone.zone_number(), i);
            assert!(!zone.zone_roman().is_empty());
        }
    }

    #[test]
    #[should_panic]
    fn invalid_zone_0() {
        let _ = JPRZone::from_number(0);
    }

    #[test]
    #[should_panic]
    fn invalid_zone_20() {
        let _ = JPRZone::from_number(20);
    }
}
