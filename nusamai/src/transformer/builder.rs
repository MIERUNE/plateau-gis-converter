use std::sync::Arc;

use nusamai_citygml::schema::Schema;
use nusamai_projection::{crs, vshift::Jgd2011ToWgs84};

use super::{transform::*, Transform};
use crate::{sink::DataRequirements, transformer};

pub struct Request {
    pub output_epsg: crs::EpsgCode,
    pub shorten_names_for_shapefile: bool,
    pub mapping_rules: Option<transformer::MappingRules>,
    pub tree_flattening: TreeFlatteningSpec,
    pub apply_appearance: bool,
    pub mergedown: MergedownSpec,
    pub key_value: KeyValueSpec,
    pub lod_filter: LodFilterSpec,
    pub geom_stats: GeometryStatsSpec,
}

impl Request {
    pub fn set_mapping_rules(&mut self, rules: Option<transformer::MappingRules>) {
        self.mapping_rules = rules;
    }
}

impl From<DataRequirements> for Request {
    fn from(req: DataRequirements) -> Self {
        Self {
            output_epsg: req.output_epsg,
            shorten_names_for_shapefile: req.shorten_names_for_shapefile,
            mapping_rules: None,
            tree_flattening: req.tree_flattening,
            apply_appearance: req.resolve_appearance,
            mergedown: req.mergedown,
            key_value: req.key_value,
            lod_filter: req.lod_filter,
            geom_stats: req.geom_stats,
        }
    }
}

pub struct LodFilterSpec {
    pub mask: LodMask,
    pub mode: LodFilterMode,
}

impl Default for LodFilterSpec {
    fn default() -> Self {
        Self {
            mask: LodMask::all(),
            mode: LodFilterMode::Highest,
        }
    }
}

pub enum TreeFlatteningSpec {
    /// No flattening at all
    None,
    // Flatten with the given options
    Flatten {
        feature: FeatureFlatteningOption,
        data: DataFlatteningOption,
        object: ObjectFlatteningOption,
    },
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
    // Jsonify nested objects and arrays
    JsonifyObjectsAndArrays,
    // Jsonify nested objects (but not arrays)
    JsonifyObjects,
    // Flatten nested objects and arrays as dot-split keys (e.g. `buildingDisasterRiskAttribute.0.rankOrg`)
    DotNotation,
}

pub enum GeometryStatsSpec {
    None,
    MinMaxHeights,
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
        transforms.push(Box::new(ProjectionTransform::new(
            self.jgd2wgs.clone(),
            self.request.output_epsg,
        )));

        match self.request.geom_stats {
            GeometryStatsSpec::None => {}
            GeometryStatsSpec::MinMaxHeights => {
                transforms.push(Box::<GeometryStatsTransform>::default());
            }
        }

        // Apply appearance to geometries
        if self.request.apply_appearance {
            transforms.push(Box::new(ApplyAppearanceTransform::new()));
        }

        transforms.push({
            let mut renamer = Box::<EditFieldNamesTransform>::default();
            if self.request.shorten_names_for_shapefile {
                renamer.load_default_map_for_shape();
            }
            // Rename rules by the user are set after `load_default_map_for_shape()`,
            // therefore it will override the default shapefile renames if there are conflicts
            if let Some(mapping_rules) = &self.request.mapping_rules {
                renamer.extend_rename_map(mapping_rules.rename.clone());
            }
            renamer
        });

        transforms.push(Box::new(FilterLodTransform::new(
            self.request.lod_filter.mask,
            self.request.lod_filter.mode,
        )));

        match self.request.tree_flattening {
            TreeFlatteningSpec::None => {}
            TreeFlatteningSpec::Flatten {
                feature,
                data,
                object,
            } => {
                transforms.push(Box::new(FlattenTreeTransform::with_options(
                    feature, data, object,
                )));
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
            KeyValueSpec::JsonifyObjectsAndArrays => {
                transforms.push(Box::new(JsonifyTransform::default().jsonify_array(true)));
            }
            KeyValueSpec::JsonifyObjects => {
                transforms.push(Box::new(JsonifyTransform::default().jsonify_array(false)));
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
