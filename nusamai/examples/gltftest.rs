
use nusamai_gltf::*;
use nusamai_gltf::import::import_gltf;
use nusamai_gltf::import::GltfReadError;

fn main(){
    match import_gltf("../../assets/glTF/glTF-Sample-Models-main/2.0/Avocado/glTF-Binary/Avocado.glb
    ".to_string()){
        Ok((gltf, seq_list)) => {

        },
        Err(GltfReadError(msg)) => {
            printrn(msg);
        }
    }

}

