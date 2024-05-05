use glam::Vec3;
use hashbrown::HashMap;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Voxel {
    pub color: [u8; 3],
}

pub type VoxelPosition = [i32; 3];

pub trait MeshVoxelizer {
    fn add_triangle(&mut self, triangle: &[[f32; 3]], voxel_size: f32);
    fn finalize(self) -> HashMap<VoxelPosition, Voxel>;
}

pub struct DdaVoxelizer {
    voxels: HashMap<VoxelPosition, Voxel>,
}

impl MeshVoxelizer for DdaVoxelizer {
    fn add_triangle(&mut self, triangle: &[[f32; 3]], voxel_size: f32) {
        fill_triangle(&mut self.voxels, voxel_size, triangle);
    }
    fn finalize(self) -> HashMap<VoxelPosition, Voxel> {
        self.voxels
    }
}

fn draw_line(voxels: &mut HashMap<VoxelPosition, Voxel>, start: Vec3, end: Vec3, voxel_size: f32) {
    let start = start + Vec3::splat(voxel_size * 0.5);
    let end = end + Vec3::splat(voxel_size * 0.5);

    // 始点と終点が既知なので方向ベクトルが算出できる
    let direction = end - start;
    // 方向ベクトルのXYZ方向の最大移動距離を取得
    // 移動距離なので、絶対値
    let max_dist = direction.abs().max_element();
    // 距離をボクセルサイズで割り、切り上げることでステップ数を算出
    // エッジを何ステップに分割するかを計算
    let steps = (max_dist / voxel_size).ceil() as i32;
    // XYZ方向へ1ステップで進む距離を算出
    let step_size = direction / steps as f32;

    let mut current_voxel = start;
    // ステップの数だけ繰り返し、各ステップで通過するボクセルを計算
    for _ in 0..=steps {
        // ボクセルの座標計算
        // 現在の座標をボクセルのサイズで割り、切り捨てることでボクセルの格子座標（整数値）を算出
        let position = (current_voxel / voxel_size).floor().as_ivec3();
        let voxel = Voxel {
            color: [255, 255, 255],
        };
        voxels.insert(position.to_array(), voxel);
        // 現在の座標を更新
        current_voxel += step_size;
    }
}

