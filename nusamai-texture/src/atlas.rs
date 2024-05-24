use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct TexturePackerConfig {
    pub max_width: u32,
    pub max_height: u32,
    pub padding: u32,
    // その他の設定項目を追加
    // 回転の許可・複数ページの許可・解像度の調整・リサンプリング手法の指定など
}

pub struct CroppedTexture {
    pub image_path: PathBuf,
    pub u: u32,
    pub v: u32,
    pub width: u32,
    pub height: u32,
}

impl CroppedTexture {
    pub fn new(uv_coords: &[(f32, f32)], image_path: &Path) -> Self {
        println!("uv_coords: {:?}", uv_coords);
        let image = image::open(image_path).expect("Failed to open image file");

        let (min_x, min_y, max_x, max_y) = uv_coords.iter().fold(
            (1.0_f32, 1.0_f32, 0.0_f32, 0.0_f32),
            |(min_x, min_y, max_x, max_y), (x, y)| {
                (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
            },
        );

        let (width, height) = image.dimensions();
        println!(
            "original image size -> width: {}, height: {}",
            width, height
        );
        let left = (min_x * width as f32) as u32;
        let top = (min_y * height as f32) as u32;
        let right = (max_x * width as f32) as u32;
        let bottom = (max_y * height as f32) as u32;

        let cropped_width = right - left;
        let cropped_height = bottom - top;
        println!(
            "cropped image size -> width: {}, height: {}",
            cropped_width, cropped_height
        );

        CroppedTexture {
            image_path: image_path.to_path_buf(),
            u: left,
            v: top,
            width: cropped_width,
            height: cropped_height,
        }
    }

    pub fn crop(&self) -> DynamicImage {
        let image = image::open(&self.image_path).expect("Failed to open image file");
        let cropped_image = image
            .view(self.u, self.v, self.width, self.height)
            .to_image();
        DynamicImage::ImageRgba8(cropped_image)
    }
}

// アトラスに配置されたテクスチャの情報
#[derive(Debug)]
pub struct PlacedTextureInfo {
    pub id: String,
    pub u: u32,
    pub v: u32,
    pub width: u32,
    pub height: u32,
}

// テクスチャの配置方法
pub trait TexturePlacer {
    fn place_texture(
        &mut self,
        id: &str,
        texture: &CroppedTexture,
        config: &TexturePackerConfig,
    ) -> Option<PlacedTextureInfo>;
}

#[derive(Default)]
pub struct SimpleTexturePlacer {
    current_x: u32,
    current_y: u32,
    max_height_in_row: u32,
}

impl TexturePlacer for SimpleTexturePlacer {
    fn place_texture(
        &mut self,
        id: &str,
        texture: &CroppedTexture,
        config: &TexturePackerConfig,
    ) -> Option<PlacedTextureInfo> {
        if self.current_x + texture.width > config.max_width {
            self.current_x = 0;
            self.current_y += self.max_height_in_row + config.padding;
            self.max_height_in_row = 0;
        }

        if self.current_y + texture.height > config.max_height {
            // テクスチャがアトラスに収まらない場合は、Noneを返す
            return None;
        }

        let texture_info = PlacedTextureInfo {
            id: id.to_string(),
            u: self.current_x,
            v: self.current_y,
            width: texture.width,
            height: texture.height,
        };

        self.current_x += texture.width + config.padding;
        self.max_height_in_row = self.max_height_in_row.max(texture.height);

        Some(texture_info)
    }
}

// アトラスの書き出し
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
        // アトラス画像のサイズを計算
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

        // アトラス画像を作成
        let mut atlas_image = ImageBuffer::new(max_width, max_height);

        // テクスチャをアトラス画像に配置
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

        // アトラス画像をWebPフォーマットで出力
        atlas_image
            .save_with_format(output_path, image::ImageFormat::WebP)
            .unwrap();
    }
}

pub struct TexturePacker<P: TexturePlacer, E: AtlasExporter> {
    textures: HashMap<String, CroppedTexture>,
    atlas_data: Vec<PlacedTextureInfo>,
    config: TexturePackerConfig,
    placer: P,
    exporter: E,
}

impl<P: TexturePlacer, E: AtlasExporter> TexturePacker<P, E> {
    pub fn new(config: TexturePackerConfig, placer: P, exporter: E) -> Self {
        TexturePacker {
            textures: HashMap::new(),
            atlas_data: Vec::new(),
            config,
            placer,
            exporter,
        }
    }

    pub fn add_texture(&mut self, id: String, texture: CroppedTexture) {
        if let Some(texture_info) = self.placer.place_texture(&id, &texture, &self.config) {
            self.textures.insert(id, texture);
            self.atlas_data.push(texture_info);
        } else {
            // テクスチャがアトラスに収まらない場合は、新しいアトラスを作成するなどの処理が必要
            panic!("Texture does not fit in the atlas");
        }
    }

    pub fn export(&self, output_path: &Path) {
        self.exporter
            .export(&self.atlas_data, &self.textures, output_path);
    }
}
