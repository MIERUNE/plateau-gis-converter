//! Gltf 構造体から glTF 形式、 glb形式のイメージを製作する関数群
//!
//! make_gltf_embedded()    : バイナリ情報を glTF ファイル内にテキストで埋込んだテキストイメージを作成
//! make_gltf()             : バイナリ情報を glTF ファイルとは別に BIN 情報として作成
//! make_glb()              : バイナリ形式の glTF ファイル glb を作成
//!

use crate::{Gltf, GltfSequence, GltfSeqList};

use byteorder::{LittleEndian, WriteBytesExt};
use clap::Parser;
use earcut_rs::{utils_3d::project3d_to_2d, Earcut};
use indexmap::IndexSet;
use nusamai_geometry::MultiPolygon3;
use nusamai_gltf::*;
use quick_xml::{
    events::Event,
    name::{Namespace, ResolveResult::Bound},
    reader::NsReader,
};
use std::{clone::Clone, collections::HashMap, default::Default, fs, io::BufWriter};
use std::{io::Write, usize};
use thiserror::Error;

/// 

/// バイナリ情報を glTF ファイルとは別に BIN 情報として作成
pub fn make_gltf(&mut gltf: Gltf, seq_list:GltfSeqList) -> (String, Vec<u8>) {

}

/// バイナリ情報を glTF ファイル内にテキストで埋込んだテキストイメージを作成
pub fn make_gltf_embedded(gltf: Gltf, seq_list:GltfSeqList) -> String {

}

/// バイナリファイルを出力
pub fn make_glb(gltf_in: Gltf, seq_list_in: GltfSeqList) -> Vec<u8> {
    let mut bin: Vec<u8> = Vec::new();
    // 出力用 Gltf
    // 入力 gltf.glb ファイルの形式によって書換える
    let mut gltf : Gltf = gltf_in.clone();
    // 出力用 GltfSeqList
    let mut seq_list : Gltf = seq_list_in.clone();
    let buffer_count = seq_list.buffers.len();
    let buffer_view_count = seq_list.buffers.len();

    

}

pub fn export_gltf(filename : String)
{

}

pub fn export_glb(filename: String)
{

}
