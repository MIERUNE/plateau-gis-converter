use crate::pipeline::{Feedback, Parcel, Sender, TransformError, Transformer};

pub struct NoopTransformer {}

impl Transformer for NoopTransformer {
    fn transform(
        &self,
        parcel: Parcel,
        sender: &Sender,
        _feedback: &Feedback,
    ) -> Result<(), TransformError> {
        // no-op
        sender.send(parcel)?;
        Ok(())
    }
}
