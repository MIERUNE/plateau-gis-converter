use std::path::{Path, PathBuf};
use std::sync::Mutex;

use atlas_packer::texture::PolygonMappedTexture;
use rand::{Rng, SeedableRng};
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

fn uv_to_pixel_coords(uv_coords: &[(f64, f64)], width: u32, height: u32) -> Vec<(u32, u32)> {
    uv_coords
        .iter()
        .map(|(u, v)| {
            (
                (u.clamp(0.0, 1.0) * width as f64).min(width as f64 - 1.0) as u32,
                ((1.0 - v.clamp(0.0, 1.0)) * height as f64).min(height as f64 - 1.0) as u32,
            )
        })
        .collect()
}

fn calc_bbox(pixel_coords: &[(u32, u32)]) -> (u32, u32, u32, u32) {
    pixel_coords.iter().fold(
        (u32::MAX, u32::MAX, 0, 0),
        |(min_x, min_y, max_x, max_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
        },
    )
}

fn main() {
    // 3D Tiles Sink passes the texture path and UV coordinates for each polygon
    let mut polygons: Vec<Polygon> = Vec::new();
    let downsample_factor = 1.0;

    let mut rng = rand::rngs::StdRng::seed_from_u64(0);

    let mut random_in_range = |min: f64, max: f64| rng.gen_range(min..max);

    for i in 0..100 {
        for j in 1..11 {
            // Specify a polygon to crop around the center of the image

            // generate random polygon
            let edge_radius = 0.3;
            let center_x = random_in_range(edge_radius, 1.0 - edge_radius);
            let center_y = random_in_range(edge_radius, 1.0 - edge_radius);

            let num_points = 5;
            let mut radians = (0..num_points)
                .map(|_| random_in_range(0.0, std::f64::consts::TAU))
                .collect::<Vec<f64>>();

            radians.sort_by(|a, b| a.total_cmp(b));

            let uv_coords = radians
                .iter()
                .map(|radian| {
                    let radius = random_in_range(edge_radius * 0.1, edge_radius);
                    let x = center_x + radius * radian.cos();
                    let y = center_y + radius * radian.sin();
                    (x, y)
                })
                .collect::<Vec<(f64, f64)>>();

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

    // cache image size
    let texture_size_cache = TextureSizeCache::new();
    // place textures on the atlas
    polygons.par_iter().for_each(|polygon| {
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
    });

    let packer = packer.into_inner().unwrap();
    let packed = packer.pack(GuillotineTexturePlacer::new(config.clone()));

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
    let mut count = 0;
    let count_limit = 20;
    polygons.iter().for_each(|polygon| {
        if count >= count_limit {
            return;
        }
        count += 1;
        if let Some(info) = packed.get_texture_info(&polygon.id) {
            let pixel_coords =
                uv_to_pixel_coords(&info.placed_uv_coords, config.width, config.height);

            let tex_bbox = calc_bbox(&pixel_coords);

            // load image
            let atlas_uri = output_dir.join(info.atlas_id.to_string() + ".jpg");
            let mut image = image::open(atlas_uri).unwrap();

            // crop image
            let cropped_image = image.crop(
                tex_bbox.0 as u32,
                tex_bbox.1 as u32,
                (tex_bbox.2 - tex_bbox.0) as u32,
                (tex_bbox.3 - tex_bbox.1) as u32,
            );

            // save image
            let cropped_image_path = output_dir.join("polygon").join(polygon.id.clone() + ".jpg");
            cropped_image.save(cropped_image_path).unwrap();
        }
    });
}
