//! Gltf 構造体 に値を格納する関数群
//!
//! import_gltf()    : Gltf構造体に gltf ファイルを読込む
//!

use crate::{Gltf, GltfSequence, GltfSeqList};
use byteorder::{LittleEndian, WriteBytesExt};
use clap::Parser;
use earcut_rs::{utils_3d::project3d_to_2d, Earcut};
use fs::File;
use indexmap::IndexSet;
use nusamai_geometry::MultiPolygon3;
use serde_json;
use quick_xml::{
    events::Event,
    name::{Namespace, ResolveResult::Bound},
    reader::NsReader,
};

use std::str;
use std::{io::prelude::*, f32::consts::E};
use std::io::{BufReader, Read};
use std::mem::transmute;
use std::path::PathBuf;
use std::{clone::Clone, collections::HashMap, default::Default, fs, io::BufWriter};
use std::{io::Write, usize};
use url::{Url, Host, Position};
use thiserror::Error;

pub enum GltfImportError {
    FileOpenError(String),
    ReadError(String),
    ParseError(serde_json::Error),
    OtherError(String),
}

/// Vec<u8> から Json を読込み Glrf 構造体に詰める
fn parse_json(bin_json: Vec<u8>) -> Result<Gltf, GltfImportError> {
    let mut gltf = Gltf::default();

    let result : Result<Gltf, serde_json::Error> = serde_json::from_str(std::str::from_utf8(&bin_json).unwrap());

    match result{
        Ok(_gltf) => {
            gltf = _gltf;
            return Ok(gltf);
        },
        Err(e) => {
            return Err(GltfImportError::ParseError(e));
        }
    }
}

// uri から bin ファイルを読込む
fn read_bin_file(uri: String, fiename: String) -> Result<Vec<u8>, GltfImportError>{

    match Url::parse(&uri.clone()){
        Ok(url) => {
            match url.scheme(){
                "https" => {
                    // skip http accsess
                    return Err(GltfImportError::OtherError(format!("url parse error. {}", uri)));
                },
                "file" => {
                    // read local file use url.path()


                },
                _ => {
                    // noop
                }

            }

        },
        Err(_e) => {
            return Err(GltfImportError::OtherError(format!("url parse error. {}", uri)));
        }
    }


    let mut bin : Vec<u8> = Vec::new();
    
    // if uri has protocol, need http access


    return Ok(bin);
}

/// gltf ファイルの読込み
fn _import_gltf(filename: String) -> Result<(Gltf, GltfSeqList), GltfImportError> {
    let mut seq_list = GltfSeqList::new();

    let mut gltf: Gltf = Gltf::default();
    let file: fs::File;
    let result = File::open(&filename);
    match result {
        Ok(f) => {
            file = f;
        }
        Err(e) => {
            return Result::Err(GltfImportError::FileOpenError(format!(
                "File open error. {}",
                filename
            )))
        }
    }

    // file size from metadata
    let filelen : usize = file.metadata().unwrap().len().try_into().unwrap();  

    let reader = BufReader::new(file);

    // parse json and directly pack to Grtf 
    let result : Result<Gltf, serde_json::Error> = serde_json::from_reader(reader);

    match result{
        Ok(_gltf) => {
            gltf = _gltf;
        },
        Err(e) => {
            return Err(GltfImportError::ParseError(e));
        }
    }

    // read bin file
    for item in gltf.buffers.iter(){

    }







    Result::Ok((gltf, seq_list))
}

