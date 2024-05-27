use image::ImageBuffer;
use std::collections::HashMap;
use std::path::Path;

use crate::place::PlacedTextureInfo;
use crate::texture::CroppedTexture;

pub trait AtlasExporter {
    fn export(
        &self,
        atlas_data: &[PlacedTextureInfo],
        textures: &HashMap<String, CroppedTexture>,
        output_path: &Path,
    );
}

pub struct WebpAtlasExporter;

impl AtlasExporter for WebpAtlasExporter {
    fn export(
        &self,
        atlas_data: &[PlacedTextureInfo],
        textures: &HashMap<String, CroppedTexture>,
        output_path: &Path,
    ) {
        let max_width = atlas_data
            .iter()
            .map(|info| info.u + info.width)
            .max()
            .unwrap_or(0);
        let max_height = atlas_data
            .iter()
            .map(|info| info.v + info.height)
            .max()
            .unwrap_or(0);

        let mut atlas_image = ImageBuffer::new(max_width, max_height);

        for info in atlas_data {
            let texture = textures.get(&info.id).unwrap();
            let cropped = texture.crop();
            let image = cropped.as_rgba8().unwrap();

            for (x, y, pixel) in image.enumerate_pixels() {
                let atlas_x = info.u + x;
                let atlas_y = info.v + y;
                atlas_image.put_pixel(atlas_x, atlas_y, *pixel);
            }
        }

        atlas_image
            .save_with_format(output_path, image::ImageFormat::WebP)
            .unwrap();
    }
}
