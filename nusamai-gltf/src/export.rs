//! Gltf 構造体から glTF 形式、 glb形式のイメージを製作する関数群
//!
//! make_gltf_embedded()    : バイナリ情報を glTF ファイル内にテキストで埋込んだテキストイメージを作成
//! make_gltf()             : バイナリ情報を glTF ファイルとは別に BIN 情報として作成
//! make_glb()              : バイナリ形式の glTF ファイル glb を作成
//! 
//! export_glb()
//! export_gltf()
//! export_gltf_embedded()
//!

use nusamai_gltf_json::models::*;
use crate::sequence::GltfSeqList;

use std::path::Path;
use std::{clone::Clone, default::Default, fs};
use std::{io::Write, usize};

pub enum GltfExportError {
    FileOpenError(String),
    WriteError(String),
    TranslationError(String),
    OtherError(String),
}

/// 
pub fn mime_type_from_uri(uri: &String)->Result<MimeType, GltfExportError> 
{
    println!("uri:{}", uri);
    let path = Path::new(&uri);
    let ext = String::from(path.extension().unwrap().to_str().unwrap());
    let lc_ext = ext.to_lowercase();
    println!("extension:{}", lc_ext);
    match lc_ext.as_str(){
        "jpeg" => {
            return Ok(MimeType::ImageJpeg);
        },
        "jpg" => {
            return Ok(MimeType::ImageJpeg);
        },
        "png" => {
            return Ok(MimeType::ImagePng);
        },
        "webp" => {
            return Ok(MimeType::ImageWebP);
        },
        _ =>{ }
    }
    return Err(GltfExportError::OtherError(format!("Can not tell mime type from uri. {}", uri)));
}

/// バイナリ情報を glTF ファイルとは別に BIN 情報として作成
pub fn make_gltf(_gltf: Gltf, _seq_list:GltfSeqList) -> Result<Vec<u8>, GltfExportError> {

    return Err(GltfExportError::OtherError("not implemented yet.".to_string()));
}

/// バイナリ情報を glTF ファイル内にテキストで埋込んだテキストイメージを作成
pub fn make_gltf_embedded(_gltf: Gltf, _seq_list:GltfSeqList) -> Result<String, GltfExportError> {

    return Err(GltfExportError::OtherError("not implemented yet.".to_string()));
}

