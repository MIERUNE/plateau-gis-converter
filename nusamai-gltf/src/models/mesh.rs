use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Primitive {
    // 必須: メッシュ属性セマンティクスとアクセサーインデックスのマッピング
    pub attributes: HashMap<String, usize>,

    // オプショナル: 頂点インデックスを含むアクセサーのインデックス
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<usize>,

    // オプショナル: このプリミティブに適用するマテリアルのインデックス
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<usize>,

    // オプショナル: プリミティブのトポロジータイプ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<PrimitiveMode>,

    // オプショナル: モーフターゲットの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<HashMap<String, usize>>>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<JsonValue>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<JsonValue>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveMode {
    Points = 0,
    Lines = 1,
    LineLoop = 2,
    LineStrip = 3,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    // 配列: レンダリングするためのプリミティブのセット
    pub primitives: Vec<Primitive>,

    // オプショナル: モーフターゲットに適用されるウェイトの配列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,

    // オプショナル: メッシュの名前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<JsonValue>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<JsonValue>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            primitives: Vec::new(),
            weights: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}

impl Primitive {
    pub fn new() -> Self {
        Primitive {
            attributes: HashMap::new(),
            indices: None,
            material: None,
            mode: None,
            targets: None,
            extensions: None,
            extras: None,
        }
    }
}
