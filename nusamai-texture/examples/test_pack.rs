use std::path::Path;

use nusamai_texture::{
    export::WebpAtlasExporter,
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::CroppedTexture,
};

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

    let mut packer = TexturePacker::new(placer, exporter);
    for i in 0..10 {
        for j in 1..11 {
            // todo:
            // マルチスレッドで実行されるので、スレッドセーフな構造体にする

            let uv_coords = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
            // todo:
            // 画像のデコードが重たいので、メモリに読み込んだ画像はキャッシュから利用するようにする
            let path_string = format!("examples/assets/{}.png", j);
            let image_path = Path::new(path_string.as_str());

            // todo:
            // スケールされたUV座標が返却されるようにする
            // 与えられたuv_coordsを修正して格納できるようにする（org_uvsとdist_uvsとか？）
            // exportするタイミングで、実際にスケーリングし、アトラスに書き込むようにする
            let texture = CroppedTexture::new(&uv_coords, image_path, downsample_factor);

            // todo:
            // 実際のポリゴンの大きさに応じて貼り付け時にスケーリングするようにした方が良い
            // ポリゴンとテクスチャの対照表的なのが動的に生成し、scale_factorが変わると
            packer.add_texture(format!("texture_{}_{}", i, j).to_string(), texture);
        }
    }

    packer.finalize();

    // todo:
    // 画像を複数枚吐き出すようにする
    // 画像の余白が大きければ、出力サイズを小さくするような処理を入れる
    let output_path = Path::new("examples/output/");
    packer.export(output_path);
}
