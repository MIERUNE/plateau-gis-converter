//! External sorting shared by vector tile sinks.

use std::{convert::Infallible, sync::mpsc};

use itertools::Itertools;

use crate::pipeline::{Feedback, PipelineError, Result};

pub(crate) type SerializedFeature = (u64, Vec<u8>);
pub(crate) type SortedTileFeatures = (u64, Vec<Vec<u8>>);

pub(crate) fn feature_sorting_stage(
    feedback: &Feedback,
    receiver_sliced: mpsc::Receiver<SerializedFeature>,
    sender_sorted: mpsc::SyncSender<SortedTileFeatures>,
) -> Result<()> {
    let config = kv_extsort::SortConfig::default()
        .max_chunk_bytes(256 * 1024 * 1024)
        .set_cancel_flag(feedback.get_cancellation_flag());

    let sorted_iter = kv_extsort::sort(
        receiver_sliced
            .into_iter()
            .map(|(tile_id, body)| std::result::Result::<_, Infallible>::Ok((tile_id, body))),
        config,
    );

    for ((_, tile_id), grouped) in &sorted_iter.chunk_by(|feature| match feature {
        Ok((tile_id, _)) => (false, *tile_id),
        Err(_) => (true, 0),
    }) {
        let grouped = grouped
            .into_iter()
            .map_ok(|(_, serialized_feature)| serialized_feature)
            .collect::<kv_extsort::Result<Vec<_>, _>>();
        match grouped {
            Ok(serialized_features) => {
                feedback.ensure_not_canceled()?;
                if sender_sorted.send((tile_id, serialized_features)).is_err() {
                    return Err(PipelineError::Canceled);
                }
            }
            Err(kv_extsort::Error::Canceled) => return Err(PipelineError::Canceled),
            Err(error) => {
                return Err(PipelineError::Other(format!(
                    "Failed to sort features: {error:?}"
                )));
            }
        }
    }

    Ok(())
}
