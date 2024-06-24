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
pub mod ply;
pub mod serde;
pub mod shapefile;

use nusamai_citygml::schema::Schema;
use nusamai_projection::crs;

use crate::{
    parameters::Parameters,
    pipeline::{Feedback, PipelineError, Receiver},
    transformer,
    transformoption::TransformOptions,
};

use ::serde::Deserialize;
use ::serde::Serialize;

pub struct SinkInfo {
    pub id_name: String,
    pub name: String,
}

pub trait DataSinkProvider: Sync {
    /// Gets basic information about the sink.
    fn info(&self) -> SinkInfo;

    /// Gets the configurable parameters of the sink.
    fn parameters(&self) -> Parameters;

    /// Gets the transform options of the sink.
    fn transform_options(&self) -> TransformOptions;

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
    fn make_requirements(&self, property: Vec<SetOptionProperty>) -> DataRequirements;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataRequirementsField {
    OutputEpsg(crs::EpsgCode),
    ShortenNamesForShapefile(bool),
    TreeFlattening(transformer::TreeFlatteningSpec),
    UseAppearance(bool),
    ResolveAppearance(bool),
    Mergedown(transformer::MergedownSpec),
    KeyValue(transformer::KeyValueSpec),
    LodFilter(transformer::LodFilterSpec),
    GeomStats(transformer::GeometryStatsSpec),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetOptionProperty {
    pub key: String,
    pub value: bool,
}

impl DataRequirements {
    pub fn set_output_epsg(&mut self, epsg: crs::EpsgCode) {
        self.output_epsg = epsg;
    }

    pub fn update_from_fields(&mut self, fields: Vec<DataRequirementsField>) {
        for field in fields {
            match field {
                DataRequirementsField::OutputEpsg(value) => self.output_epsg = value,
                DataRequirementsField::ShortenNamesForShapefile(value) => {
                    self.shorten_names_for_shapefile = value
                }
                DataRequirementsField::TreeFlattening(value) => self.tree_flattening = value,
                DataRequirementsField::UseAppearance(value) => self.use_appearance = value,
                DataRequirementsField::ResolveAppearance(value) => self.resolve_appearance = value,
                DataRequirementsField::Mergedown(value) => self.mergedown = value,
                DataRequirementsField::KeyValue(value) => self.key_value = value,
                DataRequirementsField::LodFilter(value) => self.lod_filter = value,
                DataRequirementsField::GeomStats(value) => self.geom_stats = value,
            }
        }
    }

    pub fn set_appearance(&mut self, use_appearance: bool, resolve_appearance: bool) {
        self.use_appearance = use_appearance;
        self.resolve_appearance = resolve_appearance;
    }
}
