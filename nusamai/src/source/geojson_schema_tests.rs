use super::{schema::GeoJsonSchemaBuilder, GeoJsonSourceProvider, GEOJSON_TYPENAME};
use geojson::JsonObject;
use nusamai_citygml::schema::{Attribute, FeatureTypeDef, Schema, TypeDef, TypeRef};
use serde_json::json;
use std::{io::Write, path::PathBuf};

use crate::{
    parameters::Parameters,
    pipeline::PipelineError,
    source::{citygml::CityGmlSourceProvider, DataSourceProvider},
};

fn properties(value: serde_json::Value) -> JsonObject {
    value
        .as_object()
        .expect("test value must be an object")
        .clone()
}

fn geojson_feature_type(schema: &Schema) -> &FeatureTypeDef {
    let Some(TypeDef::Feature(feature_type)) = schema.types.get(GEOJSON_TYPENAME) else {
        panic!("GeoJSON feature type must exist");
    };
    feature_type
}

#[test]
fn uses_neutral_geojson_feature_typename() {
    assert_eq!(GEOJSON_TYPENAME, "geojson:Feature");
}

#[test]
fn collects_union_in_deterministic_order_with_optional_attributes() {
    let first = properties(json!({"zeta": 1, "alpha": "first"}));
    let second = properties(json!({"middle": true, "alpha": "second"}));
    let mut builder = GeoJsonSchemaBuilder::default();

    builder.observe_properties(Some(&first));
    builder.observe_properties(None);
    builder.observe_properties(Some(&second));
    let feature_type = builder.finish();

    assert_eq!(
        feature_type
            .attributes
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["alpha", "middle", "zeta"]
    );
    assert!(!feature_type.additional_attributes);
    assert!(feature_type.attributes.values().all(|attribute| {
        attribute.min_occurs == 0
            && attribute.max_occurs == Some(1)
            && attribute.original_name.is_none()
    }));
}

#[test]
fn maps_each_json_value_kind_to_schema_type() {
    let values = properties(json!({
        "array": [1, 2],
        "boolean": true,
        "double": 1.5,
        "integer": -1,
        "object": {"nested": "value"},
        "string": "001"
    }));
    let mut builder = GeoJsonSchemaBuilder::default();

    builder.observe_properties(Some(&values));
    let feature_type = builder.finish();

    assert_eq!(
        feature_type.attributes["array"].type_ref,
        TypeRef::JsonString(Box::new(Attribute::new(TypeRef::String)))
    );
    assert_eq!(
        feature_type.attributes["boolean"].type_ref,
        TypeRef::Boolean
    );
    assert_eq!(feature_type.attributes["double"].type_ref, TypeRef::Double);
    assert_eq!(
        feature_type.attributes["integer"].type_ref,
        TypeRef::Integer
    );
    assert_eq!(
        feature_type.attributes["object"].type_ref,
        TypeRef::JsonString(Box::new(Attribute::new(TypeRef::String)))
    );
    assert_eq!(feature_type.attributes["string"].type_ref, TypeRef::String);
}

#[test]
fn widens_numeric_and_null_types_deterministically() {
    let first = properties(json!({
        "all_null": null,
        "nullable_boolean": null,
        "number": 1
    }));
    let second = properties(json!({
        "all_null": null,
        "nullable_boolean": false,
        "number": 1.25
    }));
    let mut builder = GeoJsonSchemaBuilder::default();

    builder.observe_properties(Some(&first));
    builder.observe_properties(Some(&second));
    let feature_type = builder.finish();

    assert_eq!(
        feature_type.attributes["all_null"].type_ref,
        TypeRef::String
    );
    assert_eq!(
        feature_type.attributes["nullable_boolean"].type_ref,
        TypeRef::Boolean
    );
    assert_eq!(feature_type.attributes["number"].type_ref, TypeRef::Double);
}

