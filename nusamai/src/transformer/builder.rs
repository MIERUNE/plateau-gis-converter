use std::sync::Arc;

use nusamai_citygml::schema::Schema;
use nusamai_projection::vshift::JGD2011ToWGS84;

use super::{
    transform::{ProjectionTransform, SerialTransform},
    Transform,
};

pub trait TransformBuilder: Send + Sync {
    fn build(&self) -> Box<dyn Transform>;

    fn transform_schema(&self, schema: &mut Schema) {
        self.build().transform_schema(schema);
    }
}

pub struct NusamaiTransformBuilder {
    jgd2wgs: Arc<JGD2011ToWGS84>,
}

impl TransformBuilder for NusamaiTransformBuilder {
    fn build(&self) -> Box<dyn Transform> {
        let mut transforms = SerialTransform::default();
        // TODO: build transforming graph from config
        transforms.push(Box::new(ProjectionTransform::new(self.jgd2wgs.clone())));
        Box::new(transforms)
    }
}

impl Default for NusamaiTransformBuilder {
    fn default() -> Self {
        Self {
            jgd2wgs: JGD2011ToWGS84::default().into(),
        }
    }
}