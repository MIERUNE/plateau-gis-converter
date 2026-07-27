//! Convert geometries to GeoPackage SQL Geometry Binary Format
//!
//! cf. https://www.geopackage.org/spec130/#gpb_format

use std::io::Write;

use flatgeom::{Coord, LineString, MultiLineString, MultiPoint, MultiPolygon, Polygon};

/// Magic number of the GeoPackageBinary header (`GP`)
pub(crate) const GPB_MAGIC: [u8; 2] = [0x47, 0x50];

/// GeoPackageBinary header version (`0` means version 1)
pub(crate) const GPB_VERSION: u8 = 0;

/// Flag bit telling that the header's `srs_id` and envelope are little-endian
pub(crate) const GPB_FLAG_LITTLE_ENDIAN: u8 = 0b0000_0001;

/// Size of a GeoPackageBinary header that carries no envelope
pub(crate) const GPB_HEADER_SIZE: usize = 8;

#[repr(u8)]
pub enum WkbByteOrder {
    // Big endian (XDR)
    BigEndian = 0,
    // Little endian (NDR)
    LittleEndian = 1,
}

#[repr(u32)]
pub enum WkbGeometryType {
    Point = 1,
    LineString = 2,
    Polygon = 3,
    MultiPoint = 4,
    MultiLineString = 5,
    MultiPolygon = 6,
    GeometryCollection = 7,
    PointZ = 1001,
    LineStringZ = 1002,
    PolygonZ = 1003,
    MultiPointZ = 1004,
    MultiLineStringZ = 1005,
    MultiPolygonZ = 1006,
    GeometryCollectionZ = 1007,
    PointM = 2001,
    LineStringM = 2002,
    PolygonM = 2003,
    MultiPointM = 2004,
    MultiLineStringM = 2005,
    MultiPolygonM = 2006,
    GeometryCollectionM = 2007,
    PointZM = 3001,
    LineStringZM = 3002,
    PolygonZM = 3003,
    MultiPointZM = 3004,
    MultiLineStringZM = 3005,
    MultiPolygonZM = 3006,
    GeometryCollectionZM = 3007,
}

fn write_geometry_header<W: Write>(writer: &mut W, srs_id: i32) -> std::io::Result<()> {
    writer.write_all(&GPB_MAGIC)?;
    writer.write_all(&[
        GPB_VERSION,
        GPB_FLAG_LITTLE_ENDIAN, // no envelope, standard binary type
    ])?;
    writer.write_all(&i32::to_le_bytes(srs_id))?; // SRS ID
    Ok(())
}

fn write_point_body<W: Write, T: Coord>(
    writer: &mut W,
    point: T,
    mapping: impl Fn(T) -> [f64; 3],
) -> std::io::Result<()> {
    writer.write_all(&[WkbByteOrder::LittleEndian as u8])?;
    writer.write_all(&(WkbGeometryType::PointZ as u32).to_le_bytes())?;

    let [x, y, z] = mapping(point);
    writer.write_all(&x.to_le_bytes())?;
    writer.write_all(&y.to_le_bytes())?;
    writer.write_all(&z.to_le_bytes())?;
    Ok(())
}

pub fn write_indexed_multipoint<W: Write>(
    writer: &mut W,
    vertices: &[[f64; 3]],
    multipoint: &MultiPoint<u32>,
    srs_id: i32,
) -> std::io::Result<()> {
    write_geometry_header(writer, srs_id)?;
    write_multipoint_body(writer, multipoint, |idx| vertices[idx as usize])
}

