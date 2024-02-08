//! Output format drivers (sinks)

pub mod cesiumtiles;
pub mod czml;
pub mod geojson;
pub mod geojson_transform_exp;
pub mod gltf_poc;
pub mod gpkg;
pub mod mvt;
pub mod noop;
pub mod ply;
pub mod serde;
pub mod shapefile;
pub mod kml;

use nusamai_citygml::schema::Schema;

use crate::parameters::Parameters;
use crate::pipeline::{Feedback, PipelineError, Receiver};
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
    /// Start the sink process
    fn run(
        &mut self,
        upstream: Receiver,
        feedback: &Feedback,
        schema: &Schema,
    ) -> Result<(), PipelineError>;

    /// Make a transform requirements
    fn make_transform_requirements(&self) -> transformer::Requirements;
}
