use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};

use image::{DynamicImage, GenericImageView, ImageBuffer};
use rayon::prelude::*;
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
            cache: Cache::new(capacity, 1_000_000_000).unwrap(),
        }
    }

    pub fn get_or_insert(
        &self,
        uv_coords: &[(f64, f64)],
        image_path: &PathBuf,
        downsample_factor: &f32,
    ) -> CroppedTexture {
        match self.cache.get(image_path) {
            Some(image) => {
                let image = image.value();
                CroppedTexture::new(uv_coords, image_path, image, downsample_factor)
            }
            None => {
                let image = image::open(image_path).unwrap_or_else(|_| {
                    panic!("Failed to open image file {}", image_path.display())
                });
                let cost = image.width() * image.height() * image.color().bytes_per_pixel() as u32;
                self.cache
                    .insert(image_path.to_path_buf(), image.clone(), cost as i64);
                self.cache.wait().unwrap();

                CroppedTexture::new(uv_coords, image_path, &image, downsample_factor)
            }
        }
    }

    pub fn get_image(&self, path: &PathBuf) -> DynamicImage {
        match self.cache.get(path) {
            Some(image) => image.value().clone(),
            None => {
                let image = image::open(path).expect("Failed to open image file");
                let cost = image.width() * image.height() * image.color().bytes_per_pixel() as u32;
                self.cache
                    .insert(path.to_path_buf(), image.clone(), cost as i64);
                self.cache.wait().unwrap();

                image
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
    // The origin of the cropped image in the original image (top-left corner).
    pub origin: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub downsample_factor: DownsampleFactor,
    // UV coordinates for the cropped texture (bottom-left origin).
    pub cropped_uv_coords: Vec<(f64, f64)>,
}

impl CroppedTexture {
    pub fn new(
        uv_coords: &[(f64, f64)],
        image_path: &Path,
        image: &DynamicImage,
        downsample_factor: &f32,
    ) -> Self {
        let downsample_factor = DownsampleFactor::new(downsample_factor);

        let (width, height) = image.dimensions();

        // UV to pixel coordinates with clamping
        let pixel_coords: Vec<(u32, u32)> = uv_coords
            .iter()
            .map(|(u, v)| {
                (
                    (u.clamp(0.0, 1.0) * width as f64).min(width as f64 - 1.0) as u32,
                    ((1.0 - v.clamp(0.0, 1.0)) * height as f64).min(height as f64 - 1.0) as u32,
                )
            })
            .collect();

        // calc bbox
        let (min_x, min_y, max_x, max_y) = pixel_coords.iter().fold(
            (u32::MAX, u32::MAX, 0, 0),
            |(min_x, min_y, max_x, max_y), (x, y)| {
                (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
            },
        );

        let cropped_width = max_x - min_x;
        let cropped_height = max_y - min_y;

        // UV coordinates for the cropped image
        let dest_uv_coords = pixel_coords
            .iter()
            .map(|(x, y)| {
                (
                    (*x - min_x) as f64 / cropped_width as f64,
                    1.0 - (*y - min_y) as f64 / cropped_height as f64,
                )
            })
            .collect::<Vec<(f64, f64)>>();

        CroppedTexture {
            image_path: image_path.to_path_buf(),
            origin: (min_x, min_y),
            width: cropped_width,
            height: cropped_height,
            downsample_factor,
            cropped_uv_coords: dest_uv_coords,
        }
    }

    pub fn crop(&self, image: &DynamicImage) -> DynamicImage {
        let (x, y) = self.origin;
        let cropped_image = image.view(x, y, self.width, self.height).to_image();

        // Collect pixels into a Vec and then process in parallel
        let pixels: Vec<_> = cropped_image.enumerate_pixels().collect();

        let samples = 1;
        let num_threads = rayon::current_num_threads();
        let chunk_size = (pixels.len() / num_threads).clamp(1, pixels.len());

        let (sender, receiver) = mpsc::channel();

        // If the center coordinates of the pixel are contained within a polygon composed of UV coordinates, the pixel is written
        pixels
            .par_chunks(chunk_size)
            .for_each_with(sender, |s, chunk| {
                let mut local_results = Vec::new();

                for &(px, py, pixel) in chunk {
                    let mut is_inside = false;

                    'subpixels: for sx in 0..samples {
                        for sy in 0..samples {
                            let x = (px as f64 + (sx as f64 + 0.5) / samples as f64)
                                / self.width as f64;
                            let y = 1.0
                                - (py as f64 + (sy as f64 + 0.5) / samples as f64)
                                    / self.height as f64;
                            // Adjust x and y to the center of the pixel
                            let center_x = x + 0.5 / self.width as f64;
                            let center_y = y - 0.5 / self.height as f64;

                            if is_point_inside_polygon(
                                (center_x, center_y),
                                &self.cropped_uv_coords,
                            ) {
                                is_inside = true;
                                break 'subpixels;
                            }
                        }
                    }

                    if is_inside {
                        local_results.push((px, py, *pixel));
                    } else {
                        // FIXME: Do not crop temporarily because pixel boundary jaggies will occur.
                        local_results.push((px, py, *pixel));
                    }
                }

                s.send(local_results).unwrap();
            });

        // Collect results in the main thread
        let mut clipped = ImageBuffer::new(self.width, self.height);
        for received in receiver {
            for (px, py, pixel) in received {
                clipped.put_pixel(px, py, pixel);
            }
        }

        // Downsample
        let scaled_width = (clipped.width() as f32 * self.downsample_factor.value()) as u32;
        let scaled_height = (clipped.height() as f32 * self.downsample_factor.value()) as u32;

        DynamicImage::ImageRgba8(image::imageops::resize(
            &clipped,
            scaled_width,
            scaled_height,
            image::imageops::FilterType::Triangle,
        ))
    }
}

fn is_point_inside_polygon(test_point: (f64, f64), polygon: &[(f64, f64)]) -> bool {
    let mut is_inside = false;
    let mut previous_vertex_index = polygon.len() - 1;

    for current_vertex_index in 0..polygon.len() {
        let (current_x, current_y) = polygon[current_vertex_index];
        let (previous_x, previous_y) = polygon[previous_vertex_index];

        let is_y_between_vertices = (current_y > test_point.1) != (previous_y > test_point.1);
        let does_ray_intersect = test_point.0
            < (previous_x - current_x) * (test_point.1 - current_y) / (previous_y - current_y)
                + current_x;

        if is_y_between_vertices && does_ray_intersect {
            is_inside = !is_inside;
        }

        previous_vertex_index = current_vertex_index;
    }

    is_inside
}
