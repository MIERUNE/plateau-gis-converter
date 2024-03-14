use std::io::{Result, Write};

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

pub fn write_shp(mut writer: impl Write, feature_count: usize) -> Result<()> {
    write_header(&mut writer, feature_count)?;

    for i in 0..feature_count {
        // add a record to the SHP file
        let record_number = i as u32 + 1; // record number starts from 1
        let content_length = 2; // NullShape content length is 2 (in 16-bit words)
        writer.write_u32::<BigEndian>(record_number)?;
        writer.write_u32::<BigEndian>(content_length)?;
        writer.write_u32::<LittleEndian>(0)?; // NullShape.
    }

    Ok(())
}

pub fn write_shx(mut writer: impl Write, feature_count: usize) -> Result<()> {
    // write headers for SHP and SHX files
    write_header(&mut writer, feature_count)?;

    for i in 0..feature_count {
        // add index information to SHX file
        let offset = 50 + (i * 4 * 2); // offset of record (placed after header)
        let content_length = 2; // NullShape content length is 2 (in 16-bit words)
        writer.write_i32::<BigEndian>(offset as i32 / 2)?; // offset is in 16-bit words
        writer.write_i32::<BigEndian>(content_length)?; // length of content
    }

    Ok(())
}

fn write_header(mut writer: impl Write, feature_count: usize) -> Result<()> {
    writer.write_u32::<BigEndian>(9994)?; // file code

    for _ in 0..5 {
        writer.write_u32::<BigEndian>(0)?; // unused area
    }

    let file_length = 50 + (feature_count * 4 * 2); // calculate file length (header + record count * record length)
    writer.write_u32::<BigEndian>(file_length as u32 / 2)?; // file length (in 16-bit words)

    writer.write_u32::<LittleEndian>(1000)?; // version
    writer.write_u32::<LittleEndian>(0)?; // shape type (NullShape)

    for _ in 0..4 {
        writer.write_f64::<LittleEndian>(0.0)?; // bounding box
    }
    for _ in 0..4 {
        writer.write_f64::<LittleEndian>(0.0)?; // Z-range and M-range
    }

    Ok(())
}
