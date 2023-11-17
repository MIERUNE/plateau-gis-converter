use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    // オプショナル: このノードが参照するカメラのインデックス
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<usize>,

    // オプショナル: このノードの子ノードのインデックスの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<usize>>,

    // オプショナル: このノードが参照するスキンのインデックス
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<usize>,

    // オプショナル: 4x4の変換行列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<[f32; 16]>,

    // オプショナル: このノードが参照するメッシュのインデックス
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<usize>,

    // オプショナル: ノードの四元数回転
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f32; 4]>,

    // オプショナル: ノードの非一様スケール
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,

    // オプショナル: ノードの平行移動
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f32; 3]>,

    // オプショナル: モーフターゲットのウェイト
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,

    // オプショナル: ノードの名前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<JsonValue>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<JsonValue>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            camera: None,
            children: None,
            skin: None,
            matrix: None,
            mesh: None,
            rotation: None,
            scale: None,
            translation: None,
            weights: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