fn fill_triangle(
    voxels: &mut HashMap<VoxelPosition, Voxel>,
    voxel_size: f32,
    triangle: &[[f32; 3]],
) {
    if triangle.len() != 3 {
        panic!("The number of vertices is not 3")
    }

    // 全ての三角形は反時計回りを表面とする
    let p1 = Vec3::from(triangle[0]);
    let p2 = Vec3::from(triangle[1]);
    let p3 = Vec3::from(triangle[2]);

    // 3辺の長さを算出し、三角形が小さい（すべての辺がvoxel_size未満）場合は、面を走査せずvoxelを一つだけ塗りつぶす
    if is_small_triangle(&p1, &p2, &p3, voxel_size) {
        println!("Triangles too small!");

        let p1_floor = p1.floor();
        let p2_floor = p2.floor();
        let p3_floor = p3.floor();

        voxels.insert(
            [p1_floor.x as i32, p1_floor.y as i32, p1_floor.z as i32],
            Voxel {
                color: [255, 255, 255],
            },
        );
        voxels.insert(
            [p2_floor.x as i32, p2_floor.y as i32, p2_floor.z as i32],
            Voxel {
                color: [255, 255, 255],
            },
        );
        voxels.insert(
            [p3_floor.x as i32, p3_floor.y as i32, p3_floor.z as i32],
            Voxel {
                color: [255, 255, 255],
            },
        );
    }

    // p1からp2に伸びるベクトルと、p1からp3に伸びるベクトルを考える
    let v1 = p2 - p1;
    let v2 = p3 - p1;

    // 法線ベクトルを計算
    let mut normal = v1.cross(v2);
    // 外積の計算結果はベクトルなので、その大きさ（ノルム）を求める
    let normal_length = normal.length();

    if normal_length.is_nan() || normal_length == 0.0 {
        return;
    }

    // 正規化し、法線ベクトルを単位ベクトルに変換
    normal /= normal_length;

    // 最大長の軸を求める
    // norm_axisが0(x)の場合はyz平面
    // norm_axisが1(y)の場合はzx平面
    // norm_axisが2(z)の場合はxy平面
    let norm_axis = normal
        .abs()
        .to_array()
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap();

    // 三角形のbounding boxを算出
    let mut min_point = p1;
    let mut max_point = p1;
    for p in &[p2, p3] {
        min_point = min_point.min(*p);
        max_point = max_point.max(*p);
    }
    let box_size = max_point - min_point;

    // 一番長い辺を見つける
    let sweep_axis = match norm_axis {
        0 => {
            if box_size[1] >= box_size[2] {
                1
            } else {
                2
            }
        }
        1 => {
            if box_size[2] >= box_size[0] {
                2
            } else {
                0
            }
        }
        _ => {
            if box_size[0] >= box_size[1] {
                0
            } else {
                1
            }
        }
    };

    match sweep_axis {
        // x軸に沿って仮想的なグリッドを引き、対象の三角形とエッジとの交点を見つける
        // xが小さい交点から大きい交点に向かって塗っていく
        // sweep x
        0 => {
            // 3点のx座標を比較していき、もっともx座標が小さいものが最初に、大きいものが最後になるようにソート
            // つまり、頂点の順番が反時計回りになるようにソート
            let mut sorted_triangle = [triangle[0], triangle[1], triangle[2]];
            if sorted_triangle[0][0] > sorted_triangle[1][0] {
                sorted_triangle.swap(0, 1);
            }
            if sorted_triangle[1][0] > sorted_triangle[2][0] {
                sorted_triangle.swap(1, 2);
            }
            if sorted_triangle[0][0] > sorted_triangle[1][0] {
                sorted_triangle.swap(0, 1);
            }
            assert!(sorted_triangle[1][0] >= sorted_triangle[0][0]);

            let sorted_triangle: [[f32; 3]; 3] =
                sorted_triangle.map(|inner| inner.map(|x| x as f32));

            // 始点の移動に利用するためのベクトル
            // x軸方向にvoxel_size分だけ移動し、y方向にもvoxel_size分移動する
            let mut edge_direction_1;
            let mut start_point;
            let mut end_point;
            let mut end_vertex_x;

            // 走査軸が0（X軸）でX座標を基準にソートされていることがわかっているのでその前提で処理を進めることができる
            // 絶対値の差が0より大きい（つまり最初と2番目の頂点のx座標が異なる）場合で、尚且つ最初の頂点と2番目の頂点のx座標の差がvoxel_sizeより大きい場合にTrueになる
            if (sorted_triangle[1][0] - sorted_triangle[0][0]).abs() >= 0.0
                && (sorted_triangle[1][0] - sorted_triangle[0][0].floor() >= voxel_size)
            {
                edge_direction_1 = (Vec3::from(sorted_triangle[1])
                    - Vec3::from(sorted_triangle[0]))
                    / (sorted_triangle[1][0] - sorted_triangle[0][0]);
                start_point = Vec3::from(sorted_triangle[0])
                    * (voxel_size - sorted_triangle[0][0] + sorted_triangle[0][0].floor());
            } else {
                edge_direction_1 = (Vec3::from(sorted_triangle[2])
                    - Vec3::from(sorted_triangle[1]))
                    / (sorted_triangle[2][0] - sorted_triangle[1][0]);
                start_point = Vec3::from(sorted_triangle[1])
                    * (voxel_size - sorted_triangle[1][0] + sorted_triangle[1][0].floor());
            };

            // 終点を移動させるためのベクトル
            // x軸方向にvoxel_size分だけ移動し、y方向にもvoxel_size分移動する
            let edge_direction_2 = (Vec3::from(sorted_triangle[2])
                - Vec3::from(sorted_triangle[0]))
                / (sorted_triangle[2][0] - sorted_triangle[0][0]);

            end_point = Vec3::from(sorted_triangle[0])
                * (voxel_size - sorted_triangle[0][0] + sorted_triangle[0][0].floor());
            end_vertex_x = sorted_triangle[1][0];

            if start_point.length() > 1000.0 || end_point.length() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_point[0] <= sorted_triangle[2][0] {
                draw_line(voxels, start_point, end_point, voxel_size);

                start_point += edge_direction_1;
                end_point += edge_direction_2;

                if start_point[0] >= end_vertex_x {
                    end_vertex_x = start_point[0] - sorted_triangle[1][0];
                    start_point -= edge_direction_1 * end_vertex_x;
                    if (sorted_triangle[2][0] - sorted_triangle[1][0]).abs() == 0.0 {
                        continue;
                    }
                    edge_direction_1 = (Vec3::from(sorted_triangle[2])
                        - Vec3::from(sorted_triangle[1]))
                        / (sorted_triangle[2][0] - sorted_triangle[1][0]);
                    start_point += edge_direction_1 * end_vertex_x;
                    end_vertex_x = sorted_triangle[2][0];
                }
            }
        }
        // sweep y
        1 => {
            let mut sorted_triangle = [triangle[0], triangle[1], triangle[2]];
            if sorted_triangle[0][1] > sorted_triangle[1][1] {
                sorted_triangle.swap(0, 1);
            }
            if sorted_triangle[1][1] > sorted_triangle[2][1] {
                sorted_triangle.swap(1, 2);
            }
            if sorted_triangle[0][1] > sorted_triangle[1][1] {
                sorted_triangle.swap(0, 1);
            }
            assert!(sorted_triangle[1][1] >= sorted_triangle[0][1]);

            let sorted_triangle: [[f32; 3]; 3] =
                sorted_triangle.map(|inner| inner.map(|x| x as f32));

            let mut edge_direction_1;
            let mut start_point;
            let mut end_point;
            let mut end_vertex_y;

            if (sorted_triangle[1][1] - sorted_triangle[0][1]).abs() >= 0.0
                && (sorted_triangle[1][1] - sorted_triangle[0][1].floor() >= voxel_size)
            {
                edge_direction_1 = (Vec3::from(sorted_triangle[1])
                    - Vec3::from(sorted_triangle[0]))
                    / (sorted_triangle[1][1] - sorted_triangle[0][1]);
                start_point = Vec3::from(sorted_triangle[0])
                    * (voxel_size - sorted_triangle[0][1] + sorted_triangle[0][1].floor());
            } else {
                edge_direction_1 = (Vec3::from(sorted_triangle[2])
                    - Vec3::from(sorted_triangle[1]))
                    / (sorted_triangle[2][1] - sorted_triangle[1][1]);
                start_point = Vec3::from(sorted_triangle[1])
                    * (voxel_size - sorted_triangle[1][1] + sorted_triangle[1][1].floor());
            };

            let edge_direction_2 = (Vec3::from(sorted_triangle[2])
                - Vec3::from(sorted_triangle[0]))
                / (sorted_triangle[2][1] - sorted_triangle[0][1]);

            end_point = Vec3::from(sorted_triangle[0])
                * (voxel_size - sorted_triangle[0][1] + sorted_triangle[0][1].floor());
            end_vertex_y = sorted_triangle[1][1];

            if start_point.length() > 1000.0 || end_point.length() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_point[1] <= sorted_triangle[2][1] {
                draw_line(voxels, start_point, end_point, voxel_size);

                start_point += edge_direction_1;
                end_point += edge_direction_2;

                if start_point[1] >= end_vertex_y {
                    end_vertex_y = start_point[1] - sorted_triangle[1][1];
                    start_point -= edge_direction_1 * end_vertex_y;
                    if (sorted_triangle[2][1] - sorted_triangle[1][1]).abs() == 0.0 {
                        continue;
                    }
                    edge_direction_1 = (Vec3::from(sorted_triangle[2])
                        - Vec3::from(sorted_triangle[1]))
                        / (sorted_triangle[2][1] - sorted_triangle[1][1]);
                    start_point += edge_direction_1 * end_vertex_y;
                    end_vertex_y = sorted_triangle[2][1];
                }
            }
        }
        // sweep z
        _ => {
            let mut sorted_triangle = [triangle[0], triangle[1], triangle[2]];
            if sorted_triangle[0][2] > sorted_triangle[1][2] {
                sorted_triangle.swap(0, 1);
            }
            if sorted_triangle[1][2] > sorted_triangle[2][2] {
                sorted_triangle.swap(1, 2);
            }
            if sorted_triangle[0][2] > sorted_triangle[1][2] {
                sorted_triangle.swap(0, 1);
            }
            assert!(sorted_triangle[1][2] >= sorted_triangle[0][2]);

            let sorted_triangle: [[f32; 3]; 3] =
                sorted_triangle.map(|inner| inner.map(|x| x as f32));

            let mut edge_direction_1;
            let mut start_point;
            let mut end_point;
            let mut end_vertex_z;

            if (sorted_triangle[1][2] - sorted_triangle[0][2]).abs() >= 0.0
                && (sorted_triangle[1][2] - sorted_triangle[0][2].floor() >= voxel_size)
            {
                edge_direction_1 = (Vec3::from(sorted_triangle[1])
                    - Vec3::from(sorted_triangle[0]))
                    / (sorted_triangle[1][2] - sorted_triangle[0][2]);
                start_point = Vec3::from(sorted_triangle[0])
                    * (voxel_size - sorted_triangle[0][2] + sorted_triangle[0][2].floor());
            } else {
                edge_direction_1 = (Vec3::from(sorted_triangle[2])
                    - Vec3::from(sorted_triangle[1]))
                    / (sorted_triangle[2][2] - sorted_triangle[1][2]);
                start_point = Vec3::from(sorted_triangle[1])
                    * (voxel_size - sorted_triangle[1][2] + sorted_triangle[1][2].floor());
            };

            let edge_direction_2 = (Vec3::from(sorted_triangle[2])
                - Vec3::from(sorted_triangle[0]))
                / (sorted_triangle[2][2] - sorted_triangle[0][2]);

            end_point = Vec3::from(sorted_triangle[0])
                * (voxel_size - sorted_triangle[0][2] + sorted_triangle[0][2].floor());
            end_vertex_z = sorted_triangle[1][2];

            if start_point.length() > 1000.0 || end_point.length() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_point[2] <= sorted_triangle[2][2] {
                draw_line(voxels, start_point, end_point, voxel_size);

                start_point += edge_direction_1;
                end_point += edge_direction_2;

                if start_point[2] >= end_vertex_z {
                    end_vertex_z = start_point[2] - sorted_triangle[1][2];
                    start_point -= edge_direction_1 * end_vertex_z;
                    if (sorted_triangle[2][2] - sorted_triangle[1][2]).abs() == 0.0 {
                        continue;
                    }
                    edge_direction_1 = (Vec3::from(sorted_triangle[2])
                        - Vec3::from(sorted_triangle[1]))
                        / (sorted_triangle[2][2] - sorted_triangle[1][2]);
                    start_point += edge_direction_1 * end_vertex_z;
                    end_vertex_z = sorted_triangle[2][2];
                }
            }
        }
    }
}

