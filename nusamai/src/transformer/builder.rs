use std::sync::Arc;

use nusamai_projection::vshift::JGD2011ToWGS84;

use super::{
    transform::{ProjectionTransform, SerialTransform},
    Transform,
};

pub struct TransformBuilder {
    jgd2wgs: Arc<JGD2011ToWGS84>,
}

impl TransformBuilder {
    pub fn build(&self) -> Box<dyn Transform> {
        let mut transforms = SerialTransform::default();
        transforms.push(Box::new(ProjectionTransform::new(self.jgd2wgs.clone())));
        Box::new(transforms)
    }
}

impl Default for TransformBuilder {
    fn default() -> Self {
        Self {
            jgd2wgs: Arc::new(JGD2011ToWGS84::default()),
        }
    }
}
