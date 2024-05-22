use image::{DynamicImage, GenericImageView, SubImage};
use std::collections::HashMap;
use std::path::Path;

pub struct Texture {
    pub image: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(uv_coords: &[(f32, f32)], image_path: &Path) -> Self {
        let image = image::open(image_path).expect("Failed to open image file");

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

        let cropped_image = image.view(left, top, right - left, bottom - top).to_image();

        Texture {
            image: DynamicImage::ImageRgba8(cropped_image),
            width: right - left,
            height: bottom - top,
        }
    }
}

pub struct TexturePackerConfig {
    pub max_width: u32,
    pub max_height: u32,
    pub padding: u32,
    // その他の設定項目を追加
    // 回転の許可・複数ページの許可・解像度の調整・リサンプリング手法の指定など
}

// アトラスに配置されたテクスチャの情報
pub struct TextureInfo {
    // uvは左上座標を表す
    pub id: String,
    pub u: u32,
    pub v: u32,
    pub width: u32,
    pub height: u32,
}

// テクスチャの配置方法
pub trait TexturePlacer {
    fn place_textures(
        &self,
        textures: &HashMap<String, Texture>,
        config: &TexturePackerConfig,
    ) -> Vec<TextureInfo>;
}

pub struct SimpleTexturePlacer;

impl TexturePlacer for SimpleTexturePlacer {
    fn place_textures(
        &self,
        textures: &HashMap<String, Texture>,
        config: &TexturePackerConfig,
    ) -> Vec<TextureInfo> {
        // todo: シンプルなテクスチャ配置アルゴリズムを実装

        // 返り値は、TextureInfo構造体のVec
        Vec::new()
    }
}

// アトラスの書き出し
pub trait AtlasExporter {
    fn export(&self, atlas_data: &[TextureInfo], output_path: &Path);
}

pub struct WebpAtlasExporter;

impl AtlasExporter for WebpAtlasExporter {
    fn export(&self, atlas_data: &[TextureInfo], output_path: &Path) {
        // WebPフォーマットでアトラス画像を出力する処理を実装
    }
}

pub struct TexturePacker<P: TexturePlacer, E: AtlasExporter> {
    textures: HashMap<String, Texture>,
    config: TexturePackerConfig,
    placer: P,
    exporter: E,
}

impl<P: TexturePlacer, E: AtlasExporter> TexturePacker<P, E> {
    pub fn new(config: TexturePackerConfig, placer: P, exporter: E) -> Self {
        TexturePacker {
            textures: HashMap::new(),
            config,
            placer,
            exporter,
        }
    }

    pub fn add_texture(&mut self, id: String, texture: Texture) {
        // 画像オブジェクトとIDをパッカーに追加
        self.textures.insert(id, texture);
    }

    pub fn export(&self, output_path: &Path) {
        let atlas_data = self.placer.place_textures(&self.textures, &self.config);
        self.exporter.export(&atlas_data, output_path);
    }
}
