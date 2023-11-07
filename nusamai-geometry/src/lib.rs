pub enum GeometryIndices {
    MultiPoint,
    MultiLineString {
        linestring_indices: Vec<u32>,
    },
    MultiPolygon {
        polygon_indices: Vec<u32>,
        hole_indices: Vec<u32>,
    },
}

pub struct Geometry {
    /// 頂点データ
    pub vertices: Vec<f64>,
    /// 頂点の次元 (3 or 2)
    pub dim: u8,
    /// 頂点データ配列のインデックス
    pub indices: GeometryIndices,
}
