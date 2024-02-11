use nusamai_gltf_json::Gltf;

pub trait GltfBuilder {
    fn build(&self) -> Gltf;
}

pub struct GltfJsonBuilder {}

impl GltfBuilder for GltfJsonBuilder {
    fn build(&self) -> Gltf {
        Gltf {
            ..Default::default()
        }
    }
}

pub struct GlbBuilder {}

impl GltfBuilder for GlbBuilder {
    fn build(&self) -> Gltf {
        Gltf {
            ..Default::default()
        }
    }
}
