use std::path::{Path, PathBuf};

use nusamai_texture::{
    export::WebpAtlasExporter,
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::{CroppedTexture, DownsampleFactor},
};

#[derive(Debug, Clone)]
struct Polygon {
    id: String,
    uv_coords: Vec<(f32, f32)>,
    image_path: PathBuf,
    downsample_factor: DownsampleFactor,
}

fn main() {
    let config = TexturePlacerConfig {
        max_width: 1024,
        max_height: 1024,
        padding: 0,
    };

    let placer = GuillotineTexturePlacer::new(config);
    let exporter = WebpAtlasExporter;

    // todo: マルチスレッドで実行されるので、スレッドセーフな構造体にする
    let mut packer = TexturePacker::new(placer, exporter);

    // 3D Tiles Sink passes the texture path and UV coordinates for each polygon
    let mut polygons: Vec<Polygon> = Vec::new();
    let downsample_factor = 1.0;
    for i in 0..10 {
        for j in 1..11 {
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
            let path_string: String = format!("nusamai-texture/examples/assets/{}.png", j);
            let image_path = PathBuf::from(path_string.as_str());
            polygons.push(Polygon {
                id: format!("texture_{}_{}", i, j),
                uv_coords,
                image_path,
                downsample_factor: DownsampleFactor::new(&downsample_factor),
            });
        }
    }

    polygons.iter().for_each(|polygon| {
        // todo: スケールされたUV座標が返却されるようにする
        let texture = CroppedTexture::new(
            &polygon.uv_coords,
            &polygon.image_path,
            &polygon.downsample_factor.value(),
        );
        packer.add_texture(polygon.id.clone(), texture);
    });

    packer.finalize();

    // todo: 画像を複数枚吐き出すようにする
    // todo: 画像の余白が大きければ、出力サイズを小さくするような処理を入れる
    let output_path = Path::new("nusamai-texture/examples/output/");
    packer.export(output_path);
}
