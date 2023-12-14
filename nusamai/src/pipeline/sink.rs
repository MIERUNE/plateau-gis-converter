use crate::pipeline::{Feedback, Percel};
use crate::Config;

pub struct SinkInfo {
    pub name: String,
}

pub trait SinkProvider {
    /// Creates a sink instance.
    fn create(&self) -> Box<dyn Sink>;

    /// Gets basic information about the sink.
    fn info(&self) -> SinkInfo;

    /// Gets the configuration for the sink.
    fn configuration(&self) -> Config;
}

pub trait Sink: Send {
    fn feed(&mut self, obj: Percel, feedback: &mut Feedback);
}
