use crate::pipeline::{Feedback, Parcel, Sender, TransformError, Transformer};

pub struct NoopTransformer {}

impl Transformer for NoopTransformer {
    fn transform(
        &self,
        parcel: Parcel,
        downstream: &Sender,
        feedback: &Feedback,
    ) -> Result<(), TransformError> {
        // no-op
        if downstream.send(parcel).is_err() {
            feedback.cancel();
        };
        Ok(())
    }
}
