//! Utilities for resolving appearance data (WIP)

use std::hash::{Hash, Hasher};

use flatgeom::LineString2;
use hashbrown::HashMap;
use nusamai_citygml::{appearance::TextureAssociation, Color, LocalId, SurfaceSpan};
use url::Url;

use crate::models::appearance::{self, ParameterizedTexture, SurfaceDataProperty, X3DMaterial};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub ring_id_to_texture: HashMap<LocalId, (u32, LineString2<'static>)>, // TODO: texture index is redundant
    pub surface_id_to_material: HashMap<LocalId, u32>,
    /// Mapping from surface ID to list of ring IDs (for texture targets)
    /// This preserves the surface-to-ring relationship needed for CityGML appearance output
    pub surface_id_to_rings: HashMap<LocalId, Vec<LocalId>>,
}

/// Material (CityGML's X3DMaterial)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Material {
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub ambient_intensity: f64,
    // TOOD: other parameters
    // Note: Adjust the Hash implementation if you add a new field
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
    pub textures: Vec<Texture>,
    pub materials: Vec<Material>,
    pub themes: HashMap<String, Theme>,
}

/// Texture (CityGML's ParameterizedTexture)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Texture {
    pub image_url: Url,
    // TODO: other parameters
}

impl From<ParameterizedTexture> for Texture {
    fn from(src: ParameterizedTexture) -> Self {
        let image_url = src
            .image_uri
            .map(|uri| uri.into_inner())
            .unwrap_or_else(|| {
                log::warn!("image_uri is not set");
                url::Url::parse("url_not_found.jpg").unwrap()
            });
        Self { image_url }
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
                            let surface_id = tcl.target.clone();
                            let mut ring_ids = Vec::new();
                            
                            for (ring, coords) in
                                tcl.rings.into_iter().zip(tcl.coords_list.into_iter())
                            {
                                let coords = coords
                                    .chunks_exact(2)
                                    .map(|v| [v[0], v[1]])
                                    .collect::<Vec<_>>();
                                let ls = LineString2::from_raw(coords.into());
                                theme.ring_id_to_texture.insert(ring.clone(), (tex_idx, ls));
                                ring_ids.push(ring);
                            }
                            
                            // Store surface-to-rings mapping
                            if !ring_ids.is_empty() {
                                theme.surface_id_to_rings.insert(surface_id, ring_ids);
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
            
            // If ring_ids is empty or all None, we can't filter - transfer all entries
            // Otherwise, only transfer entries for rings in ring_ids
            let ring_id_set: std::collections::HashSet<_> = ring_ids
                .iter()
                .filter_map(|v| v.clone())
                .collect();
            let filter_by_ring_ids = !ring_id_set.is_empty();

            for (theme_name, theme_src) in other.themes.iter_mut() {
                let entries: Vec<_> = if filter_by_ring_ids {
                    // Only transfer entries for rings in ring_ids
                    ring_ids
                        .iter()
                        .filter_map(|v| v.clone())
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
                        .collect()
                } else {
                    // Transfer all entries (ring_ids is empty or all None)
                    theme_src
                        .ring_id_to_texture
                        .drain()
                        .filter_map(|(ring_id, (idx, ls))| {
                            let (offset, inserted) = idx_map.insert_full(idx);
                            if inserted {
                                self.textures.push(other.textures[idx as usize].clone());
                            }
                            Some((ring_id, ((base_idx + offset) as u32, ls)))
                        })
                        .collect()
                };

                self.themes
                    .entry_ref(theme_name)
                    .or_default()
                    .ring_id_to_texture
                    .extend(entries);
            }
        }

        // merge surface_id_to_rings (for texture surface-to-ring mapping)
        // Only transfer entries where at least one ring is in ring_ids (if ring_ids is not empty)
        {
            let ring_id_set: std::collections::HashSet<_> = ring_ids
                .iter()
                .filter_map(|v| v.clone())
                .collect();
            
            for (theme_name, theme_src) in other.themes.iter_mut() {
                let entries: Vec<_> = if ring_id_set.is_empty() {
                    // If ring_ids is empty or all None, transfer all surface_id_to_rings entries
                    // This happens when the ring IDs haven't been resolved yet
                    theme_src.surface_id_to_rings.drain().collect()
                } else {
                    // Only transfer entries that have matching rings
                    theme_src
                        .surface_id_to_rings
                        .drain()
                        .filter(|(_, ring_ids)| ring_ids.iter().any(|r| ring_id_set.contains(r)))
                        .collect()
                };

                self.themes
                    .entry_ref(theme_name)
                    .or_default()
                    .surface_id_to_rings
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
                    .map(|span| span.id.clone())
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
                image_url: Url::parse("file:///local1.jpg").unwrap(),
            });
            app_local.textures.push(Texture {
                image_url: Url::parse("file:///local2.jpg").unwrap(),
            });
            app_local.textures.push(Texture {
                image_url: Url::parse("file:///local3.jpg").unwrap(),
            });
            app_local.materials.push(Material::default());
            app_local.materials.push(Material::default());
            let theme = app_local.themes.entry("default".to_string()).or_default();
            theme
                .ring_id_to_texture
                .insert(LocalId::new("aa"), (0, LineString2::default()));
            theme
                .ring_id_to_texture
                .insert(LocalId::new("bb"), (0, LineString2::default()));
            theme.surface_id_to_material.insert(LocalId::new("aa"), 0);
            theme.surface_id_to_material.insert(LocalId::new("bb"), 1);
            theme.surface_id_to_material.insert(LocalId::new("cc"), 1);

            assert_eq!(app_local.materials.len(), 2);
            assert_eq!(app_local.textures.len(), 3);
        }

