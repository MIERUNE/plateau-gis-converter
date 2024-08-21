use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader};
use rayon::prelude::*;
use stretto::Cache;
use sys_info::mem_info;

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

// Cache for storing the only size of the image
pub struct TextureSizeCache {
    cache: Cache<PathBuf, (u32, u32)>,
}

impl TextureSizeCache {
    pub fn new() -> Self {
        TextureSizeCache {
            cache: Cache::new(1_000_000, 1_000_000).unwrap(),
        }
    }

    pub fn get_or_insert(&self, image_path: &PathBuf) -> (u32, u32) {
        match self.cache.get(image_path) {
            Some(size) => *size.value(),
            None => {
                let size = get_image_size(image_path).unwrap();
                // Since it only retains the size of the texture, set the cost to 1 for everything.
                let cost = 1;
                self.cache.insert(image_path.to_path_buf(), size, cost);
                self.cache.wait().unwrap();

                size
            }
        }
    }
}

impl Default for TextureSizeCache {
    fn default() -> Self {
        TextureSizeCache::new()
    }
}

impl Drop for TextureSizeCache {
    fn drop(&mut self) {
        self.cache.close().unwrap();
    }
}

// Cache for storing the image
pub struct TextureCache {
    cache: Cache<PathBuf, DynamicImage>,
}

impl TextureCache {
    pub fn new(capacity: usize) -> Self {
        let default_capacity = get_cache_size().unwrap();
        if capacity == 0 {
            TextureCache {
                cache: Cache::new(default_capacity, 2_000_000_000).unwrap(),
            }
        } else {
            TextureCache {
                cache: Cache::new(capacity, 2_000_000_000).unwrap(),
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

fn get_cache_size() -> Result<usize, String> {
    const MIN_CACHE_SIZE: usize = 100 * 1024 * 1024; // 100MB
    const MAX_CACHE_SIZE: usize = 2 * 1024 * 1024 * 1024; // 2GB

    match mem_info() {
        Ok(mem) => {
            let total_memory = mem.total as usize * 1024;
            // 15% of total memory
            let cache_size = (total_memory as f64 * 0.15) as usize;
            Ok(cache_size.clamp(MIN_CACHE_SIZE, MAX_CACHE_SIZE))
        }

        Err(e) => Err(format!("Failed to retrieve memory information.: {}", e)),
    }
}

impl Drop for TextureCache {
    fn drop(&mut self) {
        self.cache.close().unwrap();
    }
}

// A structure that retains an image cut out from the original image.
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
        image_path: &Path,
        size: (u32, u32),
        uv_coords: &[(f64, f64)],
        downsample_factor: DownsampleFactor,
    ) -> Self {
        let pixel_coords = uv_to_pixel_coords(uv_coords, size.0, size.1);
        let (min_x, min_y, max_x, max_y) = calc_bbox(&pixel_coords);

        let cropped_width = max_x - min_x;
        let cropped_height = max_y - min_y;

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
        let chunk_size = (pixels.len() / num_threads).clamp(1, pixels.len() + 1);

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

// utils

fn get_image_size<P: AsRef<Path>>(file_path: P) -> Result<(u32, u32), image::ImageError> {
    let reader = ImageReader::open(file_path)?;
    let dimensions = reader.into_dimensions()?;
    Ok(dimensions)
}

fn uv_to_pixel_coords(uv_coords: &[(f64, f64)], width: u32, height: u32) -> Vec<(u32, u32)> {
    uv_coords
        .iter()
        .map(|(u, v)| {
            (
                (u.clamp(0.0, 1.0) * width as f64).min(width as f64 - 1.0) as u32,
                ((1.0 - v.clamp(0.0, 1.0)) * height as f64).min(height as f64 - 1.0) as u32,
            )
        })
        .collect()
}

fn calc_bbox(pixel_coords: &[(u32, u32)]) -> (u32, u32, u32, u32) {
    pixel_coords.iter().fold(
        (u32::MAX, u32::MAX, 0, 0),
        |(min_x, min_y, max_x, max_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
        },
    )
}
