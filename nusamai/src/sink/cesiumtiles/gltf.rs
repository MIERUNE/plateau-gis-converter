use std::io::Write;

use byteorder::{ByteOrder, LittleEndian};

/// とりいそぎの実装
pub fn write_gltf_glb<W: Write>(
    writer: W,
    min: [f64; 3],
    max: [f64; 3],
    translation: [f64; 3],
    vertices: impl IntoIterator<Item = [u32; 3]>,
    indices: impl IntoIterator<Item = u32>,
) -> std::io::Result<()> {
    use nusamai_gltf_json::*;

    let mut bin_content: Vec<u8> = Vec::new();

    let vertices_offset = bin_content.len();
    let mut buf = [0; 12];
    let mut vertices_count = 0;
    for v in vertices {
        LittleEndian::write_u32_into(&v, &mut buf);
        bin_content.write_all(&buf)?;
        vertices_count += 1;
    }
    let vertices_len = bin_content.len() - vertices_offset;

    let indices_offset = bin_content.len();
    let mut indices_count = 0;
    for idx in indices {
        bin_content.write_all(&idx.to_le_bytes())?;
        indices_count += 1;
    }
    let indices_len = bin_content.len() - indices_offset;

    let gltf = Gltf {
        scenes: vec![Scene {
            nodes: Some(vec![0]),
            ..Default::default()
        }],
        nodes: vec![Node {
            mesh: Some(0),
            translation,
            ..Default::default()
        }],
        materials: vec![Material {
            pbr_metallic_roughness: Some(MaterialPbrMetallicRoughness {
                base_color_factor: [0.5, 0.7, 0.7, 1.0],
                metallic_factor: 0.5,
                roughness_factor: 0.5,
                ..Default::default()
            }),
            ..Default::default()
        }],
        meshes: vec![Mesh {
            primitives: vec![MeshPrimitive {
                attributes: vec![("POSITION".to_string(), 0)].into_iter().collect(),
                indices: Some(1),
                material: Some(0),
                mode: PrimitiveMode::Triangles,
                ..Default::default()
            }],
            ..Default::default()
        }],
        accessors: vec![
            Accessor {
                buffer_view: Some(0),
                component_type: ComponentType::Float,
                count: vertices_count,
                min: Some(min.to_vec()),
                max: Some(max.to_vec()),
                type_: AccessorType::Vec3,
                ..Default::default()
            },
            Accessor {
                buffer_view: Some(1),
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            },
        ],
        buffer_views: vec![
            BufferView {
                byte_offset: vertices_offset as u32,
                byte_length: vertices_len as u32,
                target: Some(BufferViewTarget::ArrayBuffer),
                ..Default::default()
            },
            BufferView {
                byte_offset: indices_offset as u32,
                byte_length: indices_len as u32,
                target: Some(BufferViewTarget::ElementArrayBuffer),
                ..Default::default()
            },
        ],
        buffers: vec![Buffer {
            byte_length: bin_content.len() as u32,
            ..Default::default()
        }],
        ..Default::default()
    };

    // Write glb to the writer
    nusamai_gltf::glb::Glb {
        json: serde_json::to_vec(&gltf)?.into(),
        bin: Some(bin_content.into()),
    }
    .to_writer_with_alignment(writer, 8)?;

    Ok(())
}
