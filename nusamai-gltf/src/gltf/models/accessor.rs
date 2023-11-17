use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
    // オプショナル: bufferViewのインデックス
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<usize>,

    // デフォルト: 0, bufferViewが未定義の場合は定義してはならない
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<usize>,

    // 必須: アクセサーのコンポーネントのデータ型
    pub component_type: ComponentType,

    // デフォルト: false, FLOATまたはUNSIGNED_INTの場合はtrueにしてはならない
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,

    // 必須: このアクセサーによって参照される要素の数
    pub count: usize,

    // 必須: アクセサーの要素がスカラー、ベクター、またはマトリックスかを指定
    #[serde(rename = "type")]
    pub accessor_type: AccessorType,

    // オプショナル: 各コンポーネントの最大値
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f32>>,

    // オプショナル: 各コンポーネントの最小値
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f32>>,

    // オプショナル: 要素が初期値から逸脱する際のスパースなストレージ
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse: Option<Sparse>,

    // オプショナル: アクセサーの名前
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<JsonValue>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<JsonValue>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
#[serde(rename_all = "camelCase")]
pub enum ComponentType {
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    Float = 5126,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sparse {
    // 必須: 疎配列に格納される異なるアクセサ値の数
    pub count: usize,

    // 必須: 疎配列のインデックスを含むバッファビューを指すオブジェクト
    pub indices: SparseIndices,

    // 必須: 疎配列の値を含むバッファビューを指すオブジェクト
    pub values: SparseValues,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SparseIndices {
    // 必須: 疎配列のインデックスを含むバッファビューのインデックス
    pub buffer_view: usize,

    // デフォルト: 0, バッファビューの開始からのオフセット（バイト単位）
    pub byte_offset: Option<usize>,

    // 必須: インデックスのデータタイプ
    pub component_type: ComponentType,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SparseValues {
    // 必須: 疎配列の値を含むバッファビューのインデックス
    pub buffer_view: usize,

    // デフォルト: 0, バッファビューの開始からのオフセット（バイト単位）
    pub byte_offset: Option<usize>,

    // オプショナル: 拡張機能に関するフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,

    // オプショナル: カスタムプロパティを追加するためのフィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Accessor {
    pub fn new() -> Self {
        Accessor {
            buffer_view: None,
            byte_offset: None,
            component_type: ComponentType::Float,
            normalized: None,
            count: 0,
            accessor_type: AccessorType::Scalar,
            max: None,
            min: None,
            sparse: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
