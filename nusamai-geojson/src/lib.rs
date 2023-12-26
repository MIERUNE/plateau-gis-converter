mod conversion;
use citygml::{FeatureOrData, ObjectValue};
use conversion::{
    multilinestring_to_geojson_geometry, multipoint_to_geojson_geometry,
    multipolygon_to_geojson_geometry,
};
use nusamai_plateau::TopLevelCityObject;
use serde_json::Value;
use std::collections::HashMap;

fn object_value_to_value(object: &ObjectValue) -> Value {
    match object {
        ObjectValue::String(s) => Value::String(s.clone()),
        ObjectValue::Code(c) => Value::String(c.value.to_string()),
        ObjectValue::Integer(i) => Value::Number(serde_json::Number::from(*i)),
        ObjectValue::Double(d) => Value::Number(serde_json::Number::from_f64(*d).unwrap()),
        ObjectValue::Measure(m) => Value::Number(serde_json::Number::from_f64(*m).unwrap()),
        ObjectValue::Boolean(b) => Value::Bool(*b),
        ObjectValue::URI(u) => Value::String(serde_json::to_string(u).unwrap()),
        ObjectValue::Date(d) => Value::String(d.to_string()),
        // TODO: Handle Point
        // ObjectValue::Point(p) => Value::Array(vec![
        //     Value::Number(serde_json::Number::from_f64(p.x).unwrap()),
        //     Value::Number(serde_json::Number::from_f64(p.y).unwrap()),
        //     Value::Number(serde_json::Number::from_f64(p.z).unwrap()),
        // ]),
        ObjectValue::Array(a) => Value::Array(a.iter().map(object_value_to_value).collect()),
        ObjectValue::FeatureOrData(fod) => {
            let attributes = attribute_to_json(fod);
            let attributes_map: serde_json::Map<String, Value> = attributes.into_iter().collect();
            Value::Object(attributes_map)
        }
        _ => Value::Null,
    }
}

fn attribute_to_json(fod: &FeatureOrData) -> HashMap<String, Value> {
    let mut map = HashMap::new();

    if let Some(id) = &fod.id {
        map.insert("id".to_string(), Value::String(id.clone()));
    }

    for (k, v) in &fod.attributes {
        map.insert(k.clone(), object_value_to_value(v));
    }

    map
}

fn extract_attributes(obj: &TopLevelCityObject) -> HashMap<String, Value> {
    let mut attributes = HashMap::new();

    if let citygml::ObjectValue::FeatureOrData(fod) = &obj.root {
        let a = attribute_to_json(fod);
        attributes.extend(a);
    }

    attributes
}

/// Create GeoJSON features from a TopLevelCityObject
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
// TODO: Handle properties (`obj.root` -> `geojson::Feature.properties`)
// TODO: We may want to traverse the tree and create features for each semantic child in the future
pub fn toplevel_cityobj_to_geojson_features(obj: &TopLevelCityObject) -> Vec<geojson::Feature> {
    let mut geojson_features: Vec<geojson::Feature> = vec![];
    let attributes = extract_attributes(obj);

    if !obj.geometries.multipolygon.is_empty() {
        let mpoly_geojson_geom = multipolygon_to_geojson_geometry(
            &obj.geometries.vertices,
            &obj.geometries.multipolygon,
        );

        let mpoly_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoly_geojson_geom),
            id: None,
            properties: Some(attributes.clone().into_iter().collect()),
            foreign_members: None,
        };
        geojson_features.push(mpoly_geojson_feat);
    }

    if !obj.geometries.multilinestring.is_empty() {
        let mls_geojson_geom = multilinestring_to_geojson_geometry(
            &obj.geometries.vertices,
            &obj.geometries.multilinestring,
        );
        let mls_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mls_geojson_geom),
            id: None,
            properties: Some(attributes.clone().into_iter().collect()),
            foreign_members: None,
        };
        geojson_features.push(mls_geojson_feat);
    }

    if !obj.geometries.multipoint.is_empty() {
        let mpoint_geojson_geom =
            multipoint_to_geojson_geometry(&obj.geometries.vertices, &obj.geometries.multipoint);
        let mpoint_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoint_geojson_geom),
            id: None,
            properties: Some(attributes.clone().into_iter().collect()),
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

        let mpoly_geojson = geojson_features.first().unwrap();
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