/// glb ファイルの読込み
fn _import_glb(filename: String) -> Result<(Gltf, GltfSeqList), GltfImportError> {
    let mut seq_list = GltfSeqList::new();

    let mut buffer = [0; 4];

    let file: fs::File;
    let result = File::open(&filename);
    match result {
        Ok(f) => {
            file = f;
        }
        Err(e) => {
            return Result::Err(GltfImportError::FileOpenError(format!(
                "File open error. {}",
                filename
            )))
        }
    }

    let filelen : usize = file.metadata().unwrap().len().try_into().unwrap();  

    let mut reader = BufReader::new(file);

    // current position in file
    let mut pos: usize = 0;

    // to read glb header
    if filelen < 12 {
        return Result::Err(GltfImportError::ReadError(
            "Not enough file size for glb header.".to_string(),
        ));
    }

    let mut readlen: usize;
    // read glb header
    pos += reader.read(&mut buffer).unwrap();
    if buffer != [0x67, 0x6c, 0x54, 0x46] as [u8; 4] {
        println!("{:?}", buffer);
        return Result::Err(GltfImportError::ReadError("No glTF MAGIC Number".to_string()));
    }
    // read gltf version
    pos += reader.read(&mut buffer).unwrap();
    if buffer != [0x2, 0x00, 0x00, 0x00] {
        println!("{:?}", buffer);
        return Result::Err(GltfImportError::ReadError("glTF version is not 2".to_string()));
    }
    // read overall length
    pos += reader.read(&mut buffer).unwrap();
    let length: usize = unsafe { transmute::<[u8; 4], u32>(buffer).try_into().unwrap() };
    // chck file size
    if length != filelen {
        return Result::Err(GltfImportError::ReadError("length in glb header does not match actual file size.".to_string()));
    }
    // chunk header size
    if filelen - pos < 8 {
        return Result::Err(GltfImportError::ReadError("Not enough file size for JSON chunk header.".to_string()));
    }

    // to read json chunk header
    // read chunk length
    pos += reader.read(&mut buffer).unwrap();
    let json_length: usize = unsafe { transmute::<[u8; 4], u32>(buffer).try_into().unwrap() };
    // read chunk type
    pos += reader.read(&mut buffer).unwrap();
    if buffer != [0x4a, 0x53, 0x4f, 0x4e] {
        return Result::Err(GltfImportError::ReadError("No JSON Header".to_string()));
    }
    // chunk length check
    if filelen - pos < json_length {
        return Result::Err(GltfImportError::ReadError("Not enough file size for JSON chunk.".to_string()));
    }
    let startpos = pos;
    let mut bin_json: Vec<u8> = Vec::new();
    while pos - startpos < json_length {
        pos += reader.read(&mut buffer).unwrap();
        bin_json.extend(&buffer[0..]);
    }

    println!("{}", String::from_utf8(bin_json.clone()).unwrap());

    let mut gltf: Gltf = Gltf::default();
    
    //match parse_json(binjson){
    match parse_json(bin_json){
        Ok(_gltf) => {
             gltf = _gltf;
            //println!("YES!!! {}", gltf.to_string().unwrap());
         },
        Err(GltfImportError::ParseError(e)) => {
            //println!( "some parse error occured. {:?}", e);
            return Result::Err(GltfImportError::ParseError(e));
        },
        _ => {()}
    }

    // to read bin chunk header
    // chunk header size
    if filelen - pos < 8 {
        return Result::Err(GltfImportError::ReadError("Not enough file size for JSON chunk.".to_string()));
    }
    // read chunk length
    pos += reader.read(&mut buffer).unwrap();
    let bin_length: usize = unsafe { transmute::<[u8; 4], u32>(buffer).try_into().unwrap() };
    // read chunk type
    pos += reader.read(&mut buffer).unwrap();
    if buffer != [0x42, 0x49, 0x4e, 0x00] {
        return Result::Err(GltfImportError::ReadError("No 'BIN ' Header".to_string()));
    }
    // chunk length check
    if filelen - pos < bin_length {
        return Result::Err(GltfImportError::ReadError("Not enough file size for JSON chunk.".to_string()));
    }
    // BIN chunk in glb and outer .bin file can coexitence
    let buffers = gltf.buffers.clone();
    for item in buffers.iter(){
        match &item.uri{
            Some(uri) => {
                match read_bin_file(uri.clone(), filename.clone()){
                    Ok(bin) => {
                        seq_list.push(GltfSequence::Bin(bin));
                    },
                    Err(_e) => {
                        return Result::Err(GltfImportError::ReadError(format!("Reading BIN file failed. {}", uri)));
                    }
                }
                // read from outer .bin file　
            },
            None => {
                // 
                let startpos = pos;
                let mut bin: Vec<u8> = Vec::new();
                while pos - startpos < bin_length {
                    pos += reader.read(&mut buffer).unwrap();
                    bin.extend(&buffer[0..]);
                }
                seq_list.push(GltfSequence::Bin(bin));
            }
        }
    }
    return Result::Ok((gltf, seq_list));
}

/// gltf ファイルの読込み
pub fn import_gltf(filename: String) -> Result<(Gltf, GltfSeqList), GltfImportError> {
    let pbuf = PathBuf::from(filename.clone());
    // 拡張子
    //let ext_str = pbuf.extension().to_string().unwrap().as_str();
    let ext = pbuf.extension().unwrap().to_str().unwrap();
    match ext {
        "gltf" => {return _import_gltf(filename);},
        "glb" => {return _import_glb(filename);},
        _ => { return Result::Err(GltfImportError::OtherError(format!("illegal file extension. [{}]", ext))); },
    }
}
