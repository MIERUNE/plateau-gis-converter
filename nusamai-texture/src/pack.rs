use std::collections::HashMap;
use std::path::Path;

use crate::export::AtlasExporter;
use crate::place::{PlacedTextureInfo, TexturePlacer};
use crate::texture::CroppedTexture;

pub struct TexturePackerConfig {
    pub max_width: u32,
    pub max_height: u32,
    pub padding: u32,
    pub scale_factor: f32,
    // and more option
    // Allow rotation, allow multiple pages, adjust resolution, specify resampling method, etc...
}

impl Default for TexturePackerConfig {
    fn default() -> Self {
        TexturePackerConfig {
            max_width: 256,
            max_height: 256,
            padding: 0,
            scale_factor: 1.0,
        }
    }
}

pub struct TexturePacker<P: TexturePlacer, E: AtlasExporter> {
    pub textures: HashMap<String, CroppedTexture>,
    pub atlas_list: Vec<Vec<PlacedTextureInfo>>,
    pub atlas_layout: Vec<PlacedTextureInfo>,
    placer: P,
    exporter: E,
}

impl<P: TexturePlacer, E: AtlasExporter> TexturePacker<P, E> {
    pub fn new(placer: P, exporter: E) -> Self {
        TexturePacker {
            textures: HashMap::new(),
            atlas_list: Vec::new(),
            atlas_layout: Vec::new(),
            placer,
            exporter,
        }
    }

    pub fn add_texture(&mut self, id: String, texture: CroppedTexture) {
        if self.placer.can_place(&texture) {
            let texture_info = self.placer.place_texture(&id, &texture);
            self.textures.insert(id, texture);
            self.atlas_layout.push(texture_info);
        } else {
            self.atlas_list.push(self.atlas_layout.clone());
            self.atlas_layout.clear();

            self.placer.reset_param();

            let texture_info = self.placer.place_texture(&id, &texture);
            self.textures.insert(id, texture);
            self.atlas_layout.push(texture_info);
        }
    }

    pub fn finalize(&mut self) {
        if !self.atlas_layout.is_empty() {
            self.atlas_list.push(self.atlas_layout.clone());
            self.atlas_layout.clear();
        }
    }

    pub fn export(&self, output_dir: &Path) {
        for (i, atlas) in self.atlas_list.iter().enumerate() {
            let output_path = output_dir.join(format!("atlas_{}.webp", i));
            self.exporter.export(atlas, &self.textures, &output_path);
        }
    }
}
