use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    // オプショナル: バッファのURI（またはIRI）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    // 必須: バッファの長さ（バイト単位）
    pub byte_length: usize,

    // オプショナル: バッファの名前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    // 必須: バッファのインデックス
    pub buffer: usize,

    // デフォルト: 0, バッファ内のオフセット（バイト単位）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<usize>,

    // 必須: bufferViewの長さ（バイト単位）
    pub byte_length: usize,

    // オプショナル: 頂点属性間のストライド（バイト単位）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<usize>,

    // オプショナル: このbufferViewで使用することを意図されたGPUバッファタイプのヒント
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<BufferTarget>,

    // オプショナル: bufferViewの名前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
#[serde(rename_all = "camelCase")]
pub enum BufferTarget {
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

impl Buffer {
    pub fn new(byte_length: usize) -> Self {
        Buffer {
            uri: None,
            byte_length,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}

impl BufferView {
    pub fn new() -> Self {
        BufferView {
            buffer: 0,
            byte_offset: None,
            byte_length: 0,
            byte_stride: None,
            target: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
