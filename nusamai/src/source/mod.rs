pub mod citygml;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Sender};

pub struct SourceInfo {
    pub name: String,
    // ...
}

pub trait DataSourceProvider {
    /// Creates a source instance.
    fn create(&self, config: &Config) -> Box<dyn DataSource>;

    /// Gets basic information of the source.
    fn info(&self) -> SourceInfo;

    /// Gets the configurable parameters of the source.
    fn config(&self) -> Config;
}

pub trait DataSource: Send {
    fn run(&mut self, sink: Sender, feedback: &Feedback);
}
