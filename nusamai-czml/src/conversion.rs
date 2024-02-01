use nusamai_geometry::Polygon;

use crate::models::CzmlPolygon;

//
pub fn indexed_multipolygon_to_czml_polygon(
    vertices: &[[f64; 3]],
    poly_idx: &Polygon<1, u32>,
) -> CzmlPolygon {
    let mut czml_polygon = CzmlPolygon::default();

    // 外形の頂点を追加
    let mut exteriors = Vec::new();
    for idx in poly_idx.exterior().iter() {
        exteriors.push(vertices[idx[0] as usize]);
    }

    // ホールの頂点を追加
    let mut interiors = Vec::new();
    for interior in poly_idx.interiors() {
        let mut interior_vec = Vec::new();
        for idx in interior.iter() {
            interior_vec.push(vertices[idx[0] as usize]);
        }
        interiors.push(interior_vec);
    }

    czml_polygon
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test() {
        let vertices: Vec<[f64; 3]> = vec![
            // 1st polygon, exterior (vertex 0~3)
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
            // 1st polygon, interior 1 (vertex 4~7)
            [1., 1., 111.],
            [2., 1., 111.],
            [2., 2., 111.],
            [1., 2., 111.],
            // 1st polygon, interior 2 (vertex 8~11)
            [3., 3., 111.],
            [4., 3., 111.],
            [4., 4., 111.],
            [3., 4., 111.],
        ];

        let mut poly_idx = Polygon::<1, u32>::new();
        // マルチではない単体のポリゴンなので、1つリングを追加すると外形は完成し、残りのリングはすべてホールとして扱われる
        poly_idx.add_ring([[0], [1], [2], [3], [0]]);
        poly_idx.add_ring([[4], [5], [6], [7], [4]]);
        poly_idx.add_ring([[8], [9], [10], [11], [8]]);

        let value = indexed_multipolygon_to_czml_polygon(&vertices, &poly_idx);
    }
}