fn write_multipoint_body<W: Write, T: Coord>(
    writer: &mut W,
    multipoint: &MultiPoint<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> std::io::Result<()> {
    writer.write_all(&[WkbByteOrder::LittleEndian as u8])?;
    writer.write_all(&(WkbGeometryType::MultiPointZ as u32).to_le_bytes())?;
    writer.write_all(&(multipoint.len() as u32).to_le_bytes())?;

    for point in multipoint {
        write_point_body(writer, point, &mapping)?;
    }
    Ok(())
}

fn write_linestring_body<W: Write, T: Coord>(
    writer: &mut W,
    linestring: &LineString<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> std::io::Result<()> {
    writer.write_all(&[WkbByteOrder::LittleEndian as u8])?;
    writer.write_all(&(WkbGeometryType::LineStringZ as u32).to_le_bytes())?;
    writer.write_all(&(linestring.len() as u32).to_le_bytes())?;

    for point in linestring {
        let [x, y, z] = mapping(point);
        writer.write_all(&x.to_le_bytes())?;
        writer.write_all(&y.to_le_bytes())?;
        writer.write_all(&z.to_le_bytes())?;
    }
    Ok(())
}

pub fn write_indexed_multilinestring<W: Write>(
    writer: &mut W,
    vertices: &[[f64; 3]],
    multiline: &MultiLineString<u32>,
    srs_id: i32,
) -> std::io::Result<()> {
    write_geometry_header(writer, srs_id)?;
    write_multilinestring_body(writer, multiline, |idx| vertices[idx as usize])
}

fn write_multilinestring_body<W: Write, T: Coord>(
    writer: &mut W,
    multiline: &MultiLineString<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> std::io::Result<()> {
    writer.write_all(&[WkbByteOrder::LittleEndian as u8])?;
    writer.write_all(&(WkbGeometryType::MultiLineStringZ as u32).to_le_bytes())?;
    writer.write_all(&(multiline.len() as u32).to_le_bytes())?;

    for linestring in multiline {
        write_linestring_body(writer, &linestring, &mapping)?;
    }
    Ok(())
}

fn write_polygon_body<W: Write, T: Coord>(
    writer: &mut W,
    poly: &Polygon<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> std::io::Result<()> {
    // Byte order: Little endian (1)
    writer.write_all(&[WkbByteOrder::LittleEndian as u8])?;

    // Geometry type: wkbPolygonZ (1003)
    writer.write_all(&(WkbGeometryType::PolygonZ as u32).to_le_bytes())?;

    // numRings
    writer.write_all(&(poly.rings().count() as u32).to_le_bytes())?;

    for ring in poly.rings() {
        // numPoints
        writer.write_all(&(ring.iter_closed().count() as u32).to_le_bytes())?;

        for idx in ring.iter_closed() {
            let [x, y, z] = mapping(idx);
            writer.write_all(&f64::to_le_bytes(x))?;
            writer.write_all(&f64::to_le_bytes(y))?;
            writer.write_all(&f64::to_le_bytes(z))?;
        }
    }
    Ok(())
}

pub fn write_indexed_multipolygon<W: Write>(
    writer: &mut W,
    vertices: &[[f64; 3]],
    mpoly: &MultiPolygon<u32>,
    srs_id: i32,
) -> std::io::Result<()> {
    write_geometry_header(writer, srs_id)?;
    write_multipolygon_body(writer, mpoly, |idx| vertices[idx as usize])?;
    Ok(())
}

fn write_multipolygon_body<W: Write, T: Coord>(
    writer: &mut W,
    mpoly: &MultiPolygon<T>,
    mapping: impl Fn(T) -> [f64; 3],
) -> std::io::Result<()> {
    // Byte order: Little endian (1)
    writer.write_all(&[WkbByteOrder::LittleEndian as u8])?;

    // Geometry type: wkbMultiPolygonZ (1006)
    writer.write_all(&(WkbGeometryType::MultiPolygonZ as u32).to_le_bytes())?;

    // numPolygons
    writer.write_all(&(mpoly.len() as u32).to_le_bytes())?;

    for poly in mpoly {
        write_polygon_body(writer, &poly, &mapping)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multipoint_to_bytes() {
        let vertices = vec![[10., 20., 30.], [40., 50., 60.]];
        let mut multipoint = flatgeom::MultiPoint::<u32>::new();
        multipoint.push(1);
        multipoint.push(0);

        let mut bytes = Vec::new();
        write_indexed_multipoint(&mut bytes, &vertices, &multipoint, 3857).unwrap();

        assert_eq!(bytes.len(), 75);
        assert_eq!(&bytes[0..4], &[0x47, 0x50, 0x00, 0b00000001]);
        assert_eq!(&bytes[4..8], &3857_i32.to_le_bytes());
        assert_eq!(bytes[8], 0x01);
        assert_eq!(&bytes[9..13], &1004_u32.to_le_bytes());
        assert_eq!(&bytes[13..17], &2_u32.to_le_bytes());

        assert_eq!(bytes[17], 0x01);
        assert_eq!(&bytes[18..22], &1001_u32.to_le_bytes());
        assert_eq!(&bytes[22..30], &40_f64.to_le_bytes());
        assert_eq!(&bytes[30..38], &50_f64.to_le_bytes());
        assert_eq!(&bytes[38..46], &60_f64.to_le_bytes());

        assert_eq!(bytes[46], 0x01);
        assert_eq!(&bytes[47..51], &1001_u32.to_le_bytes());
        assert_eq!(&bytes[51..59], &10_f64.to_le_bytes());
        assert_eq!(&bytes[59..67], &20_f64.to_le_bytes());
        assert_eq!(&bytes[67..75], &30_f64.to_le_bytes());
    }

    #[test]
    fn test_multilinestring_to_bytes() {
        let vertices = vec![
            [10., 20., 30.],
            [40., 50., 60.],
            [70., 80., 90.],
            [100., 110., 120.],
        ];
        let mut multiline = flatgeom::MultiLineString::<u32>::new();
        multiline.add_linestring([0, 1]);
        multiline.add_linestring([2, 3, 0]);

        let mut bytes = Vec::new();
        write_indexed_multilinestring(&mut bytes, &vertices, &multiline, 4979).unwrap();

        assert_eq!(bytes.len(), 155);
        assert_eq!(&bytes[0..4], &[0x47, 0x50, 0x00, 0b00000001]);
        assert_eq!(&bytes[4..8], &4979_i32.to_le_bytes());
        assert_eq!(bytes[8], 0x01);
        assert_eq!(&bytes[9..13], &1005_u32.to_le_bytes());
        assert_eq!(&bytes[13..17], &2_u32.to_le_bytes());

        assert_eq!(bytes[17], 0x01);
        assert_eq!(&bytes[18..22], &1002_u32.to_le_bytes());
        assert_eq!(&bytes[22..26], &2_u32.to_le_bytes());
        assert_eq!(&bytes[26..34], &10_f64.to_le_bytes());
        assert_eq!(&bytes[34..42], &20_f64.to_le_bytes());
        assert_eq!(&bytes[42..50], &30_f64.to_le_bytes());
        assert_eq!(&bytes[50..58], &40_f64.to_le_bytes());
        assert_eq!(&bytes[58..66], &50_f64.to_le_bytes());
        assert_eq!(&bytes[66..74], &60_f64.to_le_bytes());

        assert_eq!(bytes[74], 0x01);
        assert_eq!(&bytes[75..79], &1002_u32.to_le_bytes());
        assert_eq!(&bytes[79..83], &3_u32.to_le_bytes());
        assert_eq!(&bytes[83..91], &70_f64.to_le_bytes());
        assert_eq!(&bytes[91..99], &80_f64.to_le_bytes());
        assert_eq!(&bytes[99..107], &90_f64.to_le_bytes());
        assert_eq!(&bytes[107..115], &100_f64.to_le_bytes());
        assert_eq!(&bytes[115..123], &110_f64.to_le_bytes());
        assert_eq!(&bytes[123..131], &120_f64.to_le_bytes());
        assert_eq!(&bytes[131..139], &10_f64.to_le_bytes());
        assert_eq!(&bytes[139..147], &20_f64.to_le_bytes());
        assert_eq!(&bytes[147..155], &30_f64.to_le_bytes());
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

        let mut mpoly = MultiPolygon::<u32>::new();
        // 1st polygon
        mpoly.add_exterior([0, 1, 2, 3, 0]);
        mpoly.add_interior([4, 5, 6, 7, 4]);

        let mut bytes = Vec::new();
        write_indexed_multipolygon(&mut bytes, &vertices, &mpoly, 1234).unwrap();

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
        assert_eq!(bytes[54..=61].to_vec(), &5_f64.to_le_bytes());
        assert_eq!(bytes[62..=69].to_vec(), &0_f64.to_le_bytes());
        assert_eq!(bytes[70..=77].to_vec(), &111_f64.to_le_bytes());

        // 3rd point
        assert_eq!(bytes[78..=85].to_vec(), &5_f64.to_le_bytes());
        assert_eq!(bytes[86..=93].to_vec(), &5_f64.to_le_bytes());
        assert_eq!(bytes[94..=101].to_vec(), &111_f64.to_le_bytes());

        // 4th point
        assert_eq!(bytes[102..=109].to_vec(), &0_f64.to_le_bytes());
        assert_eq!(bytes[110..=117].to_vec(), &5_f64.to_le_bytes());
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
        assert_eq!(bytes[178..=185].to_vec(), &2_f64.to_le_bytes());
        assert_eq!(bytes[186..=193].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[194..=201].to_vec(), &111_f64.to_le_bytes());

        // 3rd point
        assert_eq!(bytes[202..=209].to_vec(), &2_f64.to_le_bytes());
        assert_eq!(bytes[210..=217].to_vec(), &2_f64.to_le_bytes());
        assert_eq!(bytes[218..=225].to_vec(), &111_f64.to_le_bytes());

        // 4th point
        assert_eq!(bytes[226..=233].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[234..=241].to_vec(), &2_f64.to_le_bytes());
        assert_eq!(bytes[242..=249].to_vec(), &111_f64.to_le_bytes());

        // 5th point
        assert_eq!(bytes[250..=257].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[258..=265].to_vec(), &1_f64.to_le_bytes());
        assert_eq!(bytes[266..=273].to_vec(), &111_f64.to_le_bytes());
    }
}
