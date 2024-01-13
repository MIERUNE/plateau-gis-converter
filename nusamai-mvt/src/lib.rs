pub mod geometry;
pub mod tag;
pub mod tileid;
pub mod vector_tile;
pub mod webmercator;

/// Tile coordinate in (z, x, y) format.
pub type TileZXY = (u8, u32, u32);
