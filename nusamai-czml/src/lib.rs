use serde_json::Value;
use std::fs;

enum GeometryIndices {
    MultiPoint,
    MultiLineString {
        linestring_indices: Vec<Vec<u32>>,
    },
    MultiPolygon {
        polygon_indices: Vec<Vec<u32>>,
        hole_indices: Vec<Vec<u32>>,
    },
}

struct Geometry {
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

    let features = &value["features"].as_array().unwrap();

    let geom_list = features
        .iter()
        .map(|x| x["geometry"].clone())
        .collect::<Vec<_>>();

    let mut vertices: Vec<Vec<f64>> = Vec::new();
    let mut indices: Vec<Vec<u32>> = Vec::new();

    for geom in geom_list {
        let coordinates = geom["coordinates"].as_array();
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

            let max = indices.iter().flatten().max();
            if max.is_none() {
                indices.push(i);
                continue;
            }
            i = i.iter().map(|x| x + max.unwrap() + 1).collect::<Vec<_>>();
            indices.push(i);
        }
    }

    let geometry_struct = Geometry {
        vertices,
        dim: 3,
        indices: GeometryIndices::MultiPolygon {
            polygon_indices: indices,
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
        let geometry = convert_to_geometry(&geojson_data);

        assert!(geometry.is_ok(), "Geometry should be ok");
        assert_eq!(
            geometry.unwrap().vertices.len(),
            13,
            "Geometry should have 13 vertices"
        );
    }
}
