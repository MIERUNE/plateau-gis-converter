use std::io::Write;

use super::material::Material;
use ahash::HashMap;
use byteorder::{ByteOrder, LittleEndian};

pub type Primitives = HashMap<Material, Vec<u32>>;

/// とりいそぎの実装
pub fn write_gltf_glb<W: Write>(
    writer: W,
    translation: [f64; 3],
    vertices: impl IntoIterator<Item = [u32; 5]>,
    primitives: Primitives,
) -> std::io::Result<()> {
    use nusamai_gltf_json::*;

    let mut bin_content: Vec<u8> = Vec::new();

    // calculate the min/max
    let mut position_max = [f64::MIN; 3];
    let mut position_min = [f64::MAX; 3];

    // vertices
    let vertices_offset = bin_content.len();
    let mut buf = [0; 4 * 5];
    let mut vertices_count = 0;
    for v in vertices {
        let [x, y, z, u, v] = v;
        position_min = [
            f64::min(position_min[0], f32::from_bits(x) as f64),
            f64::min(position_min[1], f32::from_bits(y) as f64),
            f64::min(position_min[2], f32::from_bits(z) as f64),
        ];
        position_max = [
            f64::max(position_max[0], f32::from_bits(x) as f64),
            f64::max(position_max[1], f32::from_bits(y) as f64),
            f64::max(position_max[2], f32::from_bits(z) as f64),
        ];

        LittleEndian::write_u32_into(&[x, y, z, u, v], &mut buf);
        bin_content.write_all(&buf)?;
        vertices_count += 1;
    }
    let vertices_len = bin_content.len() - vertices_offset;

    // accessors
    let mut gltf_accessors = vec![
        Accessor {
            buffer_view: Some(0),
            component_type: ComponentType::Float,
            count: vertices_count,
            min: Some(position_min.to_vec()),
            max: Some(position_max.to_vec()),
            type_: AccessorType::Vec3,
            ..Default::default()
        },
        Accessor {
            buffer_view: Some(0),
            byte_offset: 4 * 3,
            component_type: ComponentType::Float,
            count: vertices_count,
            type_: AccessorType::Vec2,
            ..Default::default()
        },
    ];

    let mut gltf_primitives = vec![];

    // indices
    let indices_offset = bin_content.len();
    {
        let mut byte_offset = 0;

        for (mat_i, primitive) in primitives.values().enumerate() {
            let mut indices_count = 0;
            for idx in primitive {
                bin_content.write_all(&idx.to_le_bytes())?;
                indices_count += 1;
            }

            gltf_accessors.push(Accessor {
                buffer_view: Some(1),
                byte_offset,
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            });

            gltf_primitives.push(MeshPrimitive {
                attributes: vec![("POSITION".to_string(), 0), ("TEXCOORD_0".to_string(), 1)]
                    .into_iter()
                    .collect(),
                indices: Some(gltf_accessors.len() as u32 - 1),
                material: Some(mat_i as u32), // TODO
                mode: PrimitiveMode::Triangles,
                ..Default::default()
            });

            byte_offset += indices_count * 4;
        }
    }
    let indices_len = bin_content.len() - indices_offset;

    // materials
    let gltf_materials = primitives
        .keys()
        .map(|material| material.to_gltf())
        .collect();

    // Build the JSON part of glTF
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
        materials: gltf_materials,
        meshes: vec![Mesh {
            primitives: gltf_primitives,
            ..Default::default()
        }],
        accessors: gltf_accessors,
        buffer_views: vec![
            BufferView {
                byte_offset: vertices_offset as u32,
                byte_length: vertices_len as u32,
                byte_stride: Some(4 * 5),
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
