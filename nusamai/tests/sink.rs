use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Once,
};

use mlt_core::{
    geo_types::Geometry,
    geojson::{Feature, FeatureCollection},
    mvt::{mvt_to_feature_collection, mvt_to_tile_layers},
    Decoder, Layer, Parser, TileLayer,
};
use nusamai::{
    sink::{self, DataSinkProvider},
    source::{citygml::CityGmlSourceProvider, DataSourceProvider},
    transformer::{
        MultiThreadTransformer, NusamaiTransformBuilder, TransformBuilder, TransformerSettings,
    },
};
use nusamai_citygml::CityGmlElement;
use nusamai_plateau::models::TopLevelCityObject;

static INIT: Once = Once::new();

pub(crate) fn simple_run_sink<S: DataSinkProvider>(sink_provider: S, output: Option<&str>) {
    simple_run_sink_with_params(sink_provider, output, vec![])
}

pub(crate) fn simple_run_sink_with_params<S: DataSinkProvider>(
    sink_provider: S,
    output: Option<&str>,
    additional_params: Vec<(&str, &str)>,
) {
    INIT.call_once(|| {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "error")
        }
        pretty_env_logger::init();
    });

    let filenames = [
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/rwy/53395527_rwy_6697.gml",
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/brid/dorokyo_51324378_brid_6697.gml",
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/trk/53361601_trk_6697.gml",
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/tun/53361613_tun_6697.gml",
        // "../nusamai-plateau/tests/data/plateau-3_0/udx/veg/52385628_veg_6697_op.gml",
        "../nusamai-plateau/tests/data/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml",
        "../nusamai-plateau/tests/data/yokosuka-shi/udx/bldg/52397519_bldg_6697_op.gml",
        "../nusamai-plateau/tests/data/numazu-shi/udx/tran/52385608_tran_6697_op.gml",
        "../nusamai-plateau/tests/data/kawasaki-shi.zip/kawasaki-shi/udx/frn/53391597_frn_6697_op.gml",
    ];

    let source_provider: Box<dyn DataSourceProvider> = Box::new(CityGmlSourceProvider {
        filenames: filenames
            .iter()
            .map(|name| PathBuf::from_str(name).unwrap())
            .collect(),
    });
    assert_eq!(source_provider.info().name, "CityGML");

    let source = source_provider.create(&source_provider.sink_options());

    let mut sink = {
        assert!(!sink_provider.info().name.is_empty());
        let mut sink_params = sink_provider.sink_options();

        let mut params_to_update: Vec<(String, String)> = vec![];
        if let Some(output) = output {
            params_to_update.push(("@output".into(), output.into()));
        }
        for (key, value) in &additional_params {
            // output_epsg is handled separately in transformer configuration
            if key != &"output_epsg" {
                params_to_update.push((key.to_string(), value.to_string()));
            }
        }

        if !params_to_update.is_empty() {
            sink_params
                .update_values_with_str(&params_to_update)
                .unwrap();
        }

        sink_params.validate().unwrap();
        sink_provider.create(&sink_params)
    };

    let options: TransformerSettings = TransformerSettings::new();

    let (transformer, schema) = {
        let mut transform_req = sink.make_requirements(options);
        // Apply additional configuration if provided
        for (key, value) in &additional_params {
            if key == &"output_epsg" {
                if let Ok(epsg) = value.parse::<u16>() {
                    transform_req.set_output_epsg(epsg);
                }
            }
        }
        let transform_builder = NusamaiTransformBuilder::new(transform_req.into());
        let mut schema = nusamai_citygml::schema::Schema::default();
        TopLevelCityObject::collect_schema(&mut schema);
        transform_builder.transform_schema(&mut schema);
        let transformer = Box::new(MultiThreadTransformer::new(transform_builder));
        (transformer, schema)
    };

    let (handle, watcher, canceller) =
        nusamai::pipeline::run(source, transformer, sink, schema.into());
    handle.join().unwrap();

    for msg in watcher {
        println!("Feedback message from the pipeline {msg:?}");
    }

    // should not be canceled
    assert!(!canceller.is_canceled());
}

