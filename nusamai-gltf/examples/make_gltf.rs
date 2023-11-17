use std::fs::File;
use std::io::{self, BufWriter, Write as _};

use gltf_rs::*;

fn main() -> io::Result<()> {
    let mut gltf = Gltf::new();

    let byte_length = 44;
    let mut buffer = Buffer::new(byte_length);
    buffer.uri = Some("data:application/octet-stream;base64,AAABAAIAAAAAAAAAAAAAAAAAAAAAAIA/AAAAAAAAAAAAAAAAAACAPwAAAAA=".to_string());

    let mut buffer_view1 = BufferView::new();
    buffer_view1.buffer = 0;
    buffer_view1.byte_offset = Some(0);
    buffer_view1.byte_length = 6;
    buffer_view1.target = Some(BufferTarget::ElementArrayBuffer);

    let mut buffer_view2 = BufferView::new();
    buffer_view2.buffer = 0;
    buffer_view2.byte_offset = Some(8);
    buffer_view2.byte_length = 36;
    buffer_view2.target = Some(BufferTarget::ArrayBuffer);

    let mut accessor1 = Accessor::new();
    accessor1.buffer_view = Some(0);
    accessor1.byte_offset = Some(0);
    accessor1.component_type = ComponentType::UnsignedShort;
    accessor1.count = 3;
    accessor1.accessor_type = AccessorType::Scalar;
    accessor1.max = Some(vec![2.0]);
    accessor1.min = Some(vec![0.0]);

    let mut accessor2 = Accessor::new();
    accessor2.buffer_view = Some(1);
    accessor2.byte_offset = Some(0);
    accessor2.component_type = ComponentType::Float;
    accessor2.count = 3;
    accessor2.accessor_type = AccessorType::Vec3;
    accessor2.max = Some(vec![1.0, 1.0, 0.0]);
    accessor2.min = Some(vec![0.0, 0.0, 0.0]);

    let mut primitive = Primitive::new();
    primitive.attributes.insert("POSITION".to_string(), 1);
    primitive.indices = Some(0);

    let mut mesh = Mesh::new();
    mesh.primitives = vec![primitive];

    let mut node = Node::new();
    node.mesh = Some(0);

    let mut scene = Scene::new();
    scene.nodes.push(0);

    gltf.buffers = Some(vec![buffer]);
    gltf.buffer_views = Some(vec![buffer_view1, buffer_view2]);
    gltf.accessors = Some(vec![accessor1, accessor2]);
    gltf.meshes = Some(vec![mesh]);
    gltf.nodes = Some(vec![node]);
    gltf.scenes = Some(vec![scene]);
    gltf.scene = Some(0);

    println!("gltf: {:?}", gltf);

    let gltf_json = serde_json::to_value(&gltf)?;

    // gltfファイルを出力
    let mut gltf_file = BufWriter::new(File::create("data/output.gltf")?);
    gltf_file.write_all(gltf_json.to_string().as_bytes())?;

    Ok(())
}
