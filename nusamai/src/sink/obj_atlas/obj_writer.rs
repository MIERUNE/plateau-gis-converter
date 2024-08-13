use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use super::{ObjInfo, ObjMaterials};
use crate::pipeline::PipelineError;

pub fn write(
    meshes: ObjInfo,
    materials: ObjMaterials,
    folder_path: PathBuf,
    is_split: bool,
) -> Result<(), PipelineError> {
    let mut material_cache: HashMap<String, String> = HashMap::new();

    write_mtl(&materials, &mut material_cache, &folder_path)?;
    write_obj(&meshes, &mut material_cache, &folder_path, is_split)?;

    Ok(())
}

fn write_obj(
    meshes: &ObjInfo,
    material_cache: &mut HashMap<String, String>,
    folder_path: &Path,
    is_split: bool,
) -> Result<(), PipelineError> {
    let dir_name = folder_path.to_str().unwrap();
    let mut obj_writer = File::create(format!(
        "{}/{}.obj",
        dir_name,
        folder_path.file_stem().unwrap().to_str().unwrap()
    ))?;

    let mut global_vertex_offset = 0;

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
                // todo: Add error handling
                println!("Material not found: {}", material_key);
                continue;
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

    Ok(())
}

fn write_mtl(
    materials: &ObjMaterials,
    material_cache: &mut HashMap<String, String>,
    folder_path: &Path,
) -> Result<(), PipelineError> {
    let dir_name = folder_path.to_str().unwrap();
    let mut mtl_writer = File::create(format!(
        "{}/{}.mtl",
        dir_name,
        folder_path.file_stem().unwrap().to_str().unwrap()
    ))?;

    for (material_key, material) in materials {
        if material_cache.contains_key(material_key) {
            continue;
        }

        if let Some(uri) = &material.texture_uri {
            if let Ok(path) = uri.to_file_path() {
                writeln!(mtl_writer, "newmtl {}", material_key)?;

                let texture_name = path.file_name().unwrap().to_str().unwrap();
                writeln!(mtl_writer, "map_Kd .\\textures\\{}", texture_name)?;

                material_cache.insert(material_key.to_string(), path.to_str().unwrap().to_string());
            }
        } else {
            let (r, g, b) = (
                material.base_color[0],
                material.base_color[1],
                material.base_color[2],
            );
            let color_key = format!("{}_{}_{}", r, g, b);
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

    mtl_writer.flush()?;

    Ok(())
}
