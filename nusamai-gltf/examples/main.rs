mod gltf;
mod las;

use byteorder::WriteBytesExt;
use byteorder::{LittleEndian, ReadBytesExt};
use gltf::*;
use las::*;
use serde_json::json;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write as _};

fn main() -> io::Result<()> {
    let file = File::open("data/tochomaeb3f.las")?;
    let mut reader = BufReader::new(file);

    // Public Header Blockの読み込み
    let mut public_header = PublicHeader {
        ..PublicHeader::new()
    };

    public_header.read_public_header(&mut reader)?;

    // offset_to_point_dataを取得
    // LASデータの先頭からポイントデータまでのオフセット
    let offset_to_point_data = public_header.offset_to_point_data;
    println!("offset to point data: {}", offset_to_point_data);

    // ポイントデータの合計数を取得
    let points_count = public_header.legacy_number_of_point_records;
    println!("points count: {}", points_count);

    // ポイントフォーマットによって、point data recordsの長さが変わる
    let point_format = public_header.point_data_record_format;
    println!("point format: {}", point_format);

    // ヘッダーを確認
    println!("header: {:?}", public_header);

    // point dataのセクションに先頭に移動
    reader.seek(SeekFrom::Start(offset_to_point_data as u64))?;

    let mut point_record = vec![0; public_header.point_data_record_length as usize];
    let mut points: Vec<Vec<f64>> = Vec::new();
    for _ in 0..points_count {
        reader.read_exact(&mut point_record)?;

        // ポイントレコードのデータを読み取る
        let mut point_data = BufReader::new(point_record.as_slice());

        // ポイントデータを格納する配列
        let mut point: Vec<f64> = Vec::new();

        // フォーマットによって読み取るデータを変える必要があるが、一旦0のみ対応
        let x = point_data.read_i32::<LittleEndian>()?;
        point.push(x as f64 * public_header.x_scale_factor + public_header.x_offset);

        // Y座標を読み取る
        let y = point_data.read_i32::<LittleEndian>()?;
        point.push(y as f64 * public_header.y_scale_factor + public_header.y_offset);

        // Z座標を読み取る
        let z = point_data.read_i32::<LittleEndian>()?;
        point.push(z as f64 * public_header.z_scale_factor + public_header.z_offset);

        // ポイントデータを格納
        // println!("point: {:?}", point);
        points.push(point);
    }

    // pointsの最初と最後の要素を表示
    println!("first point: {:?}", points[0]);
    println!("last point: {:?}", points[points_count as usize - 1]);

    // gltf用のバイナリファイルにデータを流し込む
    let mut gltf_file = BufWriter::new(File::create("data/output.bin")?);
    for point in points {
        for coordinate in point {
            gltf_file.write_f64::<LittleEndian>(coordinate)?;
        }
    }

    let byte_length = points_count * 3 * 4;

    let mut gltf = Gltf::new();

    let mut buffer = Buffer::new(byte_length as usize);
    buffer.uri = Some("./tochomaeb3f.bin".to_string());
    gltf.buffers = Some(vec![buffer]);

    let mut buffer_view = BufferView::new();
    buffer_view.buffer = 0;
    buffer_view.byte_length = byte_length as usize;
    buffer_view.target = Some(BufferTarget::ArrayBuffer);
    gltf.buffer_views = Some(vec![buffer_view]);

    let mut accessor = Accessor::new();
    accessor.buffer_view = Some(0);
    accessor.component_type = ComponentType::Float;
    accessor.count = points_count as usize;
    accessor.accessor_type = AccessorType::Vec3;
    gltf.accessors = Some(vec![accessor]);

    let gltf_json = serde_json::to_value(&gltf)?;

    // gltfファイルを出力
    let mut gltf_file = BufWriter::new(File::create("data/output.gltf")?);
    gltf_file.write_all(gltf_json.to_string().as_bytes())?;

    Ok(())
}
