//! Output format drivers (sinks)

pub mod cesiumtiles;
pub mod czml;
pub mod geojson;
pub mod gltf;
pub mod gpkg;
pub mod kml;
pub mod mvt;
pub mod noop;
pub mod ply;
pub mod serde;
pub mod shapefile;

use nusamai_citygml::schema::Schema;

use crate::parameters::Parameters;
use crate::pipeline::{Feedback, PipelineError, Receiver};
use crate::transformer;

pub struct SinkInfo {
    pub id_name: String,
    pub name: String,
}

pub trait DataSinkProvider: Sync {
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
