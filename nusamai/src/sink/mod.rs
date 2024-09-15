//! Output format drivers (sinks)

pub mod cesiumtiles;
pub mod czml;
pub mod geojson;
pub mod gltf;
pub mod gpkg;
pub mod kml;
pub mod minecraft;
pub mod mvt;
pub mod noop;
pub mod obj;
pub mod option;
pub mod ply;
pub mod serde;
pub mod shapefile;
mod texture_resolution;

use nusamai_citygml::schema::Schema;
use nusamai_projection::crs;

use crate::{
    parameters::Parameters,
    pipeline::{Feedback, PipelineError, Receiver},
    transformer::{self, TransformerRegistry},
};

pub struct SinkInfo {
    pub id_name: String,
    pub name: String,
}

pub trait DataSinkProvider: Sync {
    /// Gets basic information about the sink.
    fn info(&self) -> SinkInfo;

    /// Gets the configurable parameters of the sink.
    fn sink_options(&self) -> Parameters;

    /// Gets the transform options of the sink.
    fn transformer_options(&self) -> TransformerRegistry;

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

    /// Make a transform requirements with options
    fn make_requirements(&mut self, property: TransformerRegistry) -> DataRequirements;
}

pub struct DataRequirements {
    pub output_epsg: crs::EpsgCode,
    /// Whether to shorten field names to 10 characters or less for Shapefiles.
    pub shorten_names_for_shapefile: bool,
    pub tree_flattening: transformer::TreeFlatteningSpec,
    /// Whether to use appearance information (if false, the pipeline can skip the appearance parsing)
    pub use_appearance: bool,
    /// Whether to bind appearance information to the geometry
    pub resolve_appearance: bool,
    pub mergedown: transformer::MergedownSpec,
    pub key_value: transformer::KeyValueSpec,
    pub lod_filter: transformer::LodFilterSpec,
    pub geom_stats: transformer::GeometryStatsSpec,
}

impl Default for DataRequirements {
    fn default() -> Self {
        Self {
            output_epsg: crs::EPSG_WGS84_GEOGRAPHIC_3D,
            shorten_names_for_shapefile: false,
            tree_flattening: transformer::TreeFlatteningSpec::None,
            use_appearance: false,
            resolve_appearance: false,
            mergedown: transformer::MergedownSpec::RemoveDescendantFeatures,
            key_value: transformer::KeyValueSpec::JsonifyObjectsAndArrays,
            lod_filter: transformer::LodFilterSpec::default(),
            geom_stats: transformer::GeometryStatsSpec::None,
        }
    }
}

impl DataRequirements {
    pub fn set_output_epsg(&mut self, epsg: crs::EpsgCode) {
        self.output_epsg = epsg;
    }

    pub fn set_appearance(&mut self, use_appearance: bool) {
        self.use_appearance = use_appearance;
    }

    pub fn set_resolve_appearance(&mut self, resolve_appearance: bool) {
        self.resolve_appearance = resolve_appearance;
    }

    pub fn set_lod_filter(&mut self, lod_filter: transformer::LodFilterSpec) {
        self.lod_filter = lod_filter;
    }
}
