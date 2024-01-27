use std::sync::Arc;

use nusamai_citygml::schema::Schema;
use nusamai_projection::vshift::Jgd2011ToWgs84;

use super::{transform::*, Transform};

pub trait TransformBuilder: Send + Sync {
    fn build(&self) -> Box<dyn Transform>;

    fn transform_schema(&self, schema: &mut Schema) {
        self.build().transform_schema(schema);
    }
}

pub struct NusamaiTransformBuilder {
    jgd2wgs: Arc<Jgd2011ToWgs84>,
}

impl TransformBuilder for NusamaiTransformBuilder {
    fn build(&self) -> Box<dyn Transform> {
        let mut transforms = SerialTransform::default();
        // TODO: build transformation based on config

        transforms.push(Box::new(ProjectionTransform::new(self.jgd2wgs.clone())));

        let renamer = {
            let mut renamer = Box::<EditFieldNamesTransform>::default();
            renamer.load_default_map_for_shape();
            renamer
        };
        transforms.push(renamer);
        transforms.push(Box::<FilterLodTransform>::default());
        transforms.push(Box::<FlattenFeatureTransform>::default());

        // transforms.push(Box::<GeometricMergedownTransform>::default());
        // transforms.push(Box::<FullMergedownTransform>::default());

        Box::new(transforms)
    }
}

impl Default for NusamaiTransformBuilder {
    fn default() -> Self {
        Self {
            jgd2wgs: Jgd2011ToWgs84::default().into(),
        }
    }
}
