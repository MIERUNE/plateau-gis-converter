use std::path::Path;

use hashbrown::HashMap;

use crate::export::AtlasExporter;
use crate::place::{PlacedTextureInfo, TexturePlacer};
use crate::texture::CroppedTexture;

pub type Atlas = Vec<PlacedTextureInfo>;

pub struct TexturePacker<P: TexturePlacer, E: AtlasExporter> {
    pub textures: HashMap<String, CroppedTexture>,
    pub current_atlas: Atlas,
    pub atlases: Vec<Atlas>,
    placer: P,
    exporter: E,
}

impl<P: TexturePlacer, E: AtlasExporter> TexturePacker<P, E> {
    pub fn new(placer: P, exporter: E) -> Self {
        TexturePacker {
            textures: HashMap::new(),
            current_atlas: Vec::new(),
            atlases: Vec::new(),
            placer,
            exporter,
        }
    }

    pub fn add_texture(&mut self, id: String, texture: CroppedTexture) -> PlacedTextureInfo {
        if self.placer.can_place(&texture) {
            let texture_info = self.placer.place_texture(&id, &texture);
            self.textures.insert(id, texture);
            self.current_atlas.push(texture_info.clone());

            texture_info
        } else {
            self.atlases.push(self.current_atlas.clone());
            self.current_atlas.clear();

            self.placer.reset_param();

            let texture_info = self.placer.place_texture(&id, &texture);
            self.textures.insert(id, texture);
            self.current_atlas.push(texture_info.clone());

            texture_info
        }
    }

    pub fn finalize(&mut self) {
        if !self.current_atlas.is_empty() {
            self.atlases.push(self.current_atlas.clone());
            self.current_atlas.clear();
        }
    }

    pub fn export(&self, output_dir: &Path) {
        for (i, atlas) in self.atlases.iter().enumerate() {
            let output_path = output_dir.join(format!("atlas_{}.webp", i));
            self.exporter.export(atlas, &self.textures, &output_path);
        }
    }
}
