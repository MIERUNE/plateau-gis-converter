use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use atlas_packer::texture::PolygonMappedTexture;
use rayon::prelude::*;

use atlas_packer::{
    export::JpegAtlasExporter,
    pack::AtlasPacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::{
        cache::{TextureCache, TextureSizeCache},
        DownsampleFactor,
    },
};

#[derive(Debug, Clone)]
struct Polygon {
    id: String,
    uv_coords: Vec<(f64, f64)>,
    texture_uri: PathBuf,
    downsample_factor: DownsampleFactor,
}

fn main() {
    let all_process_start = Instant::now();

    // 3D Tiles Sink passes the texture path and UV coordinates for each polygon
    let mut polygons: Vec<Polygon> = Vec::new();
    let downsample_factor = 1.0;
    for i in 0..200 {
        for j in 1..11 {
            // Specify a polygon to crop around the center of the image
            let uv_coords = vec![
                (0.2, 0.3),
                (0.3, 0.2),
                (0.6, 0.2),
                (0.8, 0.3),
                (0.8, 0.7),
                (0.6, 0.8),
                (0.3, 0.8),
                (0.2, 0.7),
            ];
            let path_string: String = format!("./examples/assets/{}.png", j);
            let image_path = PathBuf::from(path_string.as_str());
            polygons.push(Polygon {
                id: format!("texture_{}_{}", i, j),
                uv_coords,
                texture_uri: image_path,
                downsample_factor: DownsampleFactor::new(&downsample_factor),
            });
        }
    }

    // initialize texture packer
    let config = TexturePlacerConfig {
        width: 4096,
        height: 4096,
        padding: 0,
    };

    let packer = Mutex::new(AtlasPacker::default());

    let packing_start = Instant::now();

    // cache image size
    let texture_size_cache = TextureSizeCache::new();
    // place textures on the atlas
    polygons.par_iter().for_each(|polygon| {
        let place_start = Instant::now();
        let texture_size = texture_size_cache.get_or_insert(&polygon.texture_uri);
        let cropped_texture = PolygonMappedTexture::new(
            &polygon.texture_uri,
            texture_size,
            &polygon.uv_coords,
            polygon.downsample_factor.clone(),
        );

        packer
            .lock()
            .unwrap()
            .add_texture(polygon.id.clone(), cropped_texture);
        let place_duration = place_start.elapsed();
        println!("{}, texture place process {:?}", polygon.id, place_duration);
    });

    let packer = packer.into_inner().unwrap();
    let packed = packer.pack(GuillotineTexturePlacer::new(config.clone()));

    let duration = packing_start.elapsed();
    println!("all packing process {:?}", duration);

    let start = Instant::now();

    // Caches the original textures for exporting to an atlas.
    let texture_cache = TextureCache::new(100_000_000);
    let output_dir = Path::new("./examples/output/");
    packed.export(
        JpegAtlasExporter::default(),
        output_dir,
        &texture_cache,
        config.width(),
        config.height(),
    );
    let duration = start.elapsed();
    println!("all atlas export process {:?}", duration);

    let duration = all_process_start.elapsed();
    println!("all process {:?}", duration);
}
