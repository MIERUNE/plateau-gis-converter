use std::io::Write;

use super::material;
use ahash::{HashMap, HashSet};
use byteorder::{ByteOrder, LittleEndian};
use indexmap::IndexSet;
use nusamai_gltf_json::extensions::mesh::ext_mesh_features;

#[derive(Default)]
pub struct PrimitiveInfo {
    pub indices: Vec<u32>,
    pub feature_ids: HashSet<u32>,
}

pub type Primitives = HashMap<material::Material, PrimitiveInfo>;

/// とりいそぎの実装
pub fn write_gltf_glb<W: Write>(
    writer: W,
    translation: [f64; 3],
    vertices: impl IntoIterator<Item = [u32; 6]>,
    primitives: Primitives,
) -> std::io::Result<()> {
    use nusamai_gltf_json::*;

    // The buffer for the BIN part
    let mut bin_content: Vec<u8> = Vec::new();
    let mut gltf_buffer_views = vec![];
    let mut gltf_accessors = vec![];

    // vertices
    {
        let mut vertices_count = 0;
        let mut position_max = [f64::MIN; 3];
        let mut position_min = [f64::MAX; 3];

        const VERTEX_BYTE_STRIDE: usize = 4 * 6; // 4-bytes (u32) x 6

        let buffer_offset = bin_content.len();
        let mut buf = [0; VERTEX_BYTE_STRIDE];
        for v in vertices {
            let [x, y, z, u, v, feature_id] = v;
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

            LittleEndian::write_u32_into(&[x, y, z, u, v, feature_id], &mut buf);
            bin_content.write_all(&buf)?;
            vertices_count += 1;
        }

        gltf_buffer_views.push(BufferView {
            byte_offset: buffer_offset as u32,
            byte_length: (bin_content.len() - buffer_offset) as u32,
            byte_stride: Some(VERTEX_BYTE_STRIDE as u8),
            target: Some(BufferViewTarget::ArrayBuffer),
            ..Default::default()
        });

        // accessor (positions)
        gltf_accessors.push(Accessor {
            buffer_view: Some(gltf_buffer_views.len() as u32 - 1),
            component_type: ComponentType::Float,
            count: vertices_count,
            min: Some(position_min.to_vec()),
            max: Some(position_max.to_vec()),
            type_: AccessorType::Vec3,
            ..Default::default()
        });

        // accessor (texcoords)
        gltf_accessors.push(Accessor {
            buffer_view: Some(gltf_buffer_views.len() as u32 - 1),
            byte_offset: 4 * 3,
            component_type: ComponentType::Float,
            count: vertices_count,
            type_: AccessorType::Vec2,
            ..Default::default()
        });

        // accessor (feature_id)
        gltf_accessors.push(Accessor {
            buffer_view: Some(gltf_buffer_views.len() as u32 - 1),
            byte_offset: 4 * 5,
            component_type: ComponentType::Float,
            count: vertices_count,
            type_: AccessorType::Scalar,
            ..Default::default()
        });
    }

    let mut gltf_primitives = vec![];

    // indices
    {
        let indices_offset = bin_content.len();

        let mut byte_offset = 0;
        for (mat_i, (mat, primitive)) in primitives.iter().enumerate() {
            let mut indices_count = 0;
            for idx in &primitive.indices {
                bin_content.write_all(&idx.to_le_bytes())?;
                indices_count += 1;
            }

            gltf_accessors.push(Accessor {
                buffer_view: Some(gltf_buffer_views.len() as u32),
                byte_offset,
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            });

            let mut attributes = vec![("POSITION".to_string(), 0)];
            // TODO: It will be better for no-texture data to exclude u, v from the vertex buffer
            if mat.base_texture.is_some() {
                attributes.push(("TEXCOORD_0".to_string(), 1));
            }
            attributes.push(("_FEATURE_ID_0".to_string(), 2));

            gltf_primitives.push(MeshPrimitive {
                attributes: attributes.into_iter().collect(),
                indices: Some(gltf_accessors.len() as u32 - 1),
                material: Some(mat_i as u32), // TODO
                mode: PrimitiveMode::Triangles,
                extensions: extensions::mesh::MeshPrimitive {
                    ext_mesh_features: ext_mesh_features::ExtMeshFeatures {
                        feature_ids: vec![ext_mesh_features::FeatureId {
                            attribute: Some(0),
                            feature_count: primitive.feature_ids.len() as u32,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            });

            byte_offset += indices_count * 4;
        }

        let indices_len = bin_content.len() - indices_offset;

        gltf_buffer_views.push(BufferView {
            byte_offset: indices_offset as u32,
            byte_length: indices_len as u32,
            target: Some(BufferViewTarget::ElementArrayBuffer),
            ..Default::default()
        })
    }

    let mut image_set: IndexSet<material::Image, ahash::RandomState> = Default::default();
    let mut texture_set: IndexSet<material::Texture, ahash::RandomState> = Default::default();

    // materials
    let gltf_materials = primitives
        .keys()
        .map(|material| material.to_gltf(&mut texture_set))
        .collect();

    let gltf_textures: Vec<_> = texture_set
        .into_iter()
        .map(|t| t.to_gltf(&mut image_set))
        .collect();

    let gltf_images = image_set
        .into_iter()
        .map(|img| img.to_gltf(&mut gltf_buffer_views, &mut bin_content))
        .collect::<std::io::Result<Vec<Image>>>()?;

    let gltf_buffers = vec![Buffer {
        byte_length: bin_content.len() as u32,
        ..Default::default()
    }];

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
        meshes: vec![Mesh {
            primitives: gltf_primitives,
            ..Default::default()
        }],
        materials: gltf_materials,
        textures: gltf_textures,
        images: gltf_images,
        accessors: gltf_accessors,
        buffer_views: gltf_buffer_views,
        buffers: gltf_buffers,
        extensions_used: vec!["EXT_mesh_features".to_string()],
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
