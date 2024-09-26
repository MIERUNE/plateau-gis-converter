mod utils;

use std::path::Path;

use atlas_packer::{
    export::PngAtlasExporter,
    pack::AtlasPacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::cache::TextureCache,
};

use utils::unused_pixels;

fn main() {
    let texture_cache = TextureCache::new(100_000_000);

    let config = TexturePlacerConfig::new(500, 500, 1);

    let packer = AtlasPacker::default();
    let packed = packer.pack(GuillotineTexturePlacer::new(config.clone()));

    let output_dir = Path::new("examples/output/");
    packed.export(
        PngAtlasExporter::default(),
        output_dir,
        &texture_cache,
        config.width(),
        config.height(),
    );

    let (all_pixels, unused_pixels) = unused_pixels::unused_pixels();

    println!("unused pixels: {} / {}", unused_pixels, all_pixels);
}
