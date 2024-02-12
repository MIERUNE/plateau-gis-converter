use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use ahash::RandomState;
use byteorder::{ByteOrder, LittleEndian};
use indexmap::{IndexMap, IndexSet};

use nusamai_citygml::schema::Schema;
use nusamai_gltf_json::{
    extensions, Accessor, AccessorType, Buffer, BufferView, BufferViewTarget, ComponentType, Gltf,
    Mesh, MeshPrimitive, Node, PrimitiveMode, Scene,
};

use crate::sink::gltf::attributes::{
    attributes_to_buffer, to_gltf_class, to_gltf_property_table, Attributes,
};

use super::positions::Vertex;
use super::Buffers;

pub fn build_base_gltf(
    buffers: &IndexMap<String, Buffers>,
    translation: [f64; 3],
    min: [f64; 3],
    max: [f64; 3],
) -> (Vec<u8>, Gltf) {
    let mut bin_contents: Vec<Vec<u8>> = Vec::new();
    let mut gltf_list = Vec::new();
    for (_class_name, buffer) in buffers.iter() {
        let (bin_content, gltf) = to_gltf(&buffer.vertices, &buffer.indices, translation, min, max);
        bin_contents.push(bin_content);
        gltf_list.push(gltf);
    }

    let mut base_gltf = Gltf {
        extensions_used: vec![
            "EXT_mesh_features".to_string(),
            "EXT_structural_metadata".to_string(),
        ],
        scenes: vec![Scene {
            nodes: Some(vec![0]),
            ..Default::default()
        }],
        nodes: vec![Node {
            mesh: Some(0),
            translation,
            ..Default::default()
        }],
        ..Default::default()
    };
    for gltf in gltf_list {
        // accessorsを整列
        for accessor in gltf.accessors {
            let mut new_accessor = accessor.clone();
            new_accessor.buffer_view =
                Some(accessor.buffer_view.unwrap() + base_gltf.buffer_views.len() as u32);
            base_gltf.accessors.push(new_accessor);
        }

        // buffer_viewsを整列
        for buffer_view in gltf.buffer_views {
            let mut new_buffer_view = buffer_view.clone();
            new_buffer_view.byte_offset += bin_contents.len() as u32;
            base_gltf.buffer_views.push(new_buffer_view);
        }

        // meshes.primitivesを整列
        for mesh in gltf.meshes.iter() {
            for primitive in mesh.primitives.iter() {
                let mut new_primitive = primitive.clone();
                new_primitive.indices =
                    Some(primitive.indices.unwrap() + bin_contents.len() as u32);
                let mut new_attributes = IndexMap::new();
                for (key, value) in primitive.attributes.clone() {
                    new_attributes.insert(key, value + bin_contents.len() as u32);
                }
                let new_attributes: HashMap<String, u32> = new_attributes.into_iter().collect();
                new_primitive.attributes = new_attributes;
            }
            base_gltf.meshes.push(mesh.clone());
        }
    }

    let mut bin_content: Vec<u8> = Vec::new();
    for content in bin_contents {
        bin_content.extend(content.iter());
    }

    (bin_content, base_gltf)
}

