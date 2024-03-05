//! Material mangement

use std::{hash::Hash, path::Path, time::Instant};

use indexmap::IndexSet;
use nusamai_gltf_json::BufferView;
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

impl Material {
    pub fn to_gltf(
        &self,
        texture_set: &mut IndexSet<Texture, ahash::RandomState>,
    ) -> nusamai_gltf_json::Material {
        let tex = if let Some(texture) = &self.base_texture {
            let (tex_idx, _) = texture_set.insert_full(texture.clone());
            Some(nusamai_gltf_json::TextureInfo {
                index: tex_idx as u32,
                tex_coord: 0,
                ..Default::default()
            })
        } else {
            None
        };
        nusamai_gltf_json::Material {
            pbr_metallic_roughness: Some(nusamai_gltf_json::MaterialPbrMetallicRoughness {
                base_color_factor: to_f64x4(self.base_color),
                metallic_factor: 0.2,
                roughness_factor: 0.5,
                base_color_texture: tex,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Texture {
    pub uri: Url,
}

impl Texture {
    pub fn to_gltf(
        &self,
        images: &mut IndexSet<Image, ahash::RandomState>,
    ) -> nusamai_gltf_json::Texture {
        let (image_index, _) = images.insert_full(Image {
            uri: self.uri.clone(),
        });
        nusamai_gltf_json::Texture {
            source: Some(image_index as u32),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Image {
    pub uri: Url,
}

impl Image {
    pub fn to_gltf(
        &self,
        buffer_views: &mut Vec<BufferView>,
        bin_content: &mut Vec<u8>,
    ) -> std::io::Result<nusamai_gltf_json::Image> {
        if let Ok(path) = self.uri.to_file_path() {
            // NOTE: temporary implementation
            let content = load_image(&path)?;

            buffer_views.push(BufferView {
                name: Some("image".to_string()),
                byte_offset: bin_content.len() as u32,
                byte_length: content.len() as u32,
                ..Default::default()
            });

            bin_content.extend(content);

            Ok(nusamai_gltf_json::Image {
                mime_type: Some(nusamai_gltf_json::MimeType::ImageJpeg),
                buffer_view: Some(buffer_views.len() as u32 - 1),
                ..Default::default()
            })
        } else {
            Ok(nusamai_gltf_json::Image {
                uri: Some(self.uri.to_string()),
                ..Default::default()
            })
        }
    }
}

// NOTE: temporary implementation
fn load_image(path: &Path) -> std::io::Result<Vec<u8>> {
    log::info!("Decoding image: {:?}", path);

    if let Some(ext) = path.extension() {
        match ext.to_str().unwrap() {
            // use `image crate` for TIFF
            "tif" | "tiff" | "png" | "jpg" | "jpeg" => {
                let t = Instant::now();
                let image = image::open(path)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
                log::debug!("Image decoding took {:?}", t.elapsed());

                let content: Vec<u8> = Vec::new();
                let mut writer = std::io::Cursor::new(content);
                image
                    .write_to(&mut writer, image::ImageOutputFormat::Jpeg(100))
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
                Ok(writer.into_inner())
            }
            _ => {
                let err = format!("Unsupported image format: {:?}", path);
                log::error!("{}", err);
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
            }
        }
    } else {
        let err = format!("Unsupported image format: {:?}", path);
        log::error!("{}", err);
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
    }
}

fn to_f64x4(c: [f32; 4]) -> [f64; 4] {
    [
        f64::from(c[0]),
        f64::from(c[1]),
        f64::from(c[2]),
        f64::from(c[3]),
    ]
}
