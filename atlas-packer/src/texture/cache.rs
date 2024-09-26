use std::path::PathBuf;

use image::DynamicImage;
use stretto::Cache;
use sys_info::mem_info;

use super::utils::get_image_size;

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
