use byteorder::{LittleEndian, WriteBytesExt};
use nusamai_geometry::MultiPolygon;
use serde_json::json;
use std::{collections::HashSet, fs::File, io::Write};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Voxel {
    x: i32,
    y: i32,
    z: i32,
}

fn voxelize_mesh(vertices: &[[f64; 3]], indices: &[u32], voxel_size: f64) -> HashSet<Voxel> {
    // 占有されたボクセルを格納する
    // HashSetは重複を許さない
    let mut occupied_voxels = HashSet::new();

    // indicesの要素を2つずつ取り出すのがwindows(2)メソッド
    for window in indices.windows(2) {
        // 隣り合った2つの頂点を取り出し、これを線分（エッジ）の始点と終点とする
        let start = vertices[window[0] as usize];
        let end = vertices[window[1] as usize];
        let mut current = start;

        // 方向ベクトルの算出
        let direction = [end[0] - start[0], end[1] - start[1], end[2] - start[2]];
        // 方向ベクトルの最大距離を取得
        let max_dist = direction
            .iter()
            .fold(0.0_f64, |acc, &val| acc.max(val.abs()));
        // 距離をボクセルサイズで割り、切り上げることでステップ数を算出
        // エッジを何ステップに分割するかを計算
        let steps = (max_dist / voxel_size).ceil() as i32;
        // XYZ方向へ1ステップで進む距離を算出
        let step_size = [
            direction[0] / steps as f64,
            direction[1] / steps as f64,
            direction[2] / steps as f64,
        ];

        // ステップの数だけ繰り返し、各ステップで通過するボクセルを計算
        for _ in 0..=steps {
            // ボクセルの座標計算
            // 現在の座標をボクセルのサイズで割り、切り捨てることでボクセルの格子座標（整数値）を算出
            let voxel = Voxel {
                x: (current[0] / voxel_size).floor() as i32,
                y: (current[1] / voxel_size).floor() as i32,
                z: (current[2] / voxel_size).floor() as i32,
            };
            occupied_voxels.insert(voxel);
            // 現在の座標を更新
            current[0] += step_size[0];
            current[1] += step_size[1];
            current[2] += step_size[2];
        }
    }

    occupied_voxels
}

fn indexed_multipolygon_to_voxel(
    vertices: &[[f64; 3]],
    mpoly_idx: &MultiPolygon<u32>,
    voxel_size: f64,
) -> HashSet<Voxel> {
    let mut occupied_voxels = HashSet::new();

    for poly_idx in mpoly_idx.iter() {
        let mut indices = Vec::new();
        for ring_idx in poly_idx.rings() {
            indices.extend(ring_idx.iter());
        }

        let poly_occupied_voxels = voxelize_mesh(vertices, &indices, voxel_size);
        occupied_voxels.extend(poly_occupied_voxels);
    }

    occupied_voxels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxelize() {
        let vertices: Vec<[f64; 3]> = vec![
            // 1st polygon, exterior (vertex 0~3)
            [0., 0., 11.],
            [5., 0., 11.],
            [5., 5., 11.],
            [0., 5., 11.],
            // 1st polygon, interior 1 (vertex 4~7)
            [1., 1., 11.],
            [2., 1., 11.],
            [2., 2., 11.],
            [1., 2., 11.],
            // 1st polygon, interior 2 (vertex 8~11)
            [3., 3., 11.],
            [4., 3., 11.],
            [4., 4., 11.],
            [3., 4., 11.],
            // 2nd polygon, exterior (vertex 12~15)
            [4., 0., 22.],
            [7., 0., 22.],
            [7., 3., 22.],
            [4., 3., 22.],
            // 2nd polygon, interior (vertex 16~19)
            [5., 1., 22.],
            [6., 1., 22.],
            [6., 2., 22.],
            [5., 2., 22.],
            // 3rd polygon, exterior (vertex 20~23)
            [4., 0., 33.],
            [7., 0., 33.],
            [7., 3., 33.],
            [4., 3., 33.],
        ];

        let mut mpoly = MultiPolygon::<u32>::new();
        // 1st polygon
        mpoly.add_exterior([0, 1, 2, 3, 0]);
        mpoly.add_interior([4, 5, 6, 7, 4]);
        mpoly.add_interior([8, 9, 10, 11, 8]);
        // 2nd polygon
        mpoly.add_exterior([12, 13, 14, 15, 12]);
        mpoly.add_interior([16, 17, 18, 19, 16]);
        // 3rd polygon
        mpoly.add_exterior([20, 21, 22, 23, 20]);

        let voxel_size = 1.0;

        let occupied_voxels = indexed_multipolygon_to_voxel(&vertices, &mpoly, voxel_size);
        let points_count = occupied_voxels.len();

        // gltfの作成
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
}
