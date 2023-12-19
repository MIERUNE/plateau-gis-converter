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
                lat0: 33.0,
            },
            JPRZone::Zone5 => JPRZoneParams {
                lng0: 134.333_333_333_333_33,
                lat0: 36.0,
            },
            JPRZone::Zone6 => JPRZoneParams {
                lng0: 136.0,
                lat0: 36.0,
            },
            JPRZone::Zone7 => JPRZoneParams {
                lng0: 137.166_666_666_666_67,
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
    fn zones() {
        // test fixtures made with PROJ
        const DATA: [(f64, f64); 19] = [
            (129.5001070162252, 33.00009017667281),
            (131.00042806620712, 33.00036070613305),
            (132.1676649612675, 36.00081118731552),
            (133.501712285731, 33.001442815599916),
            (134.3361064243794, 36.002253277312406),
            (136.0039933010935, 36.00324469874967),
            (137.172102073566, 36.0044163624128),
            (138.50709942815922, 36.005768260190365),
            (139.8423187209358, 36.00730038272098),
            (140.84504648774808, 40.00900650080927),
            (140.26509038599607, 44.01088997134999),
            (142.26795943105512, 44.01295973802651),
            (144.27107818464975, 44.01520940180659),
            (142.01958184674254, 26.017691869346883),
            (127.52247965928949, 26.020309314749657),
            (124.02557746313299, 26.023107208243783),
            (131.02887528672798, 26.02608553672523),
            (136.03097002921405, 20.029267328281904),
            (154.03607111621068, 26.032583441961588),
        ];

        for zone_no in 1..=19 {
            let zone = JPRZone::from_number(zone_no);
            assert_eq!(zone.zone_number(), zone_no);
            assert!(!zone.zone_roman().is_empty());

            let proj = zone.projection();

            // (x: 0, y: 0) => zone origin (lng0, lat0)
            let (lng, lat, _) = proj.project_inverse(0., 0., 0.).unwrap();
            let params = zone.params();
            assert!((params.lng0 - lng).abs() < 1e-12);
            assert!((params.lat0 - lat).abs() < 1e-12);

            // (x: 10 * zone_no * zone_no, y: 10 * zone_no * zone_no) => DATA[zone_no - 1]
            let x = (10 * zone_no * zone_no) as f64;
            let y = (10 * zone_no * zone_no) as f64;
            let (lng, lat, _) = proj.project_inverse(x, y, 0.).unwrap();
            assert!((DATA[zone_no - 1].0 - lng).abs() < 1e-12);
            assert!((DATA[zone_no - 1].1 - lat).abs() < 1e-12);

            // round-trip
            let (x2, y2, _) = proj.project_forward(lng, lat, 0.).unwrap();
            assert!((x - x2).abs() < 1e-8);
            assert!((y - y2).abs() < 1e-8);
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
