use byteorder::{LittleEndian, WriteBytesExt};
use earcut::{utils3d::project3d_to_2d, Earcut};
use nusamai_geometry::MultiPolygon;
use serde_json::json;
use std::{fs::File, io::Write};

use nusamai_voxelize::voxelize::triangle_to_voxel;

fn main() {
    let vertices: Vec<[f64; 3]> = vec![
        // exterior
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0],
        [10.0, 10.0, 0.0],
        [0.0, 10.0, 0.0],
        [0.0, 0.0, 10.0],
        [10.0, 0.0, 10.0],
        [10.0, 10.0, 10.0],
        [0.0, 10.0, 10.0],
        // interior
        [1.0, 1.0, 1.0],
        [5.0, 1.0, 1.0],
        [5.0, 5.0, 1.0],
        [1.0, 5.0, 1.0],
        [1.0, 1.0, 5.0],
        [5.0, 1.0, 5.0],
        [5.0, 5.0, 5.0],
        [1.0, 5.0, 5.0],
    ];

    let mut mpoly = MultiPolygon::<u32>::new();

    // インデックス
    mpoly.add_exterior([0, 1, 2, 3, 0]);
    mpoly.add_exterior([4, 5, 6, 7, 4]);
    mpoly.add_exterior([0, 1, 5, 4, 0]);
    mpoly.add_exterior([1, 2, 6, 5, 1]);
    mpoly.add_exterior([2, 3, 7, 6, 2]);
    mpoly.add_exterior([3, 0, 4, 7, 3]);
    mpoly.add_interior([8, 5, 10, 11, 8]);
    mpoly.add_interior([12, 13, 14, 15, 12]);
    mpoly.add_interior([8, 5, 13, 12, 8]);
    mpoly.add_interior([5, 10, 14, 13, 5]);
    mpoly.add_interior([10, 11, 15, 14, 10]);
    mpoly.add_interior([11, 8, 12, 15, 11]);

    // triangulation
    let mut earcutter = Earcut::new();
    let mut buf3d: Vec<[f64; 3]> = Vec::new();
    let mut buf2d: Vec<[f64; 2]> = Vec::new();
    let mut index_buf: Vec<u32> = Vec::new();

    let mut triangles: Vec<[f64; 3]> = Vec::new();

    // ポリゴンを取り出す
    for idx_poly in mpoly.iter() {
        // インデックスを座標に変換
        let poly = idx_poly.transform(|idx| vertices[*idx as usize]);
        // holeがあるか確認
        let num_outer = match poly.hole_indices().first() {
            Some(&v) => v as usize,
            None => poly.raw_coords().len(),
        };

        // 3次元での座標を格納
        buf3d.clear();
        buf3d.extend(poly.raw_coords().iter());

        // 3次元座標を2次元座標に変換
        if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
            // earcut
            earcutter.earcut(buf2d.iter().cloned(), poly.hole_indices(), &mut index_buf);
            triangles.extend(index_buf.iter().map(|&idx| buf3d[idx as usize]));
        }
    }

    let voxel_size = 1.0;

    let occupied_voxels = triangle_to_voxel(&triangles, voxel_size);
    let points_count = occupied_voxels.len();

    // -------------------gltfの作成-------------------

    // voxelは整数値だが、accessorsのcomponentTypeは5126（浮動小数点数）であり、primitivesの制約でINTEGER型は使用できない
    let mut min_point = [f32::MAX; 3];
    let mut max_point = [f32::MIN; 3];

    let mut bin_file = File::create("data/output.bin").unwrap();
    for v in occupied_voxels.iter() {
        let [x, y, z] = [
            (v.x as f32) * voxel_size as f32,
            (v.y as f32) * voxel_size as f32,
            (v.z as f32) * voxel_size as f32,
        ];
        min_point = [
            f32::min(min_point[0], x),
            f32::min(min_point[1], y),
            f32::min(min_point[2], z),
        ];
        max_point = [
            f32::max(max_point[0], x),
            f32::max(max_point[1], y),
            f32::max(max_point[2], z),
        ];

        bin_file.write_f32::<LittleEndian>(x).unwrap();
        bin_file.write_f32::<LittleEndian>(y).unwrap();
        bin_file.write_f32::<LittleEndian>(z).unwrap();
    }

    // voxelの数 × 頂点の座標数（3） × 4バイト（f32）
    let byte_length = points_count * 3 * 4;

    // GLTFファイルの作成
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
                        "mode": 0,
                    },
                ],
            },
        ],
        "buffers": [
            {
                "uri": "./output.bin",
                "byteLength": byte_length,
            },
        ],
        "bufferViews": [
            {
                "buffer": 0,
                "byteOffset": 0,
                "byteLength": byte_length,
                "target": 34962,
            },
        ],
        "accessors": [
            {
                "bufferView": 0,
                "byteOffset": 0,
                "componentType": 5126,
                "count": points_count,
                "type": "VEC3",
                "min": [min_point[0], min_point[1], min_point[2]],
                "max": [max_point[0], max_point[1], max_point[2]],
            },
        ],
    });

    // gltfファイルを出力
    let mut gltf_file = File::create("data/output.gltf").unwrap();
    let _ = gltf_file.write_all(gltf_json.to_string().as_bytes());
}
