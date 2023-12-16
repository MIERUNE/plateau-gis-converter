use crate::configuration::Config;
use crate::pipeline::{Feedback, Percel};

pub struct SinkInfo {
    pub name: String,
}

pub trait SinkProvider {
    /// Creates a sink instance.
    fn create(&self, config: &Config) -> Box<dyn Sink>;

    /// Gets basic information about the sink.
    fn info(&self) -> SinkInfo;

    /// Gets the configurable parameters of the sink.
    fn config(&self) -> Config;
}

pub trait Sink: Send {
    fn receive(&mut self, percel: Percel, feedback: &mut Feedback);
    fn finalize(&mut self, feedback: &mut Feedback);
}
