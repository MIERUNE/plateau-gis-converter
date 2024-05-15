use glam::Vec3;
use hashbrown::HashMap;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Voxel {
    pub color: [u8; 3],
}

pub type VoxelPosition = [i32; 3];

pub trait MeshVoxelizer {
    fn add_triangle(&mut self, triangle: &[[f32; 3]; 3]);
    fn finalize(self) -> HashMap<VoxelPosition, Voxel>;
}

pub struct DdaVoxelizer {
    pub voxels: HashMap<VoxelPosition, Voxel>,
}

impl MeshVoxelizer for DdaVoxelizer {
    fn add_triangle(&mut self, triangle: &[[f32; 3]; 3]) {
        fill_triangle(&mut self.voxels, triangle);
    }
    fn finalize(self) -> HashMap<VoxelPosition, Voxel> {
        self.voxels
    }
}

fn draw_line(voxels: &mut HashMap<VoxelPosition, Voxel>, start: Vec3, end: Vec3) {
    let start = start + Vec3::splat(0.5);
    let end = end + Vec3::splat(0.5);
    let direction = end - start;
    let max_dist = direction.abs().max_element();
    let steps = max_dist.ceil() as i32;
    let step_size = direction / steps as f32;

    let mut current_voxel = start;
    for _ in 0..=steps {
        let position = current_voxel.as_ivec3();
        let voxel = Voxel {
            color: [255, 255, 255],
        };
        voxels.insert(position.to_array(), voxel);
        current_voxel += step_size;
    }
}

