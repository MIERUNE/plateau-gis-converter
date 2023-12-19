use super::{feedback, Parcel, Sender};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("transform error")]
    SendError(#[from] std::sync::mpsc::SendError<Parcel>),
}

pub trait Transformer: Send + Sync {
    fn transform(
        &self,
        parcel: Parcel,
        sink: &Sender,
        feedback: &feedback::Feedback,
    ) -> Result<(), TransformError>;
}
