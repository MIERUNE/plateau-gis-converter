use std::path::PathBuf;

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
                "failed to join url: {base_url:?} + {code_space:?}"
            )));
        };
        let Ok(path) = abs_url.to_file_path() else {
            return Err(ParseError::CodelistError(format!(
                "failed to convert url to file path: {abs_url:?}",
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
                    "failed to open file: {path:?}"
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
