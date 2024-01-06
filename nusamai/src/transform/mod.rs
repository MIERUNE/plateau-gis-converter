use crate::pipeline::{Feedback, Parcel, Sender, TransformError, Transformer};

pub struct NoopTransformer {}

impl Transformer for NoopTransformer {
    fn transform(
        &self,
        mut parcel: Parcel,
        downstream: &Sender,
        feedback: &Feedback,
    ) -> Result<(), TransformError> {
        // swap x and y
        parcel.cityobj.geometries.vertices.iter_mut().for_each(|v| {
            (v[0], v[1], v[2]) = (v[1], v[0], v[2]);
        });

        if downstream.send(parcel).is_err() {
            feedback.cancel();
        };
        Ok(())
    }
}
