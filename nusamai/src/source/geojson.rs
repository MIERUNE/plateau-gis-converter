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
    schema::{Schema, TypeDef},
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

#[path = "geojson_schema.rs"]
mod schema;

const GEOJSON_TYPENAME: &str = "geojson:Feature";
const DIRECT_GEOMETRY_ERROR: &str =
    "Direct geometry is not supported. Please use Feature or FeatureCollection.";

pub struct GeoJsonSourceProvider {
    pub filenames: Vec<PathBuf>,
}

impl DataSourceProvider for GeoJsonSourceProvider {
    fn collect_schema(&self, _params: &Parameters, output: &mut Schema) -> pipeline::Result<()> {
        let mut builder = schema::GeoJsonSchemaBuilder::default();

        for filename in &self.filenames {
            let geojson = read_geojson(filename)?;
            observe_geojson_schema(&mut builder, &geojson)?;
        }

        output.types.insert(
            GEOJSON_TYPENAME.to_owned(),
            TypeDef::Feature(builder.finish()),
        );
        Ok(())
    }

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

            let geojson = read_geojson(filename)?;

            // Convert PathBuf to string for ZIP file handling
            let filename_str = filename.to_string_lossy();

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
                    return Err(PipelineError::Other(DIRECT_GEOMETRY_ERROR.to_owned()));
                }
            }

            Ok::<(), PipelineError>(())
        })?;

        Ok(())
    }
}

fn read_geojson(filename: &Path) -> pipeline::Result<geojson::GeoJson> {
    let filename = filename.to_string_lossy();
    let mut file_reader = FileReader::open(&filename)?;
    let mut content = String::new();
    file_reader.read_to_string(&mut content)?;
    content
        .parse()
        .map_err(|error| PipelineError::Other(format!("Failed to parse GeoJSON: {error}")))
}

