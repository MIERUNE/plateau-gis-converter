use std::io::Write;

use super::{material, ClassFeatures, VertexData};
use crate::pipeline::{feedback, PipelineError};
use material::{load_image, Material, Texture};

pub fn write_obj<W: Write>(
    feedback: &feedback::Feedback,
    mut obj_writer: W,
    mut mtl_writer: W,
    features: ClassFeatures,
    feature_vertex_data: Vec<(u32, Vec<VertexData>)>,
    file_name: String,
    file_path: std::path::PathBuf,
) -> Result<(), PipelineError> {
    // Material
    writeln!(obj_writer, "mtllib {}.mtl", file_name)?;

    let mut global_vertex_offset = 0;
    let mut global_texture_offset = 0;

    // OBJファイル書き込み部分
    for (feature_id, feature_data) in &feature_vertex_data {
        let type_id = feature_data.first().unwrap().type_id.as_ref().unwrap();
        // if "bldg_d05b0b65-eabf-473b-ab1c-cd9245aa3437" == type_id {
        // if "bldg_51ed9798-bea2-4217-8389-e065e3586e61" == type_id {
        if true {
            writeln!(obj_writer, "o {}", type_id)?;

            // 頂点とテクスチャ座標の書き込み
            for vertex in feature_data {
                writeln!(
                    obj_writer,
                    "v {} {} {}",
                    vertex.position[0], vertex.position[1], vertex.position[2]
                )?;
            }

            for vertex in feature_data {
                writeln!(
                    obj_writer,
                    "vt {} {}",
                    vertex.tex_coord[0],
                    1.0 - vertex.tex_coord[1]
                )?;
            }

            // テクスチャとマテリアル情報のキャッシュ
            let mut texture_cache: std::collections::HashMap<String, Vec<u8>> =
                std::collections::HashMap::new();
            let mut material_written: std::collections::HashSet<String> =
                std::collections::HashSet::new();

            // マテリアルIDごとにフェイスをグループ化
            let mut faces_by_material: std::collections::HashMap<usize, Vec<(usize, &VertexData)>> =
                std::collections::HashMap::new();
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

                if let Some(Texture { uri }) = &mat.base_texture {
                    if let Ok(path) = uri.to_file_path() {
                        let image_file_name =
                            format!("Feature_{}_Material_{}.jpg", feature_id, material_id);

                        // テクスチャがキャッシュにない場合のみ読み込む
                        if !texture_cache.contains_key(&image_file_name) {
                            let content = load_image(feedback, &path)?;
                            texture_cache.insert(image_file_name.clone(), content);

                            let textures_dir = file_path.join("textures");
                            std::fs::create_dir_all(&textures_dir)?;

                            let image_path = textures_dir.join(&image_file_name);
                            std::fs::write(
                                &image_path,
                                texture_cache.get(&image_file_name).unwrap(),
                            )?;
                        }

                        let mat_key = format!("{}_{}", feature_id, material_id);
                        // マテリアル情報が未書き込みの場合のみMTLファイルに書き込む
                        if !material_written.contains(&mat_key) {
                            writeln!(mtl_writer, "newmtl Material_{}", mat_key)?;
                            writeln!(mtl_writer, "map_Kd .\\textures\\{}", image_file_name)?;
                            material_written.insert(mat_key);
                        }

                        writeln!(obj_writer, "usemtl Material_{}_{}", feature_id, material_id)?;
                    }
                } else {
                    let [r, g, b, _] = mat.base_color;

                    let color_key = format!("{:.6}_{:.6}_{:.6}", r, g, b);

                    if !material_written.contains(&color_key) {
                        writeln!(mtl_writer, "newmtl Material_{}_{}_{}", r, g, b)?;
                        writeln!(mtl_writer, "Ks {} {} {}", r, g, b)?;
                        material_written.insert(color_key);
                    }

                    writeln!(obj_writer, "usemtl Material_{}_{}_{}", r, g, b)?;
                }

                for (i, _) in faces {
                    if i % 3 == 0 {
                        writeln!(
                            obj_writer,
                            "f {}/{} {}/{} {}/{}",
                            global_vertex_offset + i + 1,
                            global_texture_offset + i + 1,
                            global_vertex_offset + i + 2,
                            global_texture_offset + i + 2,
                            global_vertex_offset + i + 3,
                            global_texture_offset + i + 3
                        )?;
                    }
                }
            }

            writeln!(
                obj_writer,
                "# ======================================================",
            )?;

            global_vertex_offset += feature_data.len();
            global_texture_offset += feature_data.len();
        }
    }

    obj_writer.flush()?;
    mtl_writer.flush()?;
    Ok(())
}