#[test]
fn run_serde_sink() {
    simple_run_sink(sink::serde::SerdeSinkProvider {}, "/dev/null".into());
}

#[test]
fn run_czml_sink() {
    simple_run_sink(sink::czml::CzmlSinkProvider {}, "/dev/null".into());
}

#[test]
fn run_gltf_sink() {
    simple_run_sink_with_params(
        sink::gltf::GltfSinkProvider {},
        "/tmp/nusamai/gltf".into(),
        vec![("output_epsg", "6669")],
    );
}

#[test]
fn run_obj_sink() {
    simple_run_sink_with_params(
        sink::obj::ObjSinkProvider {},
        "/tmp/nusamai/obj".into(),
        vec![("output_epsg", "6669")],
    );
}

#[test]
fn run_noop_sink() {
    simple_run_sink(sink::noop::NoopSinkProvider {}, None);
}

#[test]
fn run_geojson_sink() {
    simple_run_sink(
        sink::geojson::GeoJsonSinkProvider {},
        "/tmp/nusamai/geojson".into(),
    );
}

#[test]
fn run_gpkg_sink() {
    simple_run_sink(sink::gpkg::GpkgSinkProvider {}, "sqlite::memory:".into());
}

#[test]
fn run_mvt_sink() {
    simple_run_sink(sink::mvt::MvtSinkProvider {}, "/tmp/nusamai/mvt/".into());
}

#[test]
fn run_mlt_sink() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().join("mlt");
    let output_path_string = output_path.to_str().unwrap().to_string();

    simple_run_sink_with_params(
        sink::mlt::MltSinkProvider {},
        Some(output_path_string.as_str()),
        vec![("min_z", "14"), ("max_z", "14")],
    );

    let mut mlt_files = Vec::new();
    collect_files_with_extension(&output_path, "mlt", &mut mlt_files);
    assert!(
        !mlt_files.is_empty(),
        "MLT output should contain .mlt tiles"
    );

    let mut saw_typed_null = false;
    for path in &mlt_files {
        let layers = decode_mlt_tile_layers(path);
        assert!(
            !layers.is_empty(),
            "MLT tile should contain FeatureTable layers: {}",
            path.display()
        );
        for layer in &layers {
            saw_typed_null |= assert_decoded_mlt_layer_is_structurally_valid(path, layer);
        }
    }

    assert!(
        saw_typed_null,
        "MLT fixture output should contain at least one missing property represented as a typed null"
    );
}

#[test]
fn run_mlt_sink_matches_mvt_for_general_use() {
    let temp_dir = tempfile::tempdir().unwrap();
    let mvt_output_path = temp_dir.path().join("mvt");
    let mlt_output_path = temp_dir.path().join("mlt");
    let mvt_output_path_string = mvt_output_path.to_str().unwrap().to_string();
    let mlt_output_path_string = mlt_output_path.to_str().unwrap().to_string();

    simple_run_sink_with_params(
        sink::mvt::MvtSinkProvider {},
        Some(mvt_output_path_string.as_str()),
        vec![("min_z", "14"), ("max_z", "14")],
    );
    simple_run_sink_with_params(
        sink::mlt::MltSinkProvider {},
        Some(mlt_output_path_string.as_str()),
        vec![("min_z", "14"), ("max_z", "14")],
    );

    let mvt_tiles = decode_mvt_tiles_by_coord(&mvt_output_path);
    let mlt_tiles = decode_mlt_tiles_by_coord(&mlt_output_path);
    let mvt_features = decode_mvt_feature_collections_by_coord(&mvt_output_path);
    let mlt_features = decode_mlt_feature_collections_by_coord(&mlt_output_path);

    assert!(
        !mvt_tiles.is_empty(),
        "MVT fixture output should contain tiles"
    );
    assert!(
        !mlt_tiles.is_empty(),
        "MLT fixture output should contain tiles"
    );
    assert_eq!(
        mvt_tiles.keys().collect::<BTreeSet<_>>(),
        mlt_tiles.keys().collect::<BTreeSet<_>>(),
        "MVT and MLT should produce the same tile coordinate set"
    );

    for (coord, mvt_layers) in &mvt_tiles {
        let mlt_layers = mlt_tiles.get(coord).unwrap();
        assert_layers_match_for_general_use(coord, mvt_layers, mlt_layers);
        assert_feature_collections_match_for_general_use(
            coord,
            mvt_features.get(coord).unwrap(),
            mlt_features.get(coord).unwrap(),
        );
    }
}

