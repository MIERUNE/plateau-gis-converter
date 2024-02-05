use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::texture_info::TextureInfo;

/// The material's alpha rendering mode enumeration specifying the interpretation of the alpha value of the base color.
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AlphaMode {
    /// The alpha value is ignored, and the rendered output is fully opaque.
    #[default]
    Opaque,
    /// The rendered output is either fully opaque or fully transparent depending on the alpha value and the specified `alphaCutoff` value; the exact appearance of the edges **MAY** be subject to implementation-specific techniques such as "`Alpha-to-Coverage`".
    Mask,
    /// The alpha value is used to composite the source and destination areas. The rendered output is combined with the background using the normal painting operation (i.e. the Porter and Duff over operator).
    Blend,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MaterialNormalTextureInfo {
    /// The index of the texture.
    pub index: u32,

    /// This integer value is used to construct a string in the format `TEXCOORD_<set index>` which is a reference to a key in `mesh.primitives.attributes` (e.g. a value of `0` corresponds to `TEXCOORD_0`). A mesh primitive **MUST** have the corresponding texture coordinate attributes for the material to be applicable to it.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tex_coord: u32,

    /// The scalar parameter applied to each normal vector of the texture. This value scales the normal vector in X and Y directions using the formula: `scaledNormal =  normalize((<sampled normal texture value> * 2.0 - 1.0) * vec3(<normal scale>, <normal scale>, 1.0))`."""
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub scale: f64,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<MaterialNormalTextureInfoExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

impl Default for MaterialNormalTextureInfo {
    fn default() -> Self {
        Self {
            index: 0,
            tex_coord: 0,
            scale: 1.0,
            extensions: None,
            extras: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MaterialNormalTextureInfoExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MaterialOcclusionTextureInfo {
    /// The index of the texture.
    pub index: u32,

    /// This integer value is used to construct a string in the format `TEXCOORD_<set index>` which is a reference to a key in `mesh.primitives.attributes` (e.g. a value of `0` corresponds to `TEXCOORD_0`). A mesh primitive **MUST** have the corresponding texture coordinate attributes for the material to be applicable to it.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tex_coord: u32,

    /// A scalar parameter controlling the amount of occlusion applied. A value of `0.0` means no occlusion. A value of `1.0` means full occlusion. This value affects the final occlusion value as: `1.0 + strength * (<sampled occlusion texture value> - 1.0)`.
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub strength: f64,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<MaterialOcculusionTextureInfoExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

impl Default for MaterialOcclusionTextureInfo {
    fn default() -> Self {
        Self {
            index: 0,
            tex_coord: 0,
            strength: 1.0,
            extensions: None,
            extras: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MaterialOcculusionTextureInfoExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

/// A set of parameter values that are used to define the metallic-roughness material model from Physically-Based Rendering (PBR) methodology.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct MaterialPbrMetallicRoughness {
    /// The factors for the base color of the material. This value defines linear multipliers for the sampled texels of the base color texture.
    #[serde(skip_serializing_if = "is_default_color")]
    pub base_color_factor: [f64; 4],

    /// The base color texture. The first three components (RGB) **MUST** be encoded with the sRGB transfer function. They specify the base color of the material. If the fourth component (A) is present, it represents the linear alpha coverage of the material. Otherwise, the alpha coverage is equal to `1.0`. The `material.alphaMode` property specifies how alpha is interpreted. The stored texels **MUST NOT** be premultiplied. When undefined, the texture **MUST** be sampled as having `1.0` in all components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<TextureInfo>,

    /// The factor for the metalness of the material. This value defines a linear multiplier for the sampled metalness values of the metallic-roughness texture.
    #[serde(skip_serializing_if = "is_one")]
    pub metallic_factor: f64,

    /// The factor for the metalness of the material. This value defines a linear multiplier for the sampled metalness values of the metallic-roughness texture.
    #[serde(skip_serializing_if = "is_one")]
    pub roughness_factor: f64,

    /// The metallic-roughness texture. The metalness values are sampled from the B channel. The roughness values are sampled from the G channel. These values **MUST** be encoded with a linear transfer function. If other channels are present (R or A), they **MUST** be ignored for metallic-roughness calculations. When undefined, the texture **MUST** be sampled as having `1.0` in G and B components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<TextureInfo>,
}

fn is_default_color(v: &[f64; 4]) -> bool {
    *v == [1.0, 1.0, 1.0, 1.0]
}

impl Default for MaterialPbrMetallicRoughness {
    fn default() -> Self {
        Self {
            base_color_factor: [1.0, 1.0, 1.0, 1.0],
            base_color_texture: None,
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            metallic_roughness_texture: None,
        }
    }
}

/// The material appearance of a primitive.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Material {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A set of parameter values that are used to define the metallic-roughness material model from Physically Based Rendering (PBR) methodology. When undefined, all the default values of `pbrMetallicRoughness` **MUST** apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr_metallic_roughness: Option<MaterialPbrMetallicRoughness>,

    /// The tangent space normal texture. The texture encodes RGB components with linear transfer function. Each texel represents the XYZ components of a normal vector in tangent space. The normal vectors use the convention +X is right and +Y is up. +Z points toward the viewer. If a fourth component (A) is present, it **MUST** be ignored. When undefined, the material does not have a tangent space normal texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<MaterialNormalTextureInfo>,

    /// The occlusion texture. The occlusion values are linearly sampled from the R channel. Higher values indicate areas that receive full indirect lighting and lower values indicate no indirect lighting. If other channels are present (GBA), they **MUST** be ignored for occlusion calculations. When undefined, the material does not have an occlusion texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<MaterialOcclusionTextureInfo>,

    /// The emissive texture. It controls the color and intensity of the light being emitted by the material. This texture contains RGB components encoded with the sRGB transfer function. If a fourth component (A) is present, it **MUST** be ignored. When undefined, the texture **MUST** be sampled as having `1.0` in RGB components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<TextureInfo>,

    /// The factors for the emissive color of the material. This value defines linear multipliers for the sampled texels of the emissive texture.
    #[serde(skip_serializing_if = "is_default_emissive_factor")]
    pub emissive_factor: [f64; 3],

    /// The material's alpha rendering mode enumeration specifying the interpretation of the alpha value of the base color.
    #[serde(skip_serializing_if = "is_default")]
    pub alpha_mode: AlphaMode,

    /// Specifies the cutoff threshold when in `MASK` alpha mode. If the alpha value is greater than or equal to this value then it is rendered as fully opaque, otherwise, it is rendered as fully transparent. A value greater than `1.0` will render the entire material as fully transparent. This value **MUST** be ignored for other alpha modes. When `alphaMode` is not defined, this value **MUST NOT** be defined.
    #[serde(skip_serializing_if = "is_half")]
    pub alpha_cutoff: f64,

    /// Specifies whether the material is double sided. When this value is false, back-face culling is enabled. When this value is true, back-face culling is disabled and double-sided lighting is enabled. The back-face **MUST** have its normals reversed before the lighting equation is evaluated.
    #[serde(skip_serializing_if = "is_default")]
    pub double_sided: bool,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<MaterialExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: None,
            pbr_metallic_roughness: None,
            normal_texture: None,
            occlusion_texture: None,
            emissive_texture: None,
            emissive_factor: [0.0, 0.0, 0.0],
            alpha_mode: AlphaMode::Opaque,
            alpha_cutoff: 0.5,
            double_sided: false,
            extensions: None,
            extras: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MaterialExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

fn one() -> f64 {
    1.0
}

fn is_one(v: &f64) -> bool {
    *v == 1.0
}

fn is_default_emissive_factor(v: &[f64; 3]) -> bool {
    *v == [0.0, 0.0, 0.0]
}

fn is_half(v: &f64) -> bool {
    *v == 0.5
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material() {
        let m: Material = serde_json::from_str("{}").unwrap();
        assert_eq!(m.alpha_cutoff, 0.5);
        assert_eq!(m.emissive_factor, [0.0, 0.0, 0.0]);
        assert_eq!(m.alpha_mode, AlphaMode::Opaque);
        assert!(!m.double_sided);
        assert_eq!(serde_json::to_string(&m).unwrap(), "{}");
    }

    #[test]
    fn test_pbr_material() {
        let m: MaterialPbrMetallicRoughness = serde_json::from_str("{}").unwrap();
        assert_eq!(m.base_color_factor, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(m.metallic_factor, 1.0);
        assert_eq!(m.roughness_factor, 1.0);
        assert_eq!(serde_json::to_string(&m).unwrap(), "{}");
    }
}
