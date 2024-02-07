//! Utilities for resolving appearance data (WIP)

use crate::models::appearance::{
    self, Appearance, ParameterizedTexture, SurfaceDataProperty, X3DMaterial,
};
use hashbrown::HashMap;
use nusamai_citygml::{appearance::TextureAssociation, LocalHref};
use nusamai_geometry::LineString2;

#[derive(Debug, Default)]
pub struct ResolvableAppearance {
    textures: Vec<ParameterizedTexture>,
    materials: Vec<X3DMaterial>,
    ring_id_to_texture: HashMap<LocalHref, (u32, LineString2<'static>)>,
    surface_id_to_material: HashMap<LocalHref, u32>,
}

impl ResolvableAppearance {
    pub fn update(&mut self, app: appearance::Appearance) {
        for surface_data in app.surface_data_member {
            match surface_data {
                SurfaceDataProperty::ParameterizedTexture(mut texture) => {
                    let tex_idx = self.textures.len() as u32;
                    for tex_assoc in texture.target.drain(..) {
                        if let TextureAssociation::TexCoordList(tcl) = tex_assoc {
                            for (ring, coords) in
                                tcl.rings.into_iter().zip(tcl.coords_list.into_iter())
                            {
                                let ls = LineString2::from_raw(coords.into());
                                self.ring_id_to_texture.insert(ring, (tex_idx, ls));
                            }
                        }
                    }
                    self.textures.push(texture);
                }
                SurfaceDataProperty::X3DMaterial(mut material) => {
                    let mat_idx = self.materials.len() as u32;
                    for target in material.target.drain(..) {
                        self.surface_id_to_material.insert(target, mat_idx);
                    }
                    self.materials.push(material);
                }
                _ => unimplemented!(),
            }
        }
    }
}

impl From<Appearance> for ResolvableAppearance {
    fn from(app: appearance::Appearance) -> Self {
        let mut ret = Self {
            ..Default::default()
        };
        ret.update(app);
        ret
    }
}
