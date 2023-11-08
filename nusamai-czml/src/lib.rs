use serde_json::Value;
use std::fs;

pub enum GeometryIndices {
    MultiPoint,
    MultiLineString {
        linestring_indices: Vec<Vec<u32>>,
    },
    MultiPolygon {
        polygon_indices: Vec<Vec<u32>>,
        hole_indices: Vec<Vec<u32>>,
    },
}

pub struct Geometry {
    /// 頂点データ
    pub vertices: Vec<Vec<f64>>,
    /// 頂点の次元 (3 or 2)
    pub dim: u8,
    /// 頂点データ配列のインデックス
    pub indices: GeometryIndices,
}

fn load_geojson() -> String {
    // ./data/sample.geojsonを読み込む
    let data = fs::read_to_string("./data/sample.geojson").expect("Unable to read file");
    data
}

fn convert_to_geometry(geojson: &str) -> Result<Geometry, serde_json::Error> {
    // GeoJSON文字列をパースする
    let v: Value = serde_json::from_str(geojson)?;

    // "features"の配列から最初の"geometry"オブジェクトを取得する
    let geometry = &v["features"][0]["geometry"];

    // "type"プロパティを使用して、ジオメトリのタイプを取得する
    let geometry_type = geometry["type"].as_str();
    println!("geometry_type: {:?}\n", geometry_type);

    // "coordinates"を取得し、フラットな配列に変換する
    let coordinates = geometry["coordinates"].as_array();

    let mut vertices: Vec<Vec<f64>> = Vec::new();
    let mut indices: Vec<Vec<u32>> = Vec::new();

    // "coordinates"の値を順番に取得する
    for polygon in coordinates.unwrap() {
        // Polygonの頂点を取得する
        let polygon_vertices = polygon.as_array().unwrap();
        println!("polygon_vertices: {:?}\n", polygon_vertices);

        // Polygonの頂点を順番に取得する
        for vertex in polygon_vertices {
            // 頂点の座標を取得する
            let vertex_coordinates = vertex.as_array().unwrap();
            println!("vertex_coordinates: {:?}\n", vertex_coordinates);

            let mut v = Vec::new();
            // 頂点の座標を順番に取得する
            for coordinate in vertex_coordinates {
                // 頂点の座標をf64に変換してverticesに追加する
                let coordinate_f64 = coordinate.as_f64().unwrap();
                v.push(coordinate_f64);
            }
            vertices.push(v);
        }
        // indicesを更新する
        // 頂点が6つある場合、indicesは[0, 1, 2, 3, 4, 5]になる
        // indicesがすでに存在する場合は、その長さを足す
        // iはVec<u32>で出力する
        let mut i = (0..vertices.len() as u32).collect::<Vec<_>>();
        // indicesに既に要素が格納されているなら、iの要素全てにindicesの最大値を足す
        let max = indices.iter().map(|x| indices.len()).max().unwrap_or(0) as u32;
        println!("max: {:?}\n", max);

        // iは[0, 1, 2, 3, 4, 5]のような配列になっている。これにmaxを足す
        i = i.iter().map(|x| x + max).collect::<Vec<_>>();
        println!("i: {:?}\n", i);
        indices.push(i);
    }
    println!("vertices: {:?}\n", vertices);
    println!("indices: {:?}\n", indices);

    // Geometry構造体を作成する
    let geometry_struct = Geometry {
        vertices,
        dim: 3, // この例では全ての座標に3つの値があるため、次元は3
        indices: GeometryIndices::MultiPolygon {
            // polygon_indices: indices,
            polygon_indices: Vec::new(),
            hole_indices: Vec::new(), // この例には穴がないので空のVec
        },
    };
    Ok(geometry_struct)
}

fn to_czml() {}

#[cfg(test)]
mod tests {
    use super::*;

    // load_geojson関数をテストする関数です。
    #[test]
    fn test_load_geojson() {
        // この関数が実際にファイルを読み込むかをテストします。
        // テスト用のsample.geojsonファイルを作成し、適切なコンテンツを配置しておく必要があります。
        let data = load_geojson();
        assert!(!data.is_empty(), "GeoJSON data should not be empty");
    }

    // convert_to_geometry関数をテストする関数です。
    #[test]
    fn test_convert_to_geometry() {
        // まず、GeoJSONデータをロードします。
        let geojson_data = load_geojson();

        // convert_to_geometry関数を使用して変換を試みます。
        let geometry_result = convert_to_geometry(&geojson_data);
    }
}
