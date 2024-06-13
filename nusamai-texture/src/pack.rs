use std::collections::HashMap;
use std::path::Path;

use crate::export::AtlasExporter;
use crate::place::{PlacedTextureInfo, TexturePlacer};
use crate::texture::CroppedTexture;

pub type Atlas = Vec<PlacedTextureInfo>;

pub struct TexturePacker<P: TexturePlacer, E: AtlasExporter> {
    pub textures: HashMap<String, CroppedTexture>,
    pub atlas_layout: Atlas,
    pub atlases: Vec<Atlas>,
    placer: P,
    exporter: E,
}

impl<P: TexturePlacer, E: AtlasExporter> TexturePacker<P, E> {
    pub fn new(placer: P, exporter: E) -> Self {
        TexturePacker {
            textures: HashMap::new(),
            atlas_layout: Vec::new(),
            atlases: Vec::new(),
            placer,
            exporter,
        }
    }

    pub fn add_texture(&mut self, id: String, texture: CroppedTexture) -> PlacedTextureInfo {
        if self.placer.can_place(&texture) {
            // todo:
            // CroppedTextureにはオリジナルのされたUV座標が含められた
            // PlacedTextureInfoにはスケールされたUV座標の集合も含める必要がある
            let texture_info = self.placer.place_texture(&id, &texture);
            self.textures.insert(id, texture);
            self.atlas_layout.push(texture_info.clone());

            texture_info
        } else {
            self.atlases.push(self.atlas_layout.clone());
            self.atlas_layout.clear();

            self.placer.reset_param();

            let texture_info = self.placer.place_texture(&id, &texture);
            self.textures.insert(id, texture);
            self.atlas_layout.push(texture_info.clone());

            texture_info
        }
    }

    pub fn finalize(&mut self) {
        if !self.atlas_layout.is_empty() {
            self.atlases.push(self.atlas_layout.clone());
            self.atlas_layout.clear();
        }
    }

    pub fn export(&self, output_dir: &Path) {
        for (i, atlas) in self.atlases.iter().enumerate() {
            let output_path = output_dir.join(format!("atlas_{}.webp", i));
            self.exporter.export(atlas, &self.textures, &output_path);
        }
    }
}
