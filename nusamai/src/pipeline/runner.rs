use std::{
    sync::{mpsc::sync_channel, Arc},
    thread,
};

use nusamai_citygml::schema::Schema;
use rayon::ThreadPoolBuilder;

use super::{
    feedback::{watcher, Feedback, Watcher},
    Canceller,
};
use crate::{
    pipeline::PipelineError, pipeline::Receiver, sink::DataSink, source::DataSource,
    transformer::Transformer,
};

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
        feedback.info("Source thread started.".into());
        let num_threads = std::thread::available_parallelism()
            .map(|v| v.get() * 3)
            .unwrap_or(1);
        let pool = ThreadPoolBuilder::new()
            .use_current_thread()
            .num_threads(num_threads)
            .build()
            .unwrap();
        let feedback2 = feedback.component_span(super::SourceComponent::Source);
        pool.install(move || {
            if let Err(error) = source.run(sender, &feedback2) {
                feedback2.fatal_error(error);
            }
        });
        feedback.info("Source thread finished.".into());
    });
    (handle, receiver)
}

fn spawn_transformer_thread(
    transformer: Box<dyn Transformer>,
    upstream: Receiver,
    feedback: Feedback,
) -> (std::thread::JoinHandle<()>, Receiver) {
    let (sender, receiver) = sync_channel(TRANSFORMER_OUTPUT_CHANNEL_BOUND);
    let main_thread_feedback = feedback.component_span(super::SourceComponent::Transformer);
    let handle = spawn_thread("pipeline-transformer".to_string(), move || {
        feedback.info("Transformer thread started.".into());
        let pool = ThreadPoolBuilder::new()
            .use_current_thread()
            .build()
            .unwrap();
        let child_thread_feedback = feedback.component_span(super::SourceComponent::Transformer);
        pool.install(move || {
            if let Err(error) = transformer.run(upstream, sender, &child_thread_feedback) {
                child_thread_feedback.fatal_error(error);
            }
        });
        feedback.info("Transformer thread finished.".into());
    });

    // Report an error if the converted data is empty.
    if let Err(error) = receiver.recv() {
        main_thread_feedback.fatal_error(PipelineError::Other(format!(
            "Transformer thread failed to receive data due to: {}",
            error
        )));
    }

    (handle, receiver)
}

fn spawn_sink_thread(
    mut sink: Box<dyn DataSink>,
    schema: Arc<Schema>,
    upstream: Receiver,
    feedback: Feedback,
) -> std::thread::JoinHandle<()> {
    spawn_thread("pipeline-sink".to_string(), move || {
        feedback.info("Sink thread started.".into());
        let num_threads = std::thread::available_parallelism()
            .map(|v| v.get() * 3)
            .unwrap_or(1);
        let pool = ThreadPoolBuilder::new()
            .use_current_thread()
            .num_threads(num_threads)
            .build()
            .unwrap();
        let feedback2 = feedback.component_span(super::SourceComponent::Sink);
        pool.install(move || {
            if let Err(error) = sink.run(upstream, &feedback2, &schema) {
                feedback2.fatal_error(error);
            }
        });
        feedback.info("Sink thread finished.".into());
    })
}

pub struct PipelineHandle {
    source_thread_handle: std::thread::JoinHandle<()>,
    transformer_thread_handle: std::thread::JoinHandle<()>,
    sink_thread_handle: std::thread::JoinHandle<()>,
}

impl PipelineHandle {
    // Wait for the pipeline to terminate
    pub fn join(self) -> Result<(), String> {
        fn report_error(stage: &str, err: Box<dyn std::any::Any + Send>) -> String {
            let msg = if let Some(message) = err.downcast_ref::<&str>() {
                format!("{stage} thread panicked with message: {}", message)
            } else if let Some(string) = err.downcast_ref::<String>() {
                format!("{stage} thread panicked with message: {}", string)
            } else {
                format!("{stage} thread panicked with an unknown type.")
            };
            log::error!("{}", msg);
            msg.to_string()
        }

        if let Err(err) = self.source_thread_handle.join() {
            return Err(report_error("Source", err));
        }
        if let Err(err) = self.transformer_thread_handle.join() {
            return Err(report_error("Transformer", err));
        }
        if let Err(err) = self.sink_thread_handle.join() {
            return Err(report_error("Sink", err));
        }
        Ok(())
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
    let (source_thread_handle, source_receiver) = spawn_source_thread(source, feedback.clone());
    let (transformer_thread_handle, transformer_receiver) =
        spawn_transformer_thread(transformer, source_receiver, feedback.clone());
    let sink_thread_handle = spawn_sink_thread(sink, schema, transformer_receiver, feedback);

    let handle = PipelineHandle {
        source_thread_handle,
        transformer_thread_handle,
        sink_thread_handle,
    };
    (handle, watcher, canceller)
}
