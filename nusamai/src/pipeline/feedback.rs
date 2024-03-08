//! Feedback messages from the pipeline components.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::PipelineError;

const FEEDBACK_CHANNEL_BOUND: usize = 10000;

#[derive(Debug)]
pub struct FeedbackMessage {
    pub message: String,
    /// Message source (source, transformer, sink, etc.)
    pub source_component: FeedbackSourceComponent,
    pub error: Option<PipelineError>,
    // severity:
    // progress:
    // etc.
}

#[derive(Clone)]
pub struct Feedback {
    canceled: Arc<AtomicBool>,
    source_component: FeedbackSourceComponent,
    sender: std::sync::mpsc::SyncSender<FeedbackMessage>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedbackSourceComponent {
    Source,
    Transformer,
    Sink,
    Unknown,
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

    /// Create a new feedback span for the pipeline component
    #[inline]
    pub fn component_span(&self, source: FeedbackSourceComponent) -> Self {
        Self {
            source_component: source,
            ..self.clone()
        }
    }

    /// Send a message to the feedback channel
    #[inline]
    pub fn feedback_raw(&self, msg: FeedbackMessage) {
        // don't care if the receiver is dropped.
        let _ = self.sender.send(msg);
    }

    /// Report a fatal error and cancel the pipeline
    #[inline]
    pub fn report_fatal_error(&self, error: PipelineError) {
        self.cancel();
        let _ = self.sender.send(FeedbackMessage {
            message: "Fatal error".to_string(),
            source_component: self.source_component,
            error: Some(error),
        });
    }
}

pub struct Watcher {
    receiver: std::sync::mpsc::Receiver<FeedbackMessage>,
}

impl IntoIterator for Watcher {
    type Item = FeedbackMessage;
    type IntoIter = std::sync::mpsc::IntoIter<FeedbackMessage>;

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
        source_component: FeedbackSourceComponent::Unknown,
        sender,
    };
    (watcher, feedback, canceller)
}
