use nusamai_geometry::Polygon;

use crate::{
    models::CzmlPolygon, PositionList, PositionListOfLists, PositionListOfListsProperties,
    PositionListProperties,
};

//
pub fn indexed_multipolygon_to_czml_polygon(
    vertices: &[[f64; 3]],
    poly_idx: &Polygon<1, u32>,
) -> CzmlPolygon {
    let mut czml_polygon = CzmlPolygon::default();

    let mut exteriors = Vec::new();
    for idx in poly_idx.exterior().iter() {
        exteriors.push(vertices[idx[0] as usize]);
    }

    let mut interiors = Vec::new();
    for interior in poly_idx.interiors() {
        let mut interior_vec = Vec::new();
        for idx in interior.iter() {
            interior_vec.push(vertices[idx[0] as usize]);
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

#[cfg(test)]
mod tests {
    use crate::Packet;

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
        // Adding one ring completes the outline and all remaining rings are treated as holes
        poly_idx.add_ring([[0], [1], [2], [3], [0]]);
        poly_idx.add_ring([[4], [5], [6], [7], [4]]);
        poly_idx.add_ring([[8], [9], [10], [11], [8]]);

        let czml_polygon = indexed_multipolygon_to_czml_polygon(&vertices, &poly_idx);

        let packet = Packet {
            id: Some("test".to_string()),
            polygon: Some(czml_polygon),
            ..Default::default()
        };

        let packets = vec![packet];

        let json = serde_json::to_string(&packets).unwrap();

        assert_eq!(
            json,
            r#"[{"id":"test","availability":"0000-00-00T00:00:00Z/9999-12-31T24:00:00Z","polygon":{"show":true,"positions":{"referenceFrame":"FIXED","cartographicDegrees":[0.0,0.0,111.0,5.0,0.0,111.0,5.0,5.0,111.0,0.0,5.0,111.0]},"holes":{"cartographicDegrees":[[1.0,1.0,111.0,2.0,1.0,111.0,2.0,2.0,111.0,1.0,2.0,111.0],[3.0,3.0,111.0,4.0,3.0,111.0,4.0,4.0,111.0,3.0,4.0,111.0]]},"arcType":"GEODESIC","height":0.0,"heightReference":"NONE","extrudedHeightReference":"NONE","stRotation":0.0,"granularity":0.0174532,"fill":true,"material":{"solidColor":{"color":{"rgba":[255,255,255,255]}}},"outline":false,"outlineColor":{"rgba":[0,0,0,255]},"outlineWidth":1.0,"perPositionHeight":false,"closeTop":true,"closeBottom":true,"shadows":"DISABLED","classificationType":"BOTH","zIndex":0}}]"#
        )
    }
}
