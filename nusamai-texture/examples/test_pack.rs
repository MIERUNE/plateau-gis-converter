use nusamai_texture::{
    CroppedTexture, SimpleTexturePlacer, TexturePacker, TexturePackerConfig, WebpAtlasExporter,
};
use std::path::Path;

fn main() {
    let config = TexturePackerConfig {
        max_width: 512,
        max_height: 512,
        padding: 2,
    };

    let placer = SimpleTexturePlacer;
    let exporter = WebpAtlasExporter;

    let mut packer = TexturePacker::new(config, placer, exporter);

    for i in 1..11 {
        let uv_coords = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let path_string = format!("nusamai-texture/examples/assets/{}.png", i);
        let image_path = Path::new(path_string.as_str());
        let texture = Texture::new(&uv_coords, image_path);

        // todo: テクスチャを追加した瞬間にUVが確定するはずなので、これを返したい
        packer.add_texture(format!("texture{}", i).to_string(), texture);
    }

    let output_path = Path::new("nusamai-texture/examples/output/atlas.webp");
    packer.export(output_path);
}
