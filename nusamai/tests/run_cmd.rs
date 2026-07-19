use assert_cmd::cargo::cargo_bin_cmd;
use nusamai_citygml::schema::{Schema, TypeDef, TypeRef};
use nusamai_gpkg::GpkgHandler;

#[test]
fn test_run_cmd() {
    let mut cmd = cargo_bin_cmd!("nusamai");
    let assert = cmd
        .arg("../nusamai-plateau/tests/data/sendai-shi/udx/urf/574026_urf_6668_huchi_op.gml")
        .arg("--sink")
        .arg("noop")
        .arg("--output")
        .arg("dummy")
        .arg("--rules")
        .arg("./tests/rules.json")
        .arg("--schema")
        .arg("schema.json")
        .assert();
    assert.success();
}

#[test]
fn geojson_source_schema_is_collected_before_cli_transform() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.geojson");
    let schema_path = temp_dir.path().join("schema.json");
    let output_path = temp_dir.path().join("unused");
    std::fs::write(
        &input_path,
        r#"{
            "type": "Feature",
            "properties": {
                "N03_001": "北海道",
                "N03_007": "01000",
                "nullable": null
            },
            "geometry": null
        }"#,
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("nusamai");
    cmd.arg(&input_path)
        .arg("--sink")
        .arg("noop")
        .arg("--output")
        .arg(&output_path)
        .arg("--schema")
        .arg(&schema_path)
        .assert()
        .success();

    let schema: Schema = serde_json::from_slice(&std::fs::read(&schema_path).unwrap()).unwrap();
    let Some(TypeDef::Feature(feature_type)) = schema.types.get("geojson:Feature") else {
        panic!("transformed GeoJSON schema must exist");
    };

    assert_eq!(feature_type.attributes["N03_001"].type_ref, TypeRef::String);
    assert_eq!(feature_type.attributes["N03_007"].type_ref, TypeRef::String);
    assert_eq!(
        feature_type.attributes["nullable"].type_ref,
        TypeRef::String
    );
}

#[tokio::test]
async fn n03_like_geojson_is_written_to_geopackage_with_dynamic_columns() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let input_path = temp_dir.path().join("n03.geojson");
    let output_path = temp_dir.path().join("n03.gpkg");
    std::fs::write(
        &input_path,
        r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "id": "prefecture-1",
                    "properties": {
                        "N03_001": "北海道",
                        "N03_002": null,
                        "N03_003": null,
                        "N03_004": null,
                        "N03_007": "01000"
                    },
                    "geometry": {
                        "type": "Polygon",
                        "coordinates": [[
                            [141.0, 43.0],
                            [141.1, 43.0],
                            [141.1, 43.1],
                            [141.0, 43.1],
                            [141.0, 43.0]
                        ]]
                    }
                }
            ]
        }"#,
    )
    .unwrap();

    let mut cmd = cargo_bin_cmd!("nusamai");
    cmd.arg(&input_path)
        .arg("--sink")
        .arg("gpkg")
        .arg("--output")
        .arg(&output_path)
        .assert()
        .success();

    let handler = GpkgHandler::open(&format!("file:{}", output_path.to_string_lossy()))
        .await
        .unwrap();
    assert!(handler
        .table_names()
        .await
        .contains(&"geojson:Feature".to_owned()));

    let columns = handler.table_columns("\"geojson:Feature\"").await.unwrap();
    for expected in ["N03_001", "N03_002", "N03_003", "N03_004", "N03_007"] {
        assert!(
            columns
                .iter()
                .any(|(name, data_type, not_null)| name == expected
                    && data_type == "TEXT"
                    && *not_null == 0),
            "missing nullable TEXT column: {expected}"
        );
    }

    let rows = handler.fetch_rows("\"geojson:Feature\"").await.unwrap();
    assert_eq!(rows.len(), 1);
}
