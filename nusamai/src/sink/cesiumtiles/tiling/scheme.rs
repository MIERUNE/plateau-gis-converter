//! Our tiling scheme for the 3D Tiles

use std::ops::Range;

// Get the position of the most significant bit
fn msb(d: u32) -> u32 {
    u32::BITS - d.leading_zeros()
}

pub fn x_step(z: u8, y: u32) -> u32 {
    match z {
        0 | 1 => 1,
        _ => {
            let zz = 1 << z;
            if y < zz / 4 {
                u32::max(1, zz / (1 << msb(y))) / 4
            } else {
                u32::max(1, zz / (1 << msb(zz / 2 - y - 1))) / 4
            }
        }
    }
}

fn size_for_z(z: u8) -> (u32, u32) {
    match z {
        0 => (1, 1),
        1 => (2, 2),
        _ => (1 << z, 1 << (z - 1)),
    }
}

pub fn zxy_from_lng_lat(z: u8, lng: f64, lat: f64) -> (u8, u32, u32) {
    let (x_size, y_size) = size_for_z(z);
    let y = ((90.0 - lat) / 180.0 * y_size as f64).floor() as u32;
    let xs = x_step(z, y) as i32;
    let x = ((180.0 + lng) / 360.0 * x_size as f64).floor() as i32;
    (z, (x - x.rem_euclid(xs)) as u32, y)
}

pub fn calc_parent_zxy(z: u8, x: u32, y: u32) -> (u8, u32, u32) {
    match z {
        0 => panic!("z=0 has no parent"),
        1 => (z - 1, 0, 0),
        2 => (z - 1, x / 2, y),
        _ => (z - 1, x / 2, y / 2),
    }
}

pub fn y_slice_range(z: u8, y: u32) -> (f64, f64) {
    let (_, y_size) = size_for_z(z);
    let y = y as f64;
    let north = 90.0 - 180.0 * y / y_size as f64;
    let south = 90.0 - 180.0 * (y + 1.0) / y_size as f64;
    (south, north)
}

pub fn x_slice_range(z: u8, x: i32, xs: u32) -> (f64, f64) {
    let (x_size, _) = size_for_z(z);
    let west = -180.0 + 360.0 * x as f64 / x_size as f64;
    let east = -180.0 + 360.0 * (x + xs as i32) as f64 / x_size as f64;
    (west, east)
}

pub fn iter_y_slice(z: u8, south: f64, north: f64) -> Range<u32> {
    let (_, y_size) = size_for_z(z);
    let north = north.clamp(-90.0, 90.0);
    let south = south.clamp(-90.0, 90.0);
    let y_north = ((90.0 - north) / 180.0 * y_size as f64).floor() as u32;
    let y_south = ((90.0 - south) / 180.0 * y_size as f64).ceil() as u32;
    y_north..y_south
}

pub fn iter_x_slice(z: u8, y: u32, west: f64, east: f64) -> impl Iterator<Item = (i32, u32)> {
    let (x_size, _) = size_for_z(z);
    let x_west = ((180.0 + west) / 360.0 * x_size as f64).floor() as i32;
    let x_east = ((180.0 + east) / 360.0 * x_size as f64).ceil() as i32;
    let xs = x_step(z, y) as i32;
    (x_west - x_west.rem_euclid(xs)..x_east - x_east.rem_euclid(xs))
        .step_by(xs as usize)
        .map(move |x| (x, xs as u32))
}