fn is_small_triangle(p1: &Vec3, p2: &Vec3, p3: &Vec3, size: f32) -> bool {
    let d12 = p1.distance(*p2);
    let d23 = p2.distance(*p3);
    let d31 = p3.distance(*p1);

    d12 <= size && d23 <= size && d31 <= size
}

#[cfg(test)]
mod tests {
    use super::*;
    use earcut::{utils3d::project3d_to_2d, Earcut};
    use nusamai_geometry::MultiPolygon;

    #[test]
    fn test_minimum_polygon() {
        let vertices: Vec<[f64; 3]> = vec![
            [0.0, 0.0, 0.0],
            [0.1, 0.0, 0.0],
            [0.1, 0.1, 0.0],
            [0.0, 0.1, 0.0],
        ];

        let mut mpoly = MultiPolygon::<u32>::new();

        mpoly.add_exterior([0, 1, 2, 3, 0]);

        let mut earcutter = Earcut::new();
        let mut buf3d: Vec<[f64; 3]> = Vec::new();
        let mut buf2d: Vec<[f64; 2]> = Vec::new();
        let mut index_buf: Vec<u32> = Vec::new();
        let mut triangles: Vec<[f64; 3]> = Vec::new();

        for idx_poly in mpoly.iter() {
            let poly = idx_poly.transform(|idx| vertices[*idx as usize]);
            let num_outer = match poly.hole_indices().first() {
                Some(&v) => v as usize,
                None => poly.raw_coords().len(),
            };

            buf3d.clear();
            buf3d.extend(poly.raw_coords().iter());

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                earcutter.earcut(buf2d.iter().cloned(), poly.hole_indices(), &mut index_buf);
                triangles.extend(index_buf.iter().map(|&idx| buf3d[idx as usize]));
            }
        }

