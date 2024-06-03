use pmtiles2::{Compression, PMTiles, TileType};
use std::fs::File;
use std::io::Result;
// use tempfile::tempdir;

fn main() -> Result<()> {
    // let dir = tempdir().unwrap();
    // let file_path = dir.path().join("output.pmtiles");
    let file_path = "output.pmtiles";

    // 新しいPMTilesアーカイブを作成
    let mut pm_tiles = PMTiles::new(TileType::Png, Compression::None);

    // タイルデータを追加
    let _ = pm_tiles.add_tile(0, vec![0, 1, 2]);
    let _ = pm_tiles.add_tile(1, vec![3, 4, 5]);

    // ファイルに書き込む
    let mut file = File::create(file_path)?;
    pm_tiles.to_writer(&mut file)?;

    Ok(())
}
