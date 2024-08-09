use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::{material, ObjInfo, ObjMaterials};
use crate::pipeline::{feedback, PipelineError};
use material::load_image;

pub fn write_obj(
    feedback: &feedback::Feedback,
    meshes: ObjInfo,
    materials: ObjMaterials,
    folder_path: PathBuf,
    is_split: bool,
) -> Result<(), PipelineError> {
    let dir_name = folder_path.to_str().unwrap();
    let mut obj_writer = File::create(format!(
        "{}/{}.obj",
        dir_name,
        folder_path.file_stem().unwrap().to_str().unwrap()
    ))?;
    let mut mtl_writer = File::create(format!(
        "{}/{}.mtl",
        dir_name,
        folder_path.file_stem().unwrap().to_str().unwrap()
    ))?;

    let mut material_cache: HashMap<String, String> = HashMap::new();

    // Write materials
    for (material_key, material) in &materials {
        if material_cache.contains_key(material_key) {
            continue;
        }

        if let Some(uri) = &material.texture_uri {
            if let Ok(path) = uri.to_file_path() {
                writeln!(mtl_writer, "newmtl {}", material_key)?;
                let content = load_image(feedback, &path)?;

                let textures_dir = folder_path.join("textures");
                std::fs::create_dir_all(&textures_dir)?;

                let image_file_name = format!("{}.jpg", material_key);
                let image_path = textures_dir.join(&image_file_name);
                std::fs::write(&image_path, content)?;

                writeln!(mtl_writer, "map_Kd .\\textures\\{}", image_file_name)?;
                material_cache.insert(material_key.to_string(), image_file_name);
            }
        } else {
            let (r, g, b) = (
                material.base_color[0],
                material.base_color[1],
                material.base_color[2],
            );
            let color_key = format!("{:.6}_{:.6}_{:.6}", r, g, b);
            let material_key = format!("material_{}_{}_{}", r, g, b);
            if material_cache.contains_key(&material_key) {
                continue;
            }

            if !material_cache.contains_key(&color_key) {
                writeln!(mtl_writer, "newmtl material_{}_{}_{}", r, g, b)?;
                writeln!(mtl_writer, "Ka {} {} {}", r, g, b)?;
                writeln!(mtl_writer, "Kd {} {} {}", r, g, b)?;
                material_cache.insert(material_key, color_key);
            }
        }
    }

    let mut global_vertex_offset = 0;
    // Write meshes
    for (feature_id, mesh) in meshes {
        if is_split {
            writeln!(obj_writer, "o {}", feature_id)?;
        }

        for vertex in &mesh.vertices {
            writeln!(obj_writer, "v {} {} {}", vertex[0], vertex[1], vertex[2])?;
        }
        for tex_coord in &mesh.uvs {
            writeln!(obj_writer, "vt {} {}", tex_coord[0], tex_coord[1])?;
        }
        for (material_key, indices) in &mesh.primitives {
            if material_cache.contains_key(material_key) {
                writeln!(obj_writer, "usemtl {}", material_key)?;
            } else {
                let m = materials.get(material_key).unwrap();
                let (r, g, b) = (m.base_color[0], m.base_color[1], m.base_color[2]);
                writeln!(obj_writer, "usemtl material_{}_{}_{}", r, g, b)?;
            }
            for index in indices.chunks(3) {
                writeln!(
                    obj_writer,
                    "f {}/{} {}/{} {}/{}",
                    index[0] + 1 + global_vertex_offset,
                    index[0] + 1 + global_vertex_offset,
                    index[1] + 1 + global_vertex_offset,
                    index[1] + 1 + global_vertex_offset,
                    index[2] + 1 + global_vertex_offset,
                    index[2] + 1 + global_vertex_offset
                )?;
            }
        }
        global_vertex_offset += mesh.vertices.len() as u32;
    }

    obj_writer.flush()?;
    mtl_writer.flush()?;

    Ok(())
}
