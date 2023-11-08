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
    pub vertices: Vec<Vec<f64>>,
    pub dim: u8,
    pub indices: GeometryIndices,
}

fn load_geojson(filepath: String) -> String {
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    data
}

fn convert_to_geometry(geojson: &str) -> Result<Geometry, serde_json::Error> {
    let value: Value = serde_json::from_str(geojson)?;

    let geometry = &value["features"][0]["geometry"];
    let geometry_type = geometry["type"].as_str();
    let coordinates = geometry["coordinates"].as_array();

    let mut vertices: Vec<Vec<f64>> = Vec::new();
    let mut indices: Vec<Vec<u32>> = Vec::new();

    for polygon in coordinates.unwrap() {
        let polygon_vertices = polygon.as_array().unwrap();
        for vertex in polygon_vertices {
            let vertex_coordinates = vertex.as_array().unwrap();

            let mut v = Vec::new();
            for coordinate in vertex_coordinates {
                let coordinate_f64 = coordinate.as_f64().unwrap();
                v.push(coordinate_f64);
            }
            vertices.push(v);
        }
        let mut i = (0..vertices.len() as u32).collect::<Vec<_>>();
        let max = indices.iter().map(|x| indices.len()).max().unwrap_or(0) as u32;

        i = i.iter().map(|x| x + max).collect::<Vec<_>>();
        indices.push(i);
    }

    let geometry_struct = Geometry {
        vertices,
        dim: 3,
        indices: GeometryIndices::MultiPolygon {
            polygon_indices: Vec::new(),
            hole_indices: Vec::new(),
        },
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
        let geometry_result = convert_to_geometry(&geojson_data);
    }
}
