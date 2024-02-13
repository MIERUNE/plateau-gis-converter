use crate::transformer::Transform;

use nusamai_citygml::schema::Schema;
use nusamai_geometry::MultiPolygon;
use nusamai_plateau::Entity;

#[derive(Default)]
pub struct ApplyAppearanceTransform {}

impl Transform for ApplyAppearanceTransform {
    fn transform(&mut self, entity: Entity, out: &mut Vec<Entity>) {
        {
            let app = entity.appearance_store.read().unwrap();
            let theme = {
                app.themes
                    .get("rgbTexture")
                    .or_else(|| app.themes.get("FMETheme"))
            };

            if let Some(theme) = theme {
                let mut geoms = entity.geometry_store.write().unwrap();

                // find and apply materials
                {
                    let mut poly_materials = vec![None; geoms.multipolygon.len()];
                    let mut found_use = false;

                    for surface in &geoms.surface_spans {
                        if let Some(&mat) = theme.surface_id_to_material.get(&surface.id) {
                            found_use = true;
                            for idx in surface.start..surface.end {
                                poly_materials[idx as usize] = Some(mat);
                            }
                        }
                    }
                    if found_use {
                        // apply materials to polygons
                        geoms.polygon_materials = poly_materials;
                    }
                }

                // find and apply textures
                {
                    let mut ring_id_iter = geoms.ring_ids.iter();
                    let mut poly_textures = Vec::with_capacity(geoms.multilinestring.len());
                    let mut poly_uvs = MultiPolygon::new();
                    let mut found_use = false;

                    for poly in &geoms.multipolygon {
                        for (i, ring) in poly.rings().enumerate() {
                            let tex = ring_id_iter
                                .next()
                                .unwrap()
                                .and_then(|ring_id| theme.ring_id_to_texture.get(&ring_id));

                            let mut add_dummy = || {
                                let uv = [[0.0, 0.0]].into_iter().cycle().take(ring.len());
                                if i == 0 {
                                    poly_textures.push(None);
                                    poly_uvs.add_exterior(uv);
                                } else {
                                    poly_uvs.add_interior(uv);
                                }
                            };

                            match tex {
                                Some((idx, uv)) if uv.len() == ring.len() => {
                                    // texture found
                                    found_use = true;
                                    if i == 0 {
                                        poly_textures.push(Some(*idx));
                                        poly_uvs.add_exterior(uv);
                                    } else {
                                        poly_uvs.add_interior(uv);
                                    }
                                }
                                Some((_, uv)) if uv.len() != ring.len() => {
                                    // invalid texture found
                                    log::warn!("Length of UVs does not match length of ring");
                                    add_dummy();
                                }
                                _ => {
                                    // no texture found
                                    add_dummy();
                                }
                            };
                        }
                    }

                    if found_use {
                        // apply textures to polygons
                        geoms.polygon_textures = poly_textures;
                        geoms.polygon_uvs = poly_uvs;
                    }
                }
            }
        }

        out.push(entity);
    }

    fn transform_schema(&self, _schema: &mut Schema) {
        // do nothing
    }
}

impl ApplyAppearanceTransform {
    pub fn new() -> Self {
        Default::default()
    }
}
