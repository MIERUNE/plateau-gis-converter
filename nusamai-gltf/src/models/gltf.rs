use super::{
    Accessor, Animation, Asset, Buffer, BufferView, Camera, Image, Material, Mesh, Node, Sampler,
    Scene, Skin, Texture,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    /// Names of glTF extensions used in this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<Vec<String>>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<Vec<String>>,

    // An array of accessors. An accessor is a typed view into a bufferView.
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

    // オプショナル: メッシュの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meshes: Option<Vec<Mesh>>,

    // オプショナル: ノードの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,

    // オプショナル: シーンの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<usize>,

    // オプショナル: シーンの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<Scene>>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Gltf {
    pub fn new() -> Self {
        Gltf {
            asset: Asset::new(),
            accessors: None,
            buffers: None,
            buffer_views: None,
            meshes: None,
            nodes: None,
            scenes: None,
            scene: None,
            extensions: None,
            extras: None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gltf_new() {
        let gltf = Gltf::new();

        assert_eq!(gltf.asset.version, "2.0");
    }
}