#[test]
fn run_pmtiles_sink() {
    let temp_dir = std::env::temp_dir().join("nusamai_pmtiles_sink_test");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let output_path = temp_dir.join(format!("output-{}.pmtiles", std::process::id()));
    if output_path.exists() {
        std::fs::remove_file(&output_path).unwrap();
    }
    let output_path_string = output_path.to_str().unwrap().to_string();

    simple_run_sink(
        sink::pmtiles::PmTilesSinkProvider {},
        Some(output_path_string.as_str()),
    );

    assert!(
        output_path.exists(),
        "PMTiles output should be created by the pipeline"
    );
    let _ = std::fs::remove_file(output_path);
}

#[test]
fn run_shapefile_sink() {
    simple_run_sink(
        sink::shapefile::ShapefileSinkProvider {},
        "/tmp/nusamai/shapefile".into(),
    );
}

#[test]
fn run_ply_sink() {
    simple_run_sink(sink::ply::StanfordPlySinkProvider {}, "/dev/null".into());
}

#[test]
fn run_3dtiles_sink() {
    simple_run_sink(
        sink::cesiumtiles::CesiumTilesSinkProvider {},
        "/tmp/nusamai/3dtiles/".into(),
    );
}

#[test]
fn run_kml_sink() {
    simple_run_sink(sink::kml::KmlSinkProvider {}, "/tmp/nusamai/kml".into());
}

fn collect_files_with_extension(dir: &std::path::Path, extension: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files_with_extension(&path, extension, out);
        } else if path.extension().is_some_and(|ext| ext == extension) {
            out.push(path);
        }
    }
}

fn decode_mlt_tiles_by_coord(root: &Path) -> BTreeMap<PathBuf, Vec<TileLayer>> {
    collect_tile_files(root, "mlt")
        .into_iter()
        .map(|(coord, path)| (coord, decode_mlt_tile_layers(&path)))
        .collect()
}

fn decode_mvt_tiles_by_coord(root: &Path) -> BTreeMap<PathBuf, Vec<TileLayer>> {
    collect_tile_files(root, "pbf")
        .into_iter()
        .map(|(coord, path)| {
            let bytes = std::fs::read(&path).unwrap();
            let layers = mvt_to_tile_layers(bytes).unwrap();
            (coord, layers)
        })
        .collect()
}

fn decode_mlt_feature_collections_by_coord(root: &Path) -> BTreeMap<PathBuf, FeatureCollection> {
    collect_tile_files(root, "mlt")
        .into_iter()
        .map(|(coord, path)| (coord, decode_mlt_feature_collection(&path)))
        .collect()
}

fn decode_mvt_feature_collections_by_coord(root: &Path) -> BTreeMap<PathBuf, FeatureCollection> {
    collect_tile_files(root, "pbf")
        .into_iter()
        .map(|(coord, path)| {
            let bytes = std::fs::read(&path).unwrap();
            let feature_collection = mvt_to_feature_collection(bytes).unwrap();
            (coord, feature_collection)
        })
        .collect()
}

fn collect_tile_files(root: &Path, extension: &str) -> BTreeMap<PathBuf, PathBuf> {
    let mut files = Vec::new();
    collect_files_with_extension(root, extension, &mut files);
    files
        .into_iter()
        .map(|path| {
            let mut coord = path.strip_prefix(root).unwrap().to_path_buf();
            coord.set_extension("");
            (coord, path)
        })
        .collect()
}

fn decode_mlt_feature_collection(path: &Path) -> FeatureCollection {
    let bytes = std::fs::read(path).unwrap();
    let mut parser = Parser::default();
    let layers = parser.parse_layers(&bytes).unwrap();
    let mut decoder = Decoder::default();
    let decoded_layers = decoder.decode_all(layers).unwrap();
    FeatureCollection::from_layers(decoded_layers).unwrap()
}

