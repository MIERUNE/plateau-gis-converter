
use nusamai_gltf::*;


fn main(){

    let filename = "../../assets/glTF/glTF-Sample-Models-main/2.0/Avocado/glTF/Avocado.gltf";
    //match import_gltf("../../assets/glTF/glTF-Sample-Models-main/2.0/Avocado/glTF-Binary/Avocado.glb".to_string())
    match import_gltf(filename.to_string())
    {
        Ok((gltf, seq_list)) => {
            println!("{}", gltf.to_string_pretty().unwrap());
            println!("{}", seq_list.to_string());

            match export_glb("./out.glb".to_string(), gltf, seq_list){
                Ok(())=>{
                },
                Err(e)=>{
                    match e{
                        GltfExportError::FileOpenError(msg)=>{
                            println!("FileOpenError: {}", msg);
                        },
                        GltfExportError::WriteError(msg)=>{
                            println!("WriteError: {}", msg);
                        },
                        GltfExportError::TranslationError(msg)=>{
                            println!("TranslationError: {}", msg);
                        },
                        GltfExportError::OtherError(msg)=>{
                            println!("OtherError: {}", msg);
                        }

                    }
                }
            }

        },
        Err(e)=>{
            match e{
                GltfImportError::FileOpenError(msg)=>{
                    println!("FileOpenError: {}", msg);
                },
                GltfImportError::ReadError(msg)=>{
                    println!("ReadError: {}", msg);
                },
                GltfImportError::ParseError(msg)=>{
                    println!("ParseError: {}", msg);
                },
                GltfImportError::OtherError(msg)=>{
                    println!("OtherError: {}", msg);
                }
            }
        }
    }
    
}

