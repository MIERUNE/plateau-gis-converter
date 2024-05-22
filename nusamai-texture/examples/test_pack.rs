use nusamai_texture::{
    SimpleTexturePlacer, Texture, TexturePacker, TexturePackerConfig, WebpAtlasExporter,
};
use std::path::Path;

fn main() {
    let config = TexturePackerConfig {
        max_width: 1024,
        max_height: 1024,
        padding: 2,
        // その他の設定項目を指定
    };

    let placer = SimpleTexturePlacer;
    let exporter = WebpAtlasExporter;

    let mut packer = TexturePacker::new(config, placer, exporter);

    let uv_coords = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let image_path = Path::new("path/to/image.png");
    let texture = Texture::new(&uv_coords, image_path);

    packer.add_texture("texture1".to_string(), texture);

    // 他のテクスチャを追加

    let output_path = Path::new("output/atlas.webp");
    packer.export(output_path);
}
