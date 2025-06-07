use nusamai::{
    parameters::Parameters,
    pipeline::{feedback, Parcel},
    source::{geojson::GeoJsonSourceProvider, DataSourceProvider},
};
use nusamai_citygml::object::{Object, ObjectStereotype, Value};
use std::sync::mpsc::sync_channel;
use tempfile::TempDir;

#[test]
fn test_geojson_source_feature_collection() {
    let geojson_content = r#"{
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "id": "building-1",
                "properties": {
                    "name": "Test Building 1",
                    "height": 30.5,
                    "floors": 10
                },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [
                        [[139.7, 35.6], [139.71, 35.6], [139.71, 35.61], [139.7, 35.61], [139.7, 35.6]]
                    ]
                }
            },
            {
                "type": "Feature",
                "id": "building-2",
                "properties": {
                    "name": "Test Building 2",
                    "height": 45.0,
                    "floors": 15
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [139.705, 35.605]
                }
            }
        ]
    }"#;

    // Create a temporary file
    let temp_dir = TempDir::new().unwrap();
    let temp_file = temp_dir.path().join("test.geojson");
    std::fs::write(&temp_file, geojson_content).unwrap();

    let (sender, receiver) = sync_channel(100);
    let source_provider = GeoJsonSourceProvider {
        filenames: vec![temp_file],
    };
    let mut source = source_provider.create(&Parameters::default());
    let (_, feedback, _) = feedback::watcher();

    // Start the GeoJSON source
    std::thread::scope(|scope| {
        scope.spawn(move || {
            source.run(sender, &feedback).unwrap();
        });

        let parcels: Vec<Parcel> = receiver.iter().collect();
        assert_eq!(parcels.len(), 2);

        // Check first feature
        let entity1 = &parcels[0].entity;
        if let Value::Object(obj) = &entity1.root {
            assert_eq!(obj.typename, "Feature");
            if let ObjectStereotype::Feature { id, geometries } = &obj.stereotype {
                assert_eq!(id, "building-1");
                assert!(!geometries.is_empty());
                assert_eq!(
                    geometries[0].ty,
                    nusamai_citygml::geometry::GeometryType::Surface
                );
            }
            assert_eq!(
                obj.attributes.get("name"),
                Some(&Value::String("Test Building 1".to_string()))
            );
            assert_eq!(obj.attributes.get("height"), Some(&Value::Double(30.5)));
            assert_eq!(obj.attributes.get("floors"), Some(&Value::Integer(10)));
        } else {
            panic!("Expected Object");
        }

        // Check second feature
        let entity2 = &parcels[1].entity;
        if let Value::Object(obj) = &entity2.root {
            assert_eq!(obj.typename, "Feature");
            if let ObjectStereotype::Feature { id, geometries } = &obj.stereotype {
                assert_eq!(id, "building-2");
                assert!(!geometries.is_empty());
                assert_eq!(
                    geometries[0].ty,
                    nusamai_citygml::geometry::GeometryType::Point
                );
            }
            assert_eq!(
                obj.attributes.get("name"),
                Some(&Value::String("Test Building 2".to_string()))
            );
            assert_eq!(obj.attributes.get("height"), Some(&Value::Double(45.0)));
            assert_eq!(obj.attributes.get("floors"), Some(&Value::Integer(15)));
        } else {
            panic!("Expected Object");
        }
    });
}

#[test]
fn test_geojson_source_single_feature() {
    let geojson_content = r#"{
        "type": "Feature",
        "properties": {
            "category": "park",
            "area": 1234.56
        },
        "geometry": {
            "type": "MultiPolygon",
            "coordinates": [
                [
                    [[139.7, 35.6], [139.71, 35.6], [139.71, 35.61], [139.7, 35.61], [139.7, 35.6]]
                ]
            ]
        }
    }"#;

    // Create a temporary file
    let temp_dir = TempDir::new().unwrap();
    let temp_file = temp_dir.path().join("test.json");
    std::fs::write(&temp_file, geojson_content).unwrap();

    let (sender, receiver) = sync_channel(100);
    let source_provider = GeoJsonSourceProvider {
        filenames: vec![temp_file],
    };
    let mut source = source_provider.create(&Parameters::default());
    let (_, feedback, _) = feedback::watcher();

    // Start the GeoJSON source
    std::thread::scope(|scope| {
        scope.spawn(move || {
            source.run(sender, &feedback).unwrap();
        });

        let parcels: Vec<Parcel> = receiver.iter().collect();
        assert_eq!(parcels.len(), 1);

        let entity = &parcels[0].entity;
        if let Value::Object(obj) = &entity.root {
            assert_eq!(obj.typename, "Feature");
            if let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype {
                assert!(!geometries.is_empty());
                assert_eq!(
                    geometries[0].ty,
                    nusamai_citygml::geometry::GeometryType::Surface
                );
            }
            assert_eq!(
                obj.attributes.get("category"),
                Some(&Value::String("park".to_string()))
            );
            assert_eq!(obj.attributes.get("area"), Some(&Value::Double(1234.56)));
        } else {
            panic!("Expected Object");
        }
    });
}

