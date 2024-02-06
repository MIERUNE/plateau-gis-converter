use nusamai_gltf::*;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("accept only one arg, found {}", args.len() - 1);
        process::exit(0);
    }

    println!("input file: {}", &args[1]);
    let filename = args[1].clone();

    match import_gltf(filename) {
        Ok((gltf, seq_list)) => {
            println!("{}", gltf.to_string_pretty().unwrap());
            println!("{}", seq_list.to_string());

            match export_glb("./out.glb".to_string(), gltf, seq_list) {
                Ok(()) => {}
                Err(e) => match e {
                    GltfExportError::FileOpenError(msg) => {
                        println!("FileOpenError: {}", msg);
                    }
                    GltfExportError::WriteError(msg) => {
                        println!("WriteError: {}", msg);
                    }
                    GltfExportError::TranslationError(msg) => {
                        println!("TranslationError: {}", msg);
                    }
                    GltfExportError::OtherError(msg) => {
                        println!("OtherError: {}", msg);
                    }
                },
            }
        }
        Err(e) => match e {
            GltfImportError::FileOpenError(msg) => {
                println!("FileOpenError: {}", msg);
            }
            GltfImportError::ReadError(msg) => {
                println!("ReadError: {}", msg);
            }
            GltfImportError::ParseError(msg) => {
                println!("ParseError: {}", msg);
            }
            GltfImportError::OtherError(msg) => {
                println!("OtherError: {}", msg);
            }
        },
    }
}
