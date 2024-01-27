use std::sync::Arc;

use nusamai_citygml::schema::Schema;
use nusamai_projection::vshift::Jgd2011ToWgs84;

use super::{transform::*, Transform};
use crate::transformer;

pub trait TransformBuilder: Send + Sync {
    fn build(&self) -> Box<dyn Transform>;

    fn transform_schema(&self, schema: &mut Schema) {
        self.build().transform_schema(schema);
    }
}

pub struct NusamaiTransformBuilder {
    request: transformer::Request,
    jgd2wgs: Arc<Jgd2011ToWgs84>,
}

impl TransformBuilder for NusamaiTransformBuilder {
    fn build(&self) -> Box<dyn Transform> {
        let mut transforms = SerialTransform::default();
        // TODO: build transformation based on config

        // Transform the coordinate system
        transforms.push(Box::new(ProjectionTransform::new(self.jgd2wgs.clone())));

        // Change field names
        transforms.push({
            let mut renamer = Box::<EditFieldNamesTransform>::default();
            if self.request.shorten_names_for_shapefile {
                renamer.load_default_map_for_shape();
            }
            renamer
        });

        // Retain only a single LOD
        transforms.push(Box::<FilterLodTransform>::default());

        transforms.push(Box::<FlattenFeatureTransform>::default());

        // transforms.push(Box::<GeometricMergedownTransform>::default());
        // transforms.push(Box::<FullMergedownTransform>::default());

        Box::new(transforms)
    }
}

impl NusamaiTransformBuilder {
    pub fn new(req: transformer::Request) -> Self {
        Self {
            request: req,
            jgd2wgs: Jgd2011ToWgs84::default().into(),
        }
    }
}
