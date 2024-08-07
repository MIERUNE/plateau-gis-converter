use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::{material, ClassFeatures, VertexData};
use crate::pipeline::{feedback, PipelineError};
use material::{load_image, Texture};

pub fn write_obj<W: Write>(
    feedback: &feedback::Feedback,
    mut obj_writer: W,
    features: ClassFeatures,
    feature_vertex_data: Vec<(u32, Vec<VertexData>)>,
    file_name: String,
    file_path: PathBuf,
    has_split: bool,
) -> Result<(), PipelineError> {
    let dir_name = file_path.to_str().unwrap();
    let mut mtl_writer = File::create(format!("{}/{}.mtl", dir_name, file_name))?;

    let mut global_vertex_offset = 0;

    let mut texture_cache: HashMap<String, Vec<u8>> = HashMap::new();
    let mut material_written: HashSet<String> = HashSet::new();

    for (feature_id, feature_data) in &feature_vertex_data {
        // Writing of object name (option)
        if has_split {
            writeln!(obj_writer, "o Feature_{}", feature_id)?;
        };

        // Writing of vertex coordinates
        for vertex in feature_data {
            writeln!(
                obj_writer,
                "v {} {} {}",
                vertex.position[0], vertex.position[1], vertex.position[2]
            )?;
        }

        // Writing of UV coordinates
        for vertex in feature_data {
            writeln!(
                obj_writer,
                "vt {} {}",
                vertex.tex_coord[0],
                1.0 - vertex.tex_coord[1]
            )?;
        }

        // Grouping of surfaces by material_id
        let mut faces_by_material: HashMap<usize, Vec<(usize, &VertexData)>> = HashMap::new();
        for (i, vertex) in feature_data.iter().enumerate() {
            faces_by_material
                .entry(vertex.material_id)
                .or_insert_with(Vec::new)
                .push((i, vertex));
        }

        for (material_id, faces) in faces_by_material.iter() {
            let feature = features
                .features
                .iter()
                .find(|f| f.feature_id == Some(*feature_id))
                .unwrap();
            let mat = &feature.materials[*material_id];

            // Loading textures.
            if let Some(Texture { uri }) = &mat.base_texture {
                if let Ok(path) = uri.to_file_path() {
                    let image_file_name =
                        format!("Feature_{}_Material_{}.jpg", feature_id, material_id);

                    // Load only if the texture is not in the cache.
                    if !texture_cache.contains_key(&image_file_name) {
                        let content = load_image(feedback, &path)?;
                        texture_cache.insert(image_file_name.clone(), content);

                        let textures_dir = file_path.join("textures");
                        std::fs::create_dir_all(&textures_dir)?;

                        let image_path = textures_dir.join(&image_file_name);
                        std::fs::write(&image_path, texture_cache.get(&image_file_name).unwrap())?;
                    }

                    // Write to MTL file only if material information has not yet been written
                    let mat_key = format!("{}_{}", feature_id, material_id);

                    if !material_written.contains(&mat_key) {
                        writeln!(mtl_writer, "newmtl Material_{}", mat_key)?;
                        writeln!(mtl_writer, "map_Kd .\\textures\\{}", image_file_name)?;
                        material_written.insert(mat_key);
                    }

                    writeln!(obj_writer, "usemtl Material_{}_{}", feature_id, material_id)?;
                }
            } else {
                // Create a base_color material if no texture is available.
                let [r, g, b, _] = mat.base_color;
                let color_key = format!("{:.6}_{:.6}_{:.6}", r, g, b);

                if !material_written.contains(&color_key) {
                    writeln!(mtl_writer, "newmtl Material_{}_{}_{}", r, g, b)?;
                    writeln!(mtl_writer, "Ka {} {} {}", r, g, b)?;
                    writeln!(mtl_writer, "Kd {} {} {}", r, g, b)?;
                    material_written.insert(color_key);
                }

                writeln!(obj_writer, "usemtl Material_{}_{}_{}", r, g, b)?;
            }

            // Write surface
            for (i, _) in faces {
                if i % 3 == 0 {
                    writeln!(
                        obj_writer,
                        "f {}/{} {}/{} {}/{}",
                        global_vertex_offset + i + 1,
                        global_vertex_offset + i + 1,
                        global_vertex_offset + i + 2,
                        global_vertex_offset + i + 2,
                        global_vertex_offset + i + 3,
                        global_vertex_offset + i + 3
                    )?;
                }
            }
        }
        global_vertex_offset += feature_data.len();
    }

    obj_writer.flush()?;
    mtl_writer.flush()?;
    Ok(())
}
