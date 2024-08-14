use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use atlas_packer::{
    export::PngAtlasExporter,
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
    let faces = [
        (
            "yellow_dice",
            vec![
                (0.316406, 0.816406),
                (-0.000000, 0.628906),
                (0.316406, 0.628906),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.500000, 1.000000),
                (0.816406, 0.816406),
                (0.816406, 1.000000),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.500000, 0.816406),
                (0.816406, 0.628906),
                (0.816406, 0.816406),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.500000, 0.628906),
                (0.816406, 0.445312),
                (0.816406, 0.628906),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.816406, 0.816406),
                (1.000000, 0.628906),
                (0.816406, 0.628906),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.500000, 0.816406),
                (0.316406, 0.628906),
                (0.500000, 0.628906),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.500000, 0.329309),
                (0.327473, 0.656008),
                (0.327473, 0.329309),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.500000, 0.656008),
                (0.327473, 0.984543),
                (0.327473, 0.656008),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.826699, 0.656008),
                (0.999226, 0.329309),
                (0.999226, 0.656008),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.500000, 0.000774),
                (0.327473, 0.329309),
                (0.327473, 0.000774),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.327473, 0.656008),
                (0.327473, 0.329309),
                (0.000774, 0.656008),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.500000, 0.656008),
                (0.826699, 0.329309),
                (0.826699, 0.656008),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.371094, 0.871094),
                (-0.000000, 0.742188),
                (0.371094, 0.742188),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.500000, 1.000000),
                (0.871094, 0.871094),
                (0.871094, 1.000000),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.500000, 0.871094),
                (0.871094, 0.742188),
                (0.871094, 0.871094),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.500000, 0.742188),
                (0.871094, 0.613281),
                (0.871094, 0.742188),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.871094, 0.871094),
                (1.000000, 0.742188),
                (0.871094, 0.742188),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.500000, 0.871094),
                (0.371094, 0.742188),
                (0.500000, 0.742188),
            ],
        ),
    ];

    let material_to_texture = HashMap::from([
        ("blue_dice", "blue_dice.png"),
        ("red_dice", "red_dice.png"),
        ("yellow_dice", "yellow_dice.png"),
    ]);

    // 3D Tiles Sink passes the texture path and UV coordinates for each polygon
    let mut polygons: Vec<Polygon> = Vec::new();
    let downsample_factor = 1.0;

    for (idx, (material, uv_coords)) in faces.iter().enumerate() {
        let texture_file = material_to_texture.get(material).unwrap();
        let path_string = format!("examples/assets/dice/{}", texture_file);
        let image_path = PathBuf::from(path_string);
        polygons.push(Polygon {
            id: format!("texture_{}_{}", material, idx),
            uv_coords: uv_coords.iter().map(|&(u, v)| (u, v)).collect(),
            texture_uri: image_path,
            downsample_factor: DownsampleFactor::new(&downsample_factor),
        });
    }

    // initialize texture packer
    let config = TexturePlacerConfig::new(500, 500, 1);
    let placer = GuillotineTexturePlacer::new(config.clone());
    let exporter = PngAtlasExporter::default();
    let mut packer = TexturePacker::new(placer, exporter);

    // Texture cache
    let texture_cache = TextureCache::new(100_000_000);

    // Add textures to the atlas,
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

    let output_dir = Path::new("examples/output/");
    packer.export(output_dir, &texture_cache, config.width(), config.height());
}
