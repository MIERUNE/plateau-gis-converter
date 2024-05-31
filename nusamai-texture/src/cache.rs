use image::DynamicImage;
use lru::LruCache;
use std::{num::NonZeroUsize, path::PathBuf, sync::Mutex};

pub struct ImageCache {
    pub cache: Mutex<LruCache<PathBuf, DynamicImage>>,
}

impl ImageCache {
    pub fn new(capacity: usize) -> Self {
        ImageCache {
            cache: Mutex::new(LruCache::new(NonZeroUsize::new(capacity).unwrap())),
        }
    }

    pub fn get_image(&self, path: &PathBuf) -> DynamicImage {
        let mut cache = self.cache.lock().unwrap();
        if let Some(image) = cache.get(path) {
            image.clone()
        } else {
            let image = image::open(path).unwrap();
            cache.put(path.clone(), image.clone());
            image
        }
    }
}
