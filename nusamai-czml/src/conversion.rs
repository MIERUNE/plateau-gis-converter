use flatgeom::{MultiPolygon, Polygon};

use crate::{
    models::CzmlPolygon, PositionList, PositionListOfLists, PositionListOfListsProperties,
    PositionListProperties,
};

pub fn indexed_polygon_to_czml_polygon(
    vertices: &[[f64; 3]],
    poly_idx: &Polygon<u32>,
) -> CzmlPolygon {
    let mut czml_polygon = CzmlPolygon::default();

    let mut exteriors = Vec::new();
    for idx in poly_idx.exterior().iter() {
        exteriors.push(vertices[idx as usize]);
    }

    let mut interiors = Vec::new();
    for interior in poly_idx.interiors() {
        let mut interior_vec = Vec::new();
        for idx in interior.iter() {
            interior_vec.push(vertices[idx as usize]);
        }
        interiors.push(interior_vec);
    }

    czml_polygon.positions = Some(PositionList::Object(PositionListProperties {
        cartographic_degrees: Some(exteriors.into_iter().flatten().collect()),
        ..Default::default()
    }));

    czml_polygon.holes = Some(PositionListOfLists::Object(PositionListOfListsProperties {
        cartographic_degrees: Some(
            interiors
                .into_iter()
                .map(|x| x.into_iter().flatten().collect())
                .collect(),
        ),
        ..Default::default()
    }));

    czml_polygon
}

pub fn indexed_multipolygon_to_czml_polygon(
    vertices: &[[f64; 3]],
    mpoly: &MultiPolygon<u32>,
) -> CzmlPolygon {
    let mut czml_polygon = CzmlPolygon::default();

    let mut exteriors = Vec::new();
    for poly in mpoly.iter() {
        for idx in poly.exterior().iter() {
            exteriors.push(vertices[idx as usize]);
        }
    }

    let mut interiors = Vec::new();
    for poly in mpoly.iter() {
        for interior in poly.interiors() {
            let mut interior_vec = Vec::new();
            for idx in interior.iter() {
                interior_vec.push(vertices[idx as usize]);
            }
            interiors.push(interior_vec);
        }
    }

    czml_polygon.positions = Some(PositionList::Object(PositionListProperties {
        cartographic_degrees: Some(exteriors.into_iter().flatten().collect()),
        ..Default::default()
    }));

    czml_polygon.holes = Some(PositionListOfLists::Object(PositionListOfListsProperties {
        cartographic_degrees: Some(
            interiors
                .into_iter()
                .map(|x| x.into_iter().flatten().collect())
                .collect(),
        ),
        ..Default::default()
    }));

    czml_polygon
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Packet;

    #[test]
    fn test_polygon() {
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

        let mut poly_idx = Polygon::<u32>::new();
        // Adding one ring completes the outline and all remaining rings are treated as holes
        poly_idx.add_ring([0, 1, 2, 3, 0]);
        poly_idx.add_ring([4, 5, 6, 7, 4]);
        poly_idx.add_ring([8, 9, 10, 11, 8]);

        let czml_polygon = indexed_polygon_to_czml_polygon(&vertices, &poly_idx);

        let packet = Packet {
            polygon: Some(czml_polygon),
            ..Default::default()
        };

        let packets = vec![packet];

        let json = serde_json::to_string(&packets).unwrap();

        assert_eq!(
            json,
            r#"[{"polygon":{"positions":{"cartographicDegrees":[0.0,0.0,111.0,5.0,0.0,111.0,5.0,5.0,111.0,0.0,5.0,111.0]},"holes":{"cartographicDegrees":[[1.0,1.0,111.0,2.0,1.0,111.0,2.0,2.0,111.0,1.0,2.0,111.0],[3.0,3.0,111.0,4.0,3.0,111.0,4.0,4.0,111.0,3.0,4.0,111.0]]}}}]"#
        )
    }

    #[test]
    fn test_multipolygon() {
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
            // 2nd polygon, exterior (vertex 12~15)
            [4., 0., 222.],
            [7., 0., 222.],
            [7., 3., 222.],
            [4., 3., 222.],
            // 2nd polygon, interior (vertex 16~19)
            [5., 1., 222.],
            [6., 1., 222.],
            [6., 2., 222.],
            [5., 2., 222.],
            // 3rd polygon, exterior (vertex 20~23)
            [4., 0., 333.],
            [7., 0., 333.],
            [7., 3., 333.],
            [4., 3., 333.],
        ];

        let mut mpoly = MultiPolygon::<u32>::new();
        // 1st polygon
        mpoly.add_exterior([0, 1, 2, 3, 0]);
        mpoly.add_interior([4, 5, 6, 7, 4]);
        mpoly.add_interior([8, 9, 10, 11, 8]);
        // 2nd polygon
        mpoly.add_exterior([12, 13, 14, 15, 12]);
        mpoly.add_interior([16, 17, 18, 19, 16]);
        // 3rd polygon
        mpoly.add_exterior([20, 21, 22, 23, 20]);

        let czml_polygon = indexed_multipolygon_to_czml_polygon(&vertices, &mpoly);

        let packet = Packet {
            polygon: Some(czml_polygon),
            ..Default::default()
        };

        let packets = vec![packet];

        let json = serde_json::to_string(&packets).unwrap();

        assert_eq!(
            json,
            r#"[{"polygon":{"positions":{"cartographicDegrees":[0.0,0.0,111.0,5.0,0.0,111.0,5.0,5.0,111.0,0.0,5.0,111.0,4.0,0.0,222.0,7.0,0.0,222.0,7.0,3.0,222.0,4.0,3.0,222.0,4.0,0.0,333.0,7.0,0.0,333.0,7.0,3.0,333.0,4.0,3.0,333.0]},"holes":{"cartographicDegrees":[[1.0,1.0,111.0,2.0,1.0,111.0,2.0,2.0,111.0,1.0,2.0,111.0],[3.0,3.0,111.0,4.0,3.0,111.0,4.0,4.0,111.0,3.0,4.0,111.0],[5.0,1.0,222.0,6.0,1.0,222.0,6.0,2.0,222.0,5.0,2.0,222.0]]}}}]"#
        )
    }
}