fn decode_mlt_tile_layers(path: &Path) -> Vec<TileLayer> {
    let bytes = std::fs::read(path).unwrap();
    assert!(
        !bytes.is_empty(),
        "MLT tile should not be empty: {}",
        path.display()
    );

    let mut parser = Parser::default();
    let layers = parser.parse_layers(&bytes).unwrap();
    assert!(
        !layers.is_empty(),
        "MLT tile should contain FeatureTable layers: {}",
        path.display()
    );

    let mut decoder = Decoder::default();
    layers
        .into_iter()
        .map(|layer| match layer {
            Layer::Tag01(layer) => {
                assert!(
                    !layer.name().is_empty(),
                    "MLT FeatureTable layer name should not be empty: {}",
                    path.display()
                );
                assert!(
                    layer.extent().get() > 0,
                    "MLT FeatureTable extent should be non-zero: {}",
                    path.display()
                );
                layer.into_tile(&mut decoder).unwrap()
            }
            Layer::Unknown(unknown) => panic!(
                "MLT tile contains an unsupported layer tag {}: {}",
                unknown.tag(),
                path.display()
            ),
            _ => panic!(
                "MLT tile contains an unsupported layer variant: {}",
                path.display()
            ),
        })
        .collect()
}

fn assert_decoded_mlt_layer_is_structurally_valid(path: &Path, layer: &TileLayer) -> bool {
    assert!(
        !layer.name().is_empty(),
        "MLT decoded layer name should not be empty: {}",
        path.display()
    );
    assert!(
        layer.extent().get() > 0,
        "MLT decoded layer extent should be non-zero: {}",
        path.display()
    );
    assert_eq!(
        layer.feature_count(),
        layer.features().len(),
        "MLT feature count should match decoded feature rows: {} layer {}",
        path.display(),
        layer.name()
    );
    assert!(
        layer.feature_count() > 0,
        "MLT FeatureTable layer should contain features: {} layer {}",
        path.display(),
        layer.name()
    );

    let property_count = layer.property_names().len();
    let mut column_kinds = vec![None; property_count];
    let mut saw_typed_null = false;

    for (feature_index, feature) in layer.features().iter().enumerate() {
        assert!(
            geometry_is_non_empty(feature.geometry()),
            "MLT geometry column should decode to non-empty geometry: {} layer {} feature {}",
            path.display(),
            layer.name(),
            feature_index
        );
        assert_eq!(
            feature.properties().len(),
            property_count,
            "MLT property row width should match property columns: {} layer {} feature {}",
            path.display(),
            layer.name(),
            feature_index
        );

        for (property_index, value) in feature.properties().iter().enumerate() {
            let actual_kind = value.kind();
            match column_kinds[property_index] {
                Some(expected_kind) => assert_eq!(
                    actual_kind,
                    expected_kind,
                    "MLT property column type should be consistent: {} layer {} property {}",
                    path.display(),
                    layer.name(),
                    layer.property_names()[property_index]
                ),
                None => column_kinds[property_index] = Some(actual_kind),
            }

            if value.is_null() {
                saw_typed_null = true;
                assert_eq!(
                    actual_kind,
                    column_kinds[property_index].unwrap(),
                    "MLT missing property should keep the column's typed null variant: {} layer {} property {}",
                    path.display(),
                    layer.name(),
                    layer.property_names()[property_index]
                );
            }
        }
    }

    saw_typed_null
}

fn assert_layers_match_for_general_use(
    coord: &Path,
    mvt_layers: &[TileLayer],
    mlt_layers: &[TileLayer],
) {
    let mvt_by_name = layers_by_name(mvt_layers);
    let mlt_by_name = layers_by_name(mlt_layers);

    assert_eq!(
        mvt_by_name.keys().collect::<BTreeSet<_>>(),
        mlt_by_name.keys().collect::<BTreeSet<_>>(),
        "MVT and MLT should contain the same layers at tile {}",
        coord.display()
    );

    for (layer_name, mvt_layer) in mvt_by_name {
        let mlt_layer = mlt_by_name[layer_name];
        assert_eq!(
            mvt_layer.extent().get(),
            mlt_layer.extent().get(),
            "MVT and MLT layer extent should match at tile {} layer {}",
            coord.display(),
            layer_name
        );
        assert_eq!(
            mvt_layer.feature_count(),
            mlt_layer.feature_count(),
            "MVT and MLT feature counts should match at tile {} layer {}",
            coord.display(),
            layer_name
        );
        assert_eq!(
            property_name_set(mvt_layer),
            property_name_set(mlt_layer),
            "MVT and MLT property names should match at tile {} layer {}",
            coord.display(),
            layer_name
        );
    }
}

