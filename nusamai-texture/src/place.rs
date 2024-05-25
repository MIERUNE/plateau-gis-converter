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
    fn config(&self) -> &TexturePackerConfig;

    fn place_texture(&mut self, id: &str, texture: &CroppedTexture) -> PlacedTextureInfo;

    fn can_place(&self, texture: &CroppedTexture) -> bool;

    fn reset_param(&mut self);
}

pub struct SimpleTexturePlacer {
    pub config: TexturePackerConfig,
    pub current_x: u32,
    pub current_y: u32,
    pub max_height_in_row: u32,
}

impl Default for SimpleTexturePlacer {
    fn default() -> Self {
        let config = TexturePackerConfig {
            max_width: 512,
            max_height: 512,
            padding: 2,
        };
        SimpleTexturePlacer {
            config,
            current_x: 0,
            current_y: 0,
            max_height_in_row: 0,
        }
    }
}

impl TexturePlacer for SimpleTexturePlacer {
    fn config(&self) -> &TexturePackerConfig {
        &self.config
    }

    fn place_texture(&mut self, id: &str, texture: &CroppedTexture) -> PlacedTextureInfo {
        if self.current_x + texture.width > self.config().max_width {
            self.current_x = 0;
            self.current_y += self.max_height_in_row + self.config().padding;
            self.max_height_in_row = 0;
        }

        let texture_info = PlacedTextureInfo {
            id: id.to_string(),
            u: self.current_x,
            v: self.current_y,
            width: texture.width,
            height: texture.height,
        };

        self.current_x += texture.width + self.config().padding;
        self.max_height_in_row = self.max_height_in_row.max(texture.height);

        texture_info
    }

    fn can_place(&self, texture: &CroppedTexture) -> bool {
        let padding = self.config().padding;
        let max_width = self.config().max_width;
        let max_height = self.config().max_height;

        let next_x = self.current_x + texture.width + padding;
        let next_y = max(
            self.current_y + texture.height + padding,
            self.current_y + self.max_height_in_row + padding,
        );

        if next_x <= max_width && next_y <= max_height {
            true
        } else {
            next_y + texture.height + padding <= max_height
        }
    }

    fn reset_param(&mut self) {
        self.current_x = 0;
        self.current_y = 0;
        self.max_height_in_row = 0;
    }
}

pub struct GuillotineTexturePlacer {
    pub config: TexturePackerConfig,
    pub free_rects: Vec<(u32, u32, u32, u32)>,
}

impl Default for GuillotineTexturePlacer {
    fn default() -> Self {
        let config = TexturePackerConfig {
            max_width: 512,
            max_height: 512,
            padding: 2,
        };
        GuillotineTexturePlacer {
            config,
            free_rects: vec![(0, 0, 512, 512)],
        }
    }
}

impl TexturePlacer for GuillotineTexturePlacer {
    fn config(&self) -> &TexturePackerConfig {
        &self.config
    }

    fn place_texture(&mut self, id: &str, texture: &CroppedTexture) -> PlacedTextureInfo {
        let mut best_rect = None;
        let mut best_area = std::u32::MAX;

        for (i, &(x, y, w, h)) in self.free_rects.iter().enumerate() {
            if w >= texture.width && h >= texture.height {
                let area = w * h;
                if area < best_area {
                    best_rect = Some((i, x, y));
                    best_area = area;
                }
            }
        }

        if let Some((index, x, y)) = best_rect {
            let (_, _, w, h) = self.free_rects[index];
            self.free_rects.remove(index);

            if w > texture.width {
                self.free_rects
                    .push((x + texture.width, y, w - texture.width, texture.height));
            }
            if h > texture.height {
                self.free_rects
                    .push((x, y + texture.height, texture.width, h - texture.height));
            }

            PlacedTextureInfo {
                id: id.to_string(),
                u: x,
                v: y,
                width: texture.width,
                height: texture.height,
            }
        } else {
            panic!("Failed to place texture");
        }
    }

    fn can_place(&self, texture: &CroppedTexture) -> bool {
        self.free_rects
            .iter()
            .any(|&(_, _, w, h)| w >= texture.width && h >= texture.height)
    }

    fn reset_param(&mut self) {
        self.free_rects.clear();
        self.free_rects
            .push((0, 0, self.config().max_width, self.config().max_height));
    }
}
