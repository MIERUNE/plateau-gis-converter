//! Shared processing primitives for vector tile sinks.

pub(crate) mod feature;
pub(crate) mod geometry;
pub(crate) mod pipeline;
pub(crate) mod slice;
pub(crate) mod sort;
pub(crate) mod tile_id;

pub(crate) use pipeline::{
    generate_stage, run_vector_tile_pipeline, slice_stage, EncodedTile, TileEncoder,
    TilePipelineOptions, DEFAULT_MAX_COMPRESSED_TILE_SIZE, FEATURE_CHANNEL_CAPACITY,
};
pub(crate) use sort::feature_sorting_stage;