pub fn geometric_error(z: u8, y: u32) -> f64 {
    let (_, y_size) = size_for_z(z);
    if y >= y_size {
        panic!("y out of range");
    }
    if z < 2 {
        return 1e+100;
    }
    use std::f64::consts::PI;
    const Q: f64 = 525957.5361033019;
    let zz = (1 << z) as f64;
    let error1 = Q / (1 << (z - 2)) as f64;
    let lat = (1.0 - (y as f64 + 0.5) * 4.0 / zz) * PI / 2.0;
    let error2 = lat.cos() * x_step(z, y) as f64 * error1;
    f64::max(error1, error2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_zxy(z: u8, x: u32, y: u32) -> bool {
        match z {
            0 => x == 0 && y == 0,
            1 => (0..2).contains(&x) && (0..2).contains(&y),
            _ => x < 1 << z && y < 1 << (z - 1) && x.is_multiple_of(x_step(z, y)),
        }
    }

    #[test]
    fn test_msb() {
        assert_eq!(msb(0), 0);
        assert_eq!(msb(1), 1);
        assert_eq!(msb(2), 2);
        assert_eq!(msb(3), 2);
        assert_eq!(msb(4), 3);
    }

    #[test]
    fn test_x_step() {
        let fixtures = [
            ((0, 0), 1),
            ((1, 0), 1),
            ((1, 1), 1),
            ((2, 0), 1),
            ((2, 1), 1),
            ((3, 0), 2),
            ((3, 1), 1),
            ((3, 2), 1),
            ((3, 3), 2),
            ((4, 0), 4),
            ((4, 1), 2),
            ((4, 2), 1),
            ((4, 3), 1),
            ((4, 4), 1),
            ((4, 5), 1),
            ((4, 6), 2),
            ((4, 7), 4),
            ((5, 0), 8),
            ((5, 1), 4),
        ];
        for ((z, y), expected) in fixtures.into_iter() {
            assert_eq!(x_step(z, y), expected);
        }
    }

    #[test]
    fn test_is_valid_xyz() {
        assert!(is_valid_zxy(0, 0, 0));
        assert!(is_valid_zxy(1, 0, 0));
        assert!(is_valid_zxy(1, 1, 1));
        assert!(is_valid_zxy(3, 0, 0));
        assert!(!is_valid_zxy(3, 1, 0));
        assert!(is_valid_zxy(3, 2, 0));
        assert!(!is_valid_zxy(3, 3, 0));
        assert!(!is_valid_zxy(4, 1, 1));
        assert!(!is_valid_zxy(4, 3, 0));
        assert!(!is_valid_zxy(4, 3, 1));
        assert!(is_valid_zxy(4, 4, 1));
        assert!(is_valid_zxy(4, 0, 6));
        assert!(!is_valid_zxy(4, 1, 6));
        assert!(is_valid_zxy(4, 2, 6));
        assert!(is_valid_zxy(4, 0, 7));
        assert!(!is_valid_zxy(4, 7, 7));
        assert!(is_valid_zxy(4, 8, 7));
    }

    #[test]
    fn test_get_size_for_z() {
        assert_eq!(size_for_z(0), (1, 1));
        assert_eq!(size_for_z(1), (2, 2));
        assert_eq!(size_for_z(2), (4, 2));
        assert_eq!(size_for_z(3), (8, 4));
    }

    #[test]
    fn test_iter_y_slice() {
        assert_eq!(iter_y_slice(1, -25., 60.).collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(
            iter_y_slice(3, -25., 60.).collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        assert_eq!(
            iter_y_slice(4, -25., 60.).collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_iter_x_slice() {
        assert_eq!(
            iter_x_slice(4, 0, -24., 46.).collect::<Vec<_>>(),
            vec![(4, 4)]
        );
        assert_eq!(
            iter_x_slice(4, 1, -24., 46.).collect::<Vec<_>>(),
            vec![(6, 2), (8, 2)]
        );
        assert_eq!(
            iter_x_slice(4, 2, -24., 46.).collect::<Vec<_>>(),
            vec![(6, 1), (7, 1), (8, 1), (9, 1), (10, 1)]
        );
        assert_eq!(
            iter_x_slice(4, 3, -24., 46.).collect::<Vec<_>>(),
            vec![(6, 1), (7, 1), (8, 1), (9, 1), (10, 1)]
        );
        assert_eq!(
            iter_x_slice(4, 7, -24., 46.).collect::<Vec<_>>(),
            vec![(4, 4)]
        );
    }

    #[test]
    fn test_geometric_error() {
        assert!((geometric_error(1, 1) - 1e+100).abs() < 1e-7);
        assert!((geometric_error(2, 1) - 525957.5361033019).abs() < 1e-7);
        for y in 0..4 {
            assert!((geometric_error(3, y) - 262978.76805165096).abs() < 1e-7);
        }
        assert!((geometric_error(4, 0) - 131489.38402582548).abs() < 1e-7);
        assert!((geometric_error(4, 1) - 146103.17544566366).abs() < 1e-7);
        assert!((geometric_error(4, 2) - 131489.38402582548).abs() < 1e-7);
        assert!((geometric_error(4, 3) - 131489.38402582548).abs() < 1e-7);
        assert!((geometric_error(4, 6) - 146103.17544566366).abs() < 1e-7);
        assert!((geometric_error(4, 7) - 131489.38402582548).abs() < 1e-7);

        assert!((geometric_error(5, 0) - 65744.69201291274).abs() < 1e-7);
        assert!((geometric_error(5, 1) - 76338.70680864961).abs() < 1e-7);
        assert!((geometric_error(5, 2) - 65744.69201291274).abs() < 1e-7);
        assert!((geometric_error(5, 3) - 83415.98216479822).abs() < 1e-7);
        assert!((geometric_error(5, 4) - 65744.69201291274).abs() < 1e-7);
    }

    #[test]
    fn test_slice_range() {
        let (z, x, y) = (4, 4, 1);
        let xs = x_step(z, y);
        let (south, north) = y_slice_range(z, y);
        let (west, east) = x_slice_range(z, x, xs);
        assert_eq!((45.0, 67.5, -90.0, -45.0), (south, north, west, east));
    }

    #[test]
    fn test_calc_parent_zxy() {
        assert_eq!(calc_parent_zxy(2, 0, 0), (1, 0, 0));
        assert_eq!(calc_parent_zxy(2, 2, 0), (1, 1, 0));
        assert_eq!(calc_parent_zxy(2, 2, 1), (1, 1, 1));
        assert_eq!(calc_parent_zxy(2, 1, 1), (1, 0, 1));

        assert_eq!(calc_parent_zxy(3, 0, 0), (2, 0, 0));
        assert_eq!(calc_parent_zxy(3, 2, 0), (2, 1, 0));
        assert_eq!(calc_parent_zxy(3, 1, 1), (2, 0, 0));
        assert_eq!(calc_parent_zxy(3, 2, 1), (2, 1, 0));

        assert_eq!(calc_parent_zxy(4, 4, 1), (3, 2, 0));
        assert_eq!(calc_parent_zxy(4, 4, 2), (3, 2, 1));
        assert_eq!(calc_parent_zxy(4, 0, 2), (3, 0, 1));
    }
}
