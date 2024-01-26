use serde::{Deserialize, Serialize};

use crate::{
    Articulations, Color, ColorBlendMode, ColorProperties, CzmlBoolean, CzmlDouble, CzmlUri,
    DistanceDisplayCondition, HeightReference, NodeTransformations, RgbaValue, ShadowMode,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Model {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show: Option<CzmlBoolean>,

    pub gltf: CzmlUri,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<CzmlDouble>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_pixel_size: Option<CzmlDouble>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_scale: Option<CzmlDouble>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incrementally_load_textures: Option<CzmlBoolean>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_animations: Option<CzmlBoolean>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadows: Option<ShadowMode>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_reference: Option<HeightReference>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub silhouette_color: Option<Color>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub silhouette_size: Option<CzmlDouble>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_blend_mode: Option<ColorBlendMode>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_blend_amount: Option<CzmlDouble>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_display_condition: Option<DistanceDisplayCondition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_transformations: Option<NodeTransformations>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub articulations: Option<Articulations>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            show: Some(CzmlBoolean::Boolean(true)),
            gltf: CzmlUri::String("example.gltf".to_string()),
            scale: Some(CzmlDouble::Double(1.0)),
            minimum_pixel_size: Some(CzmlDouble::Double(0.0)),
            maximum_scale: Default::default(),
            incrementally_load_textures: Some(CzmlBoolean::Boolean(true)),
            run_animations: Some(CzmlBoolean::Boolean(true)),
            shadows: Some(ShadowMode::String("ENABLED".to_string())),
            height_reference: Some(HeightReference::String("NONE".to_string())),
            silhouette_color: Some(Color::Object(ColorProperties {
                rgba: Some(RgbaValue::Constant([255, 0, 0, 0])),
                rgbaf: None,
                reference: None,
                interpolatable_property: None,
                deletable_property: None,
                rgba_value_property: None,
                rgbaf_value_property: None,
                reference_value_property: None,
            })),
            silhouette_size: Some(CzmlDouble::Double(0.0)),
            color: Some(Color::Object(ColorProperties {
                rgba: Some(RgbaValue::Constant([0, 0, 0, 0])),
                rgbaf: None,
                reference: None,
                interpolatable_property: None,
                deletable_property: None,
                rgba_value_property: None,
                rgbaf_value_property: None,
                reference_value_property: None,
            })),
            color_blend_mode: Some(ColorBlendMode::String("HIGHLIGHT".to_string())),
            color_blend_amount: Some(CzmlDouble::Double(0.5)),
            distance_display_condition: Default::default(),
            node_transformations: Default::default(),
            articulations: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let model_string: Model = serde_json::from_str(r#"{"gltf":"example.gltf"}"#).unwrap();
        assert_eq!(model_string.gltf, "example.gltf".to_string(),);
    }
}
