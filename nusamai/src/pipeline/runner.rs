use std::sync::mpsc::sync_channel;

use rayon::prelude::*;

use super::{
    feedback::{watcher, Feedback, Watcher},
    Canceller,
};
use crate::pipeline::{Receiver, Sink, Source, Transformer};

const SOURCE_OUTPUT_CHANNEL_BOUND: usize = 10000;
const TRANSFORMER_OUTPUT_CHANNEL_BOUND: usize = 10000;

fn start_source_thread(
    mut source: Box<dyn Source>,
    feedback: Feedback,
) -> (std::thread::JoinHandle<()>, Receiver) {
    let (sender, receiver) = sync_channel(SOURCE_OUTPUT_CHANNEL_BOUND);
    let handle = std::thread::spawn(move || {
        source.run(sender, &feedback);
    });
    (handle, receiver)
}

fn start_transformer_thread(
    transformer: Box<dyn Transformer>,
    upstream_receiver: Receiver,
    feedback: Feedback,
) -> (std::thread::JoinHandle<()>, Receiver) {
    let (sender, receiver) = sync_channel(TRANSFORMER_OUTPUT_CHANNEL_BOUND);
    let handle = std::thread::spawn(move || {
        let _ = upstream_receiver
            .into_iter()
            .par_bridge()
            .try_for_each(|mut obj| {
                transformer.transform(&mut obj, &feedback);
                if sender.send(obj).is_err() || feedback.is_cancelled() {
                    Err(())
                } else {
                    Ok(())
                }
            });
    });
    (handle, receiver)
}

fn start_sink_thread(
    mut sink: Box<dyn Sink>,
    receiver: Receiver,
    mut feedback: Feedback,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        for obj in receiver {
            if feedback.is_cancelled() {
                break;
            }
            sink.feed(obj, &mut feedback);
        }
    })
}

pub struct PipelineHandle {
    source_thread: std::thread::JoinHandle<()>,
    transformer_thread: std::thread::JoinHandle<()>,
    sink_thread: std::thread::JoinHandle<()>,
}

impl PipelineHandle {
    // Wait for the pipeline to finish
    pub fn join(self) {
        self.source_thread.join().unwrap();
        self.transformer_thread.join().unwrap();
        self.sink_thread.join().unwrap();
    }
}

/// Run the pipeline
///
/// `[Source] ==> [Transformer] ==> [Sink]`
pub fn run(
    source: Box<dyn Source>,
    transformer: Box<dyn Transformer>,
    sink: Box<dyn Sink>,
) -> (PipelineHandle, Watcher, Canceller) {
    let (watcher, feedback, canceller) = watcher();

    // Start the pipeline
    let (source_thread, source_receiver) = start_source_thread(source, feedback.clone());
    let (transformer_thread, transformer_receiver) =
        start_transformer_thread(transformer, source_receiver, feedback.clone());
    let sink_thread = start_sink_thread(sink, transformer_receiver, feedback.clone());

    let handle = PipelineHandle {
        source_thread,
        transformer_thread,
        sink_thread,
    };
    (handle, watcher, canceller)
}
