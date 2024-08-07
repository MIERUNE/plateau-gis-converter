use std::io::Write;

use byteorder::{ByteOrder, LittleEndian};
use indexmap::IndexSet;
use nusamai_gltf_json::extensions::mesh::ext_mesh_features;

use super::{material, Primitives};
use crate::{
    pipeline::{feedback, PipelineError},
    sink::cesiumtiles::metadata,
};

pub fn write_gltf_glb<W: Write>(
    feedback: &feedback::Feedback,
    writer: W,
    vertices: impl IntoIterator<Item = [u32; 9]>,
    primitives: Primitives,
    metadata_encoder: metadata::MetadataEncoder,
) -> Result<(), PipelineError> {
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

        const VERTEX_BYTE_STRIDE: usize = 4 * 9; // 4-bytes (f32) x 9

        let buffer_offset = bin_content.len();
        let mut buf = [0; VERTEX_BYTE_STRIDE];
        for v in vertices {
            let [x, y, z, nx, ny, nz, u, v, feature_id] = v;
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

            LittleEndian::write_u32_into(&[x, y, z, nx, ny, nz, u, v, feature_id], &mut buf);
            bin_content.write_all(&buf)?;
            vertices_count += 1;
        }

        let len_vertices = bin_content.len() - buffer_offset;
        if len_vertices > 0 {
            gltf_buffer_views.push(BufferView {
                name: Some("vertices".to_string()),
                byte_offset: buffer_offset as u32,
                byte_length: len_vertices as u32,
                byte_stride: Some(VERTEX_BYTE_STRIDE as u8),
                target: Some(BufferViewTarget::ArrayBuffer),
                ..Default::default()
            });

            // accessor (positions)
            gltf_accessors.push(Accessor {
                name: Some("positions".to_string()),
                buffer_view: Some(gltf_buffer_views.len() as u32 - 1),
                component_type: ComponentType::Float,
                count: vertices_count,
                min: Some(position_min.to_vec()),
                max: Some(position_max.to_vec()),
                type_: AccessorType::Vec3,
                ..Default::default()
            });

            // accessor (normal)
            gltf_accessors.push(Accessor {
                name: Some("normals".to_string()),
                buffer_view: Some(gltf_buffer_views.len() as u32 - 1),
                byte_offset: 4 * 3,
                component_type: ComponentType::Float,
                count: vertices_count,
                type_: AccessorType::Vec3,
                ..Default::default()
            });

            // accessor (texcoords)
            gltf_accessors.push(Accessor {
                name: Some("texcoords".to_string()),
                buffer_view: Some(gltf_buffer_views.len() as u32 - 1),
                byte_offset: 4 * 6,
                component_type: ComponentType::Float,
                count: vertices_count,
                type_: AccessorType::Vec2,
                ..Default::default()
            });

            // accessor (feature_id)
            gltf_accessors.push(Accessor {
                name: Some("_feature_ids".to_string()),
                buffer_view: Some(gltf_buffer_views.len() as u32 - 1),
                byte_offset: 4 * 8,
                component_type: ComponentType::Float,
                count: vertices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            });
        }
    }

    let mut gltf_primitives = vec![];

    let structural_metadata =
        metadata_encoder.into_metadata(&mut bin_content, &mut gltf_buffer_views);

    // indices
    {
        let indices_offset = bin_content.len();

        let mut byte_offset = 0;
        for (mat_idx, (mat, primitive)) in primitives.iter().enumerate() {
            let mut indices_count = 0;
            for idx in &primitive.indices {
                bin_content.write_all(&idx.to_le_bytes())?;
                indices_count += 1;
            }

            gltf_accessors.push(Accessor {
                name: Some("indices".to_string()),
                buffer_view: Some(gltf_buffer_views.len() as u32),
                byte_offset,
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            });

            let mut attributes = vec![("POSITION".to_string(), 0), ("NORMAL".to_string(), 1)];
            // TODO: For no-texture data, it's better to exclude u, v from the vertex buffer
            if mat.base_texture.is_some() {
                attributes.push(("TEXCOORD_0".to_string(), 2));
            }
            attributes.push(("_FEATURE_ID_0".to_string(), 3));

            gltf_primitives.push(MeshPrimitive {
                attributes: attributes.into_iter().collect(),
                indices: Some(gltf_accessors.len() as u32 - 1),
                material: Some(mat_idx as u32), // TODO
                mode: PrimitiveMode::Triangles,
                extensions: extensions::mesh::MeshPrimitive {
                    ext_mesh_features: ext_mesh_features::ExtMeshFeatures {
                        feature_ids: vec![ext_mesh_features::FeatureId {
                            feature_count: primitive.feature_ids.len() as u32,
                            attribute: Some(0),
                            property_table: Some(0),
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
        if indices_len > 0 {
            gltf_buffer_views.push(BufferView {
                name: Some("indices".to_string()),
                byte_offset: indices_offset as u32,
                byte_length: indices_len as u32,
                target: Some(BufferViewTarget::ElementArrayBuffer),
                ..Default::default()
            })
        }
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
        .map(|img| {
            feedback.ensure_not_canceled()?;
            Ok(img.to_gltf(feedback, &mut gltf_buffer_views, &mut bin_content)?)
        })
        .collect::<Result<Vec<Image>, PipelineError>>()?;

    let mut gltf_meshes = vec![];
    if !gltf_primitives.is_empty() {
        gltf_meshes.push(Mesh {
            primitives: gltf_primitives,
            ..Default::default()
        });
    }

    let gltf_buffers = {
        let mut buffers = vec![];
        if !bin_content.is_empty() {
            buffers.push(Buffer {
                byte_length: bin_content.len() as u32,
                ..Default::default()
            });
        }
        buffers
    };

    feedback.ensure_not_canceled()?;

    // Build the JSON part of glTF
    let gltf = Gltf {
        scenes: vec![Scene {
            nodes: Some(vec![0]),
            ..Default::default()
        }],
        nodes: vec![Node {
            mesh: (!primitives.is_empty()).then_some(0),
            ..Default::default()
        }],
        meshes: gltf_meshes,
        materials: gltf_materials,
        textures: gltf_textures,
        images: gltf_images,
        accessors: gltf_accessors,
        buffer_views: gltf_buffer_views,
        buffers: gltf_buffers,
        extensions: nusamai_gltf_json::extensions::gltf::Gltf {
            ext_structural_metadata: structural_metadata,
            ..Default::default()
        }
        .into(),
        extensions_used: vec![
            "EXT_mesh_features".to_string(),
            "EXT_structural_metadata".to_string(),
            "EXT_texture_webp".to_string(),
        ],
        ..Default::default()
    };

    // Write glb to the writer
    nusamai_gltf::glb::Glb {
        json: serde_json::to_vec(&gltf).unwrap().into(),
        bin: Some(bin_content.into()),
    }
    .to_writer_with_alignment(writer, 8)?;

    Ok(())
}
