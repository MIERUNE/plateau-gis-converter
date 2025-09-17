use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    sync::RwLock,
};

use super::file_reader::FileReader;

use nusamai_citygml::{
    geometry::{GeometryRef, GeometryRefs, GeometryStore, GeometryType},
    object::{Map, Object, ObjectStereotype, Value},
};
use nusamai_plateau::{appearance::AppearanceStore, Entity};
use nusamai_projection::crs::EPSG_WGS84_GEOGRAPHIC_3D;
use rayon::prelude::*;
use url::Url;

use crate::{
    parameters::Parameters,
    pipeline::{self, Feedback, Parcel, PipelineError, Sender},
    source::{DataSource, DataSourceProvider, SourceInfo},
};

pub struct GeoJsonSourceProvider {
    pub filenames: Vec<PathBuf>,
}

impl DataSourceProvider for GeoJsonSourceProvider {
    fn create(&self, _params: &Parameters) -> Box<dyn DataSource> {
        Box::new(GeoJsonSource {
            filenames: self.filenames.clone(),
        })
    }

    fn info(&self) -> SourceInfo {
        SourceInfo {
            name: "GeoJSON".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        Parameters::default()
    }
}

pub struct GeoJsonSource {
    filenames: Vec<PathBuf>,
}

impl DataSource for GeoJsonSource {
    fn set_appearance_parsing(&mut self, _value: bool) {
        // GeoJSON doesn't support appearances
    }

