use nalgebra::{distance, ArrayStorage, Const, Matrix, Point3, Vector3};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Voxel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

fn draw_line(
    voxels: &mut HashSet<Voxel>,
    start: Matrix<f64, Const<3>, Const<1>, ArrayStorage<f64, 3, 1>>,
    end: Matrix<f64, Const<3>, Const<1>, ArrayStorage<f64, 3, 1>>,
    voxel_size: f64,
) {
    let start = start
        + Matrix::<f64, Const<3>, Const<1>, ArrayStorage<f64, 3, 1>>::repeat(voxel_size * 0.5);
    let end =
        end + Matrix::<f64, Const<3>, Const<1>, ArrayStorage<f64, 3, 1>>::repeat(voxel_size * 0.5);

    // 始点と終点が既知なので方向ベクトルが算出できる
    let direction = [end[0] - start[0], end[1] - start[1], end[2] - start[2]];
    // 方向ベクトルのXYZ方向の最大移動距離を取得
    // 移動距離なので、絶対値
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

    let mut current_voxel = start;
    // ステップの数だけ繰り返し、各ステップで通過するボクセルを計算
    for _ in 0..=steps {
        // ボクセルの座標計算
        // 現在の座標をボクセルのサイズで割り、切り捨てることでボクセルの格子座標（整数値）を算出
        let voxel = Voxel {
            x: (current_voxel[0] / voxel_size).floor() as i32,
            y: (current_voxel[1] / voxel_size).floor() as i32,
            z: (current_voxel[2] / voxel_size).floor() as i32,
        };
        println!(
            "fill {:?}/{:?}/{:?}",
            (current_voxel[0] / voxel_size).floor() as i32,
            (current_voxel[1] / voxel_size).floor() as i32,
            (current_voxel[2] / voxel_size).floor() as i32,
        );
        voxels.insert(voxel);
        // 現在の座標を更新
        current_voxel[0] += step_size[0];
        current_voxel[1] += step_size[1];
        current_voxel[2] += step_size[2];
    }
}

pub fn triangle_to_voxel(triangles: &[[f64; 3]], voxel_size: f64) -> HashSet<Voxel> {
    // 占有されたボクセルを格納する
    // HashSetは重複を許さない
    let mut occupied_voxels = HashSet::new();

    // indicesの要素を3つずつ取り出して三角形を構築
    for t in triangles.chunks(3) {
        fill_triangle(&mut occupied_voxels, voxel_size, t);
    }

    occupied_voxels
}

