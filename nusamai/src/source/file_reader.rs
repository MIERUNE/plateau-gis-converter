use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub enum FileReader {
    Regular(BufReader<File>),
    Zip { reader: Box<dyn Read + Send> },
}

impl FileReader {
    pub fn open(path: &str) -> std::io::Result<Self> {
        // path/to/file.zip/internal/path to path/to/file.zip , internal/path separate
        if path.contains(".zip/") {
            let parts: Vec<&str> = path.splitn(2, ".zip/").collect();
            if parts.len() == 2 {
                let zip_path = format!("{}.zip", parts[0]);
                let internal_path = parts[1];

                let file = File::open(&zip_path)?;
                let mut archive = zip::ZipArchive::new(file)?;

                let mut zip_file = archive.by_name(internal_path)?;

                let mut buffer = Vec::new();
                zip_file.read_to_end(&mut buffer)?;

                return Ok(FileReader::Zip {
                    reader: Box::new(std::io::Cursor::new(buffer)),
                });
            }
        }

        let file = File::open(path)?;
        let reader = BufReader::with_capacity(1024 * 1024, file);
        Ok(FileReader::Regular(reader))
    }

    pub fn into_buf_reader(self) -> Box<dyn BufRead + Send> {
        match self {
            FileReader::Regular(reader) => Box::new(reader),
            FileReader::Zip { reader } => Box::new(BufReader::new(reader)),
        }
    }
}

impl Read for FileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            FileReader::Regular(reader) => reader.read(buf),
            FileReader::Zip { reader } => reader.read(buf),
        }
    }
}

impl BufRead for FileReader {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            FileReader::Regular(reader) => reader.fill_buf(),
            FileReader::Zip { .. } => {
                // For ZIP files, we need to wrap in BufReader
                Err(std::io::Error::other(
                    "Use into_buf_reader() for buffered reading of ZIP files",
                ))
            }
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            FileReader::Regular(reader) => reader.consume(amt),
            FileReader::Zip { .. } => {}
        }
    }
}
