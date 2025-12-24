use std::io::{Result, Write};

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

const HEADER_LENGTH_WORDS: usize = 50; // 100 bytes / 2 (16-bit words)
const RECORD_HEADER_WORDS: usize = 4; // 8 bytes / 2 (16-bit words)
const NULL_SHAPE_CONTENT_WORDS: usize = 2; // 4 bytes / 2 (16-bit words)
const SHX_RECORD_WORDS: usize = 4; // each index entry is 8 bytes / 2 (16-bit words)

pub fn write_shp(mut writer: impl Write, feature_count: usize) -> Result<()> {
    let file_length_words =
        HEADER_LENGTH_WORDS + feature_count * (RECORD_HEADER_WORDS + NULL_SHAPE_CONTENT_WORDS); // file length in 16-bit words (header + record count * (record header + shp record size))
    write_header(&mut writer, file_length_words as u32)?;

    for i in 0..feature_count {
        // add a record to the SHP file
        let record_number = i as u32 + 1; // record number starts from 1
        writer.write_u32::<BigEndian>(record_number)?;
        writer.write_u32::<BigEndian>(NULL_SHAPE_CONTENT_WORDS as u32)?; // content length in words
        writer.write_u32::<LittleEndian>(0)?; // NullShape.
    }

    Ok(())
}

pub fn write_shx(mut writer: impl Write, feature_count: usize) -> Result<()> {
    let file_length_words = HEADER_LENGTH_WORDS + feature_count * SHX_RECORD_WORDS; // file length in 16-bit words (header + record count * shx record size)
    write_header(&mut writer, file_length_words as u32)?;

    let mut offset_words = HEADER_LENGTH_WORDS as i32;
    let record_words = (RECORD_HEADER_WORDS + NULL_SHAPE_CONTENT_WORDS) as i32;

    for _ in 0..feature_count {
        // add index information to SHX file
        writer.write_i32::<BigEndian>(offset_words)?; // offset is stored in 16-bit words
        writer.write_i32::<BigEndian>(NULL_SHAPE_CONTENT_WORDS as i32)?; // length of content in words
        offset_words += record_words;
    }

    Ok(())
}

fn write_header(mut writer: impl Write, file_length_words: u32) -> Result<()> {
    writer.write_u32::<BigEndian>(9994)?; // file code

    for _ in 0..5 {
        writer.write_u32::<BigEndian>(0)?; // unused area
    }

    writer.write_u32::<BigEndian>(file_length_words)?; // file length (in 16-bit words)

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
