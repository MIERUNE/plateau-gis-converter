//! Material mangement

use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct Material {
    pub base_color: [f32; 4],
    // NOTE: Adjust the hash implementation if you add more fields
}

impl Material {
    pub fn to_gltf(&self) -> nusamai_gltf_json::Material {
        nusamai_gltf_json::Material {
            pbr_metallic_roughness: Some(nusamai_gltf_json::MaterialPbrMetallicRoughness {
                base_color_factor: to_f64x4(self.base_color),
                metallic_factor: 0.2,
                roughness_factor: 0.5,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

fn to_f64x4(c: [f32; 4]) -> [f64; 4] {
    [
        f64::from(c[0]),
        f64::from(c[1]),
        f64::from(c[2]),
        f64::from(c[3]),
    ]
}

impl Eq for Material {}

impl Hash for Material {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base_color.iter().for_each(|c| c.to_bits().hash(state));
    }
}
