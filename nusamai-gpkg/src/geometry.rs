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

pub fn multipolygon_to_bytes(
    vertices: &[[f64; 3]],
    mpoly: &MultiPolygon<'_, 1, u32>,
    srs_id: i32,
) -> Vec<u8> {
    let mut bytes: Vec<u8> = geometry_header(srs_id);

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

    #[test]
    fn test_multipolygon_to_bytes() {
        let vertices: Vec<[f64; 3]> = vec![
            // 1st polygon, exterior (vertex 0~3)
            [0., 0., 111.],
            [5., 0., 111.],
            [5., 5., 111.],
            [0., 5., 111.],
            // 1st polygon, interior 1 (vertex 4~7)
            [1., 1., 111.],
            [2., 1., 111.],
            [2., 2., 111.],
            [1., 2., 111.],
        ];

        let mut mpoly = MultiPolygon::<'_, 1, u32>::new();
        // 1st polygon
        mpoly.add_exterior([[0], [1], [2], [3], [0]]);
        mpoly.add_interior([[4], [5], [6], [7], [4]]);

        let bytes = multipolygon_to_bytes(&vertices, &mpoly, 1234);

        assert_eq!(bytes.len(), 274);

        // header
        assert_eq!(bytes[0..=3].to_vec(), vec![0x47, 0x50, 0x00, 0b00000001]);
        assert_eq!(bytes[4..=7].to_vec(), &i32::to_le_bytes(1234));

        // Byte order: Little endian
        assert_eq!(bytes[8], 0x01);

        // Geometry type: wkbMultiPolygonZ (1006)
        assert_eq!(bytes[9..=12].to_vec(), &1006_u32.to_le_bytes());

        // numPolygons
        assert_eq!(bytes[13..=16].to_vec(), &1_u32.to_le_bytes());

        // 1st polygon
        // Byte order: Little endian
        assert_eq!(bytes[17], 0x01);

        // Geometry type: wkbPolygonZ (1003)
        assert_eq!(bytes[18..=21].to_vec(), &1003_u32.to_le_bytes());

        // numRings
        assert_eq!(bytes[22..=25].to_vec(), &2_u32.to_le_bytes());

        // exterior
        // numPoints
        assert_eq!(bytes[26..=29].to_vec(), &5_u32.to_le_bytes());

        // 1st point
        assert_eq!(bytes[30..=37].to_vec(), &0_f64.to_le_bytes());
        assert_eq!(bytes[38..=45].to_vec(), &0_f64.to_le_bytes());
        assert_eq!(bytes[46..=53].to_vec(), &111_f64.to_le_bytes());

        // 2nd point
        assert_eq!(bytes[54..=61].to_vec(), &0_f64.to_le_bytes());
        assert_eq!(bytes[62..=69].to_vec(), &5_f64.to_le_bytes());
        assert_eq!(bytes[70..=77].to_vec(), &111_f64.to_le_bytes());

        // 3rd point
        assert_eq!(bytes[78..=85].to_vec(), &5_f64.to_le_bytes());
        assert_eq!(bytes[86..=93].to_vec(), &5_f64.to_le_bytes());
        assert_eq!(bytes[94..=101].to_vec(), &111_f64.to_le_bytes());

        // 4th point
        assert_eq!(bytes[102..=109].to_vec(), &5_f64.to_le_bytes());
        assert_eq!(bytes[110..=117].to_vec(), &0_f64.to_le_bytes());
        assert_eq!(bytes[118..=125].to_vec(), &111_f64.to_le_bytes());

        // 5th point
        assert_eq!(bytes[126..=133].to_vec(), &0_f64.to_le_bytes());
        assert_eq!(bytes[134..=141].to_vec(), &0_f64.to_le_bytes());
        assert_eq!(bytes[142..=149].to_vec(), &111_f64.to_le_bytes());

        // interior
        // numPoints
        assert_eq!(bytes[150..=153].to_vec(), &5_u32.to_le_bytes());

        // 1st point
        assert_eq!(bytes[154..=161].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[162..=169].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[170..=177].to_vec(), &111_f64.to_le_bytes());

        // 2nd point
        assert_eq!(bytes[178..=185].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[186..=193].to_vec(), &2_f64.to_le_bytes());
        assert_eq!(bytes[194..=201].to_vec(), &111_f64.to_le_bytes());

        // 3rd point
        assert_eq!(bytes[202..=209].to_vec(), &2_f64.to_le_bytes());
        assert_eq!(bytes[210..=217].to_vec(), &2_f64.to_le_bytes());
        assert_eq!(bytes[218..=225].to_vec(), &111_f64.to_le_bytes());

        // 4th point
        assert_eq!(bytes[226..=233].to_vec(), &2_f64.to_le_bytes());
        assert_eq!(bytes[234..=241].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[242..=249].to_vec(), &111_f64.to_le_bytes());

        // 5th point
        assert_eq!(bytes[250..=257].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[258..=265].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[266..=273].to_vec(), &111_f64.to_le_bytes());
    }
}
