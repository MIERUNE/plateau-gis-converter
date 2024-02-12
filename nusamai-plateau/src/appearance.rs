//! Utilities for resolving appearance data (WIP)

use crate::models::appearance::{self, ParameterizedTexture, SurfaceDataProperty, X3DMaterial};
use hashbrown::HashMap;
use nusamai_citygml::{appearance::TextureAssociation, Color, LocalId, SurfaceSpan, URI};
use nusamai_geometry::LineString2;
use std::hash::{Hash, Hasher};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    ring_id_to_texture: HashMap<LocalId, (u32, LineString2<'static>)>,
    surface_id_to_material: HashMap<LocalId, u32>,
}

/// Material (CityGML's X3DMaterial)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Material {
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub ambient_intensity: f64,
    // TOOD: other parameters
}

impl From<X3DMaterial> for Material {
    fn from(src: X3DMaterial) -> Self {
        Self {
            diffuse_color: src.diffuse_color.unwrap_or(Color::new(0.8, 0.8, 0.8)),
            specular_color: src.specular_color.unwrap_or(Color::new(1., 1., 1.)),
            ambient_intensity: src.ambient_intensity.unwrap_or(0.2),
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse_color: Color::new(0.8, 0.8, 0.8),
            specular_color: Color::new(1., 1., 1.),
            ambient_intensity: 0.2,
        }
    }
}

impl Hash for Material {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.diffuse_color.hash(state);
        self.specular_color.hash(state);
        self.ambient_intensity.to_bits().hash(state);
    }
}
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AppearanceStore {
    textures: Vec<Texture>,
    materials: Vec<Material>,
    themes: HashMap<String, Theme>,
}

/// Texture (CityGML's ParameterizedTexture)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Texture {
    pub image_uri: String,
    // TOOD: other parameters
}

impl From<ParameterizedTexture> for Texture {
    fn from(src: ParameterizedTexture) -> Self {
        let url = src.image_uri.unwrap_or_else(|| {
            log::warn!("image_uri is not set");
            URI::new("url_not_found.jpg")
        });
        Self {
            image_uri: url.value().to_string(),
        }
    }
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
                    self.textures.push(texture.into());
                }
                SurfaceDataProperty::X3DMaterial(mut material) => {
                    let mat_idx = self.materials.len() as u32;
                    for target in material.target.drain(..) {
                        theme.surface_id_to_material.insert(target, mat_idx);
                    }
                    self.materials.push(material.into());
                }
                _ => unimplemented!(),
            }
        }
    }

    pub fn merge_global(
        &mut self,
        other: &mut Self,
        ring_ids: &[Option<LocalId>],
        surface_spans: &[SurfaceSpan],
    ) {
        // merge texture
        {
            let mut idx_map = indexmap::IndexSet::new();
            let base_idx = self.textures.len();

            for (theme_name, theme_src) in other.themes.iter_mut() {
                let entries: Vec<_> = ring_ids
                    .iter()
                    .filter_map(|v| *v)
                    .filter_map(|ring_id| {
                        if let Some((idx, ls)) = theme_src.ring_id_to_texture.remove(&ring_id) {
                            let (offset, inserted) = idx_map.insert_full(idx);
                            if inserted {
                                self.textures.push(other.textures[idx as usize].clone());
                            }
                            Some((ring_id, ((base_idx + offset) as u32, ls)))
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
                let entries: Vec<_> = surface_spans
                    .iter()
                    .map(|span| span.id)
                    .filter_map(|surface_id| {
                        if let Some(idx) = theme_src.surface_id_to_material.remove(&surface_id) {
                            let (offset, inserted) = idx_map.insert_full(idx);
                            if inserted {
                                self.materials.push(other.materials[idx as usize].clone());
                            }
                            Some((surface_id, (base_idx + offset) as u32))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_appearance() {
        let mut app_local = AppearanceStore::default();
        let mut app_global = AppearanceStore::default();

        {
            app_local.textures.push(Texture {
                image_uri: "local1.jpg".to_string(),
            });
            app_local.textures.push(Texture {
                image_uri: "local2.jpg".to_string(),
            });
            app_local.textures.push(Texture {
                image_uri: "local3.jpg".to_string(),
            });
            app_local.materials.push(Material::default());
            app_local.materials.push(Material::default());
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
            app_global.textures.push(Texture {
                image_uri: "global1.jpg".to_string(),
            });
            app_global.textures.push(Texture {
                image_uri: "global2.jpg".to_string(),
            });
            app_global.textures.push(Texture {
                image_uri: "global3.jpg".to_string(),
            });
            app_global.materials.push(Material::default());
            app_global.materials.push(Material::default());
            app_global.materials.push(Material::default());
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
            [3, 4, 5, 99]
                .into_iter()
                .map(|id| Some(LocalId(id)))
                .collect::<Vec<_>>()
                .as_slice(),
            [3, 4, 5, 99]
                .into_iter()
                .map(|id| SurfaceSpan {
                    id: LocalId(id),
                    start: 0,
                    end: 0,
                })
                .collect::<Vec<_>>()
                .as_slice(),
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