#[test]
fn widens_incompatible_types_to_string() {
    let first = properties(json!({"mixed": true, "structured": [1]}));
    let second = properties(json!({"mixed": 1, "structured": {"key": "value"}}));
    let third = properties(json!({"structured": "plain"}));
    let mut builder = GeoJsonSchemaBuilder::default();

    builder.observe_properties(Some(&first));
    builder.observe_properties(Some(&second));
    builder.observe_properties(Some(&third));
    let feature_type = builder.finish();

    assert_eq!(feature_type.attributes["mixed"].type_ref, TypeRef::String);
    assert_eq!(
        feature_type.attributes["structured"].type_ref,
        TypeRef::String
    );
}

#[test]
fn source_provider_collects_schema_across_feature_documents_and_files() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let single_path = temp_dir.path().join("single.geojson");
    let collection_path = temp_dir.path().join("collection.geojson");
    std::fs::write(
        &single_path,
        r#"{
            "type": "Feature",
            "properties": {"single": "001", "shared": 1},
            "geometry": null
        }"#,
    )
    .unwrap();
    std::fs::write(
        &collection_path,
        r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": {"collection": true, "shared": 1.5},
                    "geometry": null
                },
                {
                    "type": "Feature",
                    "properties": {"null_only": null},
                    "geometry": null
                }
            ]
        }"#,
    )
    .unwrap();
    let provider = GeoJsonSourceProvider {
        filenames: vec![single_path, collection_path],
    };
    let mut schema = Schema::default();

    provider
        .collect_schema(&Parameters::default(), &mut schema)
        .unwrap();
    let feature_type = geojson_feature_type(&schema);

    assert_eq!(
        feature_type
            .attributes
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["collection", "null_only", "shared", "single"]
    );
    assert_eq!(
        feature_type.attributes["collection"].type_ref,
        TypeRef::Boolean
    );
    assert_eq!(
        feature_type.attributes["null_only"].type_ref,
        TypeRef::String
    );
    assert_eq!(feature_type.attributes["shared"].type_ref, TypeRef::Double);
    assert_eq!(feature_type.attributes["single"].type_ref, TypeRef::String);
}

#[test]
fn source_provider_collects_schema_from_zip_entry() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let zip_path = temp_dir.path().join("input.zip");
    let zip_file = std::fs::File::create(&zip_path).unwrap();
    let mut writer = zip::ZipWriter::new(zip_file);
    writer
        .start_file(
            "nested/input.geojson",
            zip::write::SimpleFileOptions::default(),
        )
        .unwrap();
    writer
        .write_all(
            br#"{
                "type": "Feature",
                "properties": {"from_zip": 42},
                "geometry": null
            }"#,
        )
        .unwrap();
    writer.finish().unwrap();

    let virtual_path = PathBuf::from(format!(
        "{}/nested/input.geojson",
        zip_path.to_string_lossy()
    ));
    let provider = GeoJsonSourceProvider {
        filenames: vec![virtual_path],
    };
    let mut schema = Schema::default();

    provider
        .collect_schema(&Parameters::default(), &mut schema)
        .unwrap();

    assert_eq!(
        geojson_feature_type(&schema).attributes["from_zip"].type_ref,
        TypeRef::Integer
    );
}

#[test]
fn source_provider_rejects_bare_geometry_and_invalid_json() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let geometry_path = temp_dir.path().join("geometry.geojson");
    let invalid_path = temp_dir.path().join("invalid.geojson");
    std::fs::write(
        &geometry_path,
        r#"{"type":"Point","coordinates":[139.7,35.6]}"#,
    )
    .unwrap();
    std::fs::write(&invalid_path, "not json").unwrap();

    for (path, expected) in [
        (geometry_path, "Direct geometry is not supported"),
        (invalid_path, "Failed to parse GeoJSON"),
    ] {
        let provider = GeoJsonSourceProvider {
            filenames: vec![path],
        };
        let mut schema = Schema::default();

        let error = provider
            .collect_schema(&Parameters::default(), &mut schema)
            .unwrap_err();

        assert!(
            matches!(error, PipelineError::Other(message) if message.contains(expected)),
            "unexpected error"
        );
    }
}

#[test]
fn source_provider_collect_schema_defaults_to_no_op() {
    let provider = CityGmlSourceProvider { filenames: vec![] };
    let mut schema = Schema::default();

    provider
        .collect_schema(&Parameters::default(), &mut schema)
        .unwrap();

    assert!(schema.types.is_empty());
}
