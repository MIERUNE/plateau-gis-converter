use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::{material, Feature, ObjInfo, ObjMaterials, ObjMesh, VertexData};
use crate::pipeline::{feedback, PipelineError};
use material::{load_image, Texture};

pub fn write_obj_2(
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

    for (gml_id, mesh) in meshes {
        writeln!(obj_writer, "o {}", gml_id)?;

        for vertex in &mesh.vertices {
            writeln!(obj_writer, "v {} {} {}", vertex[0], vertex[1], vertex[2])?;
        }
        for tex_coord in &mesh.uvs {
            writeln!(obj_writer, "vt {} {}", tex_coord[0], tex_coord[1])?;
        }
        for (material_key, indices) in &mesh.primitives {
            writeln!(obj_writer, "usemtl {}", material_key)?;
            for index in indices.chunks(3) {
                writeln!(
                    obj_writer,
                    "f {}/{} {}/{} {}/{}",
                    index[0], index[0], index[1], index[1], index[2], index[2]
                )?;
            }
        }
    }

    for (material_key, material) in materials {
        writeln!(mtl_writer, "newmtl {}", material_key)?;
        writeln!(
            mtl_writer,
            "Ka {} {} {}",
            material.base_color[0], material.base_color[1], material.base_color[2]
        )?;
        writeln!(
            mtl_writer,
            "Kd {} {} {}",
            material.base_color[0], material.base_color[1], material.base_color[2]
        )?;
        if let Some(uri) = &material.texture_uri {
            if let Ok(path) = uri.to_file_path() {
                let content = load_image(feedback, &path)?;

                let textures_dir = folder_path.join("textures");
                std::fs::create_dir_all(&textures_dir)?;

                let image_file_name = format!("{}.jpg", material_key);
                let image_path = textures_dir.join(&image_file_name);
                std::fs::write(&image_path, content)?;

                writeln!(mtl_writer, "map_Kd .\\textures\\{}", image_file_name)?;
            }
        }
    }

    obj_writer.flush()?;

    Ok(())
}

pub fn write_obj<W: Write>(
    feedback: &feedback::Feedback,
    mut obj_writer: W,
    features: Vec<Feature>,
    feature_vertex_data: Vec<(u32, Vec<VertexData>)>,
    file_name: String,
    file_path: PathBuf,
    is_split: bool,
) -> Result<(), PipelineError> {
    let dir_name = file_path.to_str().unwrap();
    let mut mtl_writer = File::create(format!("{}/{}.mtl", dir_name, file_name))?;

    let mut global_vertex_offset = 0;

    let mut material_holder: HashSet<String> = HashSet::new();

    for (feature_id, feature_data) in &feature_vertex_data {
        // Writing of object name (option)
        if is_split {
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

        // TODO: In the future, modify this to skip writing texture coordinates (vt) when there's no texture information
        // Writing of UV coordinates
        for vertex in feature_data {
            writeln!(
                obj_writer,
                "vt {} {}",
                vertex.tex_coord[0], vertex.tex_coord[1]
            )?;
        }

        // Grouping of surfaces by material_id
        let mut faces_by_material: HashMap<usize, Vec<(usize, &VertexData)>> = HashMap::new();
        for (i, vertex) in feature_data.iter().enumerate() {
            faces_by_material
                .entry(vertex.material_id)
                .or_default()
                .push((i, vertex));
        }

        for (material_id, faces) in faces_by_material.iter() {
            let feature = features
                .iter()
                .find(|f| f.feature_id == Some(*feature_id))
                .unwrap();
            let mat = &feature.materials[*material_id];

            // Loading textures.
            if let Some(Texture { uri }) = &mat.base_texture {
                if let Ok(path) = uri.to_file_path() {
                    let image_file_name =
                        format!("Feature_{}_Material_{}.jpg", feature_id, material_id);

                    let mat_key = format!("{}_{}", feature_id, material_id);

                    // Write to MTL file only if material information has not yet been written
                    if !material_holder.contains(&mat_key) {
                        let content = load_image(feedback, &path)?;

                        let textures_dir = file_path.join("textures");
                        std::fs::create_dir_all(&textures_dir)?;

                        let image_path = textures_dir.join(&image_file_name);
                        std::fs::write(&image_path, content)?;

                        writeln!(mtl_writer, "newmtl Material_{}", mat_key)?;
                        writeln!(mtl_writer, "map_Kd .\\textures\\{}", image_file_name)?;
                        material_holder.insert(mat_key);
                    }

                    writeln!(obj_writer, "usemtl Material_{}_{}", feature_id, material_id)?;
                }
            } else {
                // Create a base_color material if no texture is available.
                let [r, g, b, _] = mat.base_color;
                let color_key = format!("{:.6}_{:.6}_{:.6}", r, g, b);

                if !material_holder.contains(&color_key) {
                    writeln!(mtl_writer, "newmtl Material_{}_{}_{}", r, g, b)?;
                    writeln!(mtl_writer, "Ka {} {} {}", r, g, b)?;
                    writeln!(mtl_writer, "Kd {} {} {}", r, g, b)?;
                    material_holder.insert(color_key);
                }

                writeln!(obj_writer, "usemtl Material_{}_{}_{}", r, g, b)?;
            }

            // TODO: When there's no texture information, omit texture coordinate references in face definitions (f v1 v2 v3)
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
