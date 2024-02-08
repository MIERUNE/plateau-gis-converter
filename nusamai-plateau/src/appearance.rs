//! Utilities for resolving appearance data (WIP)

use crate::models::appearance::{
    self, Appearance, ParameterizedTexture, SurfaceDataProperty, X3DMaterial,
};
use hashbrown::HashMap;
use nusamai_citygml::{appearance::TextureAssociation, LocalId};
use nusamai_geometry::LineString2;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AppearanceStore {
    textures: Vec<ParameterizedTexture>,
    materials: Vec<X3DMaterial>,
    themes: HashMap<String, Theme>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    ring_id_to_texture: HashMap<LocalId, (u32, LineString2<'static>)>,
    surface_id_to_material: HashMap<LocalId, u32>,
}

impl AppearanceStore {
    pub fn update(&mut self, app: appearance::Appearance) {
        let theme_name = app.theme.unwrap_or("default".to_string());
        let theme = self.themes.entry(theme_name).or_default();

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
                                theme.ring_id_to_texture.insert(ring, (tex_idx, ls));
                            }
                        }
                    }
                    self.textures.push(texture);
                }
                SurfaceDataProperty::X3DMaterial(mut material) => {
                    let mat_idx = self.materials.len() as u32;
                    for target in material.target.drain(..) {
                        theme.surface_id_to_material.insert(target, mat_idx);
                    }
                    self.materials.push(material);
                }
                _ => unimplemented!(),
            }
        }
    }

    pub fn merge_global(
        &mut self,
        other: &mut Self,
        ring_ids: &[LocalId],
        surface_ids: &[LocalId],
    ) {
        // merge texture
        {
            let mut idx_map = indexmap::IndexSet::new();
            let base_idx = self.textures.len();

            for (theme_name, theme_src) in other.themes.iter_mut() {
                let entries: Vec<_> = ring_ids
                    .iter()
                    .filter_map(|ring_id| {
                        if let Some((idx, ls)) = theme_src.ring_id_to_texture.remove(ring_id) {
                            let (offset, inserted) = idx_map.insert_full(idx);
                            if inserted {
                                self.textures.push(other.textures[idx as usize].clone());
                            }
                            Some((*ring_id, ((base_idx + offset) as u32, ls)))
                        } else {
                            None
                        }
                    })
                    .collect();

                self.themes
                    .entry_ref(theme_name)
                    .or_default()
                    .ring_id_to_texture
                    .extend(entries);
            }
        }

        // merge material
        {
            let mut idx_map = indexmap::IndexSet::new();
            let base_idx = self.materials.len();

            for (theme_name, theme_src) in other.themes.iter_mut() {
                let entries: Vec<_> = surface_ids
                    .iter()
                    .filter_map(|surface_id| {
                        if let Some(idx) = theme_src.surface_id_to_material.remove(surface_id) {
                            let (offset, inserted) = idx_map.insert_full(idx);
                            if inserted {
                                self.materials.push(other.materials[idx as usize].clone());
                            }
                            Some((*surface_id, (base_idx + offset) as u32))
                        } else {
                            None
                        }
                    })
                    .collect();

                self.themes
                    .entry_ref(theme_name)
                    .or_default()
                    .surface_id_to_material
                    .extend(entries);
            }
        }
    }
}

impl From<Appearance> for AppearanceStore {
    fn from(app: appearance::Appearance) -> Self {
        let mut ret = Self {
            ..Default::default()
        };
        ret.update(app);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_appearance() {
        use crate::models::appearance::{ParameterizedTexture, X3DMaterial};

        let mut app_local = AppearanceStore::default();
        let mut app_global = AppearanceStore::default();

        {
            app_local.textures.push(ParameterizedTexture::default());
            app_local.textures.push(ParameterizedTexture::default());
            app_local.textures.push(ParameterizedTexture::default());
            app_local.materials.push(X3DMaterial::default());
            app_local.materials.push(X3DMaterial::default());
            let theme = app_local.themes.entry("default".to_string()).or_default();
            theme
                .ring_id_to_texture
                .insert(LocalId::new(0), (0, LineString2::default()));
            theme
                .ring_id_to_texture
                .insert(LocalId::new(1), (0, LineString2::default()));
            theme.surface_id_to_material.insert(LocalId::new(0), 0);
            theme.surface_id_to_material.insert(LocalId::new(1), 1);
            theme.surface_id_to_material.insert(LocalId::new(2), 1);

            assert_eq!(app_local.materials.len(), 2);
            assert_eq!(app_local.textures.len(), 3);
        }

        {
            app_global.textures.push(ParameterizedTexture::default());
            app_global.textures.push(ParameterizedTexture::default());
            app_global.textures.push(ParameterizedTexture::default());
            app_global.materials.push(X3DMaterial::default());
            app_global.materials.push(X3DMaterial::default());
            app_global.materials.push(X3DMaterial::default());
            let theme = app_global.themes.entry("default".to_string()).or_default();
            theme
                .ring_id_to_texture
                .insert(LocalId::new(3), (0, LineString2::default()));
            theme
                .ring_id_to_texture
                .insert(LocalId::new(4), (1, LineString2::default()));
            theme
                .ring_id_to_texture
                .insert(LocalId::new(5), (1, LineString2::default()));
            theme.surface_id_to_material.insert(LocalId::new(3), 0);
            theme.surface_id_to_material.insert(LocalId::new(4), 1);
            theme.surface_id_to_material.insert(LocalId::new(5), 1);
        }

        // merge global to local
        app_local.merge_global(
            &mut app_global,
            &[LocalId(3), LocalId(4), LocalId(5), LocalId(99)],
            &[LocalId(3), LocalId(4), LocalId(5), LocalId(99)],
        );

        // check merge result
        assert_eq!(app_local.textures.len(), 5);
        assert_eq!(app_local.materials.len(), 4);

        let theme = app_local.themes.entry("default".to_string()).or_default();

        assert!(theme.ring_id_to_texture[&LocalId(3)].0 >= 3);
        assert!(theme.ring_id_to_texture[&LocalId(4)].0 >= 3);
        assert!(theme.ring_id_to_texture[&LocalId(5)].0 >= 3);

        assert!(theme.surface_id_to_material[&LocalId(3)] >= 2);
        assert!(theme.surface_id_to_material[&LocalId(4)] >= 2);
        assert!(theme.surface_id_to_material[&LocalId(5)] >= 2);
    }
}
