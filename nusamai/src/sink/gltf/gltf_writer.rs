use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use indexmap::IndexMap;

use indexmap::IndexSet;
use nusamai_citygml::schema::Schema;
use nusamai_citygml::Measure;
use nusamai_citygml::Value;
use nusamai_gltf_json::{
    extensions, Accessor, AccessorType, Buffer, BufferView, BufferViewTarget, ComponentType, Gltf,
    Image, Mesh, MeshPrimitive, Node, PrimitiveMode, Scene,
};

// use crate::sink::gltf::attributes::{to_gltf_class, to_gltf_property_table};

use super::material;
use super::metadata::make_metadata;
use super::Features;
use super::Primitives;

pub fn write_gltf_glb<W: Write>(
    writer: W,
    translation: [f64; 3],
    vertices: impl IntoIterator<Item = [u32; 6]>,
    primitives: Primitives,
    features: Features,
    schema: &Schema,
    typename: &str,
    num_features: &usize,
) -> std::io::Result<()> {
    // The buffer for the BIN part
    let mut bin_content: Vec<u8> = Vec::new();
    let mut gltf_buffer_views = vec![];
    let mut gltf_accessors = vec![];

    // vertices
    {
        let mut vertices_count = 0;
        let mut position_max = [f64::MIN; 3];
        let mut position_min = [f64::MAX; 3];

        const VERTEX_BYTE_STRIDE: usize = 4 * 6; // 4-bytes (f32) x 6

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

        let len_vertices = bin_content.len() - buffer_offset;
        if len_vertices > 0 {
            // make bufferView for positions, normals, tex_coords, feature_id
            gltf_buffer_views.push(BufferView {
                byte_offset: buffer_offset as u32,
                byte_length: len_vertices as u32,
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

            // // accessor (normal)
            // gltf_accessors.push(Accessor {
            //     buffer_view: Some(gltf_buffer_views.len() as u32 - 1),
            //     byte_offset: 4 * 3,
            //     component_type: ComponentType::Float,
            //     count: vertices_count,
            //     type_: AccessorType::Vec3,
            //     ..Default::default()
            // });

            // accessor (tex_coords)
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
    }

    let mut gltf_primitives = vec![];

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
                buffer_view: Some(gltf_buffer_views.len() as u32),
                byte_offset,
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            });

            let mut attributes = vec![("POSITION".to_string(), 0)];
            // TODO: For no-texture data, it's better to exclude u, v from the vertex buffer
            if mat.base_texture.is_some() {
                attributes.push(("TEXCOORD_0".to_string(), 1));
            }
            attributes.push(("_FEATURE_ID_0".to_string(), 2));

            gltf_primitives.push(MeshPrimitive {
                attributes: attributes.into_iter().collect(),
                indices: Some(gltf_accessors.len() as u32 - 1),
                material: Some(mat_idx as u32), // TODO
                mode: PrimitiveMode::Triangles,
                extensions: extensions::mesh::MeshPrimitive {
                    ext_mesh_features: extensions::mesh::ext_mesh_features::ExtMeshFeatures {
                        feature_ids: vec![extensions::mesh::ext_mesh_features::FeatureId {
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
        .map(|img| img.to_gltf(&mut gltf_buffer_views, &mut bin_content))
        .collect::<std::io::Result<Vec<Image>>>()?;

    let mut gltf_meshes = vec![];
    if !gltf_primitives.is_empty() {
        gltf_meshes.push(Mesh {
            primitives: gltf_primitives,
            ..Default::default()
        });
    }

    // metadata
    let ext_structural_metadata = make_metadata(
        *num_features,
        typename,
        &features,
        &mut bin_content,
        &mut gltf_buffer_views,
        schema,
    );

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

    // Build the JSON part of glTF
    let gltf = Gltf {
        scenes: vec![Scene {
            nodes: Some(vec![0]),
            ..Default::default()
        }],
        nodes: vec![Node {
            mesh: (!primitives.is_empty()).then_some(0),
            translation,
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
            ext_structural_metadata,
            ..Default::default()
        }
        .into(),
        extensions_used: vec![
            "EXT_mesh_features".to_string(),
            "EXT_structural_metadata".to_string(),
        ],
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

// pub struct GltfContent {
//     pub class_name: String,
//     pub gltf: Gltf,
//     pub bin_content: Vec<u8>,
// }

// pub fn build_base_gltf(class_name: &str, buffer: &Buffers, translation: [f64; 3]) -> GltfContent {
//     let mut gltf = Gltf {
//         extensions_used: vec![
//             "EXT_mesh_features".to_string(),
//             "EXT_structural_metadata".to_string(),
//         ],
//         scenes: vec![Scene {
//             nodes: Some(vec![0]),
//             ..Default::default()
//         }],
//         nodes: vec![Node {
//             mesh: Some(0),
//             translation,
//             ..Default::default()
//         }],
//         meshes: vec![Mesh {
//             primitives: vec![MeshPrimitive {
//                 attributes: vec![("POSITION".to_string(), 0)].into_iter().collect(),
//                 indices: Some(1),
//                 mode: PrimitiveMode::Triangles,
//                 ..Default::default()
//             }],
//             ..Default::default()
//         }],
//         ..Default::default()
//     };

//     let mut bin_content: Vec<u8> = Vec::new();
//     let mut mesh_primitives = Vec::new();

//     let vertices = buffer.vertices.clone();
//     let indices = buffer.indices.clone();

//     // write vertices
//     let vertices_offset = bin_content.len();
//     let mut buf = [0; 12];
//     let mut vertices_count = 0;
//     for vertex in vertices.clone() {
//         LittleEndian::write_u32_into(&vertex.position, &mut buf);
//         bin_content.write_all(&buf).unwrap();
//         vertices_count += 1;
//     }
//     let vertices_len: usize = bin_content.len() - vertices_offset;

//     // write indices
//     let indices_offset = bin_content.len();
//     let mut indices_count = 0;
//     for idx in indices.clone() {
//         bin_content.write_all(&idx.to_le_bytes()).unwrap();
//         indices_count += 1;
//     }
//     let indices_len = bin_content.len() - indices_offset;

//     let mut min = [f64::MAX; 3];
//     let mut max = [f64::MIN; 3];
//     for vertex in vertices.iter() {
//         for v in 0..3 {
//             let value = f32::from_bits(vertex.position[v]) as f64;
//             if value < min[v] {
//                 min[v] = value;
//             }
//             if value > max[v] {
//                 max[v] = value;
//             }
//         }
//     }

//     let buffer_views = vec![
//         BufferView {
//             byte_offset: vertices_offset as u32,
//             byte_length: vertices_len as u32,
//             target: Some(BufferViewTarget::ArrayBuffer),
//             ..Default::default()
//         },
//         BufferView {
//             byte_offset: indices_offset as u32,
//             byte_length: indices_len as u32,
//             target: Some(BufferViewTarget::ElementArrayBuffer),
//             ..Default::default()
//         },
//     ];
//     gltf.buffer_views.extend(buffer_views);

//     let accessors = vec![
//         Accessor {
//             buffer_view: Some(gltf.buffer_views.len() as u32 - 2),
//             component_type: ComponentType::Float,
//             count: vertices_count,
//             min: Some(min.to_vec()),
//             max: Some(max.to_vec()),
//             type_: AccessorType::Vec3,
//             ..Default::default()
//         },
//         Accessor {
//             buffer_view: Some(gltf.buffer_views.len() as u32 - 1),
//             component_type: ComponentType::UnsignedInt,
//             count: indices_count,
//             type_: AccessorType::Scalar,
//             ..Default::default()
//         },
//     ];
//     gltf.accessors.extend(accessors);

//     let buffer_view_length = gltf.buffer_views.len() as u32;

//     // write feature_ids
//     let feature_ids_offset = bin_content.len();
//     let mut feature_ids_count = 0;
//     for vertex in buffer.vertices.clone() {
//         // UnsignedInt cannot be used as vertex attributes in MeshPrimitive
//         // https://github.com/CesiumGS/glTF/blob/proposal-EXT_mesh_features/extensions/2.0/Vendor/EXT_mesh_features/README.md#feature-id-by-vertex
//         let vertex = vertex.feature_id as f32;
//         bin_content.write_all(&vertex.to_le_bytes()).unwrap();
//         feature_ids_count += 1;
//     }
//     let feature_ids_len = bin_content.len() - feature_ids_offset;

//     let buffer_view = BufferView {
//         byte_offset: feature_ids_offset as u32,
//         byte_length: feature_ids_len as u32,
//         byte_stride: Some(4),
//         target: Some(BufferViewTarget::ArrayBuffer),
//         ..Default::default()
//     };
//     gltf.buffer_views.push(buffer_view);

//     let feature_id_min = buffer.vertices.iter().map(|v| v.feature_id).min().unwrap();
//     let feature_id_max = buffer.vertices.iter().map(|v| v.feature_id).max().unwrap();
//     let accessor = Accessor {
//         buffer_view: Some(buffer_view_length),
//         component_type: ComponentType::Float,
//         count: feature_ids_count,
//         type_: AccessorType::Scalar,
//         min: Some(vec![feature_id_min as f64]),
//         max: Some(vec![feature_id_max as f64]),
//         ..Default::default()
//     };
//     gltf.accessors.push(accessor);

//     let primitives = vec![MeshPrimitive {
//         attributes: vec![
//             ("POSITION".to_string(), buffer_view_length - 2),
//             (format!("_FEATURE_ID_{}", 0), buffer_view_length),
//         ]
//         .into_iter()
//         .collect(),
//         indices: Some(buffer_view_length - 1),
//         mode: PrimitiveMode::Triangles,
//         ..Default::default()
//     }];
//     mesh_primitives.extend(primitives);

//     gltf.meshes[0].primitives = mesh_primitives;

//     GltfContent {
//         class_name: class_name.to_string(),
//         gltf,
//         bin_content,
//     }
// }

// // value to bytes
// trait ToBytes {
//     fn write_to_bytes(&self, buffer: &mut Vec<u8>);
// }

// impl ToBytes for i64 {
//     fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
//         buffer.write_i64::<LittleEndian>(*self).unwrap();
//     }
// }

// impl ToBytes for u64 {
//     fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
//         buffer.write_u64::<LittleEndian>(*self).unwrap();
//     }
// }

// impl ToBytes for f64 {
//     fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
//         buffer.write_f64::<LittleEndian>(*self).unwrap();
//     }
// }

// impl ToBytes for bool {
//     fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
//         let value = if *self { 1 } else { 0 };
//         buffer.write_i32::<LittleEndian>(value).unwrap();
//     }
// }

// impl ToBytes for Measure {
//     fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
//         let value = self.value();
//         buffer.write_f64::<LittleEndian>(value).unwrap();
//     }
// }

// impl ToBytes for Value {
//     fn write_to_bytes(&self, buffer: &mut Vec<u8>) {
//         match self {
//             Value::Integer(i) => i.write_to_bytes(buffer),
//             Value::NonNegativeInteger(u) => u.write_to_bytes(buffer),
//             Value::Double(d) => d.write_to_bytes(buffer),
//             Value::Boolean(b) => b.write_to_bytes(buffer),
//             Value::Measure(m) => m.write_to_bytes(buffer),
//             // todo: 他の型にも対応する
//             _ => {}
//         }
//     }
// }

// trait ToBuffer {
//     fn write_to_buffer(&self, buffer: &mut Vec<u8>, string_offset_buffer: &mut Vec<u32>);
// }

// impl ToBuffer for Value {
//     fn write_to_buffer(&self, buffer: &mut Vec<u8>, string_offset_buffer: &mut Vec<u32>) {
//         match self {
//             Value::String(s) => {
//                 string_offset_buffer.push(buffer.len() as u32);
//                 buffer.write_all(s.as_bytes()).unwrap();
//             }
//             Value::Code(c) => {
//                 let json = c.value();
//                 string_offset_buffer.push(buffer.len() as u32);
//                 buffer.write_all(json.as_bytes()).unwrap();
//             }
//             Value::Array(a) => {
//                 let json = serde_json::to_string(a).unwrap();
//                 string_offset_buffer.push(buffer.len() as u32);
//                 buffer.write_all(json.as_bytes()).unwrap();
//             }
//             Value::Object(o) => {
//                 let json = serde_json::to_string(o).unwrap();
//                 string_offset_buffer.push(buffer.len() as u32);
//                 buffer.write_all(json.as_bytes()).unwrap();
//             }
//             _ => self.write_to_bytes(buffer),
//         }
//     }
// }

// pub fn append_gltf_extensions(
//     content: &mut GltfContent,
//     schema: &Schema,
//     entities_list: &IndexMap<String, Vec<TriangulatedEntity>>,
// ) {
//     let class_name = &content.class_name;
//     let gltf = &mut content.gltf;
//     let bin_content = &mut content.bin_content;

//     let buffer_view_length = gltf.buffer_views.len() as u32;
//     let mut classes = HashMap::new();
//     let mut property_tables = Vec::new();

//     let mut buffer_views: Vec<BufferView> = Vec::new();

//     let entities = entities_list.get(&content.class_name).unwrap();

//     // glTF拡張のext_structural_metadataを作成
//     // トップレベルのクラスごとに処理
//     let feature_count = entities.len() as u32;

//     let type_def = schema.types.get::<String>(class_name).unwrap();

//     // classesを作成
//     let class = to_gltf_class(class_name, type_def);
//     let classes_length = classes.len() as u32;
//     classes.extend(class.clone());

//     // property_tableを作成
//     let property_table =
//         to_gltf_property_table(class_name, type_def, buffer_view_length, feature_count);
//     let property_tables_length = property_tables.len() as u32;

//     // buffer_viewを作成
//     let mut properties = property_table.properties.iter().collect::<Vec<_>>();
//     properties.sort_by(|a, b| a.1.values.cmp(&b.1.values));

//     // 抽出するべき全ての属性がproperty_nameに入る
//     for (property_name, _) in properties {
//         // property_nameに対応するbuffer_viewを作成する

//         // todo: データ型ごとの対応をする
//         // 配列の場合は、array_offsetを書き込む必要があるなど
//         let mut buffer: Vec<u8> = Vec::new();
//         let mut string_offset_buffer: Vec<u32> = Vec::new();

//         // このproperty_nameに対応するbuffer_viewを作成する
//         for entity in entities {
//             // 個別のentityから、property_nameに対応する属性を取り出す
//             // なければ、無視する
//             let attribute_name_list = entity.attributes.attributes.keys().collect::<Vec<_>>();
//             let is_hit = attribute_name_list.contains(&property_name);
//             // classから型を取り出す
//             let p = class
//                 .get(class_name)
//                 .unwrap()
//                 .properties
//                 .get(property_name)
//                 .unwrap();
//             let property_type = &p.type_;
//             let component_type = &p.component_type;

//             if !is_hit {
//                 match property_type {
//                     extensions::gltf::ext_structural_metadata::ClassPropertyType::String => {
//                         string_offset_buffer.push(buffer.len() as u32);
//                         buffer.write_all(b"0").unwrap();
//                     }
//                     extensions::gltf::ext_structural_metadata::ClassPropertyType::Boolean => {
//                         buffer.write_u8(0).unwrap();
//                     }
//                     _ => {}
//                 }

//                 // component_typeがSomeなら値を取り出す
//                 if let Some(component_type) = component_type {
//                     let component_type = component_type.clone();
//                     match component_type {
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int8 => {
//                                 buffer.write_i8(0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Uint8 => {
//                                 buffer.write_u8(0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int16 => {
//                                 buffer.write_i16::<LittleEndian>(0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Uint16 => {
//                                 buffer.write_u16::<LittleEndian>(0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int32 => {
//                                 buffer.write_i32::<LittleEndian>(0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Uint32 => {
//                                 buffer.write_u32::<LittleEndian>(0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float32 => {
//                                 buffer.write_f32::<LittleEndian>(0.0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Float64 => {
//                                 buffer.write_f64::<LittleEndian>(0.0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Int64 => {
//                                 buffer.write_i64::<LittleEndian>(0).unwrap();
//                             },
//                             extensions::gltf::ext_structural_metadata::ClassPropertyComponentType::Uint64 => {
//                                 buffer.write_u64::<LittleEndian>(0).unwrap();
//                             },
//                         }
//                 };
//                 continue;
//             }

//             // 属性名が一致するものがあれば、ちゃんと取り出す
//             // entity.attributes.attributes.get(&property_name)が効かないので、for文で取り出す
//             if let Some((_, value)) = entity
//                 .attributes
//                 .attributes
//                 .iter()
//                 .find(|(name, _)| *name == property_name)
//             {
//                 // bufferに書き込み
//                 value.write_to_buffer(&mut buffer, &mut string_offset_buffer);
//             }
//         }
//         // offsetには文字列の最後にもインデックスが必要
//         if !string_offset_buffer.is_empty() {
//             string_offset_buffer.push(buffer.len() as u32);
//         }

//         let byte_offset = bin_content.len();
//         let byte_length = buffer.len();

//         bin_content.extend(buffer.iter());

//         buffer_views.push(BufferView {
//             byte_offset: byte_offset as u32,
//             byte_length: byte_length as u32,
//             ..Default::default()
//         });

//         if !string_offset_buffer.is_empty() {
//             let byte_offset = bin_content.len();
//             let byte_length = string_offset_buffer.len() as u32 * 4;

//             bin_content.extend(string_offset_buffer.iter().flat_map(|x| x.to_le_bytes()));

//             buffer_views.push(BufferView {
//                 byte_offset: byte_offset as u32,
//                 byte_length,
//                 ..Default::default()
//             });
//         }
//     }

//     property_tables.push(property_table);

//     // feature_idsを作成
//     let feature_id = extensions::mesh::ext_mesh_features::FeatureId {
//         attribute: Some(classes_length),
//         feature_count,
//         property_table: Some(property_tables_length),
//         ..Default::default()
//     };

//     let mesh_primitive_extensions = Some(extensions::mesh::MeshPrimitive {
//         ext_mesh_features: Some(extensions::mesh::ext_mesh_features::ExtMeshFeatures {
//             feature_ids: vec![feature_id],
//             ..Default::default()
//         }),
//         ..Default::default()
//     });
//     gltf.meshes[0].primitives[property_tables.len() - 1].extensions = mesh_primitive_extensions;

//     let extensions = extensions::gltf::Gltf {
//         ext_structural_metadata: Some(
//             extensions::gltf::ext_structural_metadata::ExtStructuralMetadata {
//                 schema: Some(extensions::gltf::ext_structural_metadata::Schema {
//                     classes,
//                     ..Default::default()
//                 }),
//                 property_tables: Some(property_tables.clone()),
//                 ..Default::default()
//             },
//         ),
//         ..Default::default()
//     };

//     gltf.buffer_views.extend(buffer_views);

//     gltf.extensions = Some(extensions);

//     // Add after all binary buffers have been written
//     let buffers = vec![Buffer {
//         byte_length: bin_content.len() as u32,
//         ..Default::default()
//     }];
//     gltf.buffers = buffers;
// }

// pub fn write_gltf<W: Write>(gltf: Gltf, mut bin_content: Vec<u8>, mut writer: W) {
//     let mut json_content = serde_json::to_vec(&gltf).unwrap();

//     // append padding
//     json_content.extend(vec![0x20; (4 - (json_content.len() % 4)) % 4].iter());
//     bin_content.extend(vec![0x0; (4 - (bin_content.len() % 4)) % 4].iter());

//     let total_size = 12 + 8 + json_content.len() + 8 + bin_content.len();

//     writer.write_all(b"glTF").unwrap();
//     // magic
//     writer.write_all(&2u32.to_le_bytes()).unwrap();
//     // version: 2
//     writer
//         .write_all(&(total_size as u32).to_le_bytes())
//         .unwrap();
//     // total size

//     writer
//         .write_all(&(json_content.len() as u32).to_le_bytes())
//         .unwrap();
//     // json content
//     writer.write_all(b"JSON").unwrap();
//     // chunk type
//     writer.write_all(&json_content).unwrap();
//     // json content

//     writer
//         .write_all(&(bin_content.len() as u32).to_le_bytes())
//         .unwrap();
//     // json content
//     writer.write_all(b"BIN\0").unwrap();
//     // chunk type
//     writer.write_all(&bin_content).unwrap();
//     // bin content

//     writer.flush().unwrap();
// }

// This is the code to verify the operation with Cesium
pub fn write_3dtiles(
    bounding_volume: [f64; 6],
    output_path: &Path,
    filenames: &[String],
) -> std::io::Result<()> {
    // write 3DTiles
    let tileset_path = output_path.join("tileset.json");

    let contents: Vec<nusamai_3dtiles_json::tileset::Content> = filenames
        .iter()
        .map(|filename| {
            let uri = filename.to_string();
            nusamai_3dtiles_json::tileset::Content {
                uri,
                ..Default::default()
            }
        })
        .collect();

    let tileset = nusamai_3dtiles_json::tileset::Tileset {
        geometric_error: 1e+100,
        asset: nusamai_3dtiles_json::tileset::Asset {
            version: "1.1".to_string(),
            ..Default::default()
        },
        root: nusamai_3dtiles_json::tileset::Tile {
            bounding_volume: nusamai_3dtiles_json::tileset::BoundingVolume {
                region: Some(bounding_volume),
                ..Default::default()
            },
            contents: Some(contents),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tileset_file = File::create(tileset_path).unwrap();
    let tileset_writer = BufWriter::with_capacity(1024 * 1024, &mut tileset_file);
    serde_json::to_writer_pretty(tileset_writer, &tileset).unwrap();

    Ok(())
}
