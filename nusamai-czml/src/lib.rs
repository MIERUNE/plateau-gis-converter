use serde_json::Value;
use std::fs;

mod types;
use types::CompactMultiPolygon;

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