#[test]
fn test_geojson_source_with_nested_properties() {
    let geojson_content = r#"{
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "properties": {
                    "name": "Complex Feature",
                    "metadata": {
                        "source": "survey",
                        "date": "2024-01-01",
                        "accuracy": 0.95
                    },
                    "tags": ["building", "commercial", "high-rise"]
                },
                "geometry": {
                    "type": "LineString",
                    "coordinates": [[139.7, 35.6], [139.71, 35.61], [139.72, 35.62]]
                }
            }
        ]
    }"#;

    // Create a temporary file
    let temp_dir = TempDir::new().unwrap();
    let temp_file = temp_dir.path().join("test_complex.geojson");
    std::fs::write(&temp_file, geojson_content).unwrap();

    let (sender, receiver) = sync_channel(100);
    let source_provider = GeoJsonSourceProvider {
        filenames: vec![temp_file],
    };
    let mut source = source_provider.create(&Parameters::default());
    let (_, feedback, _) = feedback::watcher();

    // Start the GeoJSON source
    std::thread::scope(|scope| {
        scope.spawn(move || {
            source.run(sender, &feedback).unwrap();
        });

        let parcels: Vec<Parcel> = receiver.iter().collect();
        assert_eq!(parcels.len(), 1);

        let entity = &parcels[0].entity;
        if let Value::Object(obj) = &entity.root {
            assert_eq!(obj.typename, "Feature");

            // Check nested object
            if let Some(Value::Object(metadata_obj)) = obj.attributes.get("metadata") {
                assert_eq!(metadata_obj.typename, "Object");
                assert_eq!(
                    metadata_obj.attributes.get("source"),
                    Some(&Value::String("survey".to_string()))
                );
                assert_eq!(
                    metadata_obj.attributes.get("accuracy"),
                    Some(&Value::Double(0.95))
                );
            } else {
                panic!("Expected metadata object");
            }

            // Check array
            if let Some(Value::Array(tags)) = obj.attributes.get("tags") {
                assert_eq!(tags.len(), 3);
                assert_eq!(tags[0], Value::String("building".to_string()));
                assert_eq!(tags[1], Value::String("commercial".to_string()));
                assert_eq!(tags[2], Value::String("high-rise".to_string()));
            } else {
                panic!("Expected tags array");
            }

            // Check geometry
            if let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype {
                assert!(!geometries.is_empty());
                assert_eq!(
                    geometries[0].ty,
                    nusamai_citygml::geometry::GeometryType::Curve
                );
            }
        } else {
            panic!("Expected Object");
        }
    });
}

#[test]
fn test_geojson_source_multiple_files() {
    let geojson1 = r#"{
        "type": "Feature",
        "id": "file1-feature",
        "properties": {"source": "file1"},
        "geometry": {"type": "Point", "coordinates": [139.7, 35.6]}
    }"#;

    let geojson2 = r#"{
        "type": "Feature",
        "id": "file2-feature",
        "properties": {"source": "file2"},
        "geometry": {"type": "Point", "coordinates": [139.8, 35.7]}
    }"#;

    // Create temporary files
    let temp_dir = TempDir::new().unwrap();
    let temp_file1 = temp_dir.path().join("test1.geojson");
    let temp_file2 = temp_dir.path().join("test2.json");
    std::fs::write(&temp_file1, geojson1).unwrap();
    std::fs::write(&temp_file2, geojson2).unwrap();

    let (sender, receiver) = sync_channel(100);
    let source_provider = GeoJsonSourceProvider {
        filenames: vec![temp_file1, temp_file2],
    };
    let mut source = source_provider.create(&Parameters::default());
    let (_, feedback, _) = feedback::watcher();

    // Start the GeoJSON source
    std::thread::scope(|scope| {
        scope.spawn(move || {
            source.run(sender, &feedback).unwrap();
        });

        let parcels: Vec<Parcel> = receiver.iter().collect();
        assert_eq!(parcels.len(), 2);

        // Verify we got features from both files
        let sources: Vec<String> = parcels
            .iter()
            .filter_map(|p| {
                if let Value::Object(obj) = &p.entity.root {
                    if let Some(Value::String(source)) = obj.attributes.get("source") {
                        Some(source.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        assert!(sources.contains(&"file1".to_string()));
        assert!(sources.contains(&"file2".to_string()));
    });
}
