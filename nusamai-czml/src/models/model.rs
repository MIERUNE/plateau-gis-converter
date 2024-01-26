use serde::{Deserialize, Serialize};

use crate::{
    Articulation, Color, ColorBlendMode, DistanceDisplayCondition, HeightReference,
    NodeTransformation, ShadowMode,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub show: Option<bool>,
    pub gltf: String,
    pub scale: Option<f64>,
    pub minimum_pixel_size: Option<f64>,
    pub maximum_scale: Option<f64>,
    pub incrementally_load_textures: Option<bool>,
    pub run_animations: Option<bool>,
    pub shadows: Option<ShadowMode>,
    pub height_reference: Option<HeightReference>,
    pub silhouette_color: Option<Color>,
    pub silhouette_size: Option<f64>,
    pub color: Option<Color>,
    pub color_blend_mode: Option<ColorBlendMode>,
    pub color_blend_amount: Option<f64>,
    pub distance_display_condition: Option<DistanceDisplayCondition>,
    pub node_transformations: Option<NodeTransformation>,
    pub articulations: Option<Articulation>,
}
