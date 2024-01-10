pub mod tileid;
pub mod webmercator;
pub mod writer;

pub mod vector_tile {
    include!(concat!(env!("OUT_DIR"), "/vector_tile.rs"));
}
