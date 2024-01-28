use std::sync::Arc;

use nusamai_citygml::schema::Schema;
use nusamai_projection::vshift::Jgd2011ToWgs84;

use super::{transform::*, Transform};
use crate::transformer;

pub enum RequirementItem<T> {
    /// Required value (user must not override)
    Required(T),
    /// Recommended value (recommended value but user may override)
    Recommended(T),
    /// Default value
    Default(T),
}

impl<T> RequirementItem<T> {
    pub fn into_value(self) -> T {
        match self {
            Self::Required(v) => v,
            Self::Recommended(v) => v,
            Self::Default(v) => v,
        }
    }
}

pub struct Requirements {
    /// Whether to shorten field names to 10 characters or less for Shapefiles.
    pub shorten_names_for_shapefile: RequirementItem<bool>,
}

impl Default for Requirements {
    fn default() -> Self {
        Self {
            shorten_names_for_shapefile: RequirementItem::Required(false),
        }
    }
}

pub struct Request {
    pub shorten_names_for_shapefile: bool,
}

impl From<Requirements> for Request {
    fn from(req: Requirements) -> Self {
        Self {
            shorten_names_for_shapefile: req.shorten_names_for_shapefile.into_value(),
        }
    }
}

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

        transforms.push({
            let mut renamer = Box::<EditFieldNamesTransform>::default();
            renamer.load_default_map_for_shape();
            renamer
        });

        // transforms.push(Box::<FilterLodTransform>::default());

        // transforms.push(Box::new(FlattenFeatureTransform::new(true)));

        // transforms.push(Box::<GeometricMergedownTransform>::default());
        // transforms.push(Box::<FullMergedownTransform>::default());

        transforms.push({
            let mut renamer = Box::<EditFieldNamesTransform>::default();
            if self.request.shorten_names_for_shapefile {
                renamer.load_default_map_for_shape();
            }
            renamer
        });

        transforms.push(Box::<FilterLodTransform>::default());

        transforms.push(Box::new(FlattenFeatureTransform::new(true)));

        transforms.push(Box::<GeometricMergedownTransform>::default());
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
