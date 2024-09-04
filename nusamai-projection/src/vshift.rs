use japan_geoid::{gsi::MemoryGrid, Geoid};

#[derive(Debug)]
/// Convert from JGD 2011 Geograhpic 3D (EPSG:6697) to WGS84 Geograhpic 3D (EPSG:4979)
pub struct Jgd2011ToWgs84 {
    geoid: MemoryGrid<'static>,
}

impl Jgd2011ToWgs84 {
    /// Create a new instance with the embed geoid model data.
    pub fn new() -> Self {
        Self {
            geoid: japan_geoid::gsi::load_embedded_gsigeo2011(),
        }
    }

    /// JGD2011 Geographic 3D (EPSG:6697) to WGS84 Geographic 3D (EPSG:4979)
    pub fn convert(&self, lng: f64, lat: f64, height: f64) -> (f64, f64, f64) {
        let ellipsoid_height = self.geoid.get_height(lng, lat) + height;
        (lng, lat, ellipsoid_height)
    }
}

impl Default for Jgd2011ToWgs84 {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Jgd2011ToWgs84 {
    fn clone(&self) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixtures() {
        let (lng_jgd, lat_jgd, elevation) = (138.2839817085188, 37.12378643088312, 0.);
        let jgd_to_wgs = Jgd2011ToWgs84::new();
        let (lng_wgs, lat_wgs, ellips_height) = jgd_to_wgs.convert(lng_jgd, lat_jgd, elevation);
        assert!((ellips_height - 39.47387115961899).abs() < 1e-8);
        // (lng, lat) must not change.
        assert_eq!(lng_jgd, lng_wgs);
        assert_eq!(lat_jgd, lat_wgs);
    }
}
