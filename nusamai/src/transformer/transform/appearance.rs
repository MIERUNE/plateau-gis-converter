//! Apply appearance to geometries

use feedback::Feedback;
use flatgeom::MultiPolygon;
use nusamai_citygml::schema::Schema;
use nusamai_plateau::Entity;

use crate::{pipeline::feedback, transformer::Transform};

#[derive(Default)]
pub struct ApplyAppearanceTransform {}

impl Transform for ApplyAppearanceTransform {
    fn transform(&mut self, feedback: &Feedback, entity: Entity, out: &mut Vec<Entity>) {
        {
            let app = entity.appearance_store.read().unwrap();
            let theme = {
                app.themes
                    .get("rgbTexture")
                    .or_else(|| app.themes.get("FMETheme"))
            };

            let mut geoms = entity.geometry_store.write().unwrap();

            if let Some(theme) = theme {
                // find and apply materials
                {
                    let mut poly_materials = vec![None; geoms.multipolygon.len()];

                    for surface in &geoms.surface_spans {
                        if let Some(&mat) = theme.surface_id_to_material.get(&surface.id) {
                            for idx in surface.start..surface.end {
                                poly_materials[idx as usize] = Some(mat);
                            }
                        }
                    }

                    // apply materials to polygons
                    geoms.polygon_materials = poly_materials;
                }

                // find and apply textures
                {
                    let mut ring_id_iter = geoms.ring_ids.iter();
                    let mut poly_textures = Vec::with_capacity(geoms.multipolygon.len());
                    let mut poly_uvs = MultiPolygon::new();

                    for poly in &geoms.multipolygon {
                        for (i, ring) in poly.rings().enumerate() {
                            let tex = ring_id_iter
                                .next()
                                .unwrap()
                                .and_then(|ring_id| theme.ring_id_to_texture.get(&ring_id));

                            let mut add_dummy_texture = || {
                                let uv = [[0.0, 0.0]].into_iter().cycle().take(ring.len() + 1);
                                if i == 0 {
                                    poly_textures.push(None);
                                    poly_uvs.add_exterior(uv);
                                } else {
                                    poly_uvs.add_interior(uv);
                                }
                            };

                            match tex {
                                Some((idx, uv)) if ring.len() == uv.len() => {
                                    // texture found
                                    if i == 0 {
                                        poly_textures.push(Some(*idx));
                                        poly_uvs.add_exterior(uv.iter_closed());
                                    } else {
                                        poly_uvs.add_interior(uv.iter_closed());
                                    }
                                }
                                Some((_, uv)) if uv.len() != ring.len() => {
                                    // invalid texture found
                                    feedback.warn(format!(
                                        "Length of UVs does not match length of ring: {:?} {:?}",
                                        ring, uv
                                    ));
                                    add_dummy_texture();
                                }
                                _ => {
                                    // no texture found
                                    add_dummy_texture();
                                }
                            };
                        }
                    }

                    // apply textures to polygons
                    debug_assert_eq!(poly_textures.len(), geoms.multipolygon.len());
                    debug_assert_eq!(poly_uvs.len(), geoms.multipolygon.len());
                    geoms.polygon_textures = poly_textures;
                    geoms.polygon_uvs = poly_uvs;
                }
            } else {
                // set 'null' appearance if no theme found
                geoms.polygon_materials = vec![None; geoms.multipolygon.len()];
                geoms.polygon_textures = vec![None; geoms.multipolygon.len()];
                let mut poly_uvs = MultiPolygon::new();
                for poly in &geoms.multipolygon {
                    for (i, ring) in poly.rings().enumerate() {
                        let uv = [[0.0, 0.0]].into_iter().cycle().take(ring.len() + 1);
                        if i == 0 {
                            poly_uvs.add_exterior(uv);
                        } else {
                            poly_uvs.add_interior(uv);
                        }
                    }
                }
                geoms.polygon_uvs = poly_uvs;
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
