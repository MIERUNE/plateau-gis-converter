use crate::pipeline::{Feedback, Parcel, Sender, TransformError, Transformer};
use nusamai_projection::vshift::JGD2011ToWGS84;

pub struct DummyTransformer {
    jgd2wgs: JGD2011ToWGS84,
}

impl Default for DummyTransformer {
    fn default() -> Self {
        Self {
            jgd2wgs: JGD2011ToWGS84::from_embedded_model(),
        }
    }
}

impl Transformer for DummyTransformer {
    fn transform(
        &self,
        mut parcel: Parcel,
        downstream: &Sender,
        feedback: &Feedback,
    ) -> Result<(), TransformError> {
        // 仮実装
        parcel.cityobj.geometries.vertices.iter_mut().for_each(|v| {
            // Swap x and y (lat, lng -> lng, lat)
            let (lng, lat, height) = (v[1], v[0], v[2]);

            // JGD2011 to WGS 84 (elevation to ellipsoidal height)
            (v[0], v[1], v[2]) = self.jgd2wgs.convert(lng, lat, height);
        });

        if downstream.send(parcel).is_err() {
            feedback.cancel();
        };
        Ok(())
    }
}
