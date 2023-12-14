use crate::configuration::Config;
use crate::pipeline::{Feedback, Sender};

pub struct SourceInfo {
    pub name: String,
    // ...
}

pub trait SourceProvider {
    /// Creates a source instance.
    fn create(&self) -> Box<dyn Source>;

    /// Gets basic information about the sink.
    fn info(&self) -> SourceInfo;

    /// Gets the configuration for the source.
    fn configuration(&self) -> Config;
}

pub trait Source: Send {
    fn run(&mut self, sink: Sender, feedback: &Feedback);
}
