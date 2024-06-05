use pmtiles2::{Compression, PMTiles, TileType};
use std::fs::{self, File};
use std::io::Result;

fn main() -> Result<()> {
    let file_path = "nusamai-mvt/examples/output.pmtiles";

    // 新しいPMTilesアーカイブを作成
    let mut pm_tiles = PMTiles::new(TileType::Png, Compression::GZip);

    // pngを読み込む
    let png_data = fs::read("nusamai-mvt/examples/204.png").unwrap();

    // タイルデータを追加
    let _ = pm_tiles.add_tile(0, png_data);

    // メタデータを書き込む

    // ファイルに書き込む
    let mut file = File::create(file_path)?;
    pm_tiles.to_writer(&mut file)?;

    Ok(())
}
