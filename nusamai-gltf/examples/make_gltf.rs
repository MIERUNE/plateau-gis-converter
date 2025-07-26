/// glTFを生成するサンプル
///
/// % cargo run --example make_gltf --release
use std::fs::File;
use std::{
    io::{self, BufWriter, Write as _},
    vec,
};

use nusamai_gltf_json::*;

fn main() -> io::Result<()> {
    let mut gltf = Gltf {
        ..Default::default()
    };

    let byte_length = 44;
    let mut buffer = Buffer {
        byte_length,
        ..Default::default()
    };
    buffer.uri = "data:application/octet-stream;base64,AAABAAIAAAAAAAAAAAAAAAAAAAAAAIA/\
                  AAAAAAAAAAAAAAAAAACAPwAAAAA="
        .to_string()
        .into();

    let buffer_view1 = BufferView {
        name: None,
        buffer: 0,
        byte_offset: 0,
        byte_length: 6,
        byte_stride: None,
        target: Some(BufferViewTarget::ElementArrayBuffer),
    };

    let buffer_view2 = BufferView {
        name: None,
        buffer: 0,
        byte_offset: 8,
        byte_length: 36,
        byte_stride: None,
        target: Some(BufferViewTarget::ArrayBuffer),
    };

    let accessor1 = Accessor {
        buffer_view: Some(0),
        component_type: ComponentType::UnsignedShort,
        count: 3,
        type_: AccessorType::Scalar,
        max: vec![2.0].into(),
        min: vec![0.0].into(),
        ..Default::default()
    };

    let accessor2 = Accessor {
        buffer_view: Some(1),
        component_type: ComponentType::Float,
        count: 3,
        type_: AccessorType::Vec3,
        max: vec![1.0, 1.0, 0.0].into(),
        min: vec![0.0, 0.0, 0.0].into(),
        ..Default::default()
    };

    let mut primitive = MeshPrimitive {
        indices: Some(0),
        mode: PrimitiveMode::Triangles,
        ..Default::default()
    };
    primitive.attributes.insert("POSITION".to_string(), 1);

    let mesh = Mesh {
        primitives: vec![primitive],
        ..Default::default()
    };

    let node = Node {
        mesh: Some(0),
        ..Default::default()
    };

    let scene = Scene {
        name: None,
        nodes: vec![0].into(),
        ..Default::default()
    };

    gltf.buffers = vec![buffer];
    gltf.buffer_views = vec![buffer_view1, buffer_view2];
    gltf.accessors = vec![accessor1, accessor2];
    gltf.meshes = vec![mesh];
    gltf.nodes = vec![node];
    gltf.scenes = vec![scene];
    gltf.scene = 0.into();

    println!("gltf: {gltf:?}");

    let gltf_json = serde_json::to_value(&gltf)?;

    // フォルダを作成
    std::fs::create_dir_all("data")?;
    // gltfファイルを出力
    let mut gltf_file = BufWriter::new(File::create("data/output.gltf")?);
    gltf_file.write_all(gltf_json.to_string().as_bytes())?;

    Ok(())
}