/// バイナリファイルを出力
pub fn make_glb(gltf_in: Gltf, seq_list: GltfSeqList) -> Result<Vec<u8>, GltfExportError> {
    let mut bin: Vec<u8> = Vec::new();
    // 出力用 Gltf 構造体
    // 入力 gltf, glb ファイルの形式によって書換える
    // glb
    // buffers -> 要素は１つ
    // images -> 登場順に
    let mut gltf : Gltf = gltf_in.clone();
    // 出力用 GltfSeqList;

    // bufferView がない場合には 0
    for item in gltf.accessors.iter_mut(){
        match item.buffer_view{
            Some(_n)=>{
                // noop
            },
            None=>{
                item.buffer_view = Some(0);
            }
        }
    }
    // bufferView がない場合には 0 にする
    for item in gltf.accessors.iter_mut(){
        match item.buffer_view{
            Some(_n)=>{
            },
            None=>{
                item.buffer_view = Some(0);
            }
        }
    }
    // images の書換え
    let buffer_view_count: u32 = gltf.buffer_views.len() as u32;
    let mut index: u32 = buffer_view_count;
    for item in gltf.images.iter_mut(){
        match &item.mime_type{
            Some(_t)=>{
            },
            None=>{
                //return Result::Err(GltfExportError::OtherError("can not get mime type from extension.".to_string()));
            }
        }
        match &item.uri{
            Some(uri)=>{
                match mime_type_from_uri(&uri){
                    Ok(m)=>{
                        // mimeType, bufferView を追加し、uri を削除
                        item.mime_type = Some(m);
                        item.uri = None;
                        item.buffer_view = Some(index);
                        index += 1;
                    },
                    Err(e)=>{
                        return Err(e);
                    }
                }
            },
            None=>{
            }
        }
    }
    // binary sequence と offset list, size list を取得
    let (bin_seq, offsets, sizes) = seq_list.make_bin_sequence();
    println!("offsets:{:?}", offsets);
    println!("sizes:{:?}", sizes);

    println!("buffer_view_count:{}", buffer_view_count);
    // bufferViews の書換え
    // 
    // bin_seq
    // 先頭の bin 群は glb 化前の buffer 個分存在する。
    // [bin]...[bin][image]...[image]
    let buffer_count = gltf.buffers.len();
    // 参照する buffer はすべて 0 に
    // offset を buffer を統合した値に書換える。
    for i in  0..gltf.buffer_views.len(){
        match gltf.buffer_views.get_mut(i){
            Some(bv)=>{
                (*bv).byte_offset = offsets[(*bv).buffer as usize] as u32 + (*bv).byte_offset;
                //item.byte_length はそのまま
                (*bv).buffer = 0;
            },
            None=>{
                return Err(GltfExportError::OtherError(format!("bufferView index out of range. {}", i)));
            }   
        }
    }
    // bufferView の追加
    for i in buffer_count..offsets.len(){
        let mut buffer_view: BufferView = BufferView::default();
        buffer_view.buffer = 0;
        buffer_view.byte_offset = *(offsets.get(i).unwrap()) as u32;
        buffer_view.byte_length = *(sizes.get(i).unwrap()) as u32;
        gltf.buffer_views.push(buffer_view);
    }

    // buffers の書換え
    //   buffers を空に
    gltf.buffers.clear();
    let mut buffer: Buffer = Buffer::default();
    // １つの buffer に byteLength のみ
    buffer.byte_length = bin_seq.len() as u32;
    gltf.buffers.push(buffer);


    println!("{}", gltf.to_string_pretty().unwrap());

    // bin へ書込み
    let json: String = gltf.to_string().unwrap();
    let json_chunk = json.as_bytes();
    let json_chunk_padded = {
        let mut v = json_chunk.to_vec();
        while v.len() % 4 != 0 {
            v.push(0x20); // 4バイト境界に合わせるために0x20(=space)でパディング
        }
        v
    };
    let json_chunk_len = json_chunk_padded.len() as u32;

    // glb header + json chunk header + chunk length + bin chunk header + chunk length
    let total_length:u32 = 3*4 + 2*4 + json_chunk_len + 2*4 + bin_seq.len() as u32; 

    let version:u32 = 2;

    // glb header
    let glb_magic:[u8;4] = [0x67, 0x6c, 0x54, 0x46]; // "glTF"
    let _ = bin.write_all(&glb_magic);
    let _ = bin.write_all(&version.to_le_bytes());
    let _ = bin.write_all(&total_length.to_le_bytes());

    // JSONチャンクの書き込み
    let json_chunk_type:[u8;4] = [0x4a, 0x53, 0x4f, 0x4e]; // "JSON"
    let _ = bin.write_all(&json_chunk_len.to_le_bytes());
    let _ = bin.write_all(&json_chunk_type);
    let _ = bin.write_all(&json_chunk_padded);

    let bin_length: u32 = bin_seq.len() as u32; 
    // バイナリチャンクの書き込み
    let bin_chunk_type:[u8;4] = [0x42, 0x49, 0x4e, 0x00]; // "BIN "
    let _ = bin.write_all(&bin_length.to_le_bytes());
    let _ = bin.write_all(&bin_chunk_type); // "BIN "
    let _ = bin.write_all(&bin_seq);

    return Ok(bin);
}

pub fn export_gltf(_filename : String, _gltf: Gltf, _seq_list: GltfSeqList) -> Result<(), GltfExportError>
{

    return Err(GltfExportError::OtherError("not implemented yet.".to_string()));
}

pub fn export_glb(filename: String, gltf: Gltf, seq_list: GltfSeqList) -> Result<(), GltfExportError>
{
    match make_glb(gltf, seq_list){
        Ok(v) => {
            println!("export_glb v.len() : {}", v.len());
            // ファイルを作成
            println!("export_glb file : {}", filename.clone());
            match fs::File::create(filename){
                Ok(mut file)=>{
                    println!("export_glb file open succeeded.");
                    // ファイルの書き込み
                    let _ = file.write_all(v.as_slice());
                    let _ = file.flush();
                },
                Err(e)=>{
                    println!("export_glb file open error.");
                    return Err(GltfExportError::FileOpenError(e.to_string()));
                }
            }
        },
        Err(e)=>{
            return Err(e);
        }
    }
    return Ok(());
}
