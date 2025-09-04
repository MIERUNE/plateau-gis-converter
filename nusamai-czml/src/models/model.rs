use serde::{Deserialize, Serialize};

use crate::{
    Articulations, Color, ColorBlendMode, ColorProperties, CzmlBoolean, CzmlDouble, CzmlUri,
    DistanceDisplayCondition, HeightReference, HeightReferenceValue, NodeTransformations,
    RgbaValue, ShadowMode, ShadowModeValue,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Model {
    #[serde(default = "default_show")]
    pub show: CzmlBoolean,

    #[serde(default = "default_gltf")]
    pub gltf: CzmlUri,

    #[serde(default = "default_scale")]
    pub scale: CzmlDouble,

    #[serde(default = "default_minimum_pixel_size")]
    pub minimum_pixel_size: CzmlDouble,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_scale: Option<CzmlDouble>,

    #[serde(default = "default_incrementally_load_textures")]
    pub incrementally_load_textures: CzmlBoolean,

    #[serde(default = "default_run_animations")]
    pub run_animations: CzmlBoolean,

    #[serde(default = "default_shadows")]
    pub shadows: ShadowMode,

    #[serde(default = "default_height_reference")]
    pub height_reference: HeightReference,

    #[serde(default = "default_silhouette_color")]
    pub silhouette_color: Color,

    #[serde(default = "default_silhouette_size")]
    pub silhouette_size: CzmlDouble,

    #[serde(default = "default_color")]
    pub color: Color,

    #[serde(default = "default_color_blend_mode")]
    pub color_blend_mode: ColorBlendMode,

    #[serde(default = "default_color_blend_amount")]
    pub color_blend_amount: CzmlDouble,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_display_condition: Option<DistanceDisplayCondition>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_transformations: Option<NodeTransformations>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub articulations: Option<Articulations>,
}

fn default_show() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn default_gltf() -> CzmlUri {
    CzmlUri::String("".to_string())
}

fn default_scale() -> CzmlDouble {
    CzmlDouble::Double(1.0)
}

fn default_minimum_pixel_size() -> CzmlDouble {
    CzmlDouble::Double(0.0)
}

fn default_incrementally_load_textures() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn default_run_animations() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn default_shadows() -> ShadowMode {
    ShadowMode::String(ShadowModeValue::Enabled)
}

fn default_height_reference() -> HeightReference {
    HeightReference::String(HeightReferenceValue::None)
}

fn default_silhouette_color() -> Color {
    Color::Object(Box::new(ColorProperties {
        rgba: Some(RgbaValue::Constant([255, 0, 0, 0])),
        ..Default::default()
    }))
}

fn default_silhouette_size() -> CzmlDouble {
    CzmlDouble::Double(0.0)
}

fn default_color() -> Color {
    Color::Object(Box::new(ColorProperties {
        rgba: Some(RgbaValue::Constant([0, 0, 0, 0])),
        ..Default::default()
    }))
}

fn default_color_blend_mode() -> ColorBlendMode {
    ColorBlendMode::String("HIGHLIGHT".to_string())
}

fn default_color_blend_amount() -> CzmlDouble {
    CzmlDouble::Double(0.5)
}

impl Default for Model {
    fn default() -> Self {
        Self {
            show: default_show(),
            gltf: default_gltf(),
            scale: default_scale(),
            minimum_pixel_size: default_minimum_pixel_size(),
            maximum_scale: None,
            incrementally_load_textures: default_incrementally_load_textures(),
            run_animations: default_run_animations(),
            shadows: default_shadows(),
            height_reference: default_height_reference(),
            silhouette_color: default_silhouette_color(),
            silhouette_size: default_silhouette_size(),
            color: default_color(),
            color_blend_mode: default_color_blend_mode(),
            color_blend_amount: default_color_blend_amount(),
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
    fn test_default_deserialize() {
        let model: Model = serde_json::from_str("{}").unwrap();
        let uri_string = match model.gltf {
            CzmlUri::String(ref s) => s,
            _ => panic!("gltf is not string"),
        };
        assert_eq!(*uri_string, "".to_string());
    }

    #[test]
    fn test_default_serialize() {
        let model = Model::default();
        let json = serde_json::to_string(&model).unwrap();
        assert_eq!(
            json,
            r#"{"show":true,"gltf":"","scale":1.0,"minimumPixelSize":0.0,"incrementallyLoadTextures":true,"runAnimations":true,"shadows":"ENABLED","heightReference":"NONE","silhouetteColor":{"rgba":[255,0,0,0]},"silhouetteSize":0.0,"color":{"rgba":[0,0,0,0]},"colorBlendMode":"HIGHLIGHT","colorBlendAmount":0.5}"#
        )
    }
}
