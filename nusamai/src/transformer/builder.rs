use std::sync::Arc;

use nusamai_citygml::schema::Schema;
use nusamai_projection::vshift::Jgd2011ToWgs84;

use super::{transform::*, Transform};
use crate::transformer;

pub struct Requirements {
    /// Whether to shorten field names to 10 characters or less for Shapefiles.
    pub shorten_names_for_shapefile: bool,
    pub tree_flattening: TreeFlatteningSpec,
    pub resolve_appearance: bool,
    pub mergedown: MergedownSpec,
    pub key_value: KeyValueSpec,
}

impl Default for Requirements {
    fn default() -> Self {
        Self {
            shorten_names_for_shapefile: false,
            tree_flattening: TreeFlatteningSpec::None,
            resolve_appearance: false,
            mergedown: MergedownSpec::RemoveDescendantFeatures,
            key_value: KeyValueSpec::Jsonify,
        }
    }
}

pub struct Request {
    pub shorten_names_for_shapefile: bool,
    pub tree_flattening: TreeFlatteningSpec,
    pub apply_appearance: bool,
    pub mergedown: MergedownSpec,
    pub key_value: KeyValueSpec,
}

impl From<Requirements> for Request {
    fn from(req: Requirements) -> Self {
        Self {
            shorten_names_for_shapefile: req.shorten_names_for_shapefile,
            tree_flattening: req.tree_flattening,
            apply_appearance: req.resolve_appearance,
            mergedown: req.mergedown,
            key_value: req.key_value,
        }
    }
}

pub enum TreeFlatteningSpec {
    /// No feature flattening
    None,
    /// Flatten all features except thematic surfaces
    AllExceptThematicSurfaces,
    /// Flatten all features
    All,
}

pub enum MergedownSpec {
    /// No mergedown
    NoMergedown,
    /// merge the children's geometries into the root and retain the children features
    RetainDescendantFeatures,
    /// merge the children's geometries into the root and remove the children features
    RemoveDescendantFeatures,
}

/// Specifies how to transform nested objects and arrays
pub enum KeyValueSpec {
    None,
    // JSONify nested objects and arrays
    Jsonify,
    // Flatten nested objects and arrays as dot-split keys (e.g. `buildingDisasterRiskAttribute.0.rankOrg`)
    DotNotation,
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
        // TODO: build transformation based on config file

        // Transform the coordinate system
        transforms.push(Box::new(ProjectionTransform::new(self.jgd2wgs.clone())));

        // Apply appearance to geometries
        if self.request.apply_appearance {
            transforms.push(Box::new(ApplyAppearanceTransform::new()));
        }

        transforms.push({
            let mut renamer = Box::<EditFieldNamesTransform>::default();
            if self.request.shorten_names_for_shapefile {
                renamer.load_default_map_for_shape();
            }
            renamer
        });

        transforms.push(Box::<FilterLodTransform>::default());

        match self.request.tree_flattening {
            TreeFlatteningSpec::None => {}
            TreeFlatteningSpec::AllExceptThematicSurfaces => {
                transforms.push(Box::new(FlattenTreeTransform::new()));
            }
            TreeFlatteningSpec::All => {
                transforms.push(Box::new(FlattenTreeTransform::new()));
            }
        }

        match self.request.mergedown {
            MergedownSpec::NoMergedown => {}
            MergedownSpec::RemoveDescendantFeatures => {
                transforms.push(Box::new(GeometricMergedownTransform::new(false)));
            }
            MergedownSpec::RetainDescendantFeatures => {
                transforms.push(Box::new(GeometricMergedownTransform::new(true)));
            }
        }

        match self.request.key_value {
            KeyValueSpec::Jsonify => {
                transforms.push(Box::<JsonifyTransform>::default());
            }
            KeyValueSpec::DotNotation => {
                transforms.push(Box::<DotNotationTransform>::default());
            }
            KeyValueSpec::None => {
                // No-op
            }
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
