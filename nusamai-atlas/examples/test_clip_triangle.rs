use image::{ImageBuffer, Rgba};

// UV座標を画像座標に変換する関数
fn uv_to_pixel(uv: (f32, f32), image_size: (u32, u32)) -> (u32, u32) {
    let x = (uv.0 * image_size.0 as f32) as u32;
    let y = ((1.0 - uv.1) * image_size.1 as f32) as u32; // UV座標系は左下が(0,0)なので、y座標を反転
    (x, y)
}

// 点が多角形の内部にあるかを判定する関数（レイキャスティング法）
fn point_in_polygon(point: (f32, f32), polygon: &[(f32, f32)]) -> bool {
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if ((yi > point.1) != (yj > point.1))
            && (point.0 < (xj - xi) * (point.1 - yi) / (yj - yi) + xi)
        {
            inside = !inside;
        }

        j = i;
    }
    inside
}

// 多角形で画像を切り抜く関数
fn clip_polygon_and_trim(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    polygon: &[(f32, f32)],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = image.dimensions();
    let mut clipped = ImageBuffer::new(width, height);
    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y, pixel) in image.enumerate_pixels() {
        let uv = (x as f32 / width as f32, 1.0 - y as f32 / height as f32);
        if point_in_polygon(uv, polygon) {
            clipped.put_pixel(x, y, *pixel);
            if pixel[3] > 0 {
                // 不透明なピクセルの範囲を記録
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    // 不透明な部分のみを切り出す
    let trimmed = ImageBuffer::from_fn(max_x - min_x + 1, max_y - min_y + 1, |x, y| {
        *clipped.get_pixel(x + min_x, y + min_y)
    });

    trimmed
}

// アトラスに画像を追加する関数
fn add_to_atlas(
    atlas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    clipped: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    position: (u32, u32),
) {
    for (x, y, pixel) in clipped.enumerate_pixels() {
        if pixel[3] > 0 {
            // アルファ値がゼロでない場合のみコピー
            let (atlas_x, atlas_y) = (position.0 + x, position.1 + y);
            if atlas_x < atlas.width() && atlas_y < atlas.height() {
                atlas.put_pixel(atlas_x, atlas_y, *pixel);
            }
        }
    }
}

fn main() -> Result<(), image::ImageError> {
    // 画像を読み込む
    let img = image::open("nusamai-atlas/examples/assets/dice/yellow_dice.png")?.to_rgba8();

    // 多角形のUV座標を定義
    let polygon = vec![
        (0.316406, 0.816406),
        (-0.000000, 0.628906),
        (0.316406, 0.628906),
    ];

    // 多角形で画像を切り抜く
    let clipped = clip_polygon_and_trim(&img, &polygon);

    // アトラス画像を作成
    let mut atlas = ImageBuffer::new(1024, 1024);

    // 切り抜いた画像をアトラスに追加
    add_to_atlas(&mut atlas, &clipped, (0, 0));

    // アトラス画像を保存
    atlas.save("nusamai-atlas/examples/output/atlas.png")?;

    Ok(())
}
