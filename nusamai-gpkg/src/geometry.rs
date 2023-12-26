//! Convert geometries to GeoPackage SQL Geometry Binary Format
/// cf. https://www.geopackage.org/spec130/#gpb_format
use nusamai_geometry::{MultiPolygon, Polygon};

fn geometry_header(srs_id: i32) -> Vec<u8> {
    let mut header: Vec<u8> = vec![];
    header.extend_from_slice(&[0x47, 0x50]); // Magic number
    header.push(0x00); // Version
    header.push(0b00000001); // Flags
    header.extend_from_slice(&i32::to_le_bytes(srs_id)); // SRS ID
    header
}

fn polygon_to_rings(vertices: &[[f64; 3]], poly: &Polygon<1, u32>) -> Vec<Vec<Vec<f64>>> {
    let linestrings = std::iter::once(poly.exterior()).chain(poly.interiors());

    let rings: Vec<_> = linestrings
        .map(|ls| {
            let coords: Vec<_> = ls
                .iter_closed()
                .map(|idx| vertices[idx[0] as usize].to_vec()) // Get the actual coord values
                .collect();
            coords
        })
        .collect();

    rings
}

pub fn multipolygon_to_bytes(vertices: &[[f64; 3]], mpoly: &MultiPolygon<'_, 1, u32>) -> Vec<u8> {
    let mut bytes: Vec<u8> = geometry_header(4326);

    // Byte order: Little endian
    bytes.push(0x01);

    // Geometry type: wkbMultiPolygonZ (1006)
    bytes.extend_from_slice(&1006_u32.to_le_bytes());

    // numPolygons
    bytes.extend_from_slice(&(mpoly.len() as u32).to_le_bytes());

    for poly in mpoly {
        // Byte order: Little endian
        bytes.push(0x01);

        // Geometry type: wkbPolygonZ (1003)
        bytes.extend_from_slice(&1003_u32.to_le_bytes());

        let rings = polygon_to_rings(vertices, &poly);

        // numRings
        bytes.extend_from_slice(&(rings.len() as u32).to_le_bytes());

        for ring in rings {
            // numPoints
            bytes.extend_from_slice(&(ring.len() as u32).to_le_bytes());

            for coord in ring {
                let x = f64::to_le_bytes(coord[1]); // FIXME: lon,lat order
                bytes.extend_from_slice(&x);
                let y = f64::to_le_bytes(coord[0]); // FIXME: lon,lat order
                bytes.extend_from_slice(&y);
                let z = f64::to_le_bytes(coord[2]);
                bytes.extend_from_slice(&z);
            }
        }
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_to_rings() {
        let vertices: Vec<[f64; 3]> = vec![
            // exterior (vertex 0~3)
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
            // interior 1 (vertex 4~7)
            [1., 1., 111.],
            [2., 1., 111.],
            [2., 2., 111.],
            [1., 2., 111.],
            // interior 2 (vertex 8~11)
            [3., 3., 111.],
            [4., 3., 111.],
            [4., 4., 111.],
            [3., 4., 111.],
        ];

        let mut poly = Polygon::<'_, 1, u32>::new();
        poly.add_ring([[0], [1], [2], [3]]);
        poly.add_ring([[4], [5], [6], [7]]);
        poly.add_ring([[8], [9], [10], [11]]);

        let rings = polygon_to_rings(&vertices, &poly);

        assert_eq!(rings.len(), 3);

        for (i, ri) in rings.iter().enumerate() {
            match i {
                0 => {
                    assert_eq!(ri.len(), 5);
                    assert_eq!(ri[0], vec![0., 0., 111.]);
                    assert_eq!(ri[1], vec![5., 0., 111.]);
                    assert_eq!(ri[2], vec![5., 5., 111.]);
                    assert_eq!(ri[3], vec![0., 5., 111.]);
                    assert_eq!(ri[4], vec![0., 0., 111.]);
                }
                1 => {
                    assert_eq!(ri.len(), 5);
                    assert_eq!(ri[0], vec![1., 1., 111.]);
                    assert_eq!(ri[1], vec![2., 1., 111.]);
                    assert_eq!(ri[2], vec![2., 2., 111.]);
                    assert_eq!(ri[3], vec![1., 2., 111.]);
                    assert_eq!(ri[4], vec![1., 1., 111.]);
                }
                2 => {
                    assert_eq!(ri.len(), 5);
                    assert_eq!(ri[0], vec![3., 3., 111.]);
                    assert_eq!(ri[1], vec![4., 3., 111.]);
                    assert_eq!(ri[2], vec![4., 4., 111.]);
                    assert_eq!(ri[3], vec![3., 4., 111.]);
                }
                _ => panic!("Unexpected ring index"),
            }
        }
    }
}
