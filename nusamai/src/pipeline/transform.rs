use super::{feedback, Percel, Sender};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("transform error")]
    SendError(#[from] std::sync::mpsc::SendError<Percel>),
}

pub trait Transformer: Send + Sync {
    fn transform(
        &self,
        percel: Percel,
        sink: &Sender,
        feedback: &feedback::Feedback,
    ) -> Result<(), TransformError>;
}
