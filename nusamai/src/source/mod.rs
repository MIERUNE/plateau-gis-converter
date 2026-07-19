//! Input data sources (mainly CityGML)

pub mod citygml;
pub mod file_reader;
pub mod geojson;

use crate::{
    parameters::Parameters,
    pipeline::{Feedback, Result, Sender},
};
use nusamai_citygml::{schema::Schema, CityGmlElement};
use nusamai_plateau::models::TopLevelCityObject;

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
    TopLevelCityObject::collect_schema(&mut schema);
    source_provider.collect_schema(config, &mut schema)?;
    Ok(schema)
}

pub trait DataSource: Send {
    fn run(&mut self, sink: Sender, feedback: &Feedback) -> Result<()>;

    /// Set whether to parse appearances
    fn set_appearance_parsing(&mut self, _value: bool);
}
