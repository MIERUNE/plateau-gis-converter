use std::path::{Path, PathBuf};

use nusamai_atlas::{
    export::WebpAtlasExporter,
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::{DownsampleFactor, TextureCache},
};

#[derive(Debug, Clone)]
struct Polygon {
    id: String,
    uv_coords: Vec<(f32, f32)>,
    texture_uri: PathBuf,
    downsample_factor: DownsampleFactor,
}

fn main() {
    // 3D Tiles Sink passes the texture path and UV coordinates for each polygon
    let mut polygons: Vec<Polygon> = Vec::new();
    let downsample_factor = 1.0;
    for i in 0..10 {
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
            let path_string: String = format!("nusamai-atlas/examples/assets/{}.png", j);
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
    let config = TexturePlacerConfig::default();
    let placer = GuillotineTexturePlacer::new(config);
    let exporter = WebpAtlasExporter::default();
    let mut packer = TexturePacker::new(placer, exporter);

    // Texture cache
    let texture_cache = TextureCache::new(8192);

    // Add textures to the atlas
    polygons.iter().for_each(|polygon| {
        let texture = texture_cache.get_or_insert(
            &polygon.uv_coords,
            &polygon.texture_uri,
            &polygon.downsample_factor.value(),
        );
        let info = packer.add_texture(polygon.id.clone(), texture);
        println!("{:?}", info);
    });

    packer.finalize();

    let output_dir = Path::new("nusamai-atlas/examples/output/");
    packer.export(output_dir, &texture_cache);
}
