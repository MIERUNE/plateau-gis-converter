use geo_types::MultiPolygon;
use geojson::GeoJson;
use nusamai_geometry::CompactMultiPolygon;

/// geo_types to compact
fn geo_type2compact(multipolygon: &MultiPolygon) -> CompactMultiPolygon {
    let vertices: Vec<_> = multipolygon
        .0
        .iter()
        .flat_map(|polygon| {
            Some(polygon.exterior())
                .into_iter()
                .chain(polygon.interiors().iter())
                .flat_map(|ring| ring.0.iter().flat_map(|point| [point.x, point.y]))
        })
        .collect();

    let hole_indices: Vec<u32> = multipolygon
        .0
        .iter()
        .flat_map(|polygon| {
            Some(polygon.exterior())
                .into_iter()
                .chain(polygon.interiors().iter())
                .map(|ring| ring.0.len() as u32)
                .scan(0, |state, x| {
                    *state += x;
                    Some(*state)
                })
                .take(polygon.interiors().len())
        })
        .collect();

    let part_indices1 = multipolygon
        .0
        .iter()
        .map(|polygon| {
            let a = polygon.exterior().0.len() as u32;
            let b: u32 = polygon.interiors().iter().map(|r| r.0.len() as u32).sum();
            a + b
        })
        .scan(0, |state, x: u32| {
            *state += x;
            Some(*state)
        });

    let part_indices2 = multipolygon
        .0
        .iter()
        .map(|polygon| polygon.interiors().len() as u32)
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        });

    let part_indices: Vec<[u32; 2]> = part_indices1
        .zip(part_indices2)
        .map(|(i1, i2)| [i1, i2])
        .take(multipolygon.0.len() - 1)
        .collect();

    CompactMultiPolygon {
        vertices,
        dim: 2,
        hole_indices,
        part_indices,
    }
}

// fn compact2geo_type(mpoly: &CompactMultiPolygon) -> MultiPolygon {
//     todo!();
// }

fn main() {
    let geojson_str = r#"
    {
        "type": "MultiPolygon",
        "coordinates": [
            [[[102.0, 2.0], [103.0, 2.0], [103.0, 3.0], [102.0, 3.0], [102.0, 2.0]]],
            [[[102.0, 2.0], [103.0, 2.0], [103.0, 3.0], [102.0, 3.0], [102.0, 2.0]]],
            [[[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]],
             [[100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.8], [100.2, 0.2]]],
            [[[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]],
             [[100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.8], [100.2, 0.2]]]
        ]
    }
    "#;

    // Load GeoJSON
    let GeoJson::Geometry(geometry) = geojson_str.parse::<GeoJson>().unwrap() else {
        panic!();
    };

    // Convert to geo_types::MultiPolygon
    let mpoly: geo_types::MultiPolygon = geometry.value.try_into().unwrap();
    println!("mpoly: {:?}", mpoly);
    println!("↓");

    // Convert to CompactMultiPolygon
    let compact_mpoly = geo_type2compact(&mpoly);
    println!("compact mpoly: {:?}", compact_mpoly);
}
