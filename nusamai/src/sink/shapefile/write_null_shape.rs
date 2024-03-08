use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::fs::{remove_file, File};
use std::io::Result;

pub fn write(file_path: &str, feature_count: &usize) -> Result<()> {
    let shp_path = file_path.to_string();
    let shx_path = shp_path.replace(".shp", ".shx");

    remove_file(shp_path.as_str())?;
    remove_file(shx_path.as_str())?;

    let mut shp_file = File::create(shp_path)?;
    let mut shx_file = File::create(shx_path)?;

    // write headers for SHP and SHX files
    write_header(&mut shp_file, feature_count)?;
    write_header(&mut shx_file, feature_count)?;

    for i in 0..*feature_count {
        let record_number = i as u32 + 1; // record number starts from 1
        let content_length = 2; // NullShape content length is 2 (in 16-bit words)

        // add a record to the SHP file
        shp_file.write_u32::<BigEndian>(record_number)?;
        shp_file.write_u32::<BigEndian>(content_length)?;
        shp_file.write_u32::<LittleEndian>(0)?; // NullShape.

        // add index information to SHX file
        let offset = 50 + (i * 4 * 2); // offset of record (placed after header)
        shx_file.write_i32::<BigEndian>(offset as i32 / 2)?; // offset is in 16-bit words
        shx_file.write_i32::<BigEndian>(content_length as i32)?; // length of content
    }

    Ok(())
}

fn write_header(file: &mut File, feature_count: &usize) -> Result<()> {
    file.write_u32::<BigEndian>(9994)?; // file code
    for _ in 0..5 {
        file.write_u32::<BigEndian>(0)?; // unused area
    }

    let file_length = 50 + (feature_count * 4 * 2); // calculate file length (header + record count * record length)
    file.write_u32::<BigEndian>(file_length as u32 / 2)?; // file length (in 16-bit words)

    file.write_u32::<LittleEndian>(1000)?; // version
    file.write_u32::<LittleEndian>(0)?; // shape type (NullShape)

    for _ in 0..4 {
        file.write_f64::<LittleEndian>(0.0)?; // bounding box
    }
    for _ in 0..4 {
        file.write_f64::<LittleEndian>(0.0)?; // Z-range and M-range
    }

    Ok(())
}
