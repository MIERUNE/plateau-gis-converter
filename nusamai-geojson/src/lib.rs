mod conversion;

use nusamai_plateau::TopLevelCityObject;

/// Create GeoJSON features from a TopLevelCityObject
// TODO: Handle properties (`obj.root` -> `geojson::Feature.properties`)
pub fn toplevel_cityobj_to_geojson_features(obj: &TopLevelCityObject) -> Vec<geojson::Feature> {
    let mut geojson_features: Vec<geojson::Feature> = vec![];

    if !obj.geometries.multipolygon.is_empty() {
        let mpoly_geojson_geom = conversion::multipolygon_to_geojson_geometry(
            &obj.geometries.vertices,
            &obj.geometries.multipolygon,
        );
        let mpoly_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoly_geojson_geom),
            id: None,
            properties: None, // TODO: from `obj.root`
            foreign_members: None,
        };
        geojson_features.push(mpoly_geojson_feat);
    }

    if !obj.geometries.multilinestring.is_empty() {
        let mls_geojson_geom: geojson::Geometry = conversion::multilinestring_to_geojson_geometry(
            &obj.geometries.vertices,
            &obj.geometries.multilinestring,
        );
        let mls_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mls_geojson_geom),
            id: None,
            properties: None, // TODO: from `obj.root``
            foreign_members: None,
        };
        geojson_features.push(mls_geojson_feat);
    }

    if !obj.geometries.multipoint.is_empty() {
        let mpoint_geojson_geom = conversion::multipoint_to_geojson_geometry(
            &obj.geometries.vertices,
            &obj.geometries.multipoint,
        );
        let mpoint_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoint_geojson_geom),
            id: None,
            properties: None, // TODO: from `obj.root``
            foreign_members: None,
        };
        geojson_features.push(mpoint_geojson_feat);
    }

    geojson_features
}

#[cfg(test)]
mod tests {
    use super::*;
    use citygml::ObjectValue;
    use nusamai_geometry::MultiPolygon;

    #[test]
    fn test_toplevel_cityobj_multipolygon() {
        let vertices: Vec<[f64; 3]> = vec![
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
        ];
        let mut mpoly = MultiPolygon::<'_, 1, u32>::new();
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        let geometries = citygml::Geometries {
            vertices,
            multipolygon: mpoly,
            multilinestring: Default::default(),
            multipoint: Default::default(),
        };

        let obj = TopLevelCityObject {
            root: ObjectValue::String("test".to_string()),
            geometries,
        };

        let geojson_features = toplevel_cityobj_to_geojson_features(&obj);
        assert_eq!(geojson_features.len(), 1);

        let mpoly_geojson = geojson_features.get(0).unwrap();
        assert!(mpoly_geojson.bbox.is_none());
        assert!(mpoly_geojson.foreign_members.is_none());
        if let geojson::Value::MultiPolygon(rings_list) =
            mpoly_geojson.geometry.clone().unwrap().value
        {
            for (i, rings) in rings_list.iter().enumerate() {
                match i {
                    0 => {
                        assert_eq!(rings.len(), 1);
                        assert_eq!(
                            rings[0],
                            vec![
                                [0., 0., 111.],
                                [5., 0., 111.],
                                [5., 5., 111.],
                                [0., 5., 111.],
                                [0., 0., 111.]
                            ]
                        );
                    }
                    _ => unreachable!("Unexpected number of polygons"),
                }
            }
        } else {
            unreachable!("The result is not a GeoJSON MultiPolygon");
        };
    }
}
