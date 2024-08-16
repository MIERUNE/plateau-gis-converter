use std::path::Path;

use hashbrown::HashMap;
use rayon::prelude::*;

use crate::export::AtlasExporter;
use crate::place::{PlacedTextureInfo, TexturePlacer};
use crate::texture::{CroppedTexture, TextureCache};

pub type Atlas = Vec<PlacedTextureInfo>;

pub struct TexturePacker<P: TexturePlacer, E: AtlasExporter> {
    pub textures: HashMap<String, CroppedTexture>,
    pub current_atlas: Atlas,
    pub atlases: HashMap<String, Atlas>,
    placer: P,
    exporter: E,
}

impl<P: TexturePlacer, E: AtlasExporter> TexturePacker<P, E> {
    pub fn new(placer: P, exporter: E) -> Self {
        TexturePacker {
            textures: HashMap::new(),
            current_atlas: Vec::new(),
            atlases: HashMap::new(),
            placer,
            exporter,
        }
    }

    pub fn add_texture(&mut self, id: String, texture: CroppedTexture) -> PlacedTextureInfo {
        let current_atlas_id = self.atlases.len();

        if self.placer.can_place(&texture) {
            let texture_info =
                self.placer
                    .place_texture(&id, &texture, current_atlas_id.to_string().as_ref());
            self.textures.insert(id, texture);

            self.current_atlas.push(texture_info.clone());

            texture_info
        } else {
            self.atlases
                .insert(current_atlas_id.to_string(), self.current_atlas.clone());
            self.current_atlas.clear();
            self.placer.reset_param();

            let current_atlas_id = self.atlases.len();

            let texture_info =
                self.placer
                    .place_texture(&id, &texture, current_atlas_id.to_string().as_ref());
            self.textures.insert(id, texture);

            self.current_atlas.push(texture_info.clone());

            texture_info
        }
    }

    pub fn finalize(&mut self) {
        if !self.current_atlas.is_empty() {
            let current_atlas_id = self.atlases.len();

            self.atlases
                .insert(current_atlas_id.to_string(), self.current_atlas.clone());
            self.current_atlas.clear();
        }
    }

    pub fn export(&self, output_dir: &Path, texture_cache: &TextureCache, width: u32, height: u32) {
        self.atlases.par_iter().for_each(|(id, atlas)| {
            let output_path = output_dir.join(id);
            self.exporter.export(
                atlas,
                &self.textures,
                &output_path,
                texture_cache,
                width,
                height,
            );
        });
    }
}
