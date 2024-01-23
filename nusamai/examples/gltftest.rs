
use nusamai_gltf::*;

fn main(){
    match import_gltf("../../assets/glTF/glTF-Sample-Models-main/2.0/Avocado/glTF-Binary/Avocado.glb".to_string())
    {
        Ok((gltf, seq_list)) => {

            println!("YES!!! {}", gltf.to_string().unwrap());

        },
        Err(GltfImportError::FileOpenError(msg)) => {
            println!("FileOpenError: {}", msg);
        },
        Err(GltfImportError::ReadError(msg))=>{
            println!("ReadError: {}", msg);
        },
        Err(GltfImportError::OtherError(msg))=>{
            println!("OtherError: {}", msg);
        },
        _ => {}
    }

}

