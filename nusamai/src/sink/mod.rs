pub mod geojson;
pub mod gpkg;
pub mod noop;
pub mod serde;
pub mod tiling2d;

use crate::parameters::Parameters;
use crate::pipeline::{Feedback, Receiver};

pub struct SinkInfo {
    pub name: String,
}

pub trait DataSinkProvider {
    /// Gets basic information about the sink.
    fn info(&self) -> SinkInfo;

    /// Gets the configurable parameters of the sink.
    fn parameters(&self) -> Parameters;

    /// Creates a sink instance.
    fn create(&self, config: &Parameters) -> Box<dyn DataSink>;
}

pub trait DataSink: Send {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback);
}
