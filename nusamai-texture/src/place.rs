use std::cmp::max;
use std::collections::HashMap;

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

impl SimpleTexturePlacer {
    pub fn new(config: TexturePackerConfig) -> Self {
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

pub struct GuillotinePlacer {
    config: TexturePackerConfig,
    free_rects: Vec<Rect>,
    used_rects: HashMap<String, PlacedTextureInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl GuillotinePlacer {
    pub fn new(config: TexturePackerConfig) -> Self {
        let initial_rect = Rect {
            x: 0,
            y: 0,
            width: config.max_width,
            height: config.max_height,
        };
        GuillotinePlacer {
            config,
            free_rects: vec![initial_rect],
            used_rects: HashMap::new(),
        }
    }

    fn find_best_rect(&self, width: u32, height: u32) -> Option<Rect> {
        let mut best_rect = None;
        let mut best_area = std::u32::MAX;

        for rect in &self.free_rects {
            if rect.width >= width && rect.height >= height {
                let area = rect.width * rect.height;
                if area < best_area {
                    best_rect = Some(*rect);
                    best_area = area;
                }
            }
        }

        best_rect
    }

    fn split_rect(&mut self, rect: Rect, placed: &PlacedTextureInfo) {
        let shorter_axis_split = rect.width <= rect.height;

        if shorter_axis_split {
            let right_rect = Rect {
                x: rect.x + placed.width + self.config.padding,
                y: rect.y,
                width: rect.width - placed.width - self.config.padding,
                height: placed.height,
            };

            let bottom_rect = Rect {
                x: rect.x,
                y: rect.y + placed.height + self.config.padding,
                width: rect.width,
                height: rect.height - placed.height - self.config.padding,
            };

            if right_rect.width > 0 && right_rect.height > 0 {
                self.free_rects.push(right_rect);
            }
            if bottom_rect.width > 0 && bottom_rect.height > 0 {
                self.free_rects.push(bottom_rect);
            }
        } else {
            let right_rect = Rect {
                x: rect.x + placed.width + self.config.padding,
                y: rect.y,
                width: rect.width - placed.width - self.config.padding,
                height: rect.height,
            };

            let bottom_rect = Rect {
                x: rect.x,
                y: rect.y + placed.height + self.config.padding,
                width: placed.width,
                height: rect.height - placed.height - self.config.padding,
            };

            if right_rect.width > 0 && right_rect.height > 0 {
                self.free_rects.push(right_rect);
            }
            if bottom_rect.width > 0 && bottom_rect.height > 0 {
                self.free_rects.push(bottom_rect);
            }
        }
    }

    fn merge_free_rects(&mut self) {
        let mut i = 0;
        while i < self.free_rects.len() {
            let mut j = i + 1;
            while j < self.free_rects.len() {
                let rect1 = self.free_rects[i];
                let rect2 = self.free_rects[j];

                if rect1.x == rect2.x
                    && rect1.width == rect2.width
                    && rect1.y + rect1.height == rect2.y
                {
                    self.free_rects[i] = Rect {
                        x: rect1.x,
                        y: rect1.y,
                        width: rect1.width,
                        height: rect1.height + rect2.height,
                    };
                    self.free_rects.remove(j);
                } else if rect1.y == rect2.y
                    && rect1.height == rect2.height
                    && rect1.x + rect1.width == rect2.x
                {
                    self.free_rects[i] = Rect {
                        x: rect1.x,
                        y: rect1.y,
                        width: rect1.width + rect2.width,
                        height: rect1.height,
                    };
                    self.free_rects.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
}

impl TexturePlacer for GuillotinePlacer {
    fn config(&self) -> &TexturePackerConfig {
        &self.config
    }

    fn place_texture(&mut self, id: &str, texture: &CroppedTexture) -> PlacedTextureInfo {
        let width = texture.width + self.config.padding * 2;
        let height = texture.height + self.config.padding * 2;

        if let Some(rect) = self.find_best_rect(width, height) {
            let placed = PlacedTextureInfo {
                id: id.to_string(),
                u: rect.x + self.config.padding,
                v: rect.y + self.config.padding,
                width: texture.width,
                height: texture.height,
            };

            self.used_rects.insert(id.to_string(), placed.clone());
            self.free_rects.retain(|r| r != &rect);
            self.split_rect(rect, &placed);
            self.merge_free_rects();

            placed
        } else {
            panic!("Texture could not be placed: {}", id);
        }
    }

    fn can_place(&self, texture: &CroppedTexture) -> bool {
        let width = texture.width + self.config.padding * 2;
        let height = texture.height + self.config.padding * 2;
        self.free_rects
            .iter()
            .any(|r| r.width >= width && r.height >= height)
    }

    fn reset_param(&mut self) {
        let initial_rect = Rect {
            x: 0,
            y: 0,
            width: self.config.max_width,
            height: self.config.max_height,
        };
        self.free_rects = vec![initial_rect];
        self.used_rects.clear();
    }
}
