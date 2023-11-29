use std::collections::HashMap;
/// glTFを生成するサンプル
///
/// % cargo run --example make_gltf --release
use std::fs::File;
use std::io::{self, BufWriter, Write as _};
use std::vec;

use nusamai_gltf::*;

fn main() -> io::Result<()> {
    let asset = Asset {
        version: "2.0".to_string(),
        copyright: None,
        generator: None,
        min_version: None,
        extensions: None,
        extras: None,
    };

    let mut gltf = Gltf {
        extensions_used: None,
        extensions_required: None,
        accessors: None,
        animations: None,
        asset,
        buffers: None,
        buffer_views: None,
        cameras: None,
        images: None,
        materials: None,
        meshes: None,
        nodes: None,
        samplers: None,
        scene: None,
        scenes: None,
        skins: None,
        textures: None,
        extensions: None,
        extras: None,
    };

    let byte_length = 44;
    let mut buffer = Buffer {
        name: None,
        byte_length,
        uri: None,
        extensions: None,
        extras: None,
    };
    buffer.uri = Some("data:application/octet-stream;base64,AAABAAIAAAAAAAAAAAAAAAAAAAAAAIA/AAAAAAAAAAAAAAAAAACAPwAAAAA=".to_string());

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
        name: None,
        buffer_view: Some(0),
        byte_offset: 0,
        component_type: ComponentType::UnsignedShort,
        count: 3,
        type_: AccessorType::Scalar,
        max: Some(vec![2.0]),
        min: Some(vec![0.0]),
        sparse: None,
        normalized: None,
        extensions: None,
        extras: None,
    };

    let accessor2 = Accessor {
        name: None,
        buffer_view: Some(1),
        byte_offset: 0,
        component_type: ComponentType::Float,
        count: 3,
        type_: AccessorType::Vec3,
        max: Some(vec![1.0, 1.0, 0.0]),
        min: Some(vec![0.0, 0.0, 0.0]),
        sparse: None,
        normalized: None,
        extensions: None,
        extras: None,
    };

    let mut primitive = MeshPrimitive {
        attributes: HashMap::new(),
        indices: Some(0),
        material: None,
        mode: PrimitiveMode::Triangles,
        targets: None,
        extensions: None,
        extras: None,
    };
    primitive.attributes.insert("POSITION".to_string(), 1);

    let mesh = Mesh {
        primitives: vec![primitive],
        weights: None,
        name: None,
        extensions: None,
        extras: None,
    };

    let node = Node {
        camera: None,
        children: None,
        skin: None,
        matrix: None,
        mesh: Some(0),
        rotation: None,
        scale: None,
        translation: None,
        weights: None,
        name: None,
        extensions: None,
        extras: None,
    };

    let scene = Scene {
        name: None,
        nodes: Some(vec![0]),
    };

    gltf.buffers = Some(vec![buffer]);
    gltf.buffer_views = Some(vec![buffer_view1, buffer_view2]);
    gltf.accessors = Some(vec![accessor1, accessor2]);
    gltf.meshes = Some(vec![mesh]);
    gltf.nodes = Some(vec![node]);
    gltf.scenes = Some(vec![scene]);
    gltf.scene = Some(0);

    println!("gltf: {:?}", gltf);

    let gltf_json = serde_json::to_value(&gltf)?;

    // フォルダを作成
    std::fs::create_dir_all("data")?;
    // gltfファイルを出力
    let mut gltf_file = BufWriter::new(File::create("data/output.gltf")?);
    gltf_file.write_all(gltf_json.to_string().as_bytes())?;

    Ok(())
}