        {
            app_global.textures.push(Texture {
                image_url: Url::parse("file:///global1.jpg").unwrap(),
            });
            app_global.textures.push(Texture {
                image_url: Url::parse("file:///global2.jpg").unwrap(),
            });
            app_global.textures.push(Texture {
                image_url: Url::parse("file:///global3.jpg").unwrap(),
            });
            app_global.materials.push(Material::default());
            app_global.materials.push(Material::default());
            app_global.materials.push(Material::default());
            let theme = app_global.themes.entry("default".to_string()).or_default();
            theme
                .ring_id_to_texture
                .insert(LocalId::new("cc"), (0, LineString2::default()));
            theme
                .ring_id_to_texture
                .insert(LocalId::new("dd"), (1, LineString2::default()));
            theme
                .ring_id_to_texture
                .insert(LocalId::new("ee"), (1, LineString2::default()));
            theme.surface_id_to_material.insert(LocalId::new("cc"), 0);
            theme.surface_id_to_material.insert(LocalId::new("dd"), 1);
            theme.surface_id_to_material.insert(LocalId::new("ee"), 1);
        }

        // merge global to local
        app_local.merge_global(
            &mut app_global,
            ["cc", "dd", "ee", "zz"]
                .into_iter()
                .map(|id| Some(LocalId::new(id)))
                .collect::<Vec<_>>()
                .as_slice(),
            ["cc", "dd", "ee", "zz"]
                .into_iter()
                .map(|id| SurfaceSpan {
                    id: LocalId::new(id),
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

        assert!(theme.ring_id_to_texture[&LocalId::new("cc")].0 >= 3);
        assert!(theme.ring_id_to_texture[&LocalId::new("dd")].0 >= 3);
        assert!(theme.ring_id_to_texture[&LocalId::new("ee")].0 >= 3);

        assert!(theme.surface_id_to_material[&LocalId::new("cc")] >= 2);
        assert!(theme.surface_id_to_material[&LocalId::new("dd")] >= 2);
        assert!(theme.surface_id_to_material[&LocalId::new("ee")] >= 2);
    }
}
