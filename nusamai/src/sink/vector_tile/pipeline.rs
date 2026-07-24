//! Shared slicing, sorting, and tile generation orchestration.

use std::{
    fs,
    io::Write,
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
};

use flate2::{write::ZlibEncoder, Compression};
use rayon::prelude::*;

use crate::pipeline::{Feedback, PipelineError, Receiver, Result};

use super::{
    feature::SlicedFeature,
    slice::slice_cityobj_geoms,
    sort::{feature_sorting_stage, SerializedFeature, SortedTileFeatures},
    tile_id::TileIdMethod,
};

pub(crate) const FEATURE_CHANNEL_CAPACITY: usize = 2_000;
const DEFAULT_DETAIL: i32 = 12;
const MINIMUM_DETAIL: i32 = 9;
const SLICE_BUFFER_PIXELS: u32 = 5;
const FEATURE_WARNING_THRESHOLD: usize = 200_000;
const TILE_PROGRESS_INTERVAL: u64 = 10_000;

pub(crate) const DEFAULT_MAX_COMPRESSED_TILE_SIZE: usize = 500_000;

#[derive(Clone, Copy)]
pub(crate) struct TilePipelineOptions {
    pub(crate) min_z: u8,
    pub(crate) max_z: u8,
    pub(crate) max_compressed_tile_size: usize,
}

pub(crate) trait TileEncoder: Sync {
    fn encode_tile(&self, detail: i32, serialized_features: &[Vec<u8>]) -> Result<Option<Vec<u8>>>;
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
            |(z, x, y), geometry| {
                feedback.ensure_not_canceled()?;

                let feature = SlicedFeature {
                    geometry,
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
    let has_generated_tile = receiver_sorted
        .into_iter()
        .par_bridge()
        .map(|(tile_id, serialized_features)| -> Result<bool> {
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
                let Some(bytes) = encoder.encode_tile(detail, &serialized_features)? else {
                    return Ok(false);
                };
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
                return Ok(true);
            }

            Ok(false)
        })
        .try_reduce(|| false, |generated, current| Ok(generated || current))?;

    if has_generated_tile {
        Ok(())
    } else {
        Err(PipelineError::Other(
            "No vector tiles could be generated from the input geometries".to_string(),
        ))
    }
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
            let written_tile_count = AtomicU64::new(0);
            let pool = rayon::ThreadPoolBuilder::new()
                .use_current_thread()
                .build()
                .unwrap();
            pool.install(|| {
                let result = generate_stage(
                    feedback,
                    receiver_sorted,
                    tile_id_method,
                    options.max_compressed_tile_size,
                    encoder,
                    |tile| {
                        write_vector_tile(output_path, extension, tile)?;
                        let tile_count = written_tile_count.fetch_add(1, Ordering::Relaxed) + 1;
                        report_tile_progress(feedback, tile_count, "Written", "vector tile files");
                        Ok(())
                    },
                );
                match result {
                    Ok(()) => report_final_tile_count(
                        feedback,
                        written_tile_count.load(Ordering::Relaxed),
                        "writing",
                        "vector tile files",
                    ),
                    Err(error) => feedback.fatal_error(error),
                }
            });
        });
    });

    Ok(())
}

pub(crate) fn report_tile_progress(
    feedback: &Feedback,
    tile_count: u64,
    action: &str,
    progress_target: &str,
) {
    if tile_count.is_multiple_of(TILE_PROGRESS_INTERVAL) {
        feedback.info(format!("{action} {tile_count} {progress_target}"));
    }
}

pub(crate) fn report_final_tile_count(
    feedback: &Feedback,
    tile_count: u64,
    operation: &str,
    progress_target: &str,
) {
    feedback.info(format!(
        "Finished {operation} {tile_count} {progress_target}"
    ));
}

fn compressed_size(bytes: &[u8]) -> Result<usize> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes)?;
    Ok(encoder.finish()?.len())
}

fn write_vector_tile(output_path: &Path, extension: &str, tile: EncodedTile) -> Result<()> {
    let (zoom, x, y) = tile.zxy;
    let path = output_path.join(format!("{zoom}/{x}/{y}.{extension}"));
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory)?;
    }

    log::debug!(
        "Writing a tile: {} ({} bytes, {} compressed)",
        path.to_string_lossy(),
        bytesize::ByteSize(tile.bytes.len() as u64),
        bytesize::ByteSize(tile.zlib_size as u64),
    );
    fs::write(path, tile.bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::feedback::watcher;

    #[test]
    fn reports_tile_write_progress_every_ten_thousand_tiles() {
        let (watcher, feedback, _canceller) = watcher();

        for tile_count in [1, 9_999, 10_000, 10_001, 19_999, 20_000] {
            report_tile_progress(
                &feedback,
                tile_count,
                "Written",
                "tiles to test destination",
            );
        }
        drop(feedback);

        let messages = watcher
            .into_iter()
            .map(|message| (message.message, message.level))
            .collect::<Vec<_>>();
        assert_eq!(
            messages,
            [
                (
                    "Written 10000 tiles to test destination".to_string(),
                    log::Level::Info
                ),
                (
                    "Written 20000 tiles to test destination".to_string(),
                    log::Level::Info
                ),
            ]
        );
    }

    #[test]
    fn reports_tile_generation_progress_every_ten_thousand_tiles() {
        let (watcher, feedback, _canceller) = watcher();

        for tile_count in [9_999, 10_000, 10_001] {
            report_tile_progress(
                &feedback,
                tile_count,
                "Generated",
                "tiles for PMTiles archive",
            );
        }
        drop(feedback);

        let messages = watcher
            .into_iter()
            .map(|message| (message.message, message.level))
            .collect::<Vec<_>>();
        assert_eq!(
            messages,
            [(
                "Generated 10000 tiles for PMTiles archive".to_string(),
                log::Level::Info
            )]
        );
    }

    #[test]
    fn reports_final_tile_count_below_progress_interval() {
        let (watcher, feedback, _canceller) = watcher();

        report_final_tile_count(&feedback, 1_234, "writing", "vector tile files");
        drop(feedback);

        let messages = watcher
            .into_iter()
            .map(|message| (message.message, message.level))
            .collect::<Vec<_>>();
        assert_eq!(
            messages,
            [(
                "Finished writing 1234 vector tile files".to_string(),
                log::Level::Info
            )]
        );
    }
}
