use std::sync::{mpsc::sync_channel, Arc};
use std::thread;

use nusamai_citygml::schema::Schema;
use rayon::ThreadPoolBuilder;

use super::{
    feedback::{watcher, Feedback, Watcher},
    Canceller,
};
use crate::sink::DataSink;
use crate::source::DataSource;
use crate::{pipeline::Receiver, transformer::Transformer};

const SOURCE_OUTPUT_CHANNEL_BOUND: usize = 10000;
const TRANSFORMER_OUTPUT_CHANNEL_BOUND: usize = 10000;

fn spawn_thread<F, T>(name: String, f: F) -> std::thread::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new()
        .name(name)
        .spawn(f)
        .expect("Failed to spawn thread")
}

fn spawn_source_thread(
    mut source: Box<dyn DataSource>,
    feedback: Feedback,
) -> (std::thread::JoinHandle<()>, Receiver) {
    let (sender, receiver) = sync_channel(SOURCE_OUTPUT_CHANNEL_BOUND);
    let handle = spawn_thread("pipeline-source".to_string(), move || {
        log::info!("Source thread started.");
        let num_threads = std::thread::available_parallelism()
            .map(|v| v.get() * 3)
            .unwrap_or(1);
        let pool = ThreadPoolBuilder::new()
            .use_current_thread()
            .num_threads(num_threads)
            .build()
            .unwrap();
        pool.install(move || {
            if let Err(error) = source.run(sender, &feedback) {
                feedback.fatal_error(error);
            }
        });
        log::info!("Source thread finished.");
    });
    (handle, receiver)
}

fn spawn_transformer_thread(
    transformer: Box<dyn Transformer>,
    upstream: Receiver,
    feedback: Feedback,
) -> (std::thread::JoinHandle<()>, Receiver) {
    let (sender, receiver) = sync_channel(TRANSFORMER_OUTPUT_CHANNEL_BOUND);

    let handle = spawn_thread("pipeline-transformer".to_string(), move || {
        log::info!("Transformer thread started.");
        let pool = ThreadPoolBuilder::new()
            .use_current_thread()
            .build()
            .unwrap();
        pool.install(move || {
            if let Err(error) = transformer.run(upstream, sender, &feedback) {
                feedback.fatal_error(error);
            }
        });
        log::info!("Transformer thread finished.");
    });
    (handle, receiver)
}

fn spawn_sink_thread(
    mut sink: Box<dyn DataSink>,
    schema: Arc<Schema>,
    upstream: Receiver,
    feedback: Feedback,
) -> std::thread::JoinHandle<()> {
    spawn_thread("pipeline-sink".to_string(), move || {
        log::info!("Sink thread started.");
        let num_threads = std::thread::available_parallelism()
            .map(|v| v.get() * 3)
            .unwrap_or(1);
        let pool = ThreadPoolBuilder::new()
            .use_current_thread()
            .num_threads(num_threads)
            .build()
            .unwrap();
        pool.install(move || {
            if let Err(error) = sink.run(upstream, &feedback, &schema) {
                feedback.fatal_error(error);
            }
        });
        log::info!("Sink thread finished.");
    })
}

pub struct PipelineHandle {
    source_thread_handle: std::thread::JoinHandle<()>,
    transformer_thread_handle: std::thread::JoinHandle<()>,
    sink_thread_handle: std::thread::JoinHandle<()>,
}

impl PipelineHandle {
    // Wait for the pipeline to terminate
    pub fn join(self) {
        if self.source_thread_handle.join().is_err() {
            log::error!("Source thread panicked");
        }
        if self.transformer_thread_handle.join().is_err() {
            log::error!("Transformer thread panicked");
        }
        if self.sink_thread_handle.join().is_err() {
            log::error!("Sink thread panicked");
        }
    }
}

/// Run the pipeline
///
/// `[Source] ==> [Transformer] ==> [Sink]`
pub fn run(
    source: Box<dyn DataSource>,
    transformer: Box<dyn Transformer>,
    sink: Box<dyn DataSink>,
    schema: Arc<Schema>,
) -> (PipelineHandle, Watcher, Canceller) {
    let (watcher, feedback, canceller) = watcher();

    // Start the pipeline
    let (source_thread_handle, source_receiver) = spawn_source_thread(
        source,
        feedback.component_span(super::SourceComponent::Source),
    );
    let (transformer_thread_handle, transformer_receiver) = spawn_transformer_thread(
        transformer,
        source_receiver,
        feedback.component_span(super::SourceComponent::Transformer),
    );
    let sink_thread_handle = spawn_sink_thread(
        sink,
        schema,
        transformer_receiver,
        feedback.component_span(super::SourceComponent::Sink),
    );

    let handle = PipelineHandle {
        source_thread_handle,
        transformer_thread_handle,
        sink_thread_handle,
    };
    (handle, watcher, canceller)
}
