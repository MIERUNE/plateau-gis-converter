use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Read};

use hashbrown::HashMap;
use nusamai_citygml::{codelist::CodeResolver, ParseError};
use stretto::Cache;
use url::Url;

use super::xml::{parse_dictionary, Definition};

pub struct Resolver {
    cache: Cache<PathBuf, HashMap<String, Definition>>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(12960, 100000).unwrap(),
        }
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Resolver {
    fn drop(&mut self) {
        self.cache.close().unwrap();
    }
}

impl CodeResolver for Resolver {
    fn resolve(
        &self,
        base_url: &Url,
        code_space: &str,
        code: &str,
    ) -> Result<Option<String>, nusamai_citygml::ParseError> {
        let Ok(abs_url) = base_url.join(code_space) else {
            return Err(ParseError::CodelistError(format!(
                "failed to join url: {:?} + {:?}",
                base_url, code_space
            )));
        };
        
        let abs_path_str = abs_url.path();
        
        // Check if this is a ZIP file path
        if abs_path_str.contains(".zip/") {
            return self.resolve_from_zip(&abs_path_str, code);
        }
        
        let Ok(path) = abs_url.to_file_path() else {
            return Err(ParseError::CodelistError(format!(
                "failed to convert url to file path: {:?}",
                abs_url,
            )));
        };
        
        if let Some(dict) = self.cache.get(&path) {
            // found in cache
            let v = dict.value().get(code).map(|d| d.value().to_string());
            Ok(v)
        } else {
            // not found in cache
            let Ok(file) = std::fs::File::open(&path) else {
                return Err(ParseError::CodelistError(format!(
                    "failed to open file: {:?}",
                    path
                )));
            };
            let reader = std::io::BufReader::with_capacity(128 * 1024, file);
            let definitions = parse_dictionary(reader)?;

            let v = definitions.get(code).map(|d| d.value().to_string());
            let cost = definitions.len() as i64;
            self.cache.insert(path, definitions, cost);
            self.cache.wait().unwrap();
            Ok(v)
        }
    }
}

impl Resolver {
    fn resolve_from_zip(&self, zip_path: &str, code: &str) -> Result<Option<String>, ParseError> {
        // Parse ZIP path format: "/path/to/file.zip/internal/path/to/codelist.xml"
        let parts: Vec<&str> = zip_path.splitn(2, ".zip/").collect();
        if parts.len() != 2 {
            return Err(ParseError::CodelistError(format!(
                "Invalid ZIP path format: {}",
                zip_path
            )));
        }
        
        let zip_file_path = format!("{}.zip", parts[0]);
        let internal_path = parts[1];
        
        // Use the full zip path as cache key
        let cache_key = PathBuf::from(zip_path);
        
        if let Some(dict) = self.cache.get(&cache_key) {
            // found in cache
            let v = dict.value().get(code).map(|d| d.value().to_string());
            return Ok(v);
        }
        
        // Read from ZIP file
        let file = File::open(&zip_file_path).map_err(|e| {
            ParseError::CodelistError(format!(
                "Failed to open ZIP file {}: {}",
                zip_file_path, e
            ))
        })?;
        
        let mut archive = zip::ZipArchive::new(file).map_err(|e| {
            ParseError::CodelistError(format!(
                "Failed to read ZIP archive {}: {}",
                zip_file_path, e
            ))
        })?;
        
        let mut zip_file = archive.by_name(internal_path).map_err(|e| {
            ParseError::CodelistError(format!(
                "Failed to find file {} in ZIP {}: {}",
                internal_path, zip_file_path, e
            ))
        })?;
        
        // Read the entire file content
        let mut content = Vec::new();
        zip_file.read_to_end(&mut content).map_err(|e| {
            ParseError::CodelistError(format!(
                "Failed to read file {} from ZIP {}: {}",
                internal_path, zip_file_path, e
            ))
        })?;
        
        // Parse the XML
        let reader = BufReader::new(content.as_slice());
        let definitions = parse_dictionary(reader)?;
        
        let v = definitions.get(code).map(|d| d.value().to_string());
        let cost = definitions.len() as i64;
        self.cache.insert(cache_key, definitions, cost);
        self.cache.wait().unwrap();
        Ok(v)
    }
}
