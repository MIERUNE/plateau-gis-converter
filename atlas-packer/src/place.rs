use std::cmp::max;
use std::collections::HashMap;

use crate::texture::CroppedTexture;

#[derive(Debug, Clone)]
pub struct TexturePlacerConfig {
    pub width: u32,
    pub height: u32,
    pub padding: u32,
    // and more option
    // Allow rotation, allow multiple pages, adjust resolution, specify resampling method, etc...
}

impl Default for TexturePlacerConfig {
    fn default() -> Self {
        TexturePlacerConfig {
            width: 1024,
            height: 1024,
            padding: 0,
        }
    }
}

impl TexturePlacerConfig {
    // Ensure that the width and height are powers of two
    pub fn new(width: u32, height: u32, padding: u32) -> Self {
        TexturePlacerConfig {
            width: width.checked_next_power_of_two().unwrap(),
            height: height.checked_next_power_of_two().unwrap(),
            padding,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }
}

#[derive(Debug, Clone)]
pub struct PlacedTextureInfo {
    pub id: String,
    pub atlas_id: String,
    // Pixel coordinates on atlas
    pub origin: (u32, u32),
    pub width: u32,
    pub height: u32,
    // UV coordinates on atlas
    pub placed_uv_coords: Vec<(f64, f64)>,
}

pub trait TexturePlacer: Send + Sync {
    fn config(&self) -> &TexturePlacerConfig;

    fn place_texture(
        &mut self,
        id: &str,
        texture: &CroppedTexture,
        parent_atlas_id: &str,
    ) -> PlacedTextureInfo;

    fn can_place(&self, texture: &CroppedTexture) -> bool;

    fn reset_param(&mut self);

    fn scale_dimensions(&self, width: u32, height: u32, downsample_factor: f32) -> (u32, u32) {
        let scaled_width = (width as f32 * downsample_factor).max(1.0) as u32;
        let scaled_height = (height as f32 * downsample_factor).max(1.0) as u32;
        (scaled_width, scaled_height)
    }
}

pub struct SimpleTexturePlacer {
    pub config: TexturePlacerConfig,
    pub current_x: u32,
    pub current_y: u32,
    pub max_height_in_row: u32,
}

impl SimpleTexturePlacer {
    pub fn new(config: TexturePlacerConfig) -> Self {
        SimpleTexturePlacer {
            config,
            current_x: 0,
            current_y: 0,
            max_height_in_row: 0,
        }
    }
}

impl TexturePlacer for SimpleTexturePlacer {
    fn config(&self) -> &TexturePlacerConfig {
        &self.config
    }

    fn place_texture(
        &mut self,
        id: &str,
        texture: &CroppedTexture,
        parent_atlas_id: &str,
    ) -> PlacedTextureInfo {
        let (scaled_width, scaled_height) = self.scale_dimensions(
            texture.width,
            texture.height,
            texture.downsample_factor.value(),
        );

        if self.current_x + texture.width > self.config().width {
            self.current_x = 0;
            self.current_y += self.max_height_in_row + self.config().padding;
            self.max_height_in_row = 0;
        }

        let placed_uv_coords = texture
            .cropped_uv_coords
            .iter()
            .map(|(u, v)| {
                (
                    self.current_x as f64 / self.config().width as f64 + u * scaled_width as f64,
                    self.current_y as f64 / self.config().height as f64 + v * scaled_height as f64,
                )
            })
            .collect::<Vec<(f64, f64)>>();

        let texture_info = PlacedTextureInfo {
            id: id.to_string(),
            atlas_id: parent_atlas_id.to_string(),
            origin: (self.current_x, self.current_y),
            width: scaled_width,
            height: scaled_height,
            placed_uv_coords,
        };

        self.current_x += texture.width + self.config().padding;
        self.max_height_in_row = self.max_height_in_row.max(texture.height);

        texture_info
    }

    fn can_place(&self, texture: &CroppedTexture) -> bool {
        let (scaled_width, scaled_height) = self.scale_dimensions(
            texture.width,
            texture.height,
            texture.downsample_factor.value(),
        );

        let padding = self.config().padding;
        let max_width = self.config().width;
        let max_height = self.config().height;

        let next_x = self.current_x + scaled_width + padding;
        let next_y = max(
            self.current_y + scaled_height + padding,
            self.current_y + self.max_height_in_row + padding,
        );

        if next_x <= max_width && next_y <= max_height {
            true
        } else {
            next_y + scaled_height + padding <= max_height
        }
    }

    fn reset_param(&mut self) {
        self.current_x = 0;
        self.current_y = 0;
        self.max_height_in_row = 0;
    }
}

pub struct GuillotineTexturePlacer {
    config: TexturePlacerConfig,
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

impl GuillotineTexturePlacer {
    pub fn new(config: TexturePlacerConfig) -> Self {
        let initial_rect = Rect {
            x: 0,
            y: 0,
            width: config.width,
            height: config.height,
        };
        GuillotineTexturePlacer {
            config,
            free_rects: vec![initial_rect],
            used_rects: HashMap::new(),
        }
    }

    fn find_best_rect(&self, width: u32, height: u32) -> Option<Rect> {
        let mut best_rect = None;
        let mut best_area = u32::MAX;

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

    fn uv_to_pixel(&self, uv: (f64, f64), width: u32, height: u32) -> (u32, u32) {
        let x = (uv.0 * width as f64) as u32;
        let y = ((1.0 - uv.1) * height as f64) as u32;
        (x, y)
    }
}

impl TexturePlacer for GuillotineTexturePlacer {
    fn config(&self) -> &TexturePlacerConfig {
        &self.config
    }

    fn place_texture(
        &mut self,
        id: &str,
        texture: &CroppedTexture,
        parent_atlas_id: &str,
    ) -> PlacedTextureInfo {
        let (scaled_width, scaled_height) = self.scale_dimensions(
            texture.width,
            texture.height,
            texture.downsample_factor.value(),
        );

        let width = scaled_width + self.config.padding;
        let height = scaled_height + self.config.padding;

        if let Some(rect) = self.find_best_rect(width, height) {
            let placed_uv_coords = texture
                .cropped_uv_coords
                .iter()
                .map(|&(u, v)| {
                    let (x, y) = self.uv_to_pixel((u, v), scaled_width, scaled_height);
                    (
                        (rect.x as f64 + self.config.padding as f64 + x as f64)
                            / self.config.width as f64,
                        1.0 - ((rect.y as f64 + self.config.padding as f64 + y as f64)
                            / self.config.height as f64),
                    )
                })
                .collect::<Vec<(f64, f64)>>();

            let placed = PlacedTextureInfo {
                id: id.to_string(),
                atlas_id: parent_atlas_id.to_string(),
                origin: (rect.x + self.config.padding, rect.y + self.config.padding),
                width: scaled_width,
                height: scaled_height,
                placed_uv_coords,
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
        let (scaled_width, scaled_height) = self.scale_dimensions(
            texture.width,
            texture.height,
            texture.downsample_factor.value(),
        );
        let width = scaled_width + self.config.padding;
        let height = scaled_height + self.config.padding;
        self.free_rects
            .iter()
            .any(|r| r.width >= width && r.height >= height)
    }

    fn reset_param(&mut self) {
        let initial_rect = Rect {
            x: 0,
            y: 0,
            width: self.config.width,
            height: self.config.height,
        };
        self.free_rects = vec![initial_rect];
        self.used_rects.clear();
    }
}
