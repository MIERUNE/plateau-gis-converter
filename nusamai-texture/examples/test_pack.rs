use std::{default, path::Path};

use nusamai_texture::{
    export::WebpAtlasExporter,
    pack::{TexturePacker, TexturePackerConfig},
    place::{GuillotineTexturePlacer, SimpleTexturePlacer},
    texture::CroppedTexture,
};

fn main() {
    // let config = TexturePackerConfig {
    //     max_width: 512,
    //     max_height: 512,
    //     padding: 2,
    // };

    let placer = SimpleTexturePlacer::default();
    // let placer = GuillotineTexturePlacer::default();

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
