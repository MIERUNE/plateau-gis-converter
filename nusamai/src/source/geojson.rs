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
use nusamai_projection::crs::{EPSG_WGS84_GEOGRAPHIC_2D, EPSG_WGS84_GEOGRAPHIC_3D};
use rayon::prelude::*;
use url::Url;

use crate::{
    parameters::Parameters,
    pipeline::{self, Feedback, Parcel, PipelineError, Sender},
    source::{DataSource, DataSourceProvider, SourceInfo},
};

const GEOJSON_TYPENAME: &str = "gen:GenericCityObject";

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
                let zip_part_path =
                    std::fs::canonicalize(Path::new(format!("{}.zip", parts[0]).as_str())).unwrap();
                let mut zip_joined = zip_part_path.clone();
                zip_joined.push(parts[1]);

                // For ZIP files, use a file URL with the full ZIP path
                Url::from_file_path(zip_joined).unwrap()
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
        epsg: EPSG_WGS84_GEOGRAPHIC_2D,
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

    let root = Value::Object(Object {
        typename: GEOJSON_TYPENAME.into(),
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

    // GeoJSON has no LOD concept, so imported geometries are treated as LOD 0.
    let mut refs = GeometryRefs::new();

    match &geom.value {
        Point(coords) => {
            let vertex_idx = push_position(geometry_store, coords)?;
            let pos = geometry_store.multipoint.len() as u32;
            geometry_store.multipoint.push(vertex_idx);

            refs.push(GeometryRef {
                ty: GeometryType::Point,
                lod: 0,
                pos,
                len: 1,
            });
        }
        MultiPoint(points) => {
            let indices = push_positions(geometry_store, points)?;
            let pos = geometry_store.multipoint.len() as u32;
            for vertex_idx in indices {
                geometry_store.multipoint.push(vertex_idx);
            }
            let len = geometry_store.multipoint.len() as u32 - pos;
            if len > 0 {
                refs.push(GeometryRef {
                    ty: GeometryType::Point,
                    lod: 0,
                    pos,
                    len,
                });
            }
        }
        LineString(coords) => {
            if let Some(positions) = validate_linestring(coords)? {
                let indices = push_validated_positions(geometry_store, positions);
                let pos = geometry_store.multilinestring.len() as u32;
                geometry_store.multilinestring.add_linestring(indices);

                refs.push(GeometryRef {
                    ty: GeometryType::Curve,
                    lod: 0,
                    pos,
                    len: 1,
                });
            }
        }
        MultiLineString(lines) => {
            let lines = lines
                .iter()
                .map(|line| validate_linestring(line))
                .collect::<pipeline::Result<Vec<_>>>()?;
            let pos = geometry_store.multilinestring.len() as u32;
            let mut count = 0;

            for positions in lines.into_iter().flatten() {
                let indices = push_validated_positions(geometry_store, positions);
                geometry_store.multilinestring.add_linestring(indices);
                count += 1;
            }

            if count > 0 {
                refs.push(GeometryRef {
                    ty: GeometryType::Curve,
                    lod: 0,
                    pos,
                    len: count,
                });
            }
        }
        Polygon(rings) => {
            let pos = geometry_store.multipolygon.len() as u32;
            if add_polygon(geometry_store, rings)? {
                refs.push(GeometryRef {
                    ty: GeometryType::Surface,
                    lod: 0,
                    pos,
                    len: 1,
                });
            }
        }
        MultiPolygon(polygons) => {
            let pos = geometry_store.multipolygon.len() as u32;
            let mut count = 0;

            for rings in polygons {
                if add_polygon(geometry_store, rings)? {
                    count += 1;
                }
            }

            if count > 0 {
                refs.push(GeometryRef {
                    ty: GeometryType::Surface,
                    lod: 0,
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

type ValidatedPosition = ([f64; 3], bool);

fn validate_position(position: &[f64]) -> pipeline::Result<ValidatedPosition> {
    match position {
        [longitude, latitude] => Ok(([*longitude, *latitude, 0.0], false)),
        [longitude, latitude, height] => Ok(([*longitude, *latitude, *height], true)),
        _ => Err(PipelineError::Other(format!(
            "GeoJSON Position must contain exactly 2 or 3 elements, but found {}",
            position.len()
        ))),
    }
}

fn push_validated_position(
    geometry_store: &mut GeometryStore,
    (position, has_explicit_z): ValidatedPosition,
) -> u32 {
    if has_explicit_z {
        geometry_store.epsg = EPSG_WGS84_GEOGRAPHIC_3D;
    }
    let vertex_idx = geometry_store.vertices.len() as u32;
    geometry_store.vertices.push(position);
    vertex_idx
}

fn push_position(geometry_store: &mut GeometryStore, position: &[f64]) -> pipeline::Result<u32> {
    let position = validate_position(position)?;
    Ok(push_validated_position(geometry_store, position))
}

fn push_positions(
    geometry_store: &mut GeometryStore,
    positions: &[Vec<f64>],
) -> pipeline::Result<Vec<u32>> {
    let positions = validate_positions(positions)?;
    Ok(push_validated_positions(geometry_store, positions))
}

fn validate_positions(positions: &[Vec<f64>]) -> pipeline::Result<Vec<ValidatedPosition>> {
    positions
        .iter()
        .map(|position| validate_position(position))
        .collect()
}

fn validate_linestring(positions: &[Vec<f64>]) -> pipeline::Result<Option<Vec<ValidatedPosition>>> {
    if positions.is_empty() {
        return Ok(None);
    }

    let positions = validate_positions(positions)?;
    if positions.len() < 2 {
        return Err(PipelineError::Other(format!(
            "GeoJSON LineString must contain at least 2 positions, but found {}",
            positions.len()
        )));
    }

    Ok(Some(positions))
}

fn push_validated_positions(
    geometry_store: &mut GeometryStore,
    positions: Vec<ValidatedPosition>,
) -> Vec<u32> {
    positions
        .into_iter()
        .map(|position| push_validated_position(geometry_store, position))
        .collect()
}

fn push_linear_ring(
    geometry_store: &mut GeometryStore,
    ring: &[Vec<f64>],
) -> pipeline::Result<Vec<u32>> {
    if ring.len() < 4 {
        return Err(PipelineError::Other(format!(
            "GeoJSON LinearRing must contain at least 4 positions, but found {}",
            ring.len()
        )));
    }

    let positions = validate_positions(ring)?;

    if positions.first().map(|position| &position.0) != positions.last().map(|position| &position.0)
    {
        return Err(PipelineError::Other(
            "GeoJSON LinearRing must have identical first and last positions".into(),
        ));
    }

    if positions.iter().any(|position| position.1) {
        geometry_store.epsg = EPSG_WGS84_GEOGRAPHIC_3D;
    }

    Ok(positions
        .into_iter()
        .take(ring.len() - 1)
        .map(|position| push_validated_position(geometry_store, position))
        .collect())
}

fn add_polygon(
    geometry_store: &mut GeometryStore,
    rings: &[Vec<Vec<f64>>],
) -> pipeline::Result<bool> {
    let Some(exterior) = rings.first() else {
        return Ok(false);
    };

    let exterior_indices = push_linear_ring(geometry_store, exterior)?;
    geometry_store.multipolygon.add_exterior(exterior_indices);
    geometry_store.ring_ids.push(None);

    for interior in rings.iter().skip(1) {
        let interior_indices = push_linear_ring(geometry_store, interior)?;
        geometry_store.multipolygon.add_interior(interior_indices);
        geometry_store.ring_ids.push(None);
    }

    Ok(true)
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
                assert_eq!(obj.typename, GEOJSON_TYPENAME);
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
                assert_eq!(obj.typename, GEOJSON_TYPENAME);
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
                assert_eq!(obj.typename, GEOJSON_TYPENAME);
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
                assert_eq!(obj.typename, GEOJSON_TYPENAME);
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
                assert_eq!(obj.typename, GEOJSON_TYPENAME);

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
