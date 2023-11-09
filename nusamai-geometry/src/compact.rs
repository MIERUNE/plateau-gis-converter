/// 計算や空間の効率を優先したジオメトリの表現
#[derive(Debug, Clone)]
pub enum CompactGeometry {
    MultiPoint(CompactMultiPoint),
    MultiLineString(CompactMultiLineString),
    MultiPolygon(CompactMultiPolygon),
}

/// 計算や空間の効率を優先した MultiPoint の表現
#[derive(Debug, Clone)]
pub struct CompactMultiPoint {
    /// 頂点データ
    pub vertices: Vec<f64>,

    /// 頂点の次元 (3 or 2)
    pub dim: u8,
}

/// 計算や空間の効率を優先した MultiString の表現
#[derive(Debug, Clone)]
pub struct CompactMultiLineString {
    /// 頂点データ
    ///
    /// `[x0, y0, z0, x1, y1, z1, ...]`
    pub vertices: Vec<f64>,

    /// 頂点の次元 (3 or 2)
    pub dim: u8,

    /// 各 LineString の開始位置
    ///
    /// e.g. `[5, 12, 23]`
    ///
    /// 1つ目の LineString の開始位置は `0` であることが自明なので省略
    pub part_indices: Vec<u32>,
}

/// 計算や空間の効率を優先した MultiPolygon の表現
#[derive(Debug, Clone)]
pub struct CompactMultiPolygon {
    /// 頂点データ
    pub vertices: Vec<f64>,

    /// 頂点の次元 (3 or 2)
    pub dim: u8,

    /// 各 holes (interior rings) の開始インデックス
    ///
    /// 1つ目の hole の開始位置は `0` であることが明らかなので省略
    pub hole_indices: Vec<u32>,

    /// 各 Polygon の開始位置
    ///
    /// (verticesの開始インデックス, hole_indices の開始インデックス) の組
    ///
    /// e.g. `[[18, 3], [32, 5]]`
    ///
    /// 1つ目の Polygon の開始位置は `[0, 0]` であることが明らかなので省略
    pub part_indices: Vec<[u32; 2]>,
}
