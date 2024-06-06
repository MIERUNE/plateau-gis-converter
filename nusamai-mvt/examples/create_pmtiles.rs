use pmtiles2::{util::tile_id, Compression, PMTiles, TileType};
use std::fs::{self, File};
use std::io::Result;

fn main() -> Result<()> {
    let file_path = "nusamai-mvt/examples/output.pmtiles";

    // 新しいPMTilesアーカイブを作成
    let mut pm_tiles = PMTiles::new(TileType::Png, Compression::None);

    // pngを読み込む
    let png_data = fs::read("nusamai-mvt/examples/204.png").unwrap();

    // タイルデータを追加
    let _ = pm_tiles.add_tile(tile_id(9, 445, 204), png_data);

    // タイルを読み込んでみる
    let tile_data = pm_tiles.get_tile(445, 204, 9).unwrap();
    println!("tile_data: {:?}", tile_data);

    // ファイルに書き込む
    let mut file = File::create(file_path)?;
    pm_tiles.to_writer(&mut file)?;

    Ok(())
}
