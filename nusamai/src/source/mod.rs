//! Input data sources (mainly CityGML)

pub mod citygml;
pub mod file_reader;
pub mod geojson;

use crate::{
    parameters::Parameters,
    pipeline::{Feedback, Result, Sender},
};
use nusamai_citygml::schema::Schema;

pub struct SourceInfo {
    pub name: String,
    // ...
}

pub trait DataSourceProvider {
    /// Collects source-specific schema definitions before pipeline construction.
    fn collect_schema(&self, _config: &Parameters, _schema: &mut Schema) -> Result<()> {
        Ok(())
    }

    /// Creates a source instance.
    fn create(&self, config: &Parameters) -> Box<dyn DataSource>;

    /// Gets basic information of the source.
    fn info(&self) -> SourceInfo;

    /// Gets the configurable parameters of the source.
    fn sink_options(&self) -> Parameters;
}

pub fn collect_input_schema(
    source_provider: &dyn DataSourceProvider,
    config: &Parameters,
) -> Result<Schema> {
    let mut schema = Schema::default();
    source_provider.collect_schema(config, &mut schema)?;
    Ok(schema)
}

pub trait DataSource: Send {
    fn run(&mut self, sink: Sender, feedback: &Feedback) -> Result<()>;

    /// Set whether to parse appearances
    fn set_appearance_parsing(&mut self, _value: bool);
}

#[cfg(test)]
mod tests {
    use super::geojson::GeoJsonSourceProvider;
    use super::*;

    struct NoSchemaSourceProvider;

    impl DataSourceProvider for NoSchemaSourceProvider {
        fn create(&self, _config: &Parameters) -> Box<dyn DataSource> {
            unreachable!("the no-schema provider is only used to test the default schema collector")
        }

        fn info(&self) -> SourceInfo {
            SourceInfo {
                name: "No schema".to_owned(),
            }
        }

        fn sink_options(&self) -> Parameters {
            Parameters::default()
        }
    }

    #[test]
    fn provider_collect_schema_defaults_to_no_op() {
        let provider = NoSchemaSourceProvider;
        let mut schema = Schema::default();

        provider
            .collect_schema(&Parameters::default(), &mut schema)
            .unwrap();

        assert!(schema.types.is_empty());
    }

    #[test]
    fn collect_input_schema_does_not_mix_citygml_types_into_geojson() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let path = temp_dir.path().join("input.geojson");
        std::fs::write(
            &path,
            r#"{
                "type": "Feature",
                "properties": {"name": "example"},
                "geometry": null
            }"#,
        )
        .unwrap();
        let provider = GeoJsonSourceProvider {
            filenames: vec![path],
        };

        let schema = collect_input_schema(&provider, &Parameters::default()).unwrap();

        assert!(schema.types.contains_key("geojson:Feature"));
        assert!(!schema.types.contains_key("bldg:Building"));
        assert!(!schema.types.contains_key("gen:GenericCityObject"));
    }
}
