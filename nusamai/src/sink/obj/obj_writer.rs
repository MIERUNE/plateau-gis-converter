use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write as _},
    path::{Path, PathBuf},
};

use rayon::prelude::*;

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
    let file_name = folder_path.file_stem().unwrap().to_str().unwrap();
    let obj_path = format!("{}/{}.obj", dir_name, file_name);

    let mut all_vertices = Vec::new();
    let mut all_uvs = Vec::new();
    let mut mesh_data = Vec::new();

    for (feature_id, mesh) in meshes {
        let vertex_offset = all_vertices.len();
        let uv_offset = all_uvs.len();

        all_vertices.extend_from_slice(&mesh.vertices);
        all_uvs.extend_from_slice(&mesh.uvs);

        mesh_data.push((feature_id, mesh, vertex_offset, uv_offset));
    }

    let mut obj_writer = BufWriter::new(File::create(obj_path)?);

    writeln!(obj_writer, "mtllib {}.mtl", file_name)?;

    for vertex in &all_vertices {
        writeln!(obj_writer, "v {} {} {}", vertex[0], vertex[1], vertex[2])?;
    }
    for uv in &all_uvs {
        writeln!(obj_writer, "vt {} {}", uv[0], uv[1])?;
    }

    let face_data: Vec<String> = mesh_data
        .par_iter()
        .flat_map(|(feature_id, mesh, vertex_offset, uv_offset)| {
            let mut local_obj = Vec::new();

            if is_split {
                local_obj.push(format!("o {}", feature_id));
                local_obj.push(format!("g {}", feature_id));
            }

            for (material_key, indices) in &mesh.primitives {
                if material_cache.contains_key(material_key) {
                    local_obj.push(format!("usemtl {}", material_key));
                } else {
                    eprintln!("Material not found: {}", material_key);
                    continue;
                }

                for index in indices.chunks(3) {
                    local_obj.push(format!(
                        "f {}/{} {}/{} {}/{}",
                        index[0] + 1 + *vertex_offset as u32,
                        index[0] + 1 + *uv_offset as u32,
                        index[1] + 1 + *vertex_offset as u32,
                        index[1] + 1 + *uv_offset as u32,
                        index[2] + 1 + *vertex_offset as u32,
                        index[2] + 1 + *uv_offset as u32
                    ));
                }
            }

            local_obj
        })
        .collect();

    for line in face_data {
        writeln!(obj_writer, "{}", line)?;
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
