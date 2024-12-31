//! Binary glTF (GLB) format reader and writer.

use std::borrow::Cow;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

const GLB_MAGIC: &[u8; 4] = b"glTF";
const GLB_VERSION: u32 = 2;

const JSON_CHUNK_MARKER: &[u8; 4] = b"JSON";
const BIN_CHUNK_MARKER: &[u8; 4] = b"BIN\x00";

/// Binary glTF (GLB) format reader and writer.
#[derive(Debug, Clone)]
pub struct Glb<'a> {
    pub json: Cow<'a, [u8]>,
    pub bin: Option<Cow<'a, [u8]>>,
}

impl Glb<'_> {
    /// Write GLB to writer.
    pub fn to_writer<W: std::io::Write>(&self, writer: W) -> std::io::Result<()> {
        self.to_writer_with_alignment(writer, 4)
    }

    /// Write GLB to writer with specified alignment.
    pub fn to_writer_with_alignment<W: std::io::Write>(
        &self,
        mut writer: W,
        alignment: usize,
    ) -> std::io::Result<()> {
        let alignment = alignment as u32;

        let json_original_size = self.json.len() as u32;
        let json_padding_size =
            (alignment - ((12 + 8 + json_original_size + 8) % alignment)) % alignment;
        let json_padded_size = json_original_size + json_padding_size;

        let bin_original_size = self.bin.as_ref().map(|b| b.len() as u32).unwrap_or(0);
        let bin_padding_size =
            (alignment - ((12 + 16 + bin_original_size) % alignment)) % alignment;
        let bin_padded_size = bin_original_size + bin_padding_size;

        let total_size = 12 + // GLB header size
            8 + // JSON chunk header size
            json_padded_size + // JSON content size
            8 + // BIN chunk header size
            bin_padded_size // BIN content size
            ;

        // GLB header
        writer.write_all(GLB_MAGIC)?;
        writer.write_u32::<LittleEndian>(GLB_VERSION)?;
        writer.write_u32::<LittleEndian>(total_size)?;

        // JSON Chunk
        writer.write_u32::<LittleEndian>(json_padded_size)?;
        writer.write_all(JSON_CHUNK_MARKER)?;
        writer.write_all(&self.json)?;
        writer.write_all(&vec![0x20; json_padding_size as usize])?;

        // BIN Chunk (optional)
        if let Some(bin_content) = &self.bin {
            writer.write_u32::<LittleEndian>(bin_padded_size)?;
            writer.write_all(BIN_CHUNK_MARKER)?;
            writer.write_all(bin_content)?;
            writer.write_all(&vec![0x00; bin_padding_size as usize])?;
        }

        Ok(())
    }

    /// Read GLB from reader.
    pub fn from_reader<R: std::io::Read>(mut reader: R) -> std::io::Result<Self> {
        // GLB header
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        if &buf[0..4] != GLB_MAGIC {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "glb magic must be 'glTF'".to_string(),
            ));
        }
        if reader.read_u32::<LittleEndian>()? != 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "glb version must be 2".to_string(),
            ));
        }
        _ = reader.read_u32::<LittleEndian>()?;

        // JSON chunk
        let json_content_size = reader.read_u32::<LittleEndian>()?;
        reader.read_exact(&mut buf)?;
        if &buf[0..4] != b"JSON" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "First chunk must be 'JSON'".to_string(),
            ));
        }
        let mut json_content = vec![0; json_content_size as usize];
        reader.read_exact(&mut json_content)?;

        // BIN chunk
        let bin_content_size = reader.read_u32::<LittleEndian>()?;
        reader.read_exact(&mut buf)?;
        if &buf[0..4] != b"BIN\x00" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Second chunk must be 'BIN'".to_string(),
            ));
        }
        let mut bin_content = vec![0; bin_content_size as usize];
        reader.read_exact(&mut bin_content)?;

        Ok(Self {
            json: json_content.into(),
            bin: Some(bin_content.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_write_alignment() {
        let json_content = b"{xxx}";
        let bin_content = b"123";

        // 4 byte alignment
        {
            let glb = Glb {
                json: json_content[..].into(),
                bin: Some(bin_content[..].into()),
            };
            let mut buf = Vec::new();
            glb.to_writer(&mut buf).unwrap();
            assert!(buf.len() % 4 == 0, "not satisfy 4-byte alignment");
            // 4 byte alignment
            assert_eq!(&buf[36..], b"123\0");

            let reader = Cursor::new(buf);
            let glb = Glb::from_reader(reader).unwrap();

            assert_eq!(String::from_utf8_lossy(&glb.json), "{xxx}\x20\x20\x20");
            assert_eq!(glb.bin.as_ref().unwrap().as_ref(), b"123\0");
        }

        // 8 byte alignment
        {
            let glb = Glb {
                json: json_content[..].into(),
                bin: Some(bin_content[..].into()),
            };
            let mut buf = Vec::new();
            glb.to_writer_with_alignment(&mut buf, 8).unwrap();
            assert!(buf.len() % 4 == 0, "not satisfy 4-byte alignment");
            // 8 byte alignment
            assert_eq!(&buf[40..], b"123\0");

            let reader = Cursor::new(buf);
            let glb = Glb::from_reader(reader).unwrap();
            assert_eq!(
                String::from_utf8_lossy(&glb.json),
                "{xxx}\x20\x20\x20\x20\x20\x20\x20"
            );
            assert_eq!(String::from_utf8_lossy(glb.bin.as_ref().unwrap()), "123\0");
        }
    }

    #[test]
    fn test_broken_glb() {
        let json_content = b"{xxx}";
        let bin_content = b"123";

        let valid = {
            let glb = Glb {
                json: json_content[..].into(),
                bin: Some(bin_content[..].into()),
            };
            let mut buf = Vec::new();
            glb.to_writer(&mut buf).unwrap();
            buf
        };

        // broken 'glTF' magic
        {
            let mut broken = valid.clone();
            broken[0] = b'x';
            Glb::from_reader(Cursor::new(broken)).unwrap_err();
        }

        // broken version
        {
            let mut broken = valid.clone();
            broken[4] = b'x';
            Glb::from_reader(Cursor::new(broken)).unwrap_err();
        }

        // broken JSON marker
        {
            let mut broken = valid.clone();
            broken[16] = b'x';
            Glb::from_reader(Cursor::new(broken)).unwrap_err();
        }

        // broken BIN\0 marker
        {
            let mut broken = valid.clone();
            broken[35] = b'x';
            Glb::from_reader(Cursor::new(broken)).unwrap_err();
        }
    }
}
