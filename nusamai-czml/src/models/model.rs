use serde::{Deserialize, Serialize};

use crate::{
    Articulation, BooleanValueType, Color, ColorBlendMode, ColorBlendModeValueType,
    ColorProperties, ColorValueType, CzmlBoolean, CzmlDouble, CzmlUri, DistanceDisplayCondition,
    DoubleValueType, HeightReference, HeightReferenceValueType, NodeTransformations, RgbaValue,
    ShadowMode, ShadowModeValueType,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub show: Option<CzmlBoolean>,
    pub gltf: Option<CzmlUri>,
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

impl Default for Model {
    fn default() -> Self {
        Self {
            show: Some(CzmlBoolean {
                value: BooleanValueType::Boolean(true),
            }),
            gltf: None,
            scale: Some(CzmlDouble {
                value: DoubleValueType::Double(1.0),
            }),
            minimum_pixel_size: Some(CzmlDouble {
                value: DoubleValueType::Double(0.0),
            }),
            maximum_scale: Default::default(),
            incrementally_load_textures: Some(CzmlBoolean {
                value: BooleanValueType::Boolean(true),
            }),
            run_animations: Some(CzmlBoolean {
                value: BooleanValueType::Boolean(true),
            }),
            shadows: Some(ShadowMode {
                value: ShadowModeValueType::String("ENABLED".to_string()),
            }),
            height_reference: Some(HeightReference {
                value: HeightReferenceValueType::String("NONE".to_string()),
            }),
            silhouette_color: Some(Color {
                value: ColorValueType::Object(ColorProperties {
                    rgba: Some(RgbaValue::Constant([255, 0, 0, 0])),
                    rgbaf: None,
                    reference: None,
                    interpolatable_property: None,
                    deletable_property: None,
                    rgba_value_property: None,
                    rgbaf_value_property: None,
                    reference_value_property: None,
                }),
            }),
            silhouette_size: Some(CzmlDouble {
                value: DoubleValueType::Double(0.0),
            }),
            color: Some(Color {
                value: ColorValueType::Object(ColorProperties {
                    rgba: Some(RgbaValue::Constant([0, 0, 0, 0])),
                    rgbaf: None,
                    reference: None,
                    interpolatable_property: None,
                    deletable_property: None,
                    rgba_value_property: None,
                    rgbaf_value_property: None,
                    reference_value_property: None,
                }),
            }),
            color_blend_mode: Some(ColorBlendMode {
                value: ColorBlendModeValueType::String("HIGHLIGHT".to_string()),
            }),
            color_blend_amount: Some(CzmlDouble {
                value: DoubleValueType::Double(0.5),
            }),
            distance_display_condition: Default::default(),
            node_transformations: Default::default(),
            articulations: Default::default(),
        }
    }
}
