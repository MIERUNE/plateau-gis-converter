use serde::{Deserialize, Serialize};

use crate::{
    Articulation, Color, ColorBlendMode, CzmlBoolean, CzmlDouble, CzmlUri,
    DistanceDisplayCondition, HeightReference, NodeTransformations, ShadowMode,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub show: Option<CzmlBoolean>,
    pub gltf: CzmlUri,
    pub scale: Option<CzmlDouble>,
    pub minimum_pixel_size: Option<CzmlDouble>,
    pub maximum_scale: Option<CzmlDouble>,
    pub incrementally_load_textures: Option<CzmlBoolean>,
    pub run_animations: Option<CzmlBoolean>,
    pub shadows: Option<ShadowMode>,
    pub height_reference: Option<HeightReference>,
    pub silhouette_color: Option<Color>,
    pub silhouette_size: Option<CzmlDouble>,
    pub color: Option<Color>,
    pub color_blend_mode: Option<ColorBlendMode>,
    pub color_blend_amount: Option<CzmlDouble>,
    pub distance_display_condition: Option<DistanceDisplayCondition>,
    pub node_transformations: Option<NodeTransformations>,
    pub articulations: Option<Articulation>,
}
