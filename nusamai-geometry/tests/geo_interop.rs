//! geo_types <-> CompactMultiPolygon の相互変換テスト
//!
//! 処理の効率は考慮してない。
//!
//! のちほど CompactGeometry 自体にこれらの相互変換 (From, Into) を実装することを想定。

use geo_types::{Coord, LineString, MultiPolygon, Polygon};
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

fn vertices_to_linestring(vertices: &[f64], dim: usize) -> LineString<f64> {
    return LineString::new(
        vertices
            .chunks_exact(dim)
            .map(|a| Coord { x: a[0], y: a[1] })
            .collect(),
    );
}

/// compact to geo_types
fn compact2geo_type(mpoly: &CompactMultiPolygon) -> MultiPolygon {
    let dim = mpoly.dim as usize;
    let mut poly_vert_start = 0;
    let mut poly_hole_start = 0;
    let poly_end = [mpoly.vertices.len(), mpoly.hole_indices.len()];
    let mut polygons = Vec::with_capacity(mpoly.part_indices.len());

    for [vert_end, hole_end] in mpoly
        .part_indices
        .iter()
        .map(|&[vi, hi]| [vi as usize * dim, hi as usize])
        .chain(Some(poly_end))
    {
        let vertices = &mpoly.vertices[poly_vert_start..vert_end];
        let hole_indices = &mpoly.hole_indices[poly_hole_start..hole_end];

        let exterior_end = if hole_indices.is_empty() {
            vertices.len()
        } else {
            hole_indices[0] as usize * dim
        };
        let exterior = vertices_to_linestring(&vertices[0..exterior_end], dim);

        let mut interiors = Vec::with_capacity(hole_indices.len());
        let mut hole_start = exterior_end;
        for hole_end in hole_indices
            .iter()
            .map(|&start| start as usize * dim)
            .chain(Some(vertices.len()))
            .skip(1)
        {
            interiors.push(vertices_to_linestring(&vertices[hole_start..hole_end], dim));
            hole_start = hole_end;
        }

        polygons.push(Polygon::new(exterior, interiors));

        // Polygon::new(exterior.into(), interiors.into());

        // next polygon
        poly_vert_start = vert_end;
        poly_hole_start = hole_end;
    }

    MultiPolygon(polygons)
}

#[test]
fn main() {
    let geojson_str = r#"
    {
        "type": "MultiPolygon",
        "coordinates": [
            [[[102.0, 2.0], [103.0, 2.0], [103.0, 3.0], [102.0, 3.0], [102.0, 2.0]]],
            [[[102.0, 2.0], [103.0, 2.0], [103.0, 3.0], [102.0, 3.0], [102.0, 2.0]]],
            [[[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]],
             [[100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.2]]],
            [[[200.0, 0.0], [201.0, 0.0], [201.0, 1.0], [200.0, 1.0], [200.0, 0.0]],
             [[200.2, 0.2], [200.8, 0.2], [200.2, 0.8], [200.2, 0.2]],
             [[200.2, 0.2], [200.8, 0.2], [200.8, 0.8], [200.2, 0.8], [200.2, 0.2]]]
        ]
    }
    "#;

    // Load GeoJSON
    let Ok(GeoJson::Geometry(geometry)) = geojson_str.parse::<GeoJson>() else {
        panic!();
    };

    // Convert to geo_types::MultiPolygon
    let Ok(mpoly) = geometry.value.try_into() else {
        panic!();
    };
    println!("mpoly: {:?}", mpoly);
    println!("↓");

    // Convert to CompactMultiPolygon
    let compact_mpoly = geo_type2compact(&mpoly);
    println!("compact mpoly: {:?}", compact_mpoly);
    println!("↓");

    // Invert to geo_types::MultiPolygon
    let mpoly_again = compact2geo_type(&compact_mpoly);
    println!("{:?}", mpoly_again);

    // Check equality
    assert_eq!(mpoly, mpoly_again);
}
