use std::fs::File;
use std::io::Write;

pub fn write(file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    if let Ok(metadata) = std::fs::metadata(file_path) {
        if metadata.is_file() {
            std::fs::remove_file(file_path)?;
        }
    }

    let file_code: [u8; 4] = [0x00, 0x00, 0x27, 0x0A]; // ファイルコード
    let unused_space: [u8; 20] = [0x00; 20]; // 未使用領域
    let file_length: [u8; 4] = [0x00, 0x00, 0x00, 0x50]; // ファイル長 (例: 200バイト)
    let version: [u8; 4] = [0xE8, 0x03, 0x00, 0x00]; // バージョン
    let shape_type: [u8; 4] = [0x01, 0x00, 0x00, 0x00]; // シェープタイプ (例: Point)
    let bounding_box: [u8; 32] = [0x00; 32]; // バウンディングボックス

    file.write_all(&file_code)?;
    file.write_all(&unused_space)?;
    file.write_all(&file_length)?;
    file.write_all(&version)?;
    file.write_all(&shape_type)?;
    file.write_all(&bounding_box)?;

    File::create(file_path)?;

    file.flush()?;

    Ok(())
}
