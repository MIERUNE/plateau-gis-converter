//! Tile ID conversion based on Hilbert curve (compliant with PMTiles)

pub fn id_to_zxy(id: u64) -> (u8, u32, u32) {
    let z = (u64::BITS / 2 - (3 * id + 1).leading_zeros() / 2 - 1) as u8;
    let acc = ((1 << (z * 2)) - 1) / 3;
    let mut pos = id - acc;
    let (tx, ty) = (0..z).fold((0, 0), |(tx, ty), a| {
        let rx = (pos / 2) & 1;
        let ry = (pos ^ rx) & 1;
        let s = 1 << a;
        let (tx, ty) = rotate(s, tx, ty, rx, ry);
        pos /= 4;
        (tx + s * rx, ty + s * ry)
    });
    (z, tx as u32, ty as u32)
}

pub fn zxy_to_id(z: u8, x: u32, y: u32) -> u64 {
    let acc = ((1 << (z * 2)) - 1) / 3;
    let (mut tx, mut ty) = (x as u64, y as u64);
    (0..z).rev().fold(acc, |acc, a| {
        let rx = (tx >> a) & 1;
        let ry = (ty >> a) & 1;
        let s = 1 << a;
        (tx, ty) = rotate(s, tx, ty, rx, ry);
        acc + s * s * ((3 * rx) ^ ry)
    })
}

const fn rotate(n: u64, mut x: u64, mut y: u64, rx: u64, ry: u64) -> (u64, u64) {
    if ry == 0 {
        if rx == 1 {
            x = (n - 1).wrapping_sub(x);
            y = (n - 1).wrapping_sub(y);
        }
        (x, y) = (y, x)
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let fixture = vec![
            // ((x, y, z), expected_tile_id)
            //
            // z = 0
            ((0, 0, 0), 0),
            // z = 1
            ((1, 0, 0), 1),
            ((1, 0, 1), 2),
            ((1, 1, 1), 3),
            ((1, 1, 0), 4),
            // z = 2
            ((2, 0, 1), 8),
            ((2, 1, 1), 7),
            ((2, 2, 0), 19),
            ((2, 3, 3), 15),
            ((2, 3, 2), 16),
            // z= 3
            ((3, 0, 0), 21),
            ((3, 7, 0), 84),
            // z = 4
            ((4, 0, 0), 85),
            ((4, 15, 0), 340),
            // z = 18 (tileId exceeds u32)
            ((18, 1, 1), 22906492247),
        ];

        for ((x, y, z), expected_tile_id) in fixture {
            let tile_id = zxy_to_id(x, y, z);
            assert_eq!(tile_id, expected_tile_id);
            assert_eq!(id_to_zxy(tile_id), (x, y, z));
        }
    }
}
