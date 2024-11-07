use tinymvt::tileid::hilbert;

/// Tile ID calculation method
#[derive(Clone, Copy, Debug)]
pub enum TileIdMethod {
    /// Tile ID based on Hilbert curve (compliant with PMTiles)
    Hilbert,
}

impl TileIdMethod {
    pub fn zxy_to_id(&self, z: u8, x: u32, y: u32) -> u64 {
        match self {
            TileIdMethod::Hilbert => hilbert::zxy_to_id(z, x, y),
        }
    }

    pub fn id_to_zxy(&self, tile_id: u64) -> (u8, u32, u32) {
        match self {
            TileIdMethod::Hilbert => hilbert::id_to_zxy(tile_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hilbert() {
        let tile_id_method = TileIdMethod::Hilbert;
        assert_eq!(tile_id_method.zxy_to_id(2, 1, 1), 7);
        assert_eq!(tile_id_method.id_to_zxy(7), (2, 1, 1));
    }
}