fn fill_triangle(voxels: &mut HashMap<VoxelPosition, Voxel>, triangle: &[[f32; 3]; 3]) {
    let p1 = Vec3::from(triangle[0]);
    let p2 = Vec3::from(triangle[1]);
    let p3 = Vec3::from(triangle[2]);

    // Calculate the lengths of the 3 sides
    // and if the triangle is small (all sides are less than voxel_size),
    // do not scan the face and fill one voxel
    if is_small_triangle(&p1, &p2, &p3) {
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

        return;
    }

    let mut normal = (p2 - p1).cross(p3 - p1);
    let normal_length = normal.length();
    if normal_length.is_nan() || normal_length == 0.0 {
        return;
    }
    normal /= normal_length;

    // Find the axis of maximum length
    // yz plane if norm_axis is 0(x)
    // zx plane if norm_axis is 1(y)
    // xy plane if norm_axis is 2(z)
    let norm_axis = normal
        .abs()
        .to_array()
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap();

    let mut min_point = p1;
    let mut max_point = p1;
    for p in &[p2, p3] {
        min_point = min_point.min(*p);
        max_point = max_point.max(*p);
    }
    let box_size = max_point - min_point;

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

    // Draw a virtual grid along the x-axis and find the intersection of the target triangle and the edge
    // Paint from the intersection with smaller x to the intersection with larger x
    match sweep_axis {
        // sweep x
        0 => {
            let mut ordered_verts = [triangle[0], triangle[1], triangle[2]];
            if ordered_verts[0][0] > ordered_verts[1][0] {
                ordered_verts.swap(0, 1);
            }
            if ordered_verts[1][0] > ordered_verts[2][0] {
                ordered_verts.swap(1, 2);
            }
            if ordered_verts[0][0] > ordered_verts[1][0] {
                ordered_verts.swap(0, 1);
            }
            debug_assert!(ordered_verts[1][0] >= ordered_verts[0][0]);

            let ordered_verts: [[f32; 3]; 3] = ordered_verts.map(|inner| inner.map(|x| x));

            let mut start_step;
            let mut start_pos;

            if (ordered_verts[1][0] - ordered_verts[0][0]).abs() >= 0.0
                && (ordered_verts[1][0] - ordered_verts[0][0].floor() >= 1.0)
            {
                start_step = (Vec3::from(ordered_verts[1]) - Vec3::from(ordered_verts[0]))
                    / (ordered_verts[1][0] - ordered_verts[0][0]);
                start_pos = Vec3::from(ordered_verts[0])
                    + start_step
                        * ((1.0 - ordered_verts[0][0] + ordered_verts[0][0].floor()) % 1.0);
            } else {
                start_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[1]))
                    / (ordered_verts[2][0] - ordered_verts[1][0]);
                start_pos = Vec3::from(ordered_verts[1])
                    + start_step
                        * ((1.0 - ordered_verts[1][0] + ordered_verts[1][0].floor()) % 1.0);
            };

            let end_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[0]))
                / (ordered_verts[2][0] - ordered_verts[0][0]);
            let mut end_pos = Vec3::from(ordered_verts[0])
                + end_step * ((1.0 - ordered_verts[0][0] + ordered_verts[0][0].floor()) % 1.0);

            let mut end_vertex_x = ordered_verts[1][0];

            if start_step.length() > 1000.0 || end_step.length() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_pos[0] <= ordered_verts[2][0] {
                draw_line(voxels, start_pos, end_pos);

                start_pos += start_step;
                end_pos += end_step;

                if start_pos[0] >= end_vertex_x {
                    end_vertex_x = start_pos[0] - ordered_verts[1][0];
                    start_pos -= start_step * end_vertex_x;
                    if (ordered_verts[2][0] - ordered_verts[1][0]).abs() == 0.0 {
                        continue;
                    }
                    start_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[1]))
                        / (ordered_verts[2][0] - ordered_verts[1][0]);
                    start_pos += start_step * end_vertex_x;
                    end_vertex_x = ordered_verts[2][0];
                }
            }
        }
        // sweep y
        1 => {
            let mut ordered_verts = [triangle[0], triangle[1], triangle[2]];
            if ordered_verts[0][1] > ordered_verts[1][1] {
                ordered_verts.swap(0, 1);
            }
            if ordered_verts[1][1] > ordered_verts[2][1] {
                ordered_verts.swap(1, 2);
            }
            if ordered_verts[0][1] > ordered_verts[1][1] {
                ordered_verts.swap(0, 1);
            }
            debug_assert!(ordered_verts[1][1] >= ordered_verts[0][1]);

            let ordered_verts: [[f32; 3]; 3] = ordered_verts.map(|inner| inner.map(|x| x));

            let mut start_step;
            let mut start_pos;

            if (ordered_verts[1][1] - ordered_verts[0][1]).abs() >= 0.0
                && (ordered_verts[1][1] - ordered_verts[0][1].floor() >= 1.0)
            {
                start_step = (Vec3::from(ordered_verts[1]) - Vec3::from(ordered_verts[0]))
                    / (ordered_verts[1][1] - ordered_verts[0][1]);
                start_pos = Vec3::from(ordered_verts[0])
                    + start_step
                        * ((1.0 - ordered_verts[0][1] + ordered_verts[0][1].floor()) % 1.0);
            } else {
                start_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[1]))
                    / (ordered_verts[2][1] - ordered_verts[1][1]);
                start_pos = Vec3::from(ordered_verts[1])
                    + start_step
                        * ((1.0 - ordered_verts[1][1] + ordered_verts[1][1].floor()) % 1.0);
            };

            let end_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[0]))
                / (ordered_verts[2][1] - ordered_verts[0][1]);
            let mut end_pos = Vec3::from(ordered_verts[0])
                + end_step * ((1.0 - ordered_verts[0][1] + ordered_verts[0][1].floor()) % 1.0);

            let mut end_vertex_y = ordered_verts[1][1];

            if start_step.length() > 1000.0 || end_step.length() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_pos[1] <= ordered_verts[2][1] {
                draw_line(voxels, start_pos, end_pos);

                start_pos += start_step;
                end_pos += end_step;

                if start_pos[1] >= end_vertex_y {
                    end_vertex_y = start_pos[1] - ordered_verts[1][1];
                    start_pos -= start_step * end_vertex_y;
                    if (ordered_verts[2][1] - ordered_verts[1][1]).abs() == 0.0 {
                        continue;
                    }
                    start_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[1]))
                        / (ordered_verts[2][1] - ordered_verts[1][1]);
                    start_pos += start_step * end_vertex_y;
                    end_vertex_y = ordered_verts[2][1];
                }
            }
        }
        // sweep z
        _ => {
            let mut ordered_verts = [triangle[0], triangle[1], triangle[2]];
            if ordered_verts[0][2] > ordered_verts[1][2] {
                ordered_verts.swap(0, 1);
            }
            if ordered_verts[1][2] > ordered_verts[2][2] {
                ordered_verts.swap(1, 2);
            }
            if ordered_verts[0][2] > ordered_verts[1][2] {
                ordered_verts.swap(0, 1);
            }
            debug_assert!(ordered_verts[1][2] >= ordered_verts[0][2]);

            let ordered_verts: [[f32; 3]; 3] = ordered_verts.map(|inner| inner.map(|x| x));

            let mut start_step;
            let mut start_pos;

            if (ordered_verts[1][2] - ordered_verts[0][2]).abs() >= 0.0
                && (ordered_verts[1][2] - ordered_verts[0][2].floor() >= 1.0)
            {
                start_step = (Vec3::from(ordered_verts[1]) - Vec3::from(ordered_verts[0]))
                    / (ordered_verts[1][2] - ordered_verts[0][2]);
                start_pos = Vec3::from(ordered_verts[0])
                    + start_step
                        * ((1.0 - ordered_verts[0][2] + ordered_verts[0][2].floor()) % 1.0);
            } else {
                start_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[1]))
                    / (ordered_verts[2][2] - ordered_verts[1][2]);
                start_pos = Vec3::from(ordered_verts[1])
                    + start_step
                        * ((1.0 - ordered_verts[1][2] + ordered_verts[1][2].floor()) % 1.0);
            };

            let end_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[0]))
                / (ordered_verts[2][2] - ordered_verts[0][2]);
            let mut end_pos = Vec3::from(ordered_verts[0])
                + end_step * ((1.0 - ordered_verts[0][2] + ordered_verts[0][2].floor()) % 1.0);

            let mut end_vertex_z = ordered_verts[1][2];

            if start_step.length() > 1000.0 || end_step.length() > 1000.0 {
                println!("Direction vector magnitude is too large");
                return;
            }

            while end_pos[2] <= ordered_verts[2][2] {
                draw_line(voxels, start_pos, end_pos);

                start_pos += start_step;
                end_pos += end_step;

                if start_pos[2] >= end_vertex_z {
                    end_vertex_z = start_pos[2] - ordered_verts[1][2];
                    start_pos -= start_step * end_vertex_z;
                    if (ordered_verts[2][2] - ordered_verts[1][2]).abs() == 0.0 {
                        continue;
                    }
                    start_step = (Vec3::from(ordered_verts[2]) - Vec3::from(ordered_verts[1]))
                        / (ordered_verts[2][2] - ordered_verts[1][2]);
                    start_pos += start_step * end_vertex_z;
                    end_vertex_z = ordered_verts[2][2];
                }
            }
        }
    }
}

fn is_small_triangle(p1: &Vec3, p2: &Vec3, p3: &Vec3) -> bool {
    let d12 = p1.distance(*p2);
    let d23 = p2.distance(*p3);
    let d31 = p3.distance(*p1);

    d12 <= 1.0 && d23 <= 1.0 && d31 <= 1.0
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

            buf3d.clear();
            buf3d.extend(
                poly.raw_coords()
                    .iter()
                    .map(|v| [v[0] as f32, v[1] as f32, v[2] as f32]),
            );

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
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

            buf3d.clear();
            buf3d.extend(
                poly.raw_coords()
                    .iter()
                    .map(|v| [v[0] as f32, v[1] as f32, v[2] as f32]),
            );

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
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

            buf3d.clear();
            buf3d.extend(
                poly.raw_coords()
                    .iter()
                    .map(|v| [v[0] as f32, v[1] as f32, v[2] as f32]),
            );

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
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

            buf3d.clear();
            buf3d.extend(
                poly.raw_coords()
                    .iter()
                    .map(|v| [v[0] as f32, v[1] as f32, v[2] as f32]),
            );

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
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

        let occupied_voxels = voxelizer.finalize();
        assert_eq!(occupied_voxels.len(), 584);
    }
}
