use assert_cmd::cargo::cargo_bin_cmd;
use nusamai_citygml::schema::{Schema, TypeDef, TypeRef};

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
