use crate::extensions;

use super::{
    Accessor, Animation, Asset, Buffer, BufferView, Camera, Image, Material, Mesh, Node, Sampler,
    Scene, Skin, Texture,
};
use byteorder::{LittleEndian, WriteBytesExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
//use base64::{prelude::*, alphabet::STANDARD};
use base64::{engine::general_purpose::STANDARD, Engine as _};

enum GltfType {
    Acompanying,
    Embedded,
    Binary,
}

/// The root object for a glTF asset.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Gltf {
    /// Names of glTF extensions used in this asset.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions_used: Vec<String>,

    /// Names of glTF extensions required to properly load this asset.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions_required: Vec<String>,

    /// An array of accessors. An accessor is a typed view into a bufferView.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accessors: Vec<Accessor>,

    /// An array of keyframe animations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub animations: Vec<Animation>,

    /// Metadata about the glTF asset.
    pub asset: Asset,

    /// An array of bufferViews. A bufferView is a view into a buffer generally representing a subset of the buffer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buffer_views: Vec<BufferView>,

    /// An array of buffers. A buffer points to binary geometry, animation, or skins.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buffers: Vec<Buffer>,

    /// An array of cameras. A camera defines a projection matrix.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cameras: Vec<Camera>,

    /// An array of images. An image defines data used to create a texture.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Image>,

    /// An array of materials. A material defines the appearance of a primitive.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub materials: Vec<Material>,

    /// An array of meshes. A mesh is a set of primitives to be rendered.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meshes: Vec<Mesh>,

    /// An array of nodes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<Node>,

    /// An array of samplers. A sampler contains properties for texture filtering and wrapping modes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub samplers: Vec<Sampler>,

    /// The index of the default scene. This property MUST NOT be defined, when scenes is undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<u32>,

    /// An array of scenes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scenes: Vec<Scene>,

    /// An array of skins. A skin is defined by joints and matrices.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skins: Vec<Skin>,

    /// An array of textures.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub textures: Vec<Texture>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::gltf::Gltf>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

impl Gltf {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// GltfSequence
/// glTF のバイナリシーケンスに格納される情報
/// 頂点座標、indices、UV 座標、イメージなどが格納されている。
/// 互換性のために Dxxxx も定義

#[derive(Debug, Clone)]
pub enum GltfSequence {
    D5120(Vec<i8>),
    D5121(Vec<u8>),
    D5122(Vec<i16>),
    D5123(Vec<u16>),
    Indices(Vec<u32>),
    Coords(Vec<f32>),
    Image(Vec<u8>),
    Bin(Vec<u8>),
}

/// value sequences accompanying glTF Json
/// Json に付随するシーケンス群。

#[derive(Debug, Clone)]
pub struct GltfSeqList {
    seq_list: Vec<GltfSequence>,
}
impl GltfSeqList {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    /// push sequence to sequence list
    pub fn push(&mut self, sequence: GltfSequence) {
        self.seq_list.push(sequence);
    }
    /// シーケンスをバイナリに変換
    /// returns (binary_sequence, offset_list, size_list)
    pub fn make_bin_sequence(&self) -> (Vec<u8>, Vec<usize>, Vec<usize>) {
        let mut offset: usize = 0;
        let mut size: usize = 0;
        let mut offsets: Vec<usize> = Vec::new();
        let mut sizes: Vec<usize> = Vec::new();
        let mut bin_buf: Vec<u8> = Vec::new();

        for item in self.seq_list.iter() {
            match item {
                GltfSequence::D5120(data) => {
                    // i8 と u8 はサイズが同じなので as で型変換して push
                    for v in data {
                        bin_buf.push(*v as u8)
                    }
                    // 4 byte 境界に合わせるために
                    while bin_buf.len() % 4 != 0 {
                        bin_buf.push(0x0)
                    }
                }
                GltfSequence::D5121(data) => {
                    let mut copied = data.clone();
                    bin_buf.append(&mut copied);
                    // 4 byte 境界に合わせるために
                    while bin_buf.len() % 4 != 0 {
                        bin_buf.push(0x0)
                    }
                }
                GltfSequence::D5122(data) => {
                    for v in data.iter() {
                        let _ = bin_buf.write_i16::<LittleEndian>(*v).unwrap();
                    }
                    // 4 byte 境界に合わせるために
                    while bin_buf.len() % 4 != 0 {
                        bin_buf.push(0x0)
                    }
                }
                GltfSequence::D5123(data) => {
                    for v in data.iter() {
                        let _ = bin_buf.write_u16::<LittleEndian>(*v).unwrap();
                    }
                    // 4 byte 境界に合わせるために
                    while bin_buf.len() % 4 != 0 {
                        bin_buf.push(0x0)
                    }
                }
                GltfSequence::Indices(indices) => {
                    for v in indices.iter() {
                        let _ = bin_buf.write_u32::<LittleEndian>(*v).unwrap();
                    }
                }
                GltfSequence::Coords(coords) => {
                    for v in coords.iter() {
                        let _ = bin_buf.write_f32::<LittleEndian>(*v).unwrap();
                    }
                }
                GltfSequence::Image(image) => {
                    let mut copied = image.clone();
                    bin_buf.append(&mut copied);
                    // 4 byte 境界に合わせるために
                    while bin_buf.len() % 4 != 0 {
                        bin_buf.push(0x0)
                    }
                }
                GltfSequence::Bin(data) => {
                    let mut copied = data.clone();
                    bin_buf.append(&mut copied);
                    // 4 byte 境界に合わせるために
                    while bin_buf.len() % 4 != 0 {
                        bin_buf.push(0x0)
                    }
                }
            }
            // size = 現在の長さ - 前の offset
            size = bin_buf.len() - offset;
            sizes.push(size);
            offsets.push(offset);
            // 現在の長さ -> 次の offset
            offset = bin_buf.len();
        }
        (bin_buf, offsets, sizes)
    }
    /// シーケンスを Base64 文字列に変換
    pub fn make_base64_sequence(&self) -> (String, Vec<usize>, Vec<usize>) {
        let taple3 = self.make_bin_sequence();

        let base64str = STANDARD.encode(taple3.0);
        (base64str, taple3.1, taple3.2)
    }
}

impl Default for GltfSeqList {
    fn default() -> Self {
        Self {
            seq_list: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gltf_default() {
        let gltf = Gltf::default();
        assert_eq!(gltf.asset.version, "2.0");
        assert_eq!(gltf.asset.generator, Some("nusamai-gltf".into()));
    }

    #[test]
    fn seq_boundary() {
        let mut seq_list = GltfSeqList::new();
        let mut image: Vec<u8> = Vec::new();
        image.push(0x01);
        image.push(0x02);
        image.push(0x03);
        seq_list.push(GltfSequence::Image(image));

        let taple3 = seq_list.make_bin_sequence();
        assert_eq!(taple3.0.len(), 4);
    }
}
