use std::path::Path;

use nusamai_texture::{
    export::WebpAtlasExporter,
    pack::{TexturePacker, TexturePackerConfig},
    place::SimpleTexturePlacer,
    texture::CroppedTexture,
};

fn main() {
    let config = TexturePackerConfig {
        max_width: 512,
        max_height: 512,
        padding: 2,
    };

    let placer = SimpleTexturePlacer::default();
    let exporter = WebpAtlasExporter;

    let mut packer = TexturePacker::new(config, placer, exporter);
    for i in 0..5 {
        for j in 1..11 {
            let uv_coords = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
            let path_string = format!("nusamai-texture/examples/assets/{}.png", j);
            let image_path = Path::new(path_string.as_str());
            let texture = CroppedTexture::new(&uv_coords, image_path);

            packer.add_texture(format!("texture_{}_{}", i, j).to_string(), texture);
        }
    }

    packer.finalize();

    let output_path = Path::new("nusamai-texture/examples/output/");
    packer.export(output_path);
}
