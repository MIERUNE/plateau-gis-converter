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
    pub mergedown: RequirementItem<Mergedown>,
    pub feature_flattening: RequirementItem<FeatureFlattening>,
}

impl Default for Requirements {
    fn default() -> Self {
        Self {
            shorten_names_for_shapefile: RequirementItem::Required(false),
            mergedown: RequirementItem::Required(Mergedown::Geometry),
            feature_flattening: RequirementItem::Recommended(
                FeatureFlattening::AllExceptThematicSurfaces,
            ),
        }
    }
}

pub struct Request {
    pub shorten_names_for_shapefile: bool,
    pub mergedown: Mergedown,
    pub feat_flattening: FeatureFlattening,
}

impl From<Requirements> for Request {
    fn from(req: Requirements) -> Self {
        Self {
            shorten_names_for_shapefile: req.shorten_names_for_shapefile.into_value(),
            mergedown: req.mergedown.into_value(),
            feat_flattening: req.feature_flattening.into_value(),
        }
    }
}

#[derive(Default)]
pub enum FeatureFlattening {
    /// No feature flattening
    None,
    /// Flatten all features except thematic surfaces
    #[default]
    AllExceptThematicSurfaces,
    /// Flatten all features
    All,
}

#[derive(Default)]
pub enum Mergedown {
    /// No mergedown
    None,
    /// Merge all geometries into the root feature
    #[default]
    Geometry,
    /// Merge all geometries and data attributes into the root feature
    Full,
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
            if self.request.shorten_names_for_shapefile {
                renamer.load_default_map_for_shape();
            }
            renamer
        });

        transforms.push(Box::<FilterLodTransform>::default());

        match self.request.feat_flattening {
            FeatureFlattening::None => {}
            FeatureFlattening::AllExceptThematicSurfaces => {
                transforms.push(Box::new(FlattenFeatureTransform::new(false)));
            }
            FeatureFlattening::All => {
                transforms.push(Box::new(FlattenFeatureTransform::new(true)));
            }
        }

        match self.request.mergedown {
            Mergedown::Geometry => {
                transforms.push(Box::<GeometricMergedownTransform>::default());
            }
            Mergedown::Full => {
                transforms.push(Box::<FullMergedownTransform>::default());
            }
            Mergedown::None => {}
        }

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
