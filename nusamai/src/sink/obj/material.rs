//! Material mangement

use std::{hash::Hash, path::Path, time::Instant};

use crate::pipeline::Feedback;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct Material {
    pub base_color: [f32; 4],
    pub base_texture: Option<Texture>,
    // NOTE: Adjust the hash implementation if you add more fields
}

impl Eq for Material {}

impl Hash for Material {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.base_color.iter().for_each(|c| c.to_bits().hash(state));
        self.base_texture.hash(state);
    }
}

#[derive(Debug, Serialize, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Texture {
    pub uri: Url,
}

#[derive(Debug, Serialize, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Image {
    pub uri: Url,
}

// NOTE: temporary implementation
pub fn load_image(feedback: &Feedback, path: &Path) -> std::io::Result<Vec<u8>> {
    if let Some(ext) = path.extension() {
        match ext.to_ascii_lowercase().to_str() {
            Some("tif" | "tiff" | "png") => {
                feedback.info(format!("Decoding image: {:?}", path));
                let t = Instant::now();
                let image = image::open(path)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
                feedback.debug(format!("Image decoding took {:?}", t.elapsed()));

                let t = Instant::now();
                let mut writer = std::io::Cursor::new(Vec::new());
                let encoder = image::codecs::png::PngEncoder::new(&mut writer);
                image
                    .write_with_encoder(encoder)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
                log::debug!("Image encoding took {:?}", t.elapsed());

                Ok(writer.into_inner())
            }
            Some("jpg" | "jpeg") => {
                feedback.info(format!("Embedding a jpeg as is: {:?}", path));
                Ok(std::fs::read(path)?)
            }
            _ => {
                let err = format!("Unsupported image format: {:?}", path);
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
            }
        }
    } else {
        let err = format!("Unsupported image format: {:?}", path);
        log::error!("{}", err);
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
    }
}
