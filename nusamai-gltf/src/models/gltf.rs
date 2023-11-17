use super::accessor::Accessor;
use super::asset::Asset;
use super::buffer::{Buffer, BufferView};
use super::mesh::Mesh;
use super::node::Node;
use super::scene::Scene;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    // 必須: glTFアセットのメタデータ
    pub asset: Asset,

    // オプショナル: アクセサの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessors: Option<Vec<Accessor>>,

    // オプショナル: バッファの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<Buffer>>,

    // オプショナル: バッファビューの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<Vec<BufferView>>,

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
