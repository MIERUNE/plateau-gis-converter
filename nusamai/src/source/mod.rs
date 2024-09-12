//! Input data sources (mainly CityGML)

pub mod citygml;

use crate::{
    parameters::Parameters,
    pipeline::{Feedback, Result, Sender},
};

pub struct SourceInfo {
    pub name: String,
    // ...
}

pub trait DataSourceProvider {
    /// Creates a source instance.
    fn create(&self, config: &Parameters) -> Box<dyn DataSource>;

    /// Gets basic information of the source.
    fn info(&self) -> SourceInfo;

    /// Gets the configurable parameters of the source.
    fn sink_options(&self) -> Parameters;
}

pub trait DataSource: Send {
    fn run(&mut self, sink: Sender, feedback: &Feedback) -> Result<()>;

    /// Set whether to parse appearances
    fn set_appearance_parsing(&mut self, _value: bool);
}
