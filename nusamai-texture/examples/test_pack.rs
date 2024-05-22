use nusamai_texture::{
    PngAtlasExporter, SimpleTexturePlacer, Texture, TexturePacker, TexturePackerConfig,
};

fn main() {
    let config = TexturePackerConfig {
        max_width: 1024,
        max_height: 1024,
        padding: 2,
        // その他の設定項目を指定
    };

    let placer = SimpleTexturePlacer;
    let exporter = PngAtlasExporter;

    let mut packer = TexturePacker::new(config, placer, exporter);

    let uv_coords = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let image_path = "path/to/image.png";
    let texture = Texture::new(&uv_coords, image_path);

    packer.add_texture("texture1".to_string(), texture);

    // 他のテクスチャを追加

    packer.export("output/atlas.png");
}