fn observe_geojson_schema(
    builder: &mut schema::GeoJsonSchemaBuilder,
    geojson: &geojson::GeoJson,
) -> pipeline::Result<()> {
    match geojson {
        geojson::GeoJson::FeatureCollection(collection) => {
            for feature in &collection.features {
                builder.observe_properties(feature.properties.as_ref());
            }
        }
        geojson::GeoJson::Feature(feature) => {
            builder.observe_properties(feature.properties.as_ref());
        }
        geojson::GeoJson::Geometry(_) => {
            return Err(PipelineError::Other(DIRECT_GEOMETRY_ERROR.to_owned()));
        }
    }

    Ok(())
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
            if value.is_null() {
                continue;
            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PolygonRingKind {
    Exterior,
    Interior,
}

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

fn signed_ring_area_2d(positions: &[ValidatedPosition]) -> f64 {
    let Some((origin, _)) = positions.first() else {
        return 0.0;
    };
    let [origin_x, origin_y, _] = *origin;

    let mut area = 0.0;
    for index in 0..positions.len() {
        let [current_x, current_y, _] = positions[index].0;
        let [next_x, next_y, _] = positions[(index + 1) % positions.len()].0;
        area += (current_x - origin_x) * (next_y - origin_y)
            - (next_x - origin_x) * (current_y - origin_y);
    }
    area
}

fn is_clockwise(positions: &[ValidatedPosition]) -> bool {
    signed_ring_area_2d(positions) < 0.0
}

fn is_counter_clockwise(positions: &[ValidatedPosition]) -> bool {
    signed_ring_area_2d(positions) > 0.0
}

fn normalize_ring_winding(positions: &mut [ValidatedPosition], ring_kind: PolygonRingKind) {
    let needs_reversal = match ring_kind {
        PolygonRingKind::Exterior => is_clockwise(positions),
        PolygonRingKind::Interior => is_counter_clockwise(positions),
    };

    if needs_reversal {
        positions[1..].reverse();
    }
}

fn push_linear_ring(
    geometry_store: &mut GeometryStore,
    ring: &[Vec<f64>],
    ring_kind: PolygonRingKind,
) -> pipeline::Result<Vec<u32>> {
    if ring.len() < 4 {
        return Err(PipelineError::Other(format!(
            "GeoJSON LinearRing must contain at least 4 positions, but found {}",
            ring.len()
        )));
    }

    let mut positions = validate_positions(ring)?;

    if positions.first().map(|position| &position.0) != positions.last().map(|position| &position.0)
    {
        return Err(PipelineError::Other(
            "GeoJSON LinearRing must have identical first and last positions".into(),
        ));
    }

    if positions.iter().any(|position| position.1) {
        geometry_store.epsg = EPSG_WGS84_GEOGRAPHIC_3D;
    }

    positions.pop();
    normalize_ring_winding(&mut positions, ring_kind);

    Ok(push_validated_positions(geometry_store, positions))
}

fn add_polygon(
    geometry_store: &mut GeometryStore,
    rings: &[Vec<Vec<f64>>],
) -> pipeline::Result<bool> {
    let Some(exterior) = rings.first() else {
        return Ok(false);
    };

    let exterior_indices = push_linear_ring(geometry_store, exterior, PolygonRingKind::Exterior)?;
    geometry_store.multipolygon.add_exterior(exterior_indices);
    geometry_store.ring_ids.push(None);

    for interior in rings.iter().skip(1) {
        let interior_indices =
            push_linear_ring(geometry_store, interior, PolygonRingKind::Interior)?;
        geometry_store.multipolygon.add_interior(interior_indices);
        geometry_store.ring_ids.push(None);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        pipeline::{feedback, PipelineError},
        source::DataSourceProvider,
    };
    use nusamai_citygml::object::{ObjectStereotype, Value};
    use std::sync::mpsc::sync_channel;
    use tempfile::TempDir;

    fn polygon_geometry_store(rings: Vec<Vec<Vec<f64>>>) -> GeometryStore {
        let mut geometry_store = GeometryStore {
            epsg: EPSG_WGS84_GEOGRAPHIC_2D,
            ..Default::default()
        };

        assert!(add_polygon(&mut geometry_store, &rings).unwrap());
        geometry_store
    }

    fn polygon_ring_vertices(geometry_store: &GeometryStore, ring_index: usize) -> Vec<[f64; 3]> {
        multipolygon_ring_vertices(geometry_store, 0, ring_index)
    }

    fn multipolygon_ring_vertices(
        geometry_store: &GeometryStore,
        polygon_index: usize,
        ring_index: usize,
    ) -> Vec<[f64; 3]> {
        geometry_store
            .multipolygon
            .get(polygon_index)
            .rings()
            .nth(ring_index)
            .unwrap()
            .iter()
            .map(|vertex_index| geometry_store.vertices[vertex_index as usize])
            .collect()
    }

    fn assert_counter_clockwise(vertices: Vec<[f64; 3]>) {
        let vertices = vertices
            .into_iter()
            .map(|[x, y, _]| [x, y])
            .collect::<Vec<_>>();
        let ring = flatgeom::LineString2::from_raw(vertices.into());
        assert!(ring.is_ccw(), "exterior ring must be counter-clockwise");
    }

    fn run_geojson_source(content: &str) -> Vec<Parcel> {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("input.geojson");
        std::fs::write(&path, content).unwrap();

        let provider = GeoJsonSourceProvider {
            filenames: vec![path],
        };
        let mut source = provider.create(&Parameters::default());
        let (sender, receiver) = sync_channel(100);
        let (_, feedback, _) = feedback::watcher();

        source.run(sender, &feedback).unwrap();
        receiver.into_iter().collect()
    }

    fn feature_id(parcel: &Parcel) -> &str {
        let Value::Object(object) = &parcel.entity.root else {
            panic!("GeoJSON feature must become an object");
        };
        assert_eq!(object.typename, GEOJSON_TYPENAME);
        let ObjectStereotype::Feature { id, .. } = &object.stereotype else {
            panic!("GeoJSON object must have the Feature stereotype");
        };
        id
    }

    #[test]
    fn polygon_normalizes_clockwise_exterior_to_counter_clockwise() {
        let geometry_store = polygon_geometry_store(vec![vec![
            vec![0.0, 0.0],
            vec![0.0, 2.0],
            vec![2.0, 2.0],
            vec![2.0, 0.0],
            vec![0.0, 0.0],
        ]]);

        assert_eq!(
            polygon_ring_vertices(&geometry_store, 0),
            vec![
                [0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0],
                [2.0, 2.0, 0.0],
                [0.0, 2.0, 0.0],
            ]
        );
    }

    #[test]
    fn polygon_normalizes_counter_clockwise_interior_to_clockwise() {
        let geometry_store = polygon_geometry_store(vec![
            vec![
                vec![0.0, 0.0],
                vec![4.0, 0.0],
                vec![4.0, 4.0],
                vec![0.0, 4.0],
                vec![0.0, 0.0],
            ],
            vec![
                vec![1.0, 1.0],
                vec![3.0, 1.0],
                vec![3.0, 3.0],
                vec![1.0, 3.0],
                vec![1.0, 1.0],
            ],
        ]);

        assert_eq!(
            polygon_ring_vertices(&geometry_store, 1),
            vec![
                [1.0, 1.0, 0.0],
                [1.0, 3.0, 0.0],
                [3.0, 3.0, 0.0],
                [3.0, 1.0, 0.0],
            ]
        );
    }

    #[test]
    fn polygon_preserves_canonical_ring_order() {
        let exterior = vec![
            vec![0.0, 0.0],
            vec![4.0, 0.0],
            vec![4.0, 4.0],
            vec![0.0, 4.0],
            vec![0.0, 0.0],
        ];
        let interior = vec![
            vec![1.0, 1.0],
            vec![1.0, 3.0],
            vec![3.0, 3.0],
            vec![3.0, 1.0],
            vec![1.0, 1.0],
        ];

        let geometry_store = polygon_geometry_store(vec![exterior, interior]);

        assert_eq!(
            polygon_ring_vertices(&geometry_store, 0),
            vec![
                [0.0, 0.0, 0.0],
                [4.0, 0.0, 0.0],
                [4.0, 4.0, 0.0],
                [0.0, 4.0, 0.0],
            ]
        );
        assert_eq!(
            polygon_ring_vertices(&geometry_store, 1),
            vec![
                [1.0, 1.0, 0.0],
                [1.0, 3.0, 0.0],
                [3.0, 3.0, 0.0],
                [3.0, 1.0, 0.0],
            ]
        );
        assert_eq!(geometry_store.epsg, EPSG_WGS84_GEOGRAPHIC_2D);
    }

    #[test]
    fn polygon_normalization_preserves_first_position_and_xyz_values() {
        let geometry_store = polygon_geometry_store(vec![vec![
            vec![0.0, 0.0, 10.0],
            vec![0.0, 2.0, 40.0],
            vec![2.0, 2.0, 30.0],
            vec![2.0, 0.0, 20.0],
            vec![0.0, 0.0, 10.0],
        ]]);

        assert_eq!(
            polygon_ring_vertices(&geometry_store, 0),
            vec![
                [0.0, 0.0, 10.0],
                [2.0, 0.0, 20.0],
                [2.0, 2.0, 30.0],
                [0.0, 2.0, 40.0],
            ]
        );
        assert_eq!(geometry_store.epsg, EPSG_WGS84_GEOGRAPHIC_3D);
    }

    #[test]
    fn polygon_preserves_linear_ring_validation_errors() {
        let mut geometry_store = GeometryStore::default();
        let too_short = vec![vec![vec![0.0, 0.0], vec![1.0, 0.0], vec![0.0, 0.0]]];
        let error = add_polygon(&mut geometry_store, &too_short).unwrap_err();
        assert!(matches!(
            error,
            PipelineError::Other(message)
                if message == "GeoJSON LinearRing must contain at least 4 positions, but found 3"
        ));

        let mut geometry_store = GeometryStore::default();
        let not_closed = vec![vec![
            vec![0.0, 0.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
            vec![0.0, 1.0],
        ]];
        let error = add_polygon(&mut geometry_store, &not_closed).unwrap_err();
        assert!(matches!(
            error,
            PipelineError::Other(message)
                if message == "GeoJSON LinearRing must have identical first and last positions"
        ));
    }

    #[test]
    fn convert_geometry_normalizes_all_multipolygon_exteriors() {
        let geometry = geojson::Geometry::new(geojson::Value::MultiPolygon(vec![
            vec![vec![
                vec![0.0, 0.0],
                vec![0.0, 2.0],
                vec![2.0, 2.0],
                vec![2.0, 0.0],
                vec![0.0, 0.0],
            ]],
            vec![vec![
                vec![10.0, 10.0],
                vec![10.0, 12.0],
                vec![12.0, 12.0],
                vec![12.0, 10.0],
                vec![10.0, 10.0],
            ]],
        ]));
        let mut geometry_store = GeometryStore {
            epsg: EPSG_WGS84_GEOGRAPHIC_2D,
            ..Default::default()
        };

        let geometry_refs = convert_geometry(&geometry, &mut geometry_store).unwrap();

        assert_eq!(
            geometry_refs,
            vec![GeometryRef {
                ty: GeometryType::Surface,
                lod: 0,
                pos: 0,
                len: 2,
            }]
        );
        assert_eq!(geometry_store.multipolygon.len(), 2);
        assert_counter_clockwise(multipolygon_ring_vertices(&geometry_store, 0, 0));
        assert_counter_clockwise(multipolygon_ring_vertices(&geometry_store, 1, 0));
    }

    #[test]
    fn convert_geometry_maps_point_to_point() {
        let geometry = geojson::Geometry::new(geojson::Value::Point(vec![139.7, 35.6]));
        let mut geometry_store = GeometryStore {
            epsg: EPSG_WGS84_GEOGRAPHIC_2D,
            ..Default::default()
        };

        let geometry_refs = convert_geometry(&geometry, &mut geometry_store).unwrap();

        assert_eq!(
            geometry_refs,
            vec![GeometryRef {
                ty: GeometryType::Point,
                lod: 0,
                pos: 0,
                len: 1,
            }]
        );
        assert_eq!(geometry_store.vertices, vec![[139.7, 35.6, 0.0]]);
    }

    #[test]
    fn convert_geometry_maps_polygon_to_surface() {
        let geometry = geojson::Geometry::new(geojson::Value::Polygon(vec![vec![
            vec![139.7, 35.6],
            vec![139.71, 35.6],
            vec![139.71, 35.61],
            vec![139.7, 35.61],
            vec![139.7, 35.6],
        ]]));
        let mut geometry_store = GeometryStore {
            epsg: EPSG_WGS84_GEOGRAPHIC_2D,
            ..Default::default()
        };

        let geometry_refs = convert_geometry(&geometry, &mut geometry_store).unwrap();

        assert_eq!(
            geometry_refs,
            vec![GeometryRef {
                ty: GeometryType::Surface,
                lod: 0,
                pos: 0,
                len: 1,
            }]
        );
        assert_eq!(geometry_store.multipolygon.len(), 1);
    }

    #[test]
    fn convert_feature_omits_null_and_preserves_scalar_properties() {
        let feature: geojson::Feature = r#"{
            "type": "Feature",
            "properties": {
                "null_value": null,
                "string_value": "01000",
                "integer_value": 42,
                "double_value": 12.5
            },
            "geometry": null
        }"#
        .parse()
        .unwrap();
        let base_url = url::Url::parse("file:///input.geojson").unwrap();

        let entity = convert_feature_to_entity(feature, &base_url)
            .unwrap()
            .unwrap();
        let Value::Object(object) = entity.root else {
            panic!("GeoJSON feature must become an object");
        };

        assert!(!object.attributes.contains_key("null_value"));
        assert_eq!(
            object.attributes.get("string_value"),
            Some(&Value::String("01000".to_owned()))
        );
        assert_eq!(
            object.attributes.get("integer_value"),
            Some(&Value::Integer(42))
        );
        assert_eq!(
            object.attributes.get("double_value"),
            Some(&Value::Double(12.5))
        );
    }

    #[test]
    fn source_emits_all_features_from_feature_collection() {
        let geojson_content = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "id": "feature-1",
                    "properties": {},
                    "geometry": null
                },
                {
                    "type": "Feature",
                    "id": "feature-2",
                    "properties": {},
                    "geometry": null
                }
            ]
        }"#;

        let parcels = run_geojson_source(geojson_content);
        let ids = parcels.iter().map(feature_id).collect::<Vec<_>>();

        assert_eq!(ids, vec!["feature-1", "feature-2"]);
    }

    #[test]
    fn source_emits_single_top_level_feature() {
        let geojson_content = r#"{
            "type": "Feature",
            "id": "feature-1",
            "properties": {},
            "geometry": null
        }"#;

        let parcels = run_geojson_source(geojson_content);

        assert_eq!(parcels.len(), 1);
        assert_eq!(feature_id(&parcels[0]), "feature-1");
    }
}
