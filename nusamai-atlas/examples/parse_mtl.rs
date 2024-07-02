use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Material {
    name: String,
    texture: String,
}

#[derive(Debug)]
struct Face {
    material: String,
    texture_coords: Vec<(f32, f32)>,
}

fn main() -> std::io::Result<()> {
    let obj_file = File::open("nusamai-atlas/examples/assets/dice/dice.obj")?;
    let mtl_file = File::open("nusamai-atlas/examples/assets/dice/dice.mtl")?;

    let materials = parse_mtl(mtl_file)?;
    let (vertices, faces) = parse_obj(obj_file, &materials)?;

    println!("Vertices: {:?}", vertices);
    println!("Faces: {:?}", faces);

    Ok(())
}

fn parse_mtl(file: File) -> std::io::Result<HashMap<String, Material>> {
    let reader = BufReader::new(file);
    let mut materials = HashMap::new();
    let mut current_material = String::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "newmtl" => {
                current_material = parts[1].to_string();
                materials.insert(
                    current_material.clone(),
                    Material {
                        name: current_material.clone(),
                        texture: String::new(),
                    },
                );
            }
            "map_Kd" => {
                if let Some(material) = materials.get_mut(&current_material) {
                    material.texture = parts[1].to_string();
                }
            }
            _ => {}
        }
    }

    Ok(materials)
}

fn parse_obj(
    file: File,
    materials: &HashMap<String, Material>,
) -> std::io::Result<(Vec<(f32, f32, f32)>, Vec<Face>)> {
    let reader = BufReader::new(file);
    let mut vertices = Vec::new();
    let mut texture_coords = Vec::new();
    let mut faces = Vec::new();
    let mut current_material = String::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                let x = parts[1].parse().unwrap();
                let y = parts[2].parse().unwrap();
                let z = parts[3].parse().unwrap();
                vertices.push((x, y, z));
            }
            "vt" => {
                let u = parts[1].parse().unwrap();
                let v = parts[2].parse().unwrap();
                texture_coords.push((u, v));
            }
            "usemtl" => {
                current_material = parts[1].to_string();
            }
            "f" => {
                let mut face_texture_coords = Vec::new();
                for part in &parts[1..] {
                    let indices: Vec<&str> = part.split('/').collect();
                    let vt_index: usize = indices[1].parse().unwrap();
                    face_texture_coords.push(texture_coords[vt_index - 1]);
                }
                faces.push(Face {
                    material: current_material.clone(),
                    texture_coords: face_texture_coords,
                });
            }
            _ => {}
        }
    }

    Ok((vertices, faces))
}
