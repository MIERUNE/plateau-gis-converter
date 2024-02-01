//! Gltf 構造体 に値を格納する関数群
//!
//! import_gltf()    : Gltf構造体に gltf ファイルを読込む
//!

use crate::sequence::{GltfSeqList, GltfSequence};
use fs::File;
use nusamai_gltf_json::models::*;
use serde_json;

use base64::prelude::*;
use std::io::{BufReader, Read};
use std::mem::transmute;
use std::path::{Path, PathBuf};
use std::{clone::Clone, fs};
use url::Url;

pub enum GltfImportError {
    FileOpenError(String),
    ReadError(String),
    ParseError(serde_json::Error),
    OtherError(String),
}

/// Vec<u8> から Json を読込み Glrf 構造体に詰める
fn parse_json(bin_json: Vec<u8>) -> Result<Gltf, GltfImportError> {
    let result: Result<Gltf, serde_json::Error> =
        serde_json::from_str(std::str::from_utf8(&bin_json).unwrap());

    match result {
        Ok(gltf) => {
            return Ok(gltf);
        }
        Err(e) => {
            return Err(GltfImportError::ParseError(e));
        }
    }
}

// ファイルからバイナリデータを読込む。（bin, png, etc.）
fn _read_bin_file(filename: String) -> Result<Vec<u8>, GltfImportError> {
    println!("_read_bin_file: {}", filename);

    let file: fs::File;
    let result = File::open(&filename);
    match result {
        Ok(f) => {
            file = f;
        }
        Err(_e) => {
            return Result::Err(GltfImportError::FileOpenError(format!(
                "File open error. {}",
                filename
            )))
        }
    }
    let filelen: usize = file.metadata().unwrap().len().try_into().unwrap();
    println!("_read_bin_file: file size:{}", filelen);
    let mut bin: Vec<u8> = Vec::new();
    let mut buffer = [0; 4];
    let mut reader = BufReader::new(file);
    let mut readlen: usize;
    loop {
        readlen = reader.read(&mut buffer).unwrap();
        bin.extend(&buffer[0..]);
        if readlen == 0 {
            break;
        }
    }
    return Result::Ok(bin);
}

