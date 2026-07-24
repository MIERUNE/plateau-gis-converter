//! Shared processing primitives for vector tile sinks.

use nusamai_citygml::schema::Schema;
use nusamai_projection::crs::EPSG_WEB_MERCATOR;

use crate::pipeline::{PipelineError, Result};

pub(crate) mod feature;
pub(crate) mod geometry;
pub(crate) mod pipeline;
pub(crate) mod slice;
pub(crate) mod sort;
mod tile_grid;
pub(crate) mod tile_id;

pub(crate) use pipeline::{
    generate_stage, report_final_tile_count, report_tile_progress, run_vector_tile_pipeline,
    slice_stage, EncodedTile, TileEncoder, TilePipelineOptions, DEFAULT_MAX_COMPRESSED_TILE_SIZE,
    FEATURE_CHANNEL_CAPACITY,
};
pub(crate) use sort::feature_sorting_stage;

pub(crate) fn validate_vector_tile_schema_crs(schema: &Schema) -> Result<()> {
    match schema.epsg {
        Some(EPSG_WEB_MERCATOR) => Ok(()),
        Some(actual) => Err(PipelineError::Other(format!(
            "Vector tile sink requires schema CRS EPSG:{EPSG_WEB_MERCATOR}, but found EPSG:{actual}"
        ))),
        None => Err(PipelineError::Other(format!(
            "Vector tile sink requires schema CRS EPSG:{EPSG_WEB_MERCATOR}, but the CRS is missing"
        ))),
    }
}
