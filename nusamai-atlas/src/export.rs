use std::path::Path;

use hashbrown::HashMap;
use image::{ImageBuffer, ImageFormat};

use crate::place::PlacedTextureInfo;
use crate::texture::{CroppedTexture, TextureCache};

pub trait AtlasExporter {
    fn export(
        &self,
        atlas_data: &[PlacedTextureInfo],
        textures: &HashMap<String, CroppedTexture>,
        output_path: &Path,
        texture_cache: &TextureCache,
        width: u32,
        height: u32,
    );

    fn get_extension(&self) -> &str;
    fn get_image_format(&self) -> ImageFormat;
}

#[derive(Clone)]
pub struct WebpAtlasExporter {
    pub ext: String,
}

impl Default for WebpAtlasExporter {
    fn default() -> Self {
        WebpAtlasExporter {
            ext: "webp".to_string(),
        }
    }
}

impl AtlasExporter for WebpAtlasExporter {
    fn get_extension(&self) -> &str {
        &self.ext
    }

    fn get_image_format(&self) -> ImageFormat {
        ImageFormat::WebP
    }

    fn export(
        &self,
        atlas_data: &[PlacedTextureInfo],
        textures: &HashMap<String, CroppedTexture>,
        output_path: &Path,
        texture_cache: &TextureCache,
        width: u32,
        height: u32,
    ) {
        let atlas_image = create_atlas_image(atlas_data, textures, texture_cache, width, height);
        let output_path = output_path.with_extension(self.get_extension());
        atlas_image
            .save_with_format(output_path, self.get_image_format())
            .unwrap();
    }
}

#[derive(Clone)]
pub struct PngAtlasExporter {
    pub ext: String,
}

impl Default for PngAtlasExporter {
    fn default() -> Self {
        PngAtlasExporter {
            ext: "png".to_string(),
        }
    }
}

impl AtlasExporter for PngAtlasExporter {
    fn get_extension(&self) -> &str {
        &self.ext
    }

    fn get_image_format(&self) -> ImageFormat {
        ImageFormat::Png
    }

    fn export(
        &self,
        atlas_data: &[PlacedTextureInfo],
        textures: &HashMap<String, CroppedTexture>,
        output_path: &Path,
        texture_cache: &TextureCache,
        width: u32,
        height: u32,
    ) {
        let atlas_image = create_atlas_image(atlas_data, textures, texture_cache, width, height);
        let output_path = output_path.with_extension(self.get_extension());
        atlas_image
            .save_with_format(output_path, self.get_image_format())
            .unwrap();
    }
}

#[derive(Clone)]
pub struct JpegAtlasExporter {
    pub ext: String,
}

impl Default for JpegAtlasExporter {
    fn default() -> Self {
        JpegAtlasExporter {
            ext: "jpg".to_string(),
        }
    }
}

impl AtlasExporter for JpegAtlasExporter {
    fn get_extension(&self) -> &str {
        &self.ext
    }

    fn get_image_format(&self) -> ImageFormat {
        ImageFormat::Jpeg
    }

    fn export(
        &self,
        atlas_data: &[PlacedTextureInfo],
        textures: &HashMap<String, CroppedTexture>,
        output_path: &Path,
        texture_cache: &TextureCache,
        width: u32,
        height: u32,
    ) {
        let atlas_image = create_atlas_image(atlas_data, textures, texture_cache, width, height);
        let output_path = output_path.with_extension(self.get_extension());
        atlas_image
            .save_with_format(output_path, self.get_image_format())
            .unwrap();
    }
}

fn create_atlas_image(
    atlas_data: &[PlacedTextureInfo],
    textures: &HashMap<String, CroppedTexture>,
    texture_cache: &TextureCache,
    width: u32,
    height: u32,
) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let mut atlas_image = ImageBuffer::new(width, height);

    for info in atlas_data {
        let texture = textures.get(&info.id).unwrap();
        let cropped = texture.crop(&texture_cache.get_image(&texture.image_path));
        let image = cropped.as_rgba8().unwrap();

        for (x, y, pixel) in image.enumerate_pixels() {
            let atlas_x = info.origin.0 + x;
            let atlas_y = info.origin.1 + y;
            // FIXME: 貼り付けに失敗しそうな場合はエラーを出力してスキップ
            if atlas_x >= width || atlas_y >= height {
                eprintln!("Failed to paste texture: {}", info.id);
                eprintln!("Texture size: {}x{}", image.width(), image.height());
                continue;
            }
            atlas_image.put_pixel(atlas_x, atlas_y, *pixel);
        }
    }

    atlas_image
}
