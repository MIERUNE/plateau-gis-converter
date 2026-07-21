//! Shared slicing, sorting, and tile generation orchestration.

use std::{fs, io::Write, path::Path, sync::mpsc};

use flate2::{write::ZlibEncoder, Compression};
use rayon::prelude::*;

use crate::pipeline::{Feedback, PipelineError, Receiver, Result};

use super::{
    model::SlicedFeature,
    slice::slice_cityobj_geoms,
    sort::{feature_sorting_stage, SerializedFeature, SortedTileFeatures},
    tile_id::TileIdMethod,
};

pub(crate) const FEATURE_CHANNEL_CAPACITY: usize = 2_000;
const DEFAULT_DETAIL: i32 = 12;
const MINIMUM_DETAIL: i32 = 9;
const SLICE_BUFFER_PIXELS: u32 = 5;
const FEATURE_WARNING_THRESHOLD: usize = 200_000;

pub(crate) const DEFAULT_MAX_COMPRESSED_TILE_SIZE: usize = 500_000;

#[derive(Clone, Copy)]
pub(crate) struct TilePipelineOptions {
    pub(crate) min_z: u8,
    pub(crate) max_z: u8,
    pub(crate) max_compressed_tile_size: usize,
}

pub(crate) trait TileEncoder: Sync {
    fn encode_tile(&self, detail: i32, serialized_features: &[Vec<u8>]) -> Result<Vec<u8>>;
}

pub(crate) struct EncodedTile {
    pub(crate) tile_id: u64,
    pub(crate) zxy: (u8, u32, u32),
    pub(crate) bytes: Vec<u8>,
    pub(crate) zlib_size: usize,
}

pub(crate) fn slice_stage(
    feedback: &Feedback,
    upstream: Receiver,
    tile_id_method: TileIdMethod,
    sender_sliced: mpsc::SyncSender<SerializedFeature>,
    options: TilePipelineOptions,
) -> Result<()> {
    let bincode_config = bincode::config::standard();

    upstream.into_iter().par_bridge().try_for_each(|parcel| {
        feedback.ensure_not_canceled()?;
        slice_cityobj_geoms(
            &parcel.entity,
            options.min_z,
            options.max_z,
            DEFAULT_DETAIL as u32,
            SLICE_BUFFER_PIXELS,
            |(z, x, y), multipolygon| {
                feedback.ensure_not_canceled()?;

                let feature = SlicedFeature {
                    geometry: multipolygon,
                    properties: parcel.entity.root.clone(),
                };
                let bytes = bincode::serde::encode_to_vec(&feature, bincode_config).unwrap();
                let tile_id = tile_id_method.zxy_to_id(z, x, y);
                sender_sliced
                    .send((tile_id, bytes))
                    .map_err(|_| PipelineError::Canceled)
            },
        )
    })
}

pub(crate) fn generate_stage<E, C>(
    feedback: &Feedback,
    receiver_sorted: mpsc::Receiver<SortedTileFeatures>,
    tile_id_method: TileIdMethod,
    max_compressed_tile_size: usize,
    encoder: &E,
    consume_tile: C,
) -> Result<()>
where
    E: TileEncoder,
    C: Fn(EncodedTile) -> Result<()> + Sync,
{
    receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|(tile_id, serialized_features)| {
            feedback.ensure_not_canceled()?;
            let zxy = tile_id_method.id_to_zxy(tile_id);

            if serialized_features.len() > FEATURE_WARNING_THRESHOLD {
                feedback.warn(format!(
                    "Too many features in a tile ({} features)",
                    serialized_features.len()
                ));
            }

            for detail in (MINIMUM_DETAIL..=DEFAULT_DETAIL).rev() {
                feedback.ensure_not_canceled()?;
                let bytes = encoder.encode_tile(detail, &serialized_features)?;
                let zlib_size = compressed_size(&bytes)?;

                if detail != MINIMUM_DETAIL && zlib_size > max_compressed_tile_size {
                    let (zoom, x, y) = zxy;
                    let extent = 1 << detail;
                    feedback.info(format!(
                        "Tile size is too large: {zoom}/{x}/{y} (extent: {extent}), trying a \
                         lower detail level."
                    ));
                    continue;
                }

                consume_tile(EncodedTile {
                    tile_id,
                    zxy,
                    bytes,
                    zlib_size,
                })?;
                break;
            }

            Ok::<(), PipelineError>(())
        })
}

/// Runs the standard vector tile pipeline and writes each tile to `{z}/{x}/{y}.{extension}`.
pub(crate) fn run_vector_tile_pipeline<E>(
    output_path: &Path,
    extension: &str,
    upstream: Receiver,
    feedback: &Feedback,
    options: TilePipelineOptions,
    encoder: &E,
) -> Result<()>
where
    E: TileEncoder,
{
    let (sender_sliced, receiver_sliced) = mpsc::sync_channel(FEATURE_CHANNEL_CAPACITY);
    let (sender_sorted, receiver_sorted) = mpsc::sync_channel(FEATURE_CHANNEL_CAPACITY);
    let tile_id_method = TileIdMethod::Hilbert;

    std::thread::scope(|scope| {
        scope.spawn(move || {
            if let Err(error) =
                slice_stage(feedback, upstream, tile_id_method, sender_sliced, options)
            {
                feedback.fatal_error(error);
            }
        });

        scope.spawn(move || {
            if let Err(error) = feature_sorting_stage(feedback, receiver_sliced, sender_sorted) {
                feedback.fatal_error(error);
            }
        });

        scope.spawn(move || {
            let pool = rayon::ThreadPoolBuilder::new()
                .use_current_thread()
                .build()
                .unwrap();
            pool.install(|| {
                if let Err(error) = generate_stage(
                    feedback,
                    receiver_sorted,
                    tile_id_method,
                    options.max_compressed_tile_size,
                    encoder,
                    |tile| write_vector_tile(output_path, extension, feedback, tile),
                ) {
                    feedback.fatal_error(error);
                }
            });
        });
    });

    Ok(())
}

fn compressed_size(bytes: &[u8]) -> Result<usize> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes)?;
    Ok(encoder.finish()?.len())
}

fn write_vector_tile(
    output_path: &Path,
    extension: &str,
    feedback: &Feedback,
    tile: EncodedTile,
) -> Result<()> {
    let (zoom, x, y) = tile.zxy;
    let path = output_path.join(format!("{zoom}/{x}/{y}.{extension}"));
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory)?;
    }

    feedback.info(format!(
        "Writing a tile: {} ({} bytes, {} compressed)",
        path.to_string_lossy(),
        bytesize::ByteSize(tile.bytes.len() as u64),
        bytesize::ByteSize(tile.zlib_size as u64),
    ));
    fs::write(path, tile.bytes)?;
    Ok(())
}
