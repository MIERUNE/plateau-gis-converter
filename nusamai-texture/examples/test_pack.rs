use std::{default, path::Path};

use nusamai_texture::{
    export::WebpAtlasExporter,
    pack::{TexturePacker, TexturePackerConfig},
    place::{GuillotinePlacer, SimpleTexturePlacer},
    texture::CroppedTexture,
};

fn main() {
    let config = TexturePackerConfig {
        max_width: 1024,
        max_height: 1024,
        padding: 1,
    };

    let placer = SimpleTexturePlacer::new(config);
    // let placer = GuillotinePlacer::new(config);

    let exporter = WebpAtlasExporter;

    let mut packer = TexturePacker::new(placer, exporter);
    for i in 0..3 {
        for j in 1..11 {
            let uv_coords = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
            let path_string = format!("examples/assets/{}.png", j);
            let image_path = Path::new(path_string.as_str());
            let texture = CroppedTexture::new(&uv_coords, image_path);

            packer.add_texture(format!("texture_{}_{}", i, j).to_string(), texture);
        }
    }

    packer.finalize();

    let output_path = Path::new("examples/output/");
    packer.export(output_path);
}
