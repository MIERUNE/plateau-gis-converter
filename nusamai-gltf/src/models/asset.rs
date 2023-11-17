use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    // 必須: glTFのバージョンを指定するフィールド
    pub version: String,

    // オプショナル: glTFアセットを生成したツールやライブラリを指定するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,

    // オプショナル: 著作権に関する情報を指定するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    // オプショナル: glTFの前のバージョンの最小互換バージョンを指定するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<JsonValue>,

    // オプショナル: アセットにカスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<JsonValue>,
}

impl Asset {
    pub fn new() -> Self {
        Asset {
            version: "2.0".to_string(),
            generator: None,
            copyright: None,
            min_version: None,
            extensions: None,
            extras: None,
        }
    }
}
