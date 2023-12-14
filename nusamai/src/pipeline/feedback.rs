//! Feedback messages from the pipeline components.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const FEEDBACK_CHANNEL_BOUND: usize = 10000;

#[derive(Debug)]
pub struct FeedbackMessage {
    pub message: String,
    // severity:
    // progress:
    // source:
    // etc.
}

#[derive(Clone)]
pub struct Feedback {
    cancelled: Arc<AtomicBool>,
    sender: std::sync::mpsc::SyncSender<FeedbackMessage>,
}

impl Feedback {
    /// Checks if the pipeline is requested to be cancelled
    #[inline]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// Send a message to the feedback channel
    #[inline]
    pub fn send(&self, msg: FeedbackMessage) {
        // don't care if the receiver is dropped.
        let _ = self.sender.send(msg);
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

pub struct Canceller {
    cancelled: Arc<AtomicBool>,
}

impl Canceller {
    /// Cancel the pipeline
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
}

pub(crate) fn watcher() -> (Watcher, Feedback, Canceller) {
    let cancelled = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = std::sync::mpsc::sync_channel(FEEDBACK_CHANNEL_BOUND);
    let watcher = Watcher { receiver };
    let canceller = Canceller {
        cancelled: cancelled.clone(),
    };
    let feedback = Feedback {
        cancelled: cancelled.clone(),
        sender,
    };
    (watcher, feedback, canceller)
}
