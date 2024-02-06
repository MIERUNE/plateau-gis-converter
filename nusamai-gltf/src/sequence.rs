use base64::{engine::general_purpose::STANDARD, Engine as _};
use byteorder::{LittleEndian, WriteBytesExt};

/// GltfSequence
/// glTF のバイナリシーケンスに格納される情報
/// 頂点座標、indices、UV 座標、イメージなどが格納されている。

#[derive(Debug, Clone)]
pub enum GltfSequence {
    Indices(Vec<u32>),
    Coords(Vec<f32>),
    Image(Vec<u8>),
    Bin(Vec<u8>),
}

/// value sequences accompanying glTF Json
/// Json に付随するシーケンス群。

#[derive(Debug, Clone)]
pub struct GltfSeqList {
    pub sequences: Vec<GltfSequence>,
}
impl GltfSeqList {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    /// push sequence to sequence list
    pub fn push(&mut self, sequence: GltfSequence) {
        self.sequences.push(sequence);
    }
    /// シーケンスをバイナリに変換
    /// returns (binary_sequence, offset_list, size_list)
    pub fn make_bin_sequence(&self) -> (Vec<u8>, Vec<usize>, Vec<usize>) {
        let mut offset: usize = 0;
        let mut size: usize;
        let mut offsets: Vec<usize> = Vec::new();
        let mut sizes: Vec<usize> = Vec::new();
        let mut bin_buf: Vec<u8> = Vec::new();

        for item in self.sequences.iter() {
            match item {
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
            sequences: Vec::new(),
        }
    }
}

impl GltfSeqList {
    pub fn to_string(&self) -> String {
        let mut s: String = String::new();

        let mut pos: usize = 0;
        for item in self.sequences.iter() {
            match item {
                GltfSequence::Indices(v) => {
                    s.push_str(format!("Indices(offset:{}, size:{}), ", pos, v.len() * 4).as_str());
                    pos += v.len() * 4;
                }
                GltfSequence::Coords(v) => {
                    s.push_str(format!("Coords(offset:{}, size:{}), ", pos, v.len() * 4).as_str());
                    pos += v.len() * 4;
                }
                GltfSequence::Image(v) => {
                    s.push_str(format!("Image(offset:{}, size:{}), ", pos, v.len()).as_str());
                    pos += v.len();
                }
                GltfSequence::Bin(v) => {
                    s.push_str(format!("Bin(offset:{}, size:{}), ", pos, v.len()).as_str());
                    pos += v.len();
                }
            }
        }
        return s;
    }
}
