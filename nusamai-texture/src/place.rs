use std::cmp::max;

use crate::{pack::TexturePackerConfig, texture::CroppedTexture};

#[derive(Debug, Clone)]
pub struct PlacedTextureInfo {
    pub id: String,
    pub u: u32,
    pub v: u32,
    pub width: u32,
    pub height: u32,
}

pub trait TexturePlacer {
    fn place_texture(
        &mut self,
        id: &str,
        texture: &CroppedTexture,
        config: &TexturePackerConfig,
    ) -> PlacedTextureInfo;

    fn can_place(&self, texture: &CroppedTexture, config: &TexturePackerConfig) -> bool;

    fn reset_param(&mut self);
}

#[derive(Default)]
pub struct SimpleTexturePlacer {
    current_x: u32,
    current_y: u32,
    max_height_in_row: u32,
}

impl TexturePlacer for SimpleTexturePlacer {
    fn place_texture(
        &mut self,
        id: &str,
        texture: &CroppedTexture,
        config: &TexturePackerConfig,
    ) -> PlacedTextureInfo {
        if self.current_x + texture.width > config.max_width {
            self.current_x = 0;
            self.current_y += self.max_height_in_row + config.padding;
            self.max_height_in_row = 0;
        }

        let texture_info = PlacedTextureInfo {
            id: id.to_string(),
            u: self.current_x,
            v: self.current_y,
            width: texture.width,
            height: texture.height,
        };

        self.current_x += texture.width + config.padding;
        self.max_height_in_row = self.max_height_in_row.max(texture.height);

        texture_info
    }

    fn can_place(&self, texture: &CroppedTexture, config: &TexturePackerConfig) -> bool {
        let next_x = self.current_x + texture.width + config.padding;
        let next_y = max(
            self.current_y + texture.height + config.padding,
            self.current_y + self.max_height_in_row + config.padding,
        );

        if next_x <= config.max_width && next_y <= config.max_height {
            true
        } else {
            next_y + texture.height + config.padding <= config.max_height
        }
    }

    fn reset_param(&mut self) {
        self.current_x = 0;
        self.current_y = 0;
        self.max_height_in_row = 0;
    }
}
