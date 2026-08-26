//! Encoding of the .dbf file that accompanies a Shapefile.
//!
//! There is no single reliable way to tell what encoding the character fields
//! of a .dbf use: it may be declared in a .cpg sidecar file, in the LDID byte
//! of the .dbf header, or not at all. [`guess_encoding`] resolves those cases
//! for reading.

use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

use shapefile::{
    dbase::{
        self,
        encoding::{DynEncoding, EncodingRs},
        encoding_rs,
    },
    ShapeReader,
};

/// Offset of the LDID (language driver ID) in a dBASE file header.
/// This is a 0-based offset, i.e. the 30th byte of the file.
const LDID_OFFSET: usize = 29;

/// Guesses the encoding of the .dbf file that belongs to the given .shp path.
///
/// The encoding declared in the .cpg file wins if there is one. Otherwise, a
/// .dbf whose header carries no LDID is assumed to be Shift_JIS, since that is
/// what unmarked Japanese data usually turns out to be.
///
/// Returns `Ok(None)` when the .dbf does carry an LDID, in which case decoding
/// is left to dbase (e.g. 0x13 means CP932).
///
/// cf. <https://github.com/EsriJapan/shapefile_info>
pub fn guess_encoding(shp_path: &Path) -> Result<Option<DynEncoding>> {
    // First, check .cpg file
    let cpg_path = shp_path.with_extension("cpg");
    match std::fs::read_to_string(&cpg_path) {
        Ok(cpg) => {
            let name = cpg.trim().trim_start_matches('\u{feff}');
            return DynEncoding::from_name(name).map(Some).ok_or_else(|| {
                std::io::Error::other(format!("Unknown encoding in .cpg file: {name}"))
            });
        }
        // If there's no .cpg file, fall back to the LDID...
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }

    // If LDID (30th byte of a dBASE file) is not set (i.e. 0), dbase would
    // treat the file as UTF-8, but such files are most likely Shift_JIS in
    // Japanese data. If LDID is set, leave the detection to dbase.
    let mut header = [0u8; LDID_OFFSET + 1];
    File::open(shp_path.with_extension("dbf"))?.read_exact(&mut header)?;
    if header[LDID_OFFSET] == 0 {
        return Ok(Some(EncodingRs::from(encoding_rs::SHIFT_JIS).into()));
    }

    // If any LDID is set, dbase-rs should be able to handle it properly.
    Ok(None)
}

/// Opens a Shapefile, decoding its .dbf with the encoding [`guess_encoding`]
/// detects.
///
/// This is [`shapefile::Reader::from_path`] plus the Shift_JIS fallback for
/// .dbf files that carry no encoding marker at all.
pub fn reader_from_path(
    shp_path: &Path,
) -> std::result::Result<
    shapefile::Reader<std::io::BufReader<File>, std::io::BufReader<File>>,
    shapefile::Error,
> {
    let Some(encoding) = guess_encoding(shp_path)? else {
        // No hint found; dbase can figure it out from the LDID on its own.
        return shapefile::Reader::from_path(shp_path);
    };

    let shape_reader = ShapeReader::from_path(shp_path)?;
    let dbf_source = std::io::BufReader::new(File::open(shp_path.with_extension("dbf"))?);
    let dbase_reader = dbase::ReaderBuilder::new()
        .with_encoding(encoding)
        .build(dbf_source)?;

    Ok(shapefile::Reader::new(shape_reader, dbase_reader))
}

#[cfg(test)]
mod tests {
    use shapefile::dbase::encoding::AsCodePageMark;

    use super::*;

    /// Writes a minimal .dbf header with the given LDID, plus optionally a .cpg.
    fn make_files(dir: &Path, stem: &str, ldid: u8, cpg: Option<&str>) -> std::path::PathBuf {
        let shp_path = dir.join(format!("{stem}.shp"));
        let mut header = [0u8; 32];
        header[LDID_OFFSET] = ldid;
        std::fs::write(shp_path.with_extension("dbf"), header).unwrap();
        if let Some(cpg) = cpg {
            std::fs::write(shp_path.with_extension("cpg"), cpg).unwrap();
        }
        shp_path
    }

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("nusamai-shapefile-encoding-{name}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn cpg_takes_precedence_over_ldid() {
        let dir = temp_dir("cpg");
        // LDID says CP932, but the .cpg says UTF-8
        let shp_path = make_files(&dir, "foo", 0x13, Some("UTF-8"));
        let encoding = guess_encoding(&shp_path).unwrap().unwrap();
        assert_eq!(encoding.code_page_mark(), dbase::CodePageMark::Utf8);
    }

    #[test]
    fn cpg_with_bom_and_whitespace() {
        let dir = temp_dir("cpg-bom");
        let shp_path = make_files(&dir, "foo", 0, Some("\u{feff}SHIFT_JIS\n"));
        let encoding = guess_encoding(&shp_path).unwrap().unwrap();
        assert_eq!(encoding.code_page_mark(), dbase::CodePageMark::CP932);
    }

    #[test]
    fn unknown_cpg_is_an_error() {
        let dir = temp_dir("cpg-unknown");
        let shp_path = make_files(&dir, "foo", 0, Some("NO-SUCH-ENCODING"));
        assert!(guess_encoding(&shp_path).is_err());
    }

    #[test]
    fn unmarked_dbf_falls_back_to_shift_jis() {
        let dir = temp_dir("unmarked");
        let shp_path = make_files(&dir, "foo", 0, None);
        let encoding = guess_encoding(&shp_path).unwrap().unwrap();
        assert_eq!(encoding.code_page_mark(), dbase::CodePageMark::CP932);
    }

    #[test]
    fn marked_dbf_is_left_to_dbase() {
        let dir = temp_dir("marked");
        let shp_path = make_files(&dir, "foo", 0x13, None);
        assert!(guess_encoding(&shp_path).unwrap().is_none());
    }
}
