pub mod geojson;
pub mod gpkg;
pub mod noop;
pub mod serde;
pub mod tiling2d;

use crate::configuration::Config;
use crate::pipeline::{Feedback, Receiver};

pub struct SinkInfo {
    pub name: String,
}

pub trait DataSinkProvider {
    /// Creates a sink instance.
    fn create(&self, config: &Config) -> Box<dyn DataSink>;

    /// Gets basic information about the sink.
    fn info(&self) -> SinkInfo;

    /// Gets the configurable parameters of the sink.
    fn config(&self) -> Config;
}

pub trait DataSink: Send {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback);
}
