#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vertex<T> {
    pub position: [T; 3],
    pub tex_coord: [T; 2],
    pub feature_id: u32,
}