pub fn fill_triangle(voxels: &mut HashSet<Voxel>, voxel_size: f64, triangle: &[[f64; 3]]) {
    if triangle.len() != 3 {
        panic!("The number of vertices is not 3")
    }

    // 全ての三角形は反時計回りを表面とする
    let p1 = Point3::from(triangle[0]);
    let p2 = Point3::from(triangle[1]);
    let p3 = Point3::from(triangle[2]);

    // 3辺の長さを算出し、三角形が小さい（すべての辺がvoxel_size未満）場合は、面を走査せずvoxelを一つだけ塗りつぶす
    if is_small_triangle(&p1, &p2, &p3, voxel_size) {
        println!("Triangles too small!");

        let p1_floor = p1.map(|x| (x + 0.5).floor());
        let p2_floor = p2.map(|y| (y + 0.5).floor());
        let p3_floor = p3.map(|z| (z + 0.5).floor());

        voxels.insert(Voxel {
            x: p1_floor.x as i32,
            y: p1_floor.y as i32,
            z: p1_floor.z as i32,
        });
        voxels.insert(Voxel {
            x: p2_floor.x as i32,
            y: p2_floor.y as i32,
            z: p2_floor.z as i32,
        });
        voxels.insert(Voxel {
            x: p3_floor.x as i32,
            y: p3_floor.y as i32,
            z: p3_floor.z as i32,
        });
    }

    // p1からp2に伸びるベクトルと、p1からp3に伸びるベクトルを考える
    let v1 = p2 - p1;
    let v2 = p3 - p1;

    // 法線ベクトルを計算
    let mut normal = v1.cross(&v2);
    // 外積の計算結果はベクトルなので、その大きさ（ノルム）を求める
    let normal_length = normal.norm();

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
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap();

    // 三角形のbounding boxを算出
    let mut min_point = p1;
    let mut max_point = p1;
    for p in &[p2, p3] {
        for i in 0..3 {
            min_point[i] = min_point[i].min(p[i]);
            max_point[i] = max_point[i].max(p[i]);
        }
    }
    let box_size = max_point - min_point;
    println!("box_size: {:?}", box_size);

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
            let mut sorted_triangle = triangle.to_vec();
            sorted_triangle.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
            assert!(sorted_triangle[1][0] >= sorted_triangle[0][0]);
            println!(
                "p0: {:?}, p1: {:?}, p2: {:?}",
                sorted_triangle[0], sorted_triangle[1], sorted_triangle[2]
            );

            // 始点の移動に利用するためのベクトル
            // x軸方向にvoxel_size分だけ移動し、y方向にもvoxel_size分移動する
            let mut edge_direction_1;
            let mut start_point;
            let mut end_point;
            let mut end_vertex_x;

            // 走査軸が0（X軸）でX座標を基準にソートされていることがわかっているのでその前提で処理を進めることができる
            // 絶対値の差が0より大きい、つまり最初と2番目の頂点のx座標が異なる場合で、尚且つ最初の頂点と2番目の頂点のx座標の差がvoxel_sizeより大きい場合にTrueになる
            if (sorted_triangle[1][0] - sorted_triangle[0][0]).abs() > 0.0
                && (sorted_triangle[1][0] - sorted_triangle[0][0].floor() > voxel_size)
            {
                edge_direction_1 = (Vector3::from(sorted_triangle[1])
                    - Vector3::from(sorted_triangle[0]))
                    / (sorted_triangle[1][0] - sorted_triangle[0][0]);
                start_point = Vector3::from(sorted_triangle[0])
                    + edge_direction_1
                        * (voxel_size - sorted_triangle[0][0] + sorted_triangle[0][0].floor());
            } else {
                edge_direction_1 = (Vector3::from(sorted_triangle[2])
                    - Vector3::from(sorted_triangle[1]))
                    / (sorted_triangle[2][0] - sorted_triangle[1][0]);
                start_point = Vector3::from(sorted_triangle[1])
                    + edge_direction_1
                        * (voxel_size - sorted_triangle[1][0] + sorted_triangle[1][0].floor());
            };

            // 終点を移動させるためのベクトル
            // x軸方向にvoxel_size分だけ移動し、y方向にもvoxel_size分移動する
            let edge_direction_2 = (Vector3::from(sorted_triangle[2])
                - Vector3::from(sorted_triangle[0]))
                / (sorted_triangle[2][0] - sorted_triangle[0][0]);

            end_point = Vector3::from(sorted_triangle[0])
                + edge_direction_2
                    * (voxel_size - sorted_triangle[0][0] + sorted_triangle[0][0].floor());
            end_vertex_x = sorted_triangle[1][0];

            println!("edge_direction_1: {:?}", edge_direction_1);
            println!("edge_direction_2: {:?}", edge_direction_2);

            println!("start_point: {:?}, end_point: {:?}", start_point, end_point);

            if start_point.norm() > 1000.0 || end_point.norm() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_point[0] < sorted_triangle[2][0] {
                println!("axis 0 start: {:?}, end: {:?}", start_point, end_point);
                draw_line(voxels, start_point, end_point, voxel_size);

                start_point += edge_direction_1;
                end_point += edge_direction_2;

                if start_point[0] >= end_vertex_x {
                    end_vertex_x = start_point[0] - sorted_triangle[1][0];
                    start_point -= edge_direction_1 * end_vertex_x;
                    if (sorted_triangle[2][0] - sorted_triangle[1][0]).abs() < f64::EPSILON {
                        break;
                    }
                    edge_direction_1 = (Vector3::from(sorted_triangle[2])
                        - Vector3::from(sorted_triangle[1]))
                        / (sorted_triangle[2][0] - sorted_triangle[1][0]);
                    start_point += edge_direction_1 * end_vertex_x;
                    end_vertex_x = sorted_triangle[2][0];
                }
            }
        }
        // sweep y
        1 => {
            let mut sorted_triangle = triangle.to_vec();
            sorted_triangle.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
            assert!(sorted_triangle[1][1] >= sorted_triangle[0][1]);
            println!(
                "p0: {:?}, p1: {:?}, p2: {:?}",
                sorted_triangle[0], sorted_triangle[1], sorted_triangle[2]
            );

            let mut edge_direction_1;
            let mut start_point;
            let mut end_point;
            let mut end_vertex_y;

            if (sorted_triangle[1][1] - sorted_triangle[0][1]).abs() > 0.0
                && (sorted_triangle[1][1] - sorted_triangle[0][1].floor() > voxel_size)
            {
                edge_direction_1 = (Vector3::from(sorted_triangle[1])
                    - Vector3::from(sorted_triangle[0]))
                    / (sorted_triangle[1][1] - sorted_triangle[0][1]);
                start_point = Vector3::from(sorted_triangle[0])
                    + edge_direction_1
                        * (voxel_size - sorted_triangle[0][1] + sorted_triangle[0][1].floor());
            } else {
                edge_direction_1 = (Vector3::from(sorted_triangle[2])
                    - Vector3::from(sorted_triangle[1]))
                    / (sorted_triangle[2][1] - sorted_triangle[1][1]);
                start_point = Vector3::from(sorted_triangle[1])
                    + edge_direction_1
                        * (voxel_size - sorted_triangle[1][1] + sorted_triangle[1][1].floor());
            };

            let edge_direction_2 = (Vector3::from(sorted_triangle[2])
                - Vector3::from(sorted_triangle[0]))
                / (sorted_triangle[2][1] - sorted_triangle[0][1]);

            end_point = Vector3::from(sorted_triangle[0])
                + edge_direction_2
                    * (voxel_size - sorted_triangle[0][1] + sorted_triangle[0][1].floor());
            end_vertex_y = sorted_triangle[1][1];

            println!("edge_direction_1: {:?}", edge_direction_1);
            println!("edge_direction_2: {:?}", edge_direction_2);

            println!("start_point: {:?}, end_point: {:?}", start_point, end_point);

            if start_point.norm() > 1000.0 || end_point.norm() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_point[1] < sorted_triangle[2][1] {
                println!("axis 1 start: {:?}, end: {:?}", start_point, end_point);
                draw_line(voxels, start_point, end_point, voxel_size);

                start_point += edge_direction_1;
                end_point += edge_direction_2;

                if start_point[1] >= end_vertex_y {
                    end_vertex_y = start_point[1] - sorted_triangle[1][1];
                    start_point -= edge_direction_1 * end_vertex_y;
                    if (sorted_triangle[2][1] - sorted_triangle[1][1]).abs() < f64::EPSILON {
                        break;
                    }
                    edge_direction_1 = (Vector3::from(sorted_triangle[2])
                        - Vector3::from(sorted_triangle[1]))
                        / (sorted_triangle[2][1] - sorted_triangle[1][1]);
                    start_point += edge_direction_1 * end_vertex_y;
                    end_vertex_y = sorted_triangle[2][1];
                }
            }
        }
        // sweep z
        _ => {
            let mut sorted_triangle = triangle.to_vec();
            sorted_triangle.sort_by(|a, b| a[2].partial_cmp(&b[2]).unwrap());
            assert!(sorted_triangle[1][2] >= sorted_triangle[0][2]);
            println!(
                "p0: {:?}, p1: {:?}, p2: {:?}",
                sorted_triangle[0], sorted_triangle[1], sorted_triangle[2]
            );

            let mut edge_direction_1;
            let mut start_point;
            let mut end_point;
            let mut end_vertex_z;

            if (sorted_triangle[1][2] - sorted_triangle[0][2]).abs() > 0.0
                && (sorted_triangle[1][2] - sorted_triangle[0][2].floor() > voxel_size)
            {
                edge_direction_1 = (Vector3::from(sorted_triangle[1])
                    - Vector3::from(sorted_triangle[0]))
                    / (sorted_triangle[1][2] - sorted_triangle[0][2]);
                start_point = Vector3::from(sorted_triangle[0])
                    + edge_direction_1
                        * (voxel_size - sorted_triangle[0][2] + sorted_triangle[0][2].floor());
            } else {
                edge_direction_1 = (Vector3::from(sorted_triangle[2])
                    - Vector3::from(sorted_triangle[1]))
                    / (sorted_triangle[2][2] - sorted_triangle[1][2]);
                start_point = Vector3::from(sorted_triangle[1])
                    + edge_direction_1
                        * (voxel_size - sorted_triangle[1][2] + sorted_triangle[1][2].floor());
            };

            let edge_direction_2 = (Vector3::from(sorted_triangle[2])
                - Vector3::from(sorted_triangle[0]))
                / (sorted_triangle[2][2] - sorted_triangle[0][2]);

            end_point = Vector3::from(sorted_triangle[0])
                + edge_direction_2
                    * (voxel_size - sorted_triangle[0][2] + sorted_triangle[0][2].floor());
            end_vertex_z = sorted_triangle[1][2];

            println!("edge_direction_1: {:?}", edge_direction_1);
            println!("edge_direction_2: {:?}", edge_direction_2);

            println!("start_point: {:?}, end_point: {:?}", start_point, end_point);

            if start_point.norm() > 1000.0 || end_point.norm() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_point[2] < sorted_triangle[2][2] {
                println!("axis 2 start: {:?}, end: {:?}", start_point, end_point);
                draw_line(voxels, start_point, end_point, voxel_size);

                start_point += edge_direction_1;
                end_point += edge_direction_2;

                if start_point[2] >= end_vertex_z {
                    end_vertex_z = start_point[2] - sorted_triangle[1][2];
                    start_point -= edge_direction_1 * end_vertex_z;
                    if (sorted_triangle[2][2] - sorted_triangle[1][2]).abs() < f64::EPSILON {
                        break;
                    }
                    edge_direction_1 = (Vector3::from(sorted_triangle[2])
                        - Vector3::from(sorted_triangle[1]))
                        / (sorted_triangle[2][2] - sorted_triangle[1][2]);
                    start_point += edge_direction_1 * end_vertex_z;
                    end_vertex_z = sorted_triangle[2][2];
                }
            }
        }
    }
    println!();
}

pub fn is_small_triangle(p1: &Point3<f64>, p2: &Point3<f64>, p3: &Point3<f64>, size: f64) -> bool {
    let d12 = distance(p1, p2);
    let d23 = distance(p2, p3);
    let d31 = distance(p3, p1);

    d12 <= size && d23 <= size && d31 <= size
}
