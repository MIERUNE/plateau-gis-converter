pub mod citygml;

use crate::parameters::Parameters;
use crate::pipeline::{Feedback, Sender};

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
    fn parameters(&self) -> Parameters;
}

pub trait DataSource: Send {
    fn run(&mut self, sink: Sender, feedback: &Feedback);
}
