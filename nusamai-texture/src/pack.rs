use std::collections::HashMap;
use std::path::Path;

use crate::export::AtlasExporter;
use crate::place::{PlacedTextureInfo, TexturePlacer};
use crate::texture::CroppedTexture;

pub struct TexturePackerConfig {
    pub max_width: u32,
    pub max_height: u32,
    pub padding: u32,
    // and more option
    // Allow rotation, allow multiple pages, adjust resolution, specify resampling method, etc...
}

pub struct TexturePacker<P: TexturePlacer, E: AtlasExporter> {
    pub textures: HashMap<String, CroppedTexture>,
    pub atlas_data: Vec<Vec<PlacedTextureInfo>>,
    pub texture_info_list: Vec<PlacedTextureInfo>,
    config: TexturePackerConfig,
    placer: P,
    exporter: E,
}

impl<P: TexturePlacer, E: AtlasExporter> TexturePacker<P, E> {
    pub fn new(config: TexturePackerConfig, placer: P, exporter: E) -> Self {
        TexturePacker {
            textures: HashMap::new(),
            atlas_data: Vec::new(),
            texture_info_list: Vec::new(),
            config,
            placer,
            exporter,
        }
    }

    pub fn add_texture(&mut self, id: String, texture: CroppedTexture) {
        if self.placer.can_place(&texture, &self.config) {
            let texture_info = self.placer.place_texture(&id, &texture, &self.config);
            self.textures.insert(id, texture);
            self.texture_info_list.push(texture_info);
        } else {
            self.atlas_data.push(self.texture_info_list.clone());
            self.texture_info_list.clear();

            self.placer.reset_param();

            let texture_info = self.placer.place_texture(&id, &texture, &self.config);
            self.textures.insert(id, texture);
            self.texture_info_list.push(texture_info);
        }
    }

    pub fn finalize(&mut self) {
        if !self.texture_info_list.is_empty() {
            self.atlas_data.push(self.texture_info_list.clone());
            self.texture_info_list.clear();
        }
    }

    pub fn export(&self, output_dir: &Path) {
        for (i, atlas) in self.atlas_data.iter().enumerate() {
            let output_path = output_dir.join(format!("atlas_{}.webp", i));
            self.exporter.export(atlas, &self.textures, &output_path);
        }
    }
}
