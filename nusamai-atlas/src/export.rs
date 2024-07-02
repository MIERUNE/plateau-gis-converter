use std::io::Write as _;
use std::path::Path;

use hashbrown::HashMap;
use image::ImageBuffer;

use crate::place::PlacedTextureInfo;
use crate::texture::{CroppedTexture, TextureCache};

pub trait AtlasExporter {
    fn export(
        &self,
        atlas_data: &[PlacedTextureInfo],
        textures: &HashMap<String, CroppedTexture>,
        output_path: &Path,
        texture_cache: &TextureCache,
    );

    fn get_extension(&self) -> &str;
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

    fn export(
        &self,
        atlas_data: &[PlacedTextureInfo],
        textures: &HashMap<String, CroppedTexture>,
        output_path: &Path,
        texture_cache: &TextureCache,
    ) {
        let max_width = atlas_data
            .iter()
            .map(|info| info.origin.0 + info.width)
            .max()
            .unwrap_or(0);
        let max_height = atlas_data
            .iter()
            .map(|info| info.origin.1 + info.height)
            .max()
            .unwrap_or(0);

        let mut atlas_image = ImageBuffer::new(max_width, max_height);

        // let uv_coord_memo_path = output_path.with_extension("txt");
        // let mut uv_coord_memo_file = std::fs::File::create(uv_coord_memo_path).unwrap();
        // let mut contents = String::new();

        for info in atlas_data {
            // let id = info.id.clone();
            // let atlas_id = info.atlas_id.clone();
            // let origin = info.origin;
            // let placed_uv_coords = info.placed_uv_coords.clone();

            // contents.push_str(&format!(
            //     "id: {}, atlas_id: {}, origin: ({}, {}), uv: {:?}\n",
            //     id, atlas_id, origin.0, origin.1, placed_uv_coords,
            // ));

            let texture = textures.get(&info.id).unwrap();
            let cropped = texture.crop(&texture_cache.get_image(&texture.image_path));
            let image = cropped.as_rgba8().unwrap();

            for (x, y, pixel) in image.enumerate_pixels() {
                let atlas_x = info.origin.0 + x;
                let atlas_y = info.origin.1 + y;
                atlas_image.put_pixel(atlas_x, atlas_y, *pixel);
            }
        }
        // uv_coord_memo_file.write_all(contents.as_bytes()).unwrap();

        let output_path = output_path.with_extension(self.get_extension());

        atlas_image
            .save_with_format(output_path, image::ImageFormat::WebP)
            .unwrap();
    }
}