pub fn to_gltf(
    vertices: &IndexSet<Vertex<u32>>,
    indices: &Vec<u32>,
    translation: [f64; 3],
    min: [f64; 3],
    max: [f64; 3],
) -> (Vec<u8>, Gltf) {
    let mut bin_content: Vec<u8> = Vec::new();

    // write vertices
    let vertices_offset = bin_content.len();
    let mut buf = [0; 12];
    let mut vertices_count = 0;
    for vertex in vertices.clone() {
        LittleEndian::write_u32_into(&vertex.position, &mut buf);
        bin_content.write_all(&buf).unwrap();
        vertices_count += 1;
    }
    let vertices_len: usize = bin_content.len() - vertices_offset;

    // write indices
    let indices_offset = bin_content.len();
    let mut indices_count = 0;
    for idx in indices {
        bin_content.write_all(&idx.to_le_bytes()).unwrap();
        indices_count += 1;
    }
    let indices_len = bin_content.len() - indices_offset;

    // write feature_ids
    let feature_ids_offset = bin_content.len();
    let mut feature_ids_count = 0;
    for vertex in vertices.clone() {
        bin_content
            .write_all(&vertex.feature_id.to_le_bytes())
            .unwrap();
        feature_ids_count += 1;
    }
    let feature_ids_len = bin_content.len() - feature_ids_offset;

    // make base gltf structure
    let gltf = Gltf {
        extensions_used: vec![
            "EXT_mesh_features".to_string(),
            "EXT_structural_metadata".to_string(),
        ],
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
            primitives: vec![MeshPrimitive {
                // todo: _FEATURE_ID_〇〇は地物型が複数存在すると動的に変化させた方が良いかもしれない
                attributes: vec![
                    ("POSITION".to_string(), 0),
                    ("_FEATURE_ID_0".to_string(), 2),
                ]
                .into_iter()
                .collect(),
                indices: Some(1),
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
            Accessor {
                buffer_view: Some(2),
                component_type: ComponentType::UnsignedInt,
                count: feature_ids_count,
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
            BufferView {
                byte_offset: feature_ids_offset as u32,
                byte_length: feature_ids_len as u32,
                target: Some(BufferViewTarget::ArrayBuffer),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    (bin_content, gltf)
}

pub fn append_gltf_extensions(
    gltf: &mut Gltf,
    bin_content: &mut Vec<u8>,
    class_names: HashSet<std::borrow::Cow<'_, str>>,
    schema: &Schema,
    vertices: IndexSet<Vertex<u32>, RandomState>,
    attributes: Vec<Attributes>,
) {
    let mut buffer_view_length = gltf.buffer_views.len() as u32;
    let feature_count = vertices.iter().map(|v| v.feature_id).max().unwrap();

    let mut classes = HashMap::new();
    let mut property_tables = Vec::new();

    for class_name_cow in class_names.iter() {
        let class_name = class_name_cow.as_ref().to_string();
        let type_def = schema.types.get::<String>(&class_name).unwrap();

        let class = to_gltf_class(&class_name, type_def);
        classes.extend(class);

        let (property_table, count) =
            to_gltf_property_table(&class_name, type_def, buffer_view_length, feature_count);
        property_tables.push(property_table);
        buffer_view_length = count;
    }

    let extensions = extensions::gltf::Gltf {
        ext_structural_metadata: Some(
            extensions::gltf::ext_structural_metadata::ExtStructuralMetadata {
                schema: Some(extensions::gltf::ext_structural_metadata::Schema {
                    classes,
                    ..Default::default()
                }),
                property_tables: Some(property_tables),
                ..Default::default()
            },
        ),
        ..Default::default()
    };

    // mesh primitiveのextensionを追加
    let mesh_primitive_extensions = Some(extensions::mesh::MeshPrimitive {
        ext_mesh_features: Some(extensions::mesh::ext_mesh_features::ExtMeshFeatures {
            feature_ids: vec![extensions::mesh::ext_mesh_features::FeatureId {
                // todo: 複数の地物型を出力するときには、地物型ごとにfeature_idを付与していくので、attributesやproperty_tableは動的に変化する
                // todo: meshes.primitives.attributesには、地物型ごとのfeature_idを付与するので、以下のattributesへのインデックスも動的に変わる
                attribute: Some(0),
                feature_count: vertices.iter().map(|v| v.feature_id).max().unwrap(),
                property_table: Some(0),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    });
    // todo: 必ず0番目に入れる、というわけではないので対応
    gltf.meshes[0].primitives[0].extensions = mesh_primitive_extensions;

    let attributes_bin_contents = attributes_to_buffer(schema, &attributes);
    let mut buffer_views: Vec<BufferView> = Vec::new();
    for (_, content) in attributes_bin_contents.iter() {
        let byte_offset = bin_content.len();
        let byte_length = content.len();

        bin_content.extend(content.iter());

        buffer_views.push(BufferView {
            byte_offset: byte_offset as u32,
            byte_length: byte_length as u32,
            ..Default::default()
        });
    }

    gltf.buffer_views.extend(buffer_views);

    gltf.extensions = Some(extensions);

    // Add after all binary buffers have been written
    let buffers = vec![Buffer {
        byte_length: bin_content.len() as u32,
        ..Default::default()
    }];
    gltf.buffers = buffers;
}

pub fn write_gltf<W: Write>(gltf: Gltf, mut bin_content: Vec<u8>, mut writer: W) {
    let mut json_content = serde_json::to_vec(&gltf).unwrap();

    // append padding
    json_content.extend(vec![0x20; (4 - (json_content.len() % 4)) % 4].iter());
    bin_content.extend(vec![0x0; (4 - (bin_content.len() % 4)) % 4].iter());

    let total_size = 12 + 8 + json_content.len() + 8 + bin_content.len();

    writer.write_all(b"glTF").unwrap();
    // magic
    writer.write_all(&2u32.to_le_bytes()).unwrap();
    // version: 2
    writer
        .write_all(&(total_size as u32).to_le_bytes())
        .unwrap();
    // total size

    writer
        .write_all(&(json_content.len() as u32).to_le_bytes())
        .unwrap();
    // json content
    writer.write_all(b"JSON").unwrap();
    // chunk type
    writer.write_all(&json_content).unwrap();
    // json content

    writer
        .write_all(&(bin_content.len() as u32).to_le_bytes())
        .unwrap();
    // json content
    writer.write_all(b"BIN\0").unwrap();
    // chunk type
    writer.write_all(&bin_content).unwrap();
    // bin content

    writer.flush().unwrap();
}

// This is the code to verify the operation with Cesium
pub fn write_3dtiles(bounding_volume: [f64; 6], output_path: &PathBuf) {
    // write 3DTiles
    let tileset_path = output_path.with_file_name("tileset.json");
    let content_uri = output_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

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
            content: Some(nusamai_3dtiles_json::tileset::Content {
                uri: content_uri,
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut tileset_file = File::create(&tileset_path).unwrap();
    let tileset_writer = BufWriter::with_capacity(1024 * 1024, &mut tileset_file);
    serde_json::to_writer_pretty(tileset_writer, &tileset).unwrap();
}
