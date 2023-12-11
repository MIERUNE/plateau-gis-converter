use japan_geoid::{Geoid, MemoryGrid};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/data/"]
struct Asset;

/// Convert from JGD 2011 Geograhpic 3D (EPSG:6697) to WGS84 Geograhpic 3D (EPSG:4979)
pub struct JGD2011ToWGS84 {
    geoid: MemoryGrid<'static>,
}

impl JGD2011ToWGS84 {
    /// Create a new instance with the embed geoid model data.
    pub fn from_embed_model() -> Self {
        let Some(embed_file) = Asset::get("gsigeo2011_ver2_2.bin.lz4") else {
            panic!("Failed to load embedded model file");
        };
        let decompressed = &lz4_flex::decompress_size_prepended(&embed_file.data).unwrap();
        let mut reader = std::io::Cursor::new(decompressed);

        Self {
            geoid: MemoryGrid::from_binary_reader(&mut reader).unwrap(),
        }
    }

    /// JGD2011 Geographic 3D (EPSG:6697) to WGS84 Geographic 3D (EPSG:4979)
    pub fn convert(&self, lng: f64, lat: f64, height: f64) -> (f64, f64, f64) {
        let ellipsoid_height = self.geoid.get_height(lng, lat) + height;
        (lng, lat, ellipsoid_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixtures() {
        let (lng_jgd, lat_jgd, elevation) = (138.2839817085188, 37.12378643088312, 0.);
        let jgd_to_wgs = JGD2011ToWGS84::from_embed_model();
        let (lng_wgs, lat_wgs, ellips_height) = jgd_to_wgs.convert(lng_jgd, lat_jgd, elevation);
        assert!((ellips_height - 39.47387115961899).abs() < 1e-8);
        // (lng, lat) must not change.
        assert_eq!(lng_jgd, lng_wgs);
        assert_eq!(lat_jgd, lat_wgs);
    }
}
