use std::path::Path;

use nusamai_texture::{
    export::WebpAtlasExporter,
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::CroppedTexture,
};

fn main() {
    let scale_factor = 1.0;
    let config = TexturePlacerConfig {
        max_width: 1024,
        max_height: 1024,
        padding: 0,
        scale_factor,
    };

    let placer = GuillotineTexturePlacer::new(config);

    let exporter = WebpAtlasExporter;

    let mut packer = TexturePacker::new(placer, exporter);
    for i in 0..10 {
        for j in 1..11 {
            let uv_coords = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
            let path_string = format!("examples/assets/{}.png", j);
            let image_path = Path::new(path_string.as_str());
            // todo:
            // スケールされたUV座標が返却されるようにする
            // 与えられたuv_coordsを修正して格納できるようにする（org_uvsとdist_uvsとか？）
            // exportするタイミングで、実際にスケーリングし、アトラスに書き込むようにする
            let texture = CroppedTexture::new(&uv_coords, image_path, scale_factor);

            packer.add_texture(format!("texture_{}_{}", i, j).to_string(), texture);
        }
    }

    packer.finalize();

    let output_path = Path::new("examples/output/");
    packer.export(output_path);
}