// uri から bin ファイルを読込む
// uri が path を含んでいない場合は入力ファイル（filename）のディレクトリからファイルを探す。
fn read_bin_file(uri: String, filename: String) -> Result<Vec<u8>, GltfImportError> {
    //println!("read_bin_file: uri:{}  filename:{}", uri, filename);
    // uri に BIN が Base64 で格納されているか確認
    let head = String::from("data:application/octet-stream;base64,");
    match uri.find(&head) {
        Some(pos) => {
            // base64 で BIN データが格納されている。
            if pos == 0 {
                //println!("read_bin_file: uri contains base64.");
                match BASE64_URL_SAFE.decode(&uri[head.len()..]) {
                    Ok(v) => {
                        return Ok(v);
                    }
                    Err(_e) => {
                        return Result::Err(GltfImportError::OtherError(
                            "Base64 buffer decode Error.".to_string(),
                        ));
                    }
                }
            } else {
                // ここに来ることはなさそう
                return Result::Err(GltfImportError::OtherError(
                    "Illegal base64 buffer.".to_string(),
                ));
            }
        }
        None => {
            // Noop
        }
    }
    // uri が url であるか確認
    match Url::parse(&uri) {
        Ok(url) => {
            //println!("read_bin_file: uri is url.");
            match url.scheme() {
                "https" => {
                    return Err(GltfImportError::OtherError(
                        "http file access currently not supported.".to_string(),
                    ));
                }
                "file" => {
                    // ローカルファイルをフルパスで指定する？! 一応読むけど
                    match _read_bin_file(String::from(url.path())) {
                        Ok(v) => {
                            return Ok(v);
                        }
                        Err(e) => {
                            return Result::Err(e);
                        }
                    }
                }
                _ => {
                    return Err(GltfImportError::OtherError(format!(
                        "buffer uri scheme {} not supported.",
                        url.scheme()
                    )));
                }
            }
        }
        Err(_e) => {
            // Noop
        }
    }
    // assumed normal file
    let path = Path::new(&uri);
    // 実行ディレクトリ直下に存在？
    if path.is_file() {
        match _read_bin_file(String::from(path.to_str().unwrap())) {
            Ok(v) => {
                return Ok(v);
            }
            Err(e) => {
                return Result::Err(e);
            }
        }
    }
    // 入力 gltf から相対パスかも
    let pathbuf = PathBuf::from(&filename);
    match pathbuf.parent() {
        Some(_parent) => {
            let mut parent = PathBuf::from(_parent);
            parent.push(path);
            match _read_bin_file(String::from(parent.to_str().unwrap())) {
                Ok(v) => {
                    return Ok(v);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        None => {
            // Noop
        }
    }
    return Err(GltfImportError::OtherError(
        "Unexpected error in read_bin_file().".to_string(),
    ));
}

/// gltf ファイルの読込み
fn _import_gltf(filename: String) -> Result<(Gltf, GltfSeqList), GltfImportError> {
    let mut seq_list = GltfSeqList::new();

    let file: fs::File;
    let result = File::open(&filename);
    match result {
        Ok(f) => {
            file = f;
        }
        Err(_e) => {
            return Err(GltfImportError::FileOpenError(format!(
                "File open error. {}",
                filename
            )))
        }
    }

    let reader = BufReader::new(file);

    // parse json and directly pack to Grtf
    let result: Result<Gltf, serde_json::Error> = serde_json::from_reader(reader);

    let gltf;
    match result {
        Ok(_gltf) => {
            gltf = _gltf;
        }
        Err(e) => {
            return Err(GltfImportError::ParseError(e));
        }
    }

    // read bin file
    for item in gltf.buffers.iter() {
        match &item.uri {
            Some(uri) => match read_bin_file(uri.clone(), filename.clone()) {
                Ok(v) => {
                    seq_list.push(GltfSequence::Bin(v));
                }
                Err(e) => {
                    return Err(e);
                }
            },
            None => {
                // if gltf having None uri become error
                return Err(GltfImportError::OtherError("No uri in buffer".to_string()));
            }
        }
    }

    // read image file
    for item in gltf.images.iter() {
        match &item.uri {
            Some(uri) => match read_bin_file(uri.clone(), filename.clone()) {
                Ok(v) => {
                    seq_list.push(GltfSequence::Image(v));
                }
                Err(e) => {
                    return Err(e);
                }
            },
            None => {
                // if gltf having None uri become error.
                return Err(GltfImportError::OtherError("No uri in image".to_string()));
            }
        }
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
        Err(_e) => {
            return Result::Err(GltfImportError::FileOpenError(format!(
                "File open error. {}",
                filename
            )))
        }
    }

    let filelen: usize = file.metadata().unwrap().len().try_into().unwrap();

    let mut reader = BufReader::new(file);

    // current position in file
    let mut pos: usize = 0;

    // to read glb header
    if filelen < 12 {
        return Result::Err(GltfImportError::ReadError(
            "Not enough file size for glb header.".to_string(),
        ));
    }

    // read glb header
    pos += reader.read(&mut buffer).unwrap();
    if buffer != [0x67, 0x6c, 0x54, 0x46] as [u8; 4] {
        //println!("{:?}", buffer);
        return Result::Err(GltfImportError::ReadError(
            "No glTF MAGIC Number".to_string(),
        ));
    }
    // read gltf version
    pos += reader.read(&mut buffer).unwrap();
    if buffer != [0x2, 0x00, 0x00, 0x00] {
        //println!("{:?}", buffer);
        return Result::Err(GltfImportError::ReadError(
            "glTF version is not 2".to_string(),
        ));
    }
    // read overall length
    pos += reader.read(&mut buffer).unwrap();
    let length: usize = unsafe { transmute::<[u8; 4], u32>(buffer).try_into().unwrap() };
    // chck file size
    if length != filelen {
        return Result::Err(GltfImportError::ReadError(
            "length in glb header does not match actual file size.".to_string(),
        ));
    }
    // chunk header size
    if filelen - pos < 8 {
        return Result::Err(GltfImportError::ReadError(
            "Not enough file size for JSON chunk header.".to_string(),
        ));
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
        return Result::Err(GltfImportError::ReadError(
            "Not enough file size for JSON chunk.".to_string(),
        ));
    }
    let startpos = pos;
    let mut bin_json: Vec<u8> = Vec::new();
    while pos - startpos < json_length {
        pos += reader.read(&mut buffer).unwrap();
        bin_json.extend(&buffer[0..]);
    }

    println!("{}", String::from_utf8(bin_json.clone()).unwrap());

    let gltf: Gltf;

    //match parse_json(binjson){
    match parse_json(bin_json) {
        Ok(_gltf) => {
            gltf = _gltf;
            //println!("YES!!! {}", gltf.to_string().unwrap());
        }
        Err(GltfImportError::ParseError(e)) => {
            //println!( "some parse error occured. {:?}", e);
            return Result::Err(GltfImportError::ParseError(e));
        }
        _ => {
            return Result::Err(GltfImportError::OtherError("Unknown error".to_string()));
        }
    }

    // to read bin chunk header
    // chunk header size
    if filelen - pos < 8 {
        return Result::Err(GltfImportError::ReadError(
            "Not enough file size for JSON chunk.".to_string(),
        ));
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
        return Result::Err(GltfImportError::ReadError(
            "Not enough file size for JSON chunk.".to_string(),
        ));
    }
    // BIN chunk in glb and outer .bin file can coexitence
    let buffers = gltf.buffers.clone();
    for item in buffers.iter() {
        match &item.uri {
            Some(uri) => {
                match read_bin_file(uri.clone(), filename.clone()) {
                    Ok(bin) => {
                        seq_list.push(GltfSequence::Bin(bin));
                    }
                    Err(_e) => {
                        return Result::Err(GltfImportError::ReadError(format!(
                            "Reading BIN file failed. {}",
                            uri
                        )));
                    }
                }
                // read from outer .bin file
            }
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
    let path = Path::new(filename.as_str());
    // 拡張子
    let ext = String::from(path.extension().unwrap().to_str().unwrap());
    let lcext = ext.to_lowercase();
    match lcext.as_str() {
        "gltf" => {
            return _import_gltf(filename);
        }
        "glb" => {
            return _import_glb(filename);
        }
        _ => {
            return Result::Err(GltfImportError::OtherError(format!(
                "illegal file extension. [{}]",
                ext
            )));
        }
    }
}