    fn run(&mut self, downstream: Sender, feedback: &Feedback) -> pipeline::Result<()> {
        self.filenames.par_iter().try_for_each(|filename| {
            feedback.ensure_not_canceled()?;

            feedback.info(format!("Parsing GeoJSON file: {filename:?} ..."));

            // Convert PathBuf to string for ZIP file handling
            let filename_str = filename.to_string_lossy();
            let mut file_reader = FileReader::open(&filename_str)?;

            // Read the entire content
            let mut content = String::new();
            file_reader.read_to_string(&mut content)?;

            // Create source URL - use the original path for regular files, zip path for ZIP files
            let source_url = if filename_str.contains(".zip/") {
                let parts: Vec<&str> = filename_str.splitn(2, ".zip/").collect();
                let zip_part_path = std::fs::canonicalize(Path::new(parts[0])).unwrap();
                let zip_part_string = zip_part_path.to_string_lossy();
                let filename_str = format!("{}.zip/{}", zip_part_string, parts[1]);

                // For ZIP files, use a file URL with the full ZIP path
                Url::parse(&format!("file://{filename_str}")).unwrap()
            } else {
                Url::from_file_path(fs::canonicalize(Path::new(filename))?).unwrap()
            };

            let geojson: geojson::GeoJson = content.parse()
                .map_err(|e| PipelineError::Other(format!("Failed to parse GeoJSON: {e}")))?;

            match geojson {
                geojson::GeoJson::FeatureCollection(collection) => {
                    for feature in collection.features {
                        if feedback.is_canceled() {
                            return Err(PipelineError::Canceled);
                        }

                        if let Some(entity) = convert_feature_to_entity(feature, &source_url)? {
                            if downstream.send(Parcel { entity }).is_err() {
                                feedback.cancel();
                                return Ok(());
                            }
                        }
                    }
                }
                geojson::GeoJson::Feature(feature) => {
                    if let Some(entity) = convert_feature_to_entity(feature, &source_url)? {
                        if downstream.send(Parcel { entity }).is_err() {
                            feedback.cancel();
                            return Ok(());
                        }
                    }
                }
                geojson::GeoJson::Geometry(_) => {
                    return Err(PipelineError::Other(
                        "Direct geometry is not supported. Please use Feature or FeatureCollection.".to_string()
                    ));
                }
            }

            Ok::<(), PipelineError>(())
        })?;

        Ok(())
    }
}

fn convert_feature_to_entity(
    feature: geojson::Feature,
    base_url: &Url,
) -> pipeline::Result<Option<Entity>> {
    let mut geometry_store = GeometryStore {
        epsg: EPSG_WGS84_GEOGRAPHIC_3D, // GeoJSON uses WGS84 (EPSG:4979 for 3D)
        ..Default::default()
    };
    let mut attributes = Map::default();

    // If the ID is present, it will be extracted. If not, a UUID will be generated.
    let id = feature
        .id
        .map(|id| match id {
            geojson::feature::Id::String(s) => s,
            geojson::feature::Id::Number(n) => n.to_string(),
        })
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Convert properties to attributes
    if let Some(props) = feature.properties {
        for (key, value) in props {
            attributes.insert(key, json_value_to_value(value));
        }
    }

    // Convert geometry
    let geometry_refs = if let Some(geom) = feature.geometry {
        convert_geometry(&geom, &mut geometry_store)?
    } else {
        GeometryRefs::new()
    };

    // ジオメトリに基づいて適切なCityGMLタイプを決定
    let typename = match geometry_refs.first() {
        Some(geom_ref) => match geom_ref.ty {
            GeometryType::Surface => "bldg:Building",
            GeometryType::Curve => "tran:Road",
            GeometryType::Point => "frn:CityFurniture",
            _ => "gen:GenericCityObject",
        },
        None => "gen:GenericCityObject",
    };

    let root = Value::Object(Object {
        typename: typename.into(),
        stereotype: ObjectStereotype::Feature {
            id: id.clone(),
            geometries: geometry_refs,
        },
        attributes,
    });

    Ok(Some(Entity {
        root,
        base_url: base_url.clone(),
        geometry_store: RwLock::new(geometry_store).into(),
        appearance_store: RwLock::new(AppearanceStore::default()).into(),
    }))
}

fn json_value_to_value(json: serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::String("null".to_string()),
        serde_json::Value::Bool(b) => Value::Boolean(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(u) = n.as_u64() {
                Value::NonNegativeInteger(u)
            } else if let Some(f) = n.as_f64() {
                Value::Double(f)
            } else {
                Value::String(n.to_string())
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            Value::Array(arr.into_iter().map(json_value_to_value).collect())
        }
        serde_json::Value::Object(obj) => {
            let mut attributes = Map::default();
            for (k, v) in obj {
                attributes.insert(k, json_value_to_value(v));
            }
            Value::Object(Object {
                typename: "Object".into(),
                stereotype: ObjectStereotype::Data,
                attributes,
            })
        }
    }
}

fn convert_geometry(
    geom: &geojson::Geometry,
    geometry_store: &mut GeometryStore,
) -> pipeline::Result<GeometryRefs> {
    use geojson::Value::*;

    // GeoJSON is primarily 2D (LOD 0), but elevation may be specified optionally.
    // Use LOD 1 for safety to handle potential 3D data.
    let mut refs = GeometryRefs::new();

    match &geom.value {
        Point(coords) => {
            if coords.len() >= 2 {
                let vertex_idx = geometry_store.vertices.len() as u32;
                geometry_store.vertices.push([
                    coords[0],
                    coords[1],
                    coords.get(2).copied().unwrap_or(0.0),
                ]);

                let pos = geometry_store.multipoint.len() as u32;
                geometry_store.multipoint.push(vertex_idx);

                refs.push(GeometryRef {
                    ty: GeometryType::Point,
                    lod: 1,
                    pos,
                    len: 1,
                });
            }
        }
        MultiPoint(points) => {
            let pos = geometry_store.multipoint.len() as u32;
            for coords in points {
                if coords.len() >= 2 {
                    let vertex_idx = geometry_store.vertices.len() as u32;
                    geometry_store.vertices.push([
                        coords[0],
                        coords[1],
                        coords.get(2).copied().unwrap_or(0.0),
                    ]);
                    geometry_store.multipoint.push(vertex_idx);
                }
            }
            let len = geometry_store.multipoint.len() as u32 - pos;
            if len > 0 {
                refs.push(GeometryRef {
                    ty: GeometryType::Point,
                    lod: 1,
                    pos,
                    len,
                });
            }
        }
        LineString(coords) => {
            if coords.len() >= 2 {
                let start_idx = geometry_store.vertices.len() as u32;
                let indices: Vec<u32> = coords
                    .iter()
                    .enumerate()
                    .map(|(i, c)| {
                        geometry_store.vertices.push([
                            c[0],
                            c[1],
                            c.get(2).copied().unwrap_or(0.0),
                        ]);
                        start_idx + i as u32
                    })
                    .collect();

                let pos = geometry_store.multilinestring.len() as u32;
                geometry_store.multilinestring.add_linestring(indices);

                refs.push(GeometryRef {
                    ty: GeometryType::Curve,
                    lod: 1,
                    pos,
                    len: 1,
                });
            }
        }
        MultiLineString(lines) => {
            let pos = geometry_store.multilinestring.len() as u32;
            let mut count = 0;

            for line in lines {
                if line.len() >= 2 {
                    let start_idx = geometry_store.vertices.len() as u32;
                    let indices: Vec<u32> = line
                        .iter()
                        .enumerate()
                        .map(|(i, c)| {
                            geometry_store.vertices.push([
                                c[0],
                                c[1],
                                c.get(2).copied().unwrap_or(0.0),
                            ]);
                            start_idx + i as u32
                        })
                        .collect();

                    geometry_store.multilinestring.add_linestring(indices);
                    count += 1;
                }
            }

            if count > 0 {
                refs.push(GeometryRef {
                    ty: GeometryType::Curve,
                    lod: 1,
                    pos,
                    len: count,
                });
            }
        }
        Polygon(rings) => {
            if let Some(exterior) = rings.first() {
                if exterior.len() >= 3 {
                    let pos = geometry_store.multipolygon.len() as u32;

                    // Add exterior ring
                    let start_idx = geometry_store.vertices.len() as u32;
                    let exterior_indices: Vec<u32> = exterior
                        .iter()
                        .enumerate()
                        .map(|(i, c)| {
                            geometry_store.vertices.push([
                                c[0],
                                c[1],
                                c.get(2).copied().unwrap_or(0.0),
                            ]);
                            start_idx + i as u32
                        })
                        .collect();

                    geometry_store.multipolygon.add_exterior(exterior_indices);
                    geometry_store.ring_ids.push(None);

                    // Add interior rings (holes)
                    for interior in rings.iter().skip(1) {
                        if interior.len() >= 3 {
                            let start_idx = geometry_store.vertices.len() as u32;
                            let interior_indices: Vec<u32> = interior
                                .iter()
                                .enumerate()
                                .map(|(i, c)| {
                                    geometry_store.vertices.push([
                                        c[0],
                                        c[1],
                                        c.get(2).copied().unwrap_or(0.0),
                                    ]);
                                    start_idx + i as u32
                                })
                                .collect();

                            geometry_store.multipolygon.add_interior(interior_indices);
                            geometry_store.ring_ids.push(None);
                        }
                    }

                    refs.push(GeometryRef {
                        ty: GeometryType::Surface,
                        lod: 1,
                        pos,
                        len: 1,
                    });
                }
            }
        }
        MultiPolygon(polygons) => {
            let pos = geometry_store.multipolygon.len() as u32;
            let mut count = 0;

            for rings in polygons {
                if let Some(exterior) = rings.first() {
                    if exterior.len() >= 3 {
                        // Add exterior ring
                        let start_idx = geometry_store.vertices.len() as u32;
                        let exterior_indices: Vec<u32> = exterior
                            .iter()
                            .enumerate()
                            .map(|(i, c)| {
                                geometry_store.vertices.push([
                                    c[0],
                                    c[1],
                                    c.get(2).copied().unwrap_or(0.0),
                                ]);
                                start_idx + i as u32
                            })
                            .collect();

                        geometry_store.multipolygon.add_exterior(exterior_indices);
                        geometry_store.ring_ids.push(None);

                        // Add interior rings (holes)
                        for interior in rings.iter().skip(1) {
                            if interior.len() >= 3 {
                                let start_idx = geometry_store.vertices.len() as u32;
                                let interior_indices: Vec<u32> = interior
                                    .iter()
                                    .enumerate()
                                    .map(|(i, c)| {
                                        geometry_store.vertices.push([
                                            c[0],
                                            c[1],
                                            c.get(2).copied().unwrap_or(0.0),
                                        ]);
                                        start_idx + i as u32
                                    })
                                    .collect();

                                geometry_store.multipolygon.add_interior(interior_indices);
                                geometry_store.ring_ids.push(None);
                            }
                        }

                        count += 1;
                    }
                }
            }

            if count > 0 {
                refs.push(GeometryRef {
                    ty: GeometryType::Surface,
                    lod: 1,
                    pos,
                    len: count,
                });
            }
        }
        GeometryCollection(geometries) => {
            for geom in geometries {
                let sub_refs = convert_geometry(geom, geometry_store)?;
                refs.extend(sub_refs);
            }
        }
    }

    Ok(refs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::feedback;
    use nusamai_citygml::object::{ObjectStereotype, Value};
    use std::sync::mpsc::sync_channel;
    use tempfile::TempDir;

    #[test]
    fn test_geojson_source() {
        let geojson_content = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "id": "test-1",
                    "properties": {
                        "name": "Test Feature",
                        "value": 42
                    },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [139.7, 35.6]
                    }
                }
            ]
        }"#;

        // 一時ファイルを作成
        let temp_dir = tempfile::TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test.geojson");
        std::fs::write(&temp_file, geojson_content).unwrap();

        let (sender, receiver) = sync_channel(100);
        let source_provider = GeoJsonSourceProvider {
            filenames: vec![temp_file],
        };
        let mut source = source_provider.create(&Parameters::default());
        let (_, feedback, _) = feedback::watcher();

        // GeoJSONソースを開始
        std::thread::scope(|scope| {
            scope.spawn(move || {
                source.run(sender, &feedback).unwrap();
            });

            let parcels: Vec<_> = receiver.iter().collect();
            assert_eq!(parcels.len(), 1);

            let entity = &parcels[0].entity;
            if let Value::Object(obj) = &entity.root {
                assert_eq!(obj.typename, "frn:CityFurniture");
                if let ObjectStereotype::Feature { id, .. } = &obj.stereotype {
                    assert_eq!(id, "test-1");
                }
                assert_eq!(
                    obj.attributes.get("name"),
                    Some(&Value::String("Test Feature".to_string()))
                );
                assert_eq!(obj.attributes.get("value"), Some(&Value::Integer(42)));
            } else {
                panic!("Expected Object");
            }
        });
    }

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

        // 一時ファイルを作成
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test.geojson");
        std::fs::write(&temp_file, geojson_content).unwrap();

        let (sender, receiver) = sync_channel(100);
        let source_provider = GeoJsonSourceProvider {
            filenames: vec![temp_file],
        };
        let mut source = source_provider.create(&Parameters::default());
        let (_, feedback, _) = feedback::watcher();

        // GeoJSONソースを開始
        std::thread::scope(|scope| {
            scope.spawn(move || {
                source.run(sender, &feedback).unwrap();
            });

            let parcels: Vec<_> = receiver.iter().collect();
            assert_eq!(parcels.len(), 2);

            // 最初のフィーチャをチェック
            let entity1 = &parcels[0].entity;
            if let Value::Object(obj) = &entity1.root {
                assert_eq!(obj.typename, "bldg:Building");
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

            // 2番目のフィーチャをチェック
            let entity2 = &parcels[1].entity;
            if let Value::Object(obj) = &entity2.root {
                assert_eq!(obj.typename, "frn:CityFurniture");
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

        // 一時ファイルを作成
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test.json");
        std::fs::write(&temp_file, geojson_content).unwrap();

        let (sender, receiver) = sync_channel(100);
        let source_provider = GeoJsonSourceProvider {
            filenames: vec![temp_file],
        };
        let mut source = source_provider.create(&Parameters::default());
        let (_, feedback, _) = feedback::watcher();

        // GeoJSONソースを開始
        std::thread::scope(|scope| {
            scope.spawn(move || {
                source.run(sender, &feedback).unwrap();
            });

            let parcels: Vec<_> = receiver.iter().collect();
            assert_eq!(parcels.len(), 1);

            let entity = &parcels[0].entity;
            if let Value::Object(obj) = &entity.root {
                assert_eq!(obj.typename, "bldg:Building");
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

        // 一時ファイルを作成
        let temp_dir = TempDir::new().unwrap();
        let temp_file = temp_dir.path().join("test_complex.geojson");
        std::fs::write(&temp_file, geojson_content).unwrap();

        let (sender, receiver) = sync_channel(100);
        let source_provider = GeoJsonSourceProvider {
            filenames: vec![temp_file],
        };
        let mut source = source_provider.create(&Parameters::default());
        let (_, feedback, _) = feedback::watcher();

        // GeoJSONソースを開始
        std::thread::scope(|scope| {
            scope.spawn(move || {
                source.run(sender, &feedback).unwrap();
            });

            let parcels: Vec<_> = receiver.iter().collect();
            assert_eq!(parcels.len(), 1);

            let entity = &parcels[0].entity;
            if let Value::Object(obj) = &entity.root {
                assert_eq!(obj.typename, "tran:Road");

                // ネストしたオブジェクトをチェック
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

                // 配列をチェック
                if let Some(Value::Array(tags)) = obj.attributes.get("tags") {
                    assert_eq!(tags.len(), 3);
                    assert_eq!(tags[0], Value::String("building".to_string()));
                    assert_eq!(tags[1], Value::String("commercial".to_string()));
                    assert_eq!(tags[2], Value::String("high-rise".to_string()));
                } else {
                    panic!("Expected tags array");
                }

                // ジオメトリをチェック
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

        // 一時ファイルを作成
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

        // GeoJSONソースを開始
        std::thread::scope(|scope| {
            scope.spawn(move || {
                source.run(sender, &feedback).unwrap();
            });

            let parcels: Vec<_> = receiver.iter().collect();
            assert_eq!(parcels.len(), 2);

            // 両方のファイルからフィーチャが取得できることを確認
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
}