fn layers_by_name(layers: &[TileLayer]) -> BTreeMap<&str, &TileLayer> {
    let mut by_name = BTreeMap::new();
    for layer in layers {
        assert!(
            by_name.insert(layer.name(), layer).is_none(),
            "duplicate tile layer name: {}",
            layer.name()
        );
    }
    by_name
}

fn property_name_set(layer: &TileLayer) -> BTreeSet<&str> {
    layer.property_names().iter().map(String::as_str).collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ComparableFeature {
    id: Option<u64>,
    layer: String,
    geometry_type: &'static str,
    geometry_bounds: Option<GeometryBounds>,
    properties: BTreeMap<String, String>,
}

fn assert_feature_collections_match_for_general_use(
    coord: &Path,
    mvt_features: &FeatureCollection,
    mlt_features: &FeatureCollection,
) {
    assert_eq!(
        mvt_features.features.len(),
        mlt_features.features.len(),
        "MVT and MLT feature counts should match in FeatureCollection at tile {}",
        coord.display()
    );

    let mvt_multiset = feature_multiset(mvt_features);
    let mlt_multiset = feature_multiset(mlt_features);
    if mvt_multiset != mlt_multiset {
        let missing_from_mlt = multiset_difference(&mvt_multiset, &mlt_multiset);
        let missing_from_mvt = multiset_difference(&mlt_multiset, &mvt_multiset);
        panic!(
            "MVT and MLT feature ids, geometry type/bounds, and non-null properties should match at tile {}. missing_from_mlt={:?}; missing_from_mvt={:?}",
            coord.display(),
            missing_from_mlt.first(),
            missing_from_mvt.first()
        );
    }
}

fn feature_multiset(feature_collection: &FeatureCollection) -> BTreeMap<ComparableFeature, usize> {
    let mut features = BTreeMap::new();
    for feature in &feature_collection.features {
        *features.entry(comparable_feature(feature)).or_default() += 1;
    }
    features
}

fn multiset_difference(
    expected: &BTreeMap<ComparableFeature, usize>,
    actual: &BTreeMap<ComparableFeature, usize>,
) -> Vec<(ComparableFeature, usize)> {
    expected
        .iter()
        .filter_map(|(feature, expected_count)| {
            let actual_count = actual.get(feature).copied().unwrap_or_default();
            (actual_count < *expected_count)
                .then(|| (feature.clone(), expected_count - actual_count))
        })
        .take(3)
        .collect()
}

fn comparable_feature(feature: &Feature) -> ComparableFeature {
    ComparableFeature {
        id: feature.id,
        layer: feature
            .properties
            .get("_layer")
            .and_then(|value| value.as_str())
            .unwrap_or("")
            .to_string(),
        geometry_type: geometry_type_name(&feature.geometry),
        geometry_bounds: geometry_bounds(&feature.geometry),
        properties: comparable_json_properties(&feature.properties),
    }
}

fn comparable_json_properties(
    properties: &BTreeMap<String, serde_json::Value>,
) -> BTreeMap<String, String> {
    properties
        .iter()
        .map(|(name, value)| (name.clone(), comparable_json_value(value)))
        .collect()
}

fn comparable_json_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(value) => format!("bool:{value}"),
        serde_json::Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                format!("int:{value}")
            } else if let Some(value) = value.as_u64() {
                format!("int:{value}")
            } else {
                format_float_value(value.as_f64().unwrap())
            }
        }
        serde_json::Value::String(value) => format!("str:{value}"),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            format!("json:{}", serde_json::to_string(value).unwrap())
        }
    }
}

fn format_float_value(value: f64) -> String {
    let value = if value == 0.0 { 0.0 } else { value };
    format!("float:{value:.6}")
}

