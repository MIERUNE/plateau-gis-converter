#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vertex {
    pub position: [u32; 3],  // f32.to_bits()
    pub tex_coord: [u32; 2], // f32.to_bits()
    pub feature_id: u32,
}