        let voxel_size = 1.0 as f32;

        let mut voxelizer = DdaVoxelizer {
            voxels: HashMap::new(),
        };
        let triangles: Vec<[f32; 3]> = triangles
            .into_iter()
            .map(|arr| [arr[0] as f32, arr[1] as f32, arr[2] as f32])
            .collect();
        for t in triangles.chunks(3) {
            voxelizer.add_triangle(t, voxel_size);
        }
        let occupied_voxels = voxelizer.finalize();

        let mut test_voxels: HashMap<VoxelPosition, Voxel> = HashMap::new();
        test_voxels.insert(
            [0, 0, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );

        assert_eq!(occupied_voxels, test_voxels);
    }

    #[test]
    fn test_square_polygon() {
        let vertices: Vec<[f64; 3]> = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];

        let mut mpoly = MultiPolygon::<u32>::new();

        mpoly.add_exterior([0, 1, 2, 3, 0]);

        let mut earcutter = Earcut::new();
        let mut buf3d: Vec<[f64; 3]> = Vec::new();
        let mut buf2d: Vec<[f64; 2]> = Vec::new();
        let mut index_buf: Vec<u32> = Vec::new();
        let mut triangles: Vec<[f64; 3]> = Vec::new();

        for idx_poly in mpoly.iter() {
            let poly = idx_poly.transform(|idx| vertices[*idx as usize]);
            let num_outer = match poly.hole_indices().first() {
                Some(&v) => v as usize,
                None => poly.raw_coords().len(),
            };

            buf3d.clear();
            buf3d.extend(poly.raw_coords().iter());

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                earcutter.earcut(buf2d.iter().cloned(), poly.hole_indices(), &mut index_buf);
                triangles.extend(index_buf.iter().map(|&idx| buf3d[idx as usize]));
            }
        }

        let voxel_size = 1.0;

        let mut voxelizer = DdaVoxelizer {
            voxels: HashMap::new(),
        };
        let triangles: Vec<[f32; 3]> = triangles
            .into_iter()
            .map(|arr| [arr[0] as f32, arr[1] as f32, arr[2] as f32])
            .collect();
        for t in triangles.chunks(3) {
            voxelizer.add_triangle(t, voxel_size);
        }
        let occupied_voxels = voxelizer.finalize();

        let mut test_voxels: HashMap<VoxelPosition, Voxel> = HashMap::new();
        test_voxels.insert(
            [0, 0, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [1, 0, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [0, 1, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [1, 1, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );

        assert_eq!(occupied_voxels, test_voxels);
    }

    #[test]
    fn test_hole_polygon() {
        // holeの大きさがvoxel_sizeと同じ場合、holeが埋まる
        let vertices: Vec<[f64; 3]> = vec![
            [0.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [3.0, 3.0, 0.0],
            [0.0, 3.0, 0.0],
            [1.0, 1.0, 0.0],
            [2.0, 1.0, 0.0],
            [2.0, 2.0, 0.0],
            [1.0, 2.0, 0.0],
        ];

        let mut mpoly = MultiPolygon::<u32>::new();

        mpoly.add_exterior([0, 1, 2, 3, 0]);
        mpoly.add_interior([4, 5, 6, 7, 4]);

        let mut earcutter = Earcut::new();
        let mut buf3d: Vec<[f64; 3]> = Vec::new();
        let mut buf2d: Vec<[f64; 2]> = Vec::new();
        let mut index_buf: Vec<u32> = Vec::new();
        let mut triangles: Vec<[f64; 3]> = Vec::new();

        for idx_poly in mpoly.iter() {
            let poly = idx_poly.transform(|idx| vertices[*idx as usize]);
            let num_outer = match poly.hole_indices().first() {
                Some(&v) => v as usize,
                None => poly.raw_coords().len(),
            };

            buf3d.clear();
            buf3d.extend(poly.raw_coords().iter());

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                earcutter.earcut(buf2d.iter().cloned(), poly.hole_indices(), &mut index_buf);
                triangles.extend(index_buf.iter().map(|&idx| buf3d[idx as usize]));
            }
        }

        let voxel_size = 1.0;

        let mut voxelizer = DdaVoxelizer {
            voxels: HashMap::new(),
        };
        let triangles: Vec<[f32; 3]> = triangles
            .into_iter()
            .map(|arr| [arr[0] as f32, arr[1] as f32, arr[2] as f32])
            .collect();
        for t in triangles.chunks(3) {
            voxelizer.add_triangle(t, voxel_size);
        }
        let occupied_voxels = voxelizer.finalize();

        let mut test_voxels: HashMap<VoxelPosition, Voxel> = HashMap::new();
        test_voxels.insert(
            [0, 0, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [1, 0, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [2, 0, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [3, 0, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [0, 1, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [1, 1, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [2, 1, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [3, 1, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [0, 2, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [1, 2, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [2, 2, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [3, 2, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [0, 3, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [1, 3, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [2, 3, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );
        test_voxels.insert(
            [3, 3, 0],
            Voxel {
                color: [255, 255, 255],
            },
        );

        assert_eq!(occupied_voxels, test_voxels);
    }

    #[test]
    fn test_cube() {
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
            [3.0, 3.0, 0.0],
            [7.0, 3.0, 0.0],
            [7.0, 7.0, 0.0],
            [3.0, 7.0, 0.0],
            [3.0, 3.0, 10.0],
            [7.0, 3.0, 10.0],
            [7.0, 7.0, 10.0],
            [3.0, 7.0, 10.0],
        ];

        let mut mpoly = MultiPolygon::<u32>::new();

        // index
        // 1st polygon
        mpoly.add_exterior([0, 1, 2, 3, 0]);
        mpoly.add_interior([8, 9, 10, 11, 8]);
        // 2nd polygon
        mpoly.add_exterior([4, 5, 6, 7, 4]);
        mpoly.add_interior([12, 13, 14, 15, 12]);
        // 3rd polygon
        mpoly.add_exterior([0, 1, 5, 4, 0]);
        // 4th polygon
        mpoly.add_exterior([1, 2, 6, 5, 1]);
        // 6th polygon
        mpoly.add_exterior([2, 3, 7, 6, 2]);
        // 6th polygon
        mpoly.add_exterior([3, 0, 4, 7, 3]);

        let mut earcutter = Earcut::new();
        let mut buf3d: Vec<[f64; 3]> = Vec::new();
        let mut buf2d: Vec<[f64; 2]> = Vec::new();
        let mut index_buf: Vec<u32> = Vec::new();
        let mut triangles: Vec<[f64; 3]> = Vec::new();

        for idx_poly in mpoly.iter() {
            let poly = idx_poly.transform(|idx| vertices[*idx as usize]);
            let num_outer = match poly.hole_indices().first() {
                Some(&v) => v as usize,
                None => poly.raw_coords().len(),
            };

            buf3d.clear();
            buf3d.extend(poly.raw_coords().iter());

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                earcutter.earcut(buf2d.iter().cloned(), poly.hole_indices(), &mut index_buf);
                triangles.extend(index_buf.iter().map(|&idx| buf3d[idx as usize]));
            }
        }

        let voxel_size = 1.0;

        let mut voxelizer = DdaVoxelizer {
            voxels: HashMap::new(),
        };
        let triangles: Vec<[f32; 3]> = triangles
            .into_iter()
            .map(|arr| [arr[0] as f32, arr[1] as f32, arr[2] as f32])
            .collect();
        for t in triangles.chunks(3) {
            voxelizer.add_triangle(t, voxel_size);
        }
        let occupied_voxels = voxelizer.finalize();

        assert_eq!(occupied_voxels.len(), 584);
    }
}
