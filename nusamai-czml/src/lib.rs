use serde_json::Value;
use std::fs;

/// 計算や空間上の効率を優先したジオメトリの表現
#[derive(Debug)]
pub enum CompactGeometry {
    MultiPoint(CompactMultiPoint),
    MultiLineString(CompactMultiLineString),
    MultiPolygon(CompactMultiPolygon),
}

/// 計算や空間上の効率を優先した MultiPoint の表現
#[derive(Debug)]
pub struct CompactMultiPoint {
    /// 頂点データ
    pub vertices: Vec<f64>,

    /// 頂点の次元 (3 or 2)
    pub dim: u8,
}

/// 計算や空間上の効率を優先した MultiString の表現
#[derive(Debug)]
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

/// 計算や空間上の効率を優先した MultiPolygon の表現
#[derive(Debug)]
pub struct CompactMultiPolygon {
    /// 頂点データ
    pub vertices: Vec<f64>,

    /// 頂点の次元 (3 or 2)
    pub dim: u8,

    /// 各 holes (interior rings) の開始インデックス
    /// ```
    /// [
    ///    6, 10, 16,
    ///    5, 10,
    /// ]
    /// ```
    pub hole_indices: Vec<u32>,

    /// 各 LineString の開始位置
    /// (hole_indices のインデックス, 頂点のインデックス)
    ///
    /// e.g. `[ [3, 18], [5, 32] ]`
    ///
    /// 1つ目の Polygon の開始位置は `[0, 0]` であることが自明なので省略
    pub part_indices: Vec<[u32; 2]>,
}

struct CzmlBase {
    pub id: String,
    pub name: String,
    pub description: String,
}

fn load_geojson(filepath: String) -> String {
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    data
}

fn convert_to_geometry(geojson: &str) -> Result<CompactMultiPolygon, serde_json::Error> {
    let value: Value = serde_json::from_str(geojson).unwrap();

    let features = &value["features"].as_array().unwrap();

    let geom_list = features
        .iter()
        .map(|x| x["geometry"].clone())
        .collect::<Vec<_>>();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for geom in geom_list {
        let coordinates = geom["coordinates"].as_array();

        for polygon in coordinates.unwrap() {
            let polygon_vertices = polygon.as_array().unwrap();

            for vertex in polygon_vertices {
                let vertex_coordinates = vertex.as_array().unwrap();

                for coordinate in vertex_coordinates {
                    let coordinate_f64 = coordinate.as_f64().unwrap();
                    vertices.push(coordinate_f64);
                }
            }
            let index = vertices.len() as u32;
            // 穴あきポリゴンは一切存在しないという想定で仮の実装
            indices.push([index, 0]);
        }
    }

    let geometry_struct = CompactMultiPolygon {
        vertices,
        dim: 3,
        hole_indices: Vec::new(),
        part_indices: indices,
    };
    Ok(geometry_struct)
}

fn to_czml() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_geojson() {
        let data = load_geojson(String::from("./data/sample.geojson"));
        assert!(!data.is_empty(), "GeoJSON data should not be empty");
    }

    #[test]
    fn test_convert_to_geometry() {
        let geojson_data = load_geojson(String::from("./data/sample.geojson"));
        let geometry = convert_to_geometry(&geojson_data);

        assert!(geometry.is_ok(), "Geometry should be ok");
        let vertices_length = geometry.as_ref().unwrap().vertices.len();
        let part_indices_length = geometry.as_ref().unwrap().part_indices.len();
        assert_eq!(vertices_length, 39, "Geometry should have 39 vertices");
        assert_eq!(part_indices_length, 2, "Geometry should have 2 parts")
    }
}
