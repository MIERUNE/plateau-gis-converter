use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    // 配列: シーンのルートノードのインデックス
    pub nodes: Vec<usize>,

    // オプショナル: シーンの名前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<JsonValue>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<JsonValue>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            nodes: Vec::new(),
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
