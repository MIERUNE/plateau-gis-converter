use std::path::{Path, PathBuf};

use image::{DynamicImage, GenericImageView};
use stretto::Cache;

#[derive(Debug, Clone)]
pub struct DownsampleFactor(f32);

impl DownsampleFactor {
    pub fn new(factor: &f32) -> Self {
        if (0.0..=1.0).contains(factor) {
            DownsampleFactor(*factor)
        } else {
            panic!("The argument must be entered between 0~1.") //FIXME: panic! is not recommended
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

pub struct TextureCache {
    cache: Cache<PathBuf, DynamicImage>,
}

impl TextureCache {
    pub fn new(capacity: usize) -> Self {
        TextureCache {
            cache: Cache::new(capacity, 10_000_000).unwrap(),
        }
    }

    pub fn get_or_insert(
        &self,
        uv_coords: &[(f32, f32)],
        image_path: &PathBuf,
        downsample_factor: &f32,
    ) -> CroppedTexture {
        match self.cache.get(image_path) {
            Some(image) => {
                let image = image.value();
                CroppedTexture::new(uv_coords, image_path, image, downsample_factor)
            }
            None => {
                let image = image::open(image_path).expect("Failed to open image file");
                let cost = image.width() * image.height() * image.color().bytes_per_pixel() as u32;
                self.cache
                    .insert(image_path.to_path_buf(), image.clone(), cost as i64);
                self.cache.wait().unwrap();

                CroppedTexture::new(uv_coords, image_path, &image, downsample_factor)
            }
        }
    }
}

impl Drop for TextureCache {
    fn drop(&mut self) {
        self.cache.close().unwrap();
    }
}

pub struct CroppedTexture {
    pub image_path: PathBuf,
    pub origin: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub downsample_factor: DownsampleFactor,
    // PLATEAU textures contain multiple surface/building textures in a single image,
    // so it is necessary to specify UV coordinates for each polygon and cut them out.
    pub cropped_uv_coords: Vec<(f32, f32)>,
}

impl CroppedTexture {
    pub fn new(
        uv_coords: &[(f32, f32)],
        image_path: &Path,
        image: &DynamicImage,
        downsample_factor: &f32,
    ) -> Self {
        let downsample_factor = DownsampleFactor::new(downsample_factor);

        let (min_x, min_y, max_x, max_y) = uv_coords.iter().fold(
            (1.0_f32, 1.0_f32, 0.0_f32, 0.0_f32),
            |(min_x, min_y, max_x, max_y), (x, y)| {
                (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
            },
        );

        let (width, height) = image.dimensions();

        let left = (min_x * width as f32) as u32;
        let top = (min_y * height as f32) as u32;
        let right = (max_x * width as f32) as u32;
        let bottom = (max_y * height as f32) as u32;

        let cropped_width = right - left;
        let cropped_height = bottom - top;

        let dest_uv_coords = uv_coords
            .iter()
            .map(|(u, v)| ((u - min_x) / (max_x - min_x), (v - min_y) / (max_y - min_y)))
            .collect::<Vec<(f32, f32)>>();

        CroppedTexture {
            image_path: image_path.to_path_buf(),
            origin: (left, top),
            width: cropped_width,
            height: cropped_height,
            downsample_factor,
            cropped_uv_coords: dest_uv_coords.to_vec(),
        }
    }

    pub fn crop(&self, image: &DynamicImage) -> DynamicImage {
        let (x, y) = self.origin;
        let cropped_image = image.view(x, y, self.width, self.height).to_image();

        let scaled_width = (self.width as f32 * self.downsample_factor.value()) as u32;
        let scaled_height = (self.height as f32 * self.downsample_factor.value()) as u32;

        DynamicImage::ImageRgba8(image::imageops::resize(
            &cropped_image,
            scaled_width,
            scaled_height,
            image::imageops::FilterType::Triangle,
        ))
    }
}
