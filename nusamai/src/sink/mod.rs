// pub mod cesiumtiles;
pub mod geojson;
pub mod geojson_transform_exp;
pub mod gpkg;
pub mod mvt;
pub mod noop;
pub mod serde;

use nusamai_citygml::schema::Schema;

use crate::parameters::Parameters;
use crate::pipeline::{Feedback, Receiver};
use crate::transformer;

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
    fn run(&mut self, upstream: Receiver, feedback: &Feedback, schema: &Schema);

    /// Make a transform requirements
    fn make_transform_requirements(&self) -> transformer::Requirements;
}
