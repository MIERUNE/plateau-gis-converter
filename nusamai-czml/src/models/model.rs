use serde::{Deserialize, Serialize};

use crate::{
    Articulations, Color, ColorBlendMode, ColorProperties, CzmlBoolean, CzmlDouble, CzmlUri,
    DistanceDisplayCondition, HeightReference, NodeTransformations, RgbaValue, ShadowMode,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub show: CzmlBoolean,

    pub gltf: CzmlUri,

    pub scale: CzmlDouble,

    pub minimum_pixel_size: CzmlDouble,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_scale: Option<CzmlDouble>,

    pub incrementally_load_textures: CzmlBoolean,

    pub run_animations: CzmlBoolean,

    pub shadows: ShadowMode,

    pub height_reference: HeightReference,

    pub silhouette_color: Color,

    pub silhouette_size: CzmlDouble,

    pub color: Color,

    pub color_blend_mode: ColorBlendMode,

    pub color_blend_amount: CzmlDouble,

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
            show: CzmlBoolean::Boolean(true),
            gltf: CzmlUri::String("".to_string()),
            scale: CzmlDouble::Double(1.0),
            minimum_pixel_size: CzmlDouble::Double(0.0),
            maximum_scale: None,
            incrementally_load_textures: CzmlBoolean::Boolean(true),
            run_animations: CzmlBoolean::Boolean(true),
            shadows: ShadowMode::String("ENABLED".to_string()),
            height_reference: HeightReference::String("NONE".to_string()),
            silhouette_color: Color::Object(ColorProperties {
                rgba: Some(RgbaValue::Constant([255, 0, 0, 0])),
                rgbaf: None,
                reference: None,
                interpolatable_property: None,
                deletable_property: None,
                rgba_value_property: None,
                rgbaf_value_property: None,
                reference_value_property: None,
            }),
            silhouette_size: CzmlDouble::Double(0.0),
            color: Color::Object(ColorProperties {
                rgba: Some(RgbaValue::Constant([0, 0, 0, 0])),
                rgbaf: None,
                reference: None,
                interpolatable_property: None,
                deletable_property: None,
                rgba_value_property: None,
                rgbaf_value_property: None,
                reference_value_property: None,
            }),
            color_blend_mode: ColorBlendMode::String("HIGHLIGHT".to_string()),
            color_blend_amount: CzmlDouble::Double(0.5),
            distance_display_condition: None,
            node_transformations: None,
            articulations: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gltf_string() {
        let model_string: Model = serde_json::from_str(r#"{"gltf":"example.gltf"}"#).unwrap();
        assert_eq!(
            model_string.gltf,
            CzmlUri::String("example.gltf".to_string())
        );
    }

    #[test]
    fn test_default() {
        let model_default = Model::default();
        assert_eq!(model_default.show, Some(CzmlBoolean::Boolean(true)));
    }
}
