use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

pub enum FileReader {
    Regular(BufReader<File>),
    Zip(BufReader<std::io::Cursor<Vec<u8>>>),
}

impl FileReader {
    pub fn is_zip_path(path: &str) -> bool {
        path.contains(".zip/") || path.contains(".zip\\")
    }

    pub fn open(path: &str) -> std::io::Result<Self> {
        // path/to/file.zip/internal/path to path/to/file.zip , internal/path separate
        if let Some((zip_path, internal_path)) = split_zip_path(path) {
            let file = File::open(&zip_path)?;
            let mut archive = zip::ZipArchive::new(file)?;

            let mut zip_file = archive.by_name(&internal_path)?;

            let mut buffer = Vec::new();
            zip_file.read_to_end(&mut buffer)?;

            return Ok(FileReader::Zip(BufReader::new(std::io::Cursor::new(
                buffer,
            ))));
        }

        let file = File::open(path)?;
        let reader = BufReader::with_capacity(1024 * 1024, file);
        Ok(FileReader::Regular(reader))
    }

    pub fn into_buf_reader(self) -> Box<dyn BufRead + Send> {
        match self {
            FileReader::Regular(reader) => Box::new(reader),
            FileReader::Zip(reader) => Box::new(reader),
        }
    }
}

impl Read for FileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            FileReader::Regular(reader) => reader.read(buf),
            FileReader::Zip(reader) => reader.read(buf),
        }
    }
}

impl BufRead for FileReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            FileReader::Regular(reader) => reader.fill_buf(),
            FileReader::Zip(reader) => reader.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            FileReader::Regular(reader) => reader.consume(amt),
            FileReader::Zip(reader) => reader.consume(amt),
        }
    }
}

impl Seek for FileReader {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match self {
            FileReader::Regular(reader) => reader.seek(pos),
            FileReader::Zip(reader) => reader.seek(pos),
        }
    }
}

fn split_zip_path(path: &str) -> Option<(String, String)> {
    for delim in [".zip/", ".zip\\"] {
        if let Some(idx) = path.rfind(delim) {
            let zip_path = format!("{}{}", &path[..idx], ".zip");
            let internal_path = path[idx + delim.len()..].replace('\\', "/");
            return Some((zip_path, internal_path));
        }
    }
    None
}
