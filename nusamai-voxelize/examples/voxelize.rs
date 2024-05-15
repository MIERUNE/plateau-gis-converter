use std::{
    fs::File,
    io::{BufWriter, Write},
};

use byteorder::{LittleEndian, WriteBytesExt};
use earcut::{utils3d::project3d_to_2d, Earcut};
use hashbrown::HashMap;
use indexmap::IndexSet;
use nusamai_geometry::MultiPolygon;
use serde_json::json;

use nusamai_voxelize::{DdaVoxelizer, MeshVoxelizer};

fn main() {
    let vertices: Vec<[f64; 3]> = vec![
        // exterior
        [21.5, 0.5, 0.5],
        [0.5, 0.5, 21.5],
        [-20.5, 0.5, 0.5],
        [0.5, 0.5, -20.5],
        [0.5, 45.5, 0.5],
        [0.5, -44.5, 0.5],
    ];

    let mut mpoly = MultiPolygon::<u32>::new();

    // index
    mpoly.add_exterior([0, 1, 2, 3]);
    mpoly.add_exterior([0, 1, 4]);
    mpoly.add_exterior([1, 2, 4]);
    mpoly.add_exterior([2, 3, 4]);
    mpoly.add_exterior([3, 0, 4]);
    mpoly.add_exterior([0, 1, 5]);
    mpoly.add_exterior([1, 2, 5]);
    mpoly.add_exterior([2, 3, 5]);
    mpoly.add_exterior([3, 0, 5]);

    // triangulation
    let mut earcutter = Earcut::new();
    let mut buf3d: Vec<[f32; 3]> = Vec::new();
    let mut buf2d: Vec<[f32; 2]> = Vec::new();
    let mut index_buf: Vec<u32> = Vec::new();

    let mut voxelizer = DdaVoxelizer {
        voxels: HashMap::new(),
    };

    for idx_poly in mpoly.iter() {
        let poly = idx_poly.transform(|idx| vertices[*idx as usize]);
        let num_outer = match poly.hole_indices().first() {
            Some(&v) => v as usize,
            None => poly.raw_coords().len(),
        };

        for axis in 0..=2 {
            buf3d.clear();
            buf3d.extend(poly.raw_coords().iter().map(|v| match axis {
                0 => [v[0] as f32, v[1] as f32, v[2] as f32],
                1 => [v[1] as f32, v[0] as f32, v[2] as f32],
                2 => [v[0] as f32, v[2] as f32, v[1] as f32],
                _ => unreachable!(),
            }));
            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                // earcut
                earcutter.earcut(buf2d.iter().cloned(), poly.hole_indices(), &mut index_buf);
                for indx in index_buf.chunks_exact(3) {
                    voxelizer.add_triangle(&[
                        buf3d[indx[0] as usize],
                        buf3d[indx[1] as usize],
                        buf3d[indx[2] as usize],
                    ]);
                }
            }
        }
    }

    let occupied_voxels = voxelizer.finalize();

    // -------------------gltfの作成-------------------

    // voxel is an integer value, but componentType of accessors is 5126 (floating point number),
    // and INTEGER type cannot be used due to primitives constraints

    let mut positions = IndexSet::new();
    let mut indices = Vec::new();

    for (position, _) in occupied_voxels.iter() {
        let [x, y, z] = [position[0] as f32, position[1] as f32, position[2] as f32];

        // Make a voxel cube
        let (idx0, _) = positions.insert_full([
            (x + 0.5).to_bits(),
            (y - 0.5).to_bits(),
            (z + 0.5).to_bits(),
        ]);
        let (idx1, _) = positions.insert_full([
            (x - 0.5).to_bits(),
            (y - 0.5).to_bits(),
            (z + 0.5).to_bits(),
        ]);
        let (idx2, _) = positions.insert_full([
            (x + 0.5).to_bits(),
            (y - 0.5).to_bits(),
            (z - 0.5).to_bits(),
        ]);
        let (idx3, _) = positions.insert_full([
            (x - 0.5).to_bits(),
            (y - 0.5).to_bits(),
            (z - 0.5).to_bits(),
        ]);
        let (idx4, _) = positions.insert_full([
            (x + 0.5).to_bits(),
            (y + 0.5).to_bits(),
            (z + 0.5).to_bits(),
        ]);
        let (idx5, _) = positions.insert_full([
            (x - 0.5).to_bits(),
            (y + 0.5).to_bits(),
            (z + 0.5).to_bits(),
        ]);
        let (idx6, _) = positions.insert_full([
            (x + 0.5).to_bits(),
            (y + 0.5).to_bits(),
            (z - 0.5).to_bits(),
        ]);
        let (idx7, _) = positions.insert_full([
            (x - 0.5).to_bits(),
            (y + 0.5).to_bits(),
            (z - 0.5).to_bits(),
        ]);
        indices.extend(
            [
                idx0, idx1, idx2, idx2, idx1, idx3, idx6, idx5, idx4, idx5, idx6, idx7, idx2, idx3,
                idx6, idx7, idx6, idx3, idx4, idx1, idx0, idx1, idx4, idx5, idx0, idx2, idx4, idx6,
                idx4, idx2, idx5, idx3, idx1, idx3, idx5, idx7,
            ]
            .iter()
            .map(|&idx| idx as u32),
        );
    }

    let mut min_point = [f32::MAX; 3];
    let mut max_point = [f32::MIN; 3];
    {
        let mut bin_file = BufWriter::new(File::create("output.bin").unwrap());
        for pos in &positions {
            min_point[0] = f32::min(min_point[0], f32::from_bits(pos[0]));
            min_point[1] = f32::min(min_point[1], f32::from_bits(pos[1]));
            min_point[2] = f32::min(min_point[2], f32::from_bits(pos[2]));
            max_point[0] = f32::max(max_point[0], f32::from_bits(pos[0]));
            max_point[1] = f32::max(max_point[1], f32::from_bits(pos[1]));
            max_point[2] = f32::max(max_point[2], f32::from_bits(pos[2]));

            bin_file.write_u32::<LittleEndian>(pos[0]).unwrap();
            bin_file.write_u32::<LittleEndian>(pos[1]).unwrap();
            bin_file.write_u32::<LittleEndian>(pos[2]).unwrap();
        }

        for &idx in &indices {
            bin_file.write_u32::<LittleEndian>(idx).unwrap();
        }
    }

    // number of voxels x number of vertex coordinates (3) x 4 bytes (f32)
    let positions_size = positions.len() * 12;
    let indices_size = indices.len() * 4;
    let total_size = positions_size + indices_size;

    // make glTF
    let gltf_json = json!( {
        "asset": {
            "version": "2.0",
        },
        "scene": 0,
        "scenes": [
            {
                "nodes": [0],
            },
        ],
        "nodes": [
            {"mesh": 0},
        ],
        "meshes": [
            {
                "primitives": [
                    {
                        "attributes": {"POSITION": 0},
                        "indices": 1,
                        "mode": 4, // TRIANGLES
                    },
                ],
            },
        ],
        "buffers": [
            {
                "uri": "./output.bin",
                "byteLength": total_size,
            },
        ],
        "bufferViews": [
            {
                "buffer": 0,
                "byteOffset": 0,
                "byteLength": positions_size,
                "target": 34962,
            },
            {
                "buffer": 0,
                "byteOffset": positions_size,
                "byteLength": indices_size,
                "target": 34963,
            },
        ],
        "accessors": [
            {
                "bufferView": 0,
                "byteOffset": 0,
                "componentType": 5126, // FLOAT
                "count": positions.len(),
                "type": "VEC3",
                "min": [min_point[0], min_point[1], min_point[2]],
                "max": [max_point[0], max_point[1], max_point[2]],
            },
            {
                "bufferView": 1,
                "byteOffset": 0,
                "componentType": 5125, // UNSIGNED_INT
                "count": indices.len(),
                "type": "SCALAR",
            },
        ],
    });

    // gltfファイルを出力
    println!("write glTF");
    let mut gltf_file = File::create("output.gltf").unwrap();
    let _ = gltf_file.write_all(gltf_json.to_string().as_bytes());
}
