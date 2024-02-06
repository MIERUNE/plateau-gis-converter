use crate::models::appearance::{
    self, Appearance, ParameterizedTexture, SurfaceDataProperty, X3DMaterial,
};
use hashbrown::HashMap;
use nusamai_citygml::{appearance::TextureAssociation, LocalHref};

pub struct ResolvableAppearance {
    pub textures: Vec<ParameterizedTexture>,
    pub materials: Vec<X3DMaterial>,
    pub ring_id_to_texture: HashMap<LocalHref, u32>,
    pub surface_id_to_material: HashMap<LocalHref, u32>,
}

impl ResolvableAppearance {}

impl From<Appearance> for ResolvableAppearance {
    fn from(app: appearance::Appearance) -> Self {
        let mut textures = Vec::new();
        let mut materials = Vec::new();
        let mut ring_id_to_texture = HashMap::new();
        let mut surface_id_to_material = HashMap::new();

        for surface_data in app.surface_data_member {
            match surface_data {
                SurfaceDataProperty::ParameterizedTexture(texture) => {
                    let tex_idx = textures.len() as u32;
                    for tex_assoc in &texture.target {
                        if let TextureAssociation::TexCoordList(tcl) = &tex_assoc {
                            for ring in &tcl.rings {
                                ring_id_to_texture.insert(*ring, tex_idx);
                            }
                        }
                    }
                    textures.push(texture);
                }
                SurfaceDataProperty::X3DMaterial(material) => {
                    let mat_idx = materials.len() as u32;
                    for target in &material.target {
                        surface_id_to_material.insert(*target, mat_idx);
                    }
                    materials.push(material);
                }
                _ => unimplemented!(),
            }
        }

        Self {
            textures,
            materials,
            ring_id_to_texture,
            surface_id_to_material,
        }
    }
}
