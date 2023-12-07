use super::{
    Accessor, Animation, Asset, Buffer, BufferView, Camera, Image, Material, Mesh, Node, Sampler,
    Scene, Skin, Texture,
};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The root object for a glTF asset.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Gltf {
    /// Names of glTF extensions used in this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<Vec<String>>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<Vec<String>>,

    /// An array of accessors. An accessor is a typed view into a bufferView.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessors: Option<Vec<Accessor>>,

    /// An array of keyframe animations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<Animation>>,

    /// Metadata about the glTF asset.
    pub asset: Asset,

    /// An array of buffers. A buffer points to binary geometry, animation, or skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<Buffer>>,

    /// An array of bufferViews. A bufferView is a view into a buffer generally representing a subset of the buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<Vec<BufferView>>,

    /// An array of cameras. A camera defines a projection matrix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cameras: Option<Vec<Camera>>,

    /// An array of images. An image defines data used to create a texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,

    /// An array of materials. A material defines the appearance of a primitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<Material>>,

    /// An array of meshes. A mesh is a set of primitives to be rendered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meshes: Option<Vec<Mesh>>,

    /// An array of nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,

    /// An array of samplers. A sampler contains properties for texture filtering and wrapping modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samplers: Option<Vec<Sampler>>,

    /// The index of the default scene. This property MUST NOT be defined, when scenes is undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<u32>,

    /// An array of scenes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<Scene>>,

    /// An array of skins. A skin is defined by joints and matrices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins: Option<Vec<Skin>>,

    /// An array of textures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<Vec<Texture>>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GLTFExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GLTFExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

impl Gltf {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
