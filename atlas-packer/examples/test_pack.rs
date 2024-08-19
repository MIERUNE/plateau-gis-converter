use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;

use rayon::prelude::*;

use atlas_packer::{
    export::JpegAtlasExporter,
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::{DownsampleFactor, TextureCache},
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
            let path_string: String = format!("atlas-packer/examples/assets/{}.png", j);
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
    let placer = GuillotineTexturePlacer::new(config.clone());
    let exporter = JpegAtlasExporter::default();
    let packer = Mutex::new(TexturePacker::new(placer, exporter));

    // Texture cache
    // todo: この段階でキャッシュする必要はないので、TextureCacheを使わないように変更する
    // アトラスをエンコードするときにキャッシュすれば良い
    let texture_cache = TextureCache::new(100_000_000);

    let start = Instant::now();

    // Add textures to the atlas
    // todo: get_or_insertを使わないように変更する
    // 画像のurlとUV座標を受けとり、画像のサイズと配置するピクセル座標を返す関数を実装
    polygons.par_iter().for_each(|polygon| {
        let texture = texture_cache.get_or_insert(
            &polygon.uv_coords,
            &polygon.texture_uri,
            &polygon.downsample_factor.value(),
        );
        let _ = packer
            .lock()
            .unwrap()
            .add_texture(polygon.id.clone(), texture);
        // println!("{:?}", info);
    });

    let duration = start.elapsed();
    println!("atlas process {:?}", duration);

    let mut packer = packer.into_inner().unwrap();

    packer.finalize();

    let start = Instant::now();

    let output_dir = Path::new("atlas-packer/examples/output/");
    packer.export(output_dir, &texture_cache, config.width(), config.height());

    let duration = start.elapsed();
    println!("atlas export process {:?}", duration);

    let duration = all_process_start.elapsed();
    println!("all process {:?}", duration);
}