fn geometry_is_non_empty(geometry: &Geometry<i32>) -> bool {
    match geometry {
        Geometry::Point(_) => true,
        Geometry::Line(_) => true,
        Geometry::LineString(line_string) => !line_string.0.is_empty(),
        Geometry::Polygon(polygon) => !polygon.exterior().0.is_empty(),
        Geometry::MultiPoint(points) => !points.0.is_empty(),
        Geometry::MultiLineString(lines) => !lines.0.is_empty(),
        Geometry::MultiPolygon(polygons) => !polygons.0.is_empty(),
        Geometry::GeometryCollection(geometries) => !geometries.0.is_empty(),
        Geometry::Rect(_) => true,
        Geometry::Triangle(_) => true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct GeometryBounds {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

fn geometry_type_name(geometry: &Geometry<i32>) -> &'static str {
    match geometry {
        Geometry::Point(_) => "Point",
        Geometry::Line(_) => "Line",
        Geometry::LineString(_) => "LineString",
        Geometry::Polygon(_) => "Polygonal",
        Geometry::MultiPoint(_) => "MultiPoint",
        Geometry::MultiLineString(_) => "MultiLineString",
        Geometry::MultiPolygon(_) => "Polygonal",
        Geometry::GeometryCollection(_) => "GeometryCollection",
        Geometry::Rect(_) => "Rect",
        Geometry::Triangle(_) => "Triangle",
    }
}

fn geometry_bounds(geometry: &Geometry<i32>) -> Option<GeometryBounds> {
    let mut bounds = None;
    collect_geometry_bounds(geometry, &mut bounds);
    bounds
}

fn collect_geometry_bounds(geometry: &Geometry<i32>, bounds: &mut Option<GeometryBounds>) {
    match geometry {
        Geometry::Point(point) => include_coord(bounds, point.x(), point.y()),
        Geometry::Line(line) => {
            include_coord(bounds, line.start.x, line.start.y);
            include_coord(bounds, line.end.x, line.end.y);
        }
        Geometry::LineString(line_string) => {
            for coord in &line_string.0 {
                include_coord(bounds, coord.x, coord.y);
            }
        }
        Geometry::Polygon(polygon) => {
            for coord in &polygon.exterior().0 {
                include_coord(bounds, coord.x, coord.y);
            }
            for ring in polygon.interiors() {
                for coord in &ring.0 {
                    include_coord(bounds, coord.x, coord.y);
                }
            }
        }
        Geometry::MultiPoint(points) => {
            for point in &points.0 {
                include_coord(bounds, point.x(), point.y());
            }
        }
        Geometry::MultiLineString(lines) => {
            for line in &lines.0 {
                for coord in &line.0 {
                    include_coord(bounds, coord.x, coord.y);
                }
            }
        }
        Geometry::MultiPolygon(polygons) => {
            for polygon in &polygons.0 {
                for coord in &polygon.exterior().0 {
                    include_coord(bounds, coord.x, coord.y);
                }
                for ring in polygon.interiors() {
                    for coord in &ring.0 {
                        include_coord(bounds, coord.x, coord.y);
                    }
                }
            }
        }
        Geometry::GeometryCollection(geometries) => {
            for geometry in &geometries.0 {
                collect_geometry_bounds(geometry, bounds);
            }
        }
        Geometry::Rect(rect) => {
            let min = rect.min();
            let max = rect.max();
            include_coord(bounds, min.x, min.y);
            include_coord(bounds, max.x, max.y);
        }
        Geometry::Triangle(triangle) => {
            for coord in triangle.to_array() {
                include_coord(bounds, coord.x, coord.y);
            }
        }
    }
}

fn include_coord(bounds: &mut Option<GeometryBounds>, x: i32, y: i32) {
    match bounds {
        Some(bounds) => {
            bounds.min_x = bounds.min_x.min(x);
            bounds.min_y = bounds.min_y.min(y);
            bounds.max_x = bounds.max_x.max(x);
            bounds.max_y = bounds.max_y.max(y);
        }
        None => {
            *bounds = Some(GeometryBounds {
                min_x: x,
                min_y: y,
                max_x: x,
                max_y: y,
            });
        }
    }
}
