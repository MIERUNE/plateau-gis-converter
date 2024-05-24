//! Feedback messages from the pipeline components.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::PipelineError;

const FEEDBACK_CHANNEL_BOUND: usize = 10000;

#[derive(Debug)]
pub struct Message {
    /// Log message body
    pub message: String,
    /// Log level
    pub level: log::Level,
    /// Message source (source, transformer, sink, pipeline, etc.)
    pub source_component: SourceComponent,
    pub error: Option<PipelineError>,
    // progress:
    // etc.
}

#[derive(Clone)]
pub struct Feedback {
    canceled: Arc<AtomicBool>,
    source_component: SourceComponent,
    sender: std::sync::mpsc::SyncSender<Message>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceComponent {
    Source,
    Transformer,
    Sink,
    Pipeline,
}

impl std::fmt::Display for SourceComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceComponent::Source => write!(f, "source"),
            SourceComponent::Transformer => write!(f, "transformer"),
            SourceComponent::Sink => write!(f, "sink"),
            SourceComponent::Pipeline => write!(f, "pipeline"),
        }
    }
}

impl Feedback {
    /// Checks if the pipeline is requested to be canceled
    #[inline]
    pub fn is_canceled(&self) -> bool {
        self.canceled.load(Ordering::Relaxed)
    }

    /// Returns an error if the pipeline is requested to be canceled
    #[inline]
    pub fn ensure_not_canceled(&self) -> Result<(), PipelineError> {
        if self.canceled.load(Ordering::Relaxed) {
            Err(PipelineError::Canceled)
        } else {
            Ok(())
        }
    }

    /// Request the pipeline to be canceled
    #[inline]
    pub fn cancel(&self) {
        self.canceled.store(true, Ordering::Relaxed)
    }

    /// Get the internal cancellation flag
    #[inline]
    pub fn get_cancellation_flag(&self) -> Arc<AtomicBool> {
        self.canceled.clone()
    }

    /// Create a new feedback span for the pipeline component
    #[inline]
    pub fn component_span(&self, source: SourceComponent) -> Self {
        Self {
            source_component: source,
            ..self.clone()
        }
    }

    /// Send a message to the feedback channel
    #[inline]
    pub fn send_raw_message(&self, msg: Message) {
        // don't care if the receiver is dropped.
        let _ = self.sender.send(msg);
    }

    #[inline]
    pub fn send_message(&self, message: String, level: log::Level) {
        self.send_raw_message(Message {
            message,
            level,
            source_component: self.source_component,
            error: None,
        })
    }

    /// Send a debug log
    #[inline]
    pub fn debug(&self, message: String) {
        self.send_message(message, log::Level::Debug)
    }

    /// Send a info log
    #[inline]
    pub fn info(&self, message: String) {
        self.send_message(message, log::Level::Info)
    }

    /// Send a warning log
    #[inline]
    pub fn warn(&self, message: String) {
        self.send_message(message, log::Level::Warn)
    }

    /// Send an error log
    #[inline]
    pub fn error(&self, message: String) {
        self.send_message(message, log::Level::Error)
    }

    /// Report a fatal error and cancel the pipeline
    #[inline]
    pub fn fatal_error(&self, error: PipelineError) {
        match error {
            PipelineError::Canceled => {
                // do nothing
            }
            _ => {
                self.cancel();
                let _ = self.sender.send(Message {
                    message: "Fatal error".to_string(),
                    level: log::Level::Error,
                    source_component: self.source_component,
                    error: Some(error),
                });
            }
        }
    }
}

pub struct Watcher {
    receiver: std::sync::mpsc::Receiver<Message>,
}

impl IntoIterator for Watcher {
    type Item = Message;
    type IntoIter = std::sync::mpsc::IntoIter<Message>;

    fn into_iter(self) -> Self::IntoIter {
        self.receiver.into_iter()
    }
}

#[derive(Clone, Default)]
pub struct Canceller {
    canceled: Arc<AtomicBool>,
}

impl Canceller {
    /// Cancel the pipeline
    pub fn cancel(&self) {
        self.canceled.store(true, Ordering::Relaxed);
    }

    /// Checks if the pipeline is canceled
    pub fn is_canceled(&self) -> bool {
        self.canceled.load(Ordering::Relaxed)
    }
}

pub(crate) fn watcher() -> (Watcher, Feedback, Canceller) {
    let canceled = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = std::sync::mpsc::sync_channel(FEEDBACK_CHANNEL_BOUND);
    let watcher = Watcher { receiver };
    let canceller = Canceller {
        canceled: canceled.clone(),
    };
    let feedback = Feedback {
        canceled: canceled.clone(),
        source_component: SourceComponent::Pipeline,
        sender,
    };
    (watcher, feedback, canceller)
}
