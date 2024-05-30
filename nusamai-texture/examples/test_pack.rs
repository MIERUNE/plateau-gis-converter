use std::path::{Path, PathBuf};

use nusamai_texture::{
    export::WebpAtlasExporter,
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::CroppedTexture,
};

#[derive(Debug, Clone)]
struct Feature {
    id: String,
    uv_coords: Vec<(f32, f32)>,
    image_path: PathBuf,
}

fn main() {
    let downsample_factor = 1.0;
    let config = TexturePlacerConfig {
        max_width: 1024,
        max_height: 1024,
        padding: 0,
        scale_factor: downsample_factor,
    };

    let placer = GuillotineTexturePlacer::new(config);
    let exporter = WebpAtlasExporter;

    // todo: マルチスレッドで実行されるので、スレッドセーフな構造体にする
    let mut packer = TexturePacker::new(placer, exporter);

    let mut features: Vec<Feature> = Vec::new();
    for i in 0..10 {
        for j in 1..11 {
            let uv_coords = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
            // todo: 画像のデコードが重たいので、メモリに読み込んだ画像はキャッシュから利用するようにする
            let path_string = format!("nusamai-texture/examples/assets/{}.png", j);
            let image_path = PathBuf::from(path_string.as_str());
            features.push(Feature {
                id: format!("texture_{}_{}", i, j),
                uv_coords,
                image_path,
            });
        }
    }

    features.iter().for_each(|feature| {
        // todo: スケールされたUV座標が返却されるようにする
        let texture =
            CroppedTexture::new(&feature.uv_coords, &feature.image_path, downsample_factor);
        // todo: ポリゴンとテクスチャの対照表のようなものを動的に生成し、実際のポリゴンの大きに応じて貼り付け時にスケーリングするようにした方が良い
        packer.add_texture(feature.id.clone(), texture);
    });

    packer.finalize();

    // todo: 画像を複数枚吐き出すようにする
    // todo: 画像の余白が大きければ、出力サイズを小さくするような処理を入れる
    let output_path = Path::new("nusamai-texture/examples/output/");
    packer.export(output_path);
}
