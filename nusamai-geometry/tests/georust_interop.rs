//! geo_types <-> CompactMultiPolygon の相互変換テスト
//!
//! 処理の効率は考慮してない。
//!
//! のちほど CompactGeometry 自体にこれらの相互変換 (From, Into) を実装することを想定。

use geo_types::{Coord, LineString, MultiPolygon, Polygon};
use geojson::GeoJson;
use nusamai_geometry::CompactMultiPolygon;

/// geo_types to compact
fn georust2compact(multipolygon: &MultiPolygon) -> CompactMultiPolygon {
    // 以下は、愚直に命令的に書けば、もっと効率的に短かく書けるはず

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

    let part_indices: Vec<[u32; 2]> = {
        let part_indices_verts = multipolygon
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

        let part_indices_holes = multipolygon
            .0
            .iter()
            .map(|polygon| polygon.interiors().len() as u32)
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            });

        part_indices_verts
            .zip(part_indices_holes)
            .map(|(i1, i2)| [i1, i2])
            .take(multipolygon.0.len() - 1)
            .collect()
    };

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
fn compact2georust(mpoly: &CompactMultiPolygon) -> MultiPolygon {
    let dim = mpoly.dim as usize;

    let mut poly_vert_start = 0;
    let mut poly_hole_start = 0;
    let mut polygons = Vec::with_capacity(mpoly.part_indices.len());

    for [poly_vert_end, poly_hole_end] in mpoly
        .part_indices
        .iter()
        .map(|&[vi, hi]| [vi as usize * dim, hi as usize])
        .chain(Some([mpoly.vertices.len(), mpoly.hole_indices.len()]))
    {
        let vertices = &mpoly.vertices[poly_vert_start..poly_vert_end];
        let hole_indices = &mpoly.hole_indices[poly_hole_start..poly_hole_end];

        // exterior ring
        let exterior_end = hole_indices
            .first()
            .map(|&idx| idx as usize * dim)
            .unwrap_or(vertices.len());
        let exterior = vertices_to_linestring(&vertices[0..exterior_end], dim);

        // interior rings
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

        // next polygon
        poly_vert_start = poly_vert_end;
        poly_hole_start = poly_hole_end;
    }

    MultiPolygon(polygons)
}

#[test]
fn test_georust_multipolygon_interop() {
    let geojson_str = r#"
    {
        "type": "MultiPolygon",
        "coordinates": [
            [
                [[102.0, 2.0], [103.0, 2.0], [103.0, 3.0], [102.0, 3.0], [102.0, 2.0]]
            ],
            [
                [[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]],
                [[100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.2]]
            ],
            [
                [[200.0, 0.0], [201.0, 0.0], [201.0, 1.0], [200.0, 1.0], [200.0, 0.0]],
                [[200.2, 0.2], [200.8, 0.2], [200.2, 0.8], [200.2, 0.2]],
                [[200.2, 0.2], [200.8, 0.2], [200.8, 0.8], [200.2, 0.8], [200.2, 0.2]]
            ]
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

    // Convert to CompactMultiPolygon
    let compact_mpoly = georust2compact(&mpoly);

    // Invert to geo_types::MultiPolygon
    let mpoly_again = compact2georust(&compact_mpoly);

    // Check equality
    assert_eq!(mpoly, mpoly_again);
}
