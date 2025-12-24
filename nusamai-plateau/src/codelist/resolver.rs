use std::path::{Path, PathBuf};

use hashbrown::HashMap;
use nusamai_citygml::{codelist::CodeResolver, ParseError};
use stretto::Cache;
use url::Url;

use super::xml::{parse_dictionary, Definition};

/// Internal error type to distinguish file-not-found from other errors
enum ResolveError {
    UrlJoinFailed,
    NotFilePath,
    FileNotFound,
    IoError(std::io::Error),
    ParseError(ParseError),
}

pub struct Resolver {
    cache: Cache<PathBuf, HashMap<String, Definition>>,
    fallback_base_urls: Vec<Url>,
}

impl Resolver {
    /// Create a new resolver with no fallback paths.
    pub fn new() -> Self {
        Self {
            cache: Cache::new(12960, 100000).unwrap(),
            fallback_base_urls: Vec::new(),
        }
    }

    /// Create a new resolver with fallback base URLs.
    ///
    /// When a codelist file is not found at the primary location (base_url + code_space),
    /// the resolver will try each fallback URL in order, joining it with just the
    /// filename portion of code_space.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let fallback = Url::parse("file:///common/codelists/").unwrap();
    /// let resolver = Resolver::with_fallback(vec![fallback]);
    ///
    /// // If CityGML at file:///data/city.gml references "../codelists/Building_usage.xml"
    /// // and file:///data/../codelists/Building_usage.xml doesn't exist,
    /// // it will try file:///common/codelists/Building_usage.xml
    /// ```
    pub fn with_fallback(fallback_base_urls: Vec<Url>) -> Self {
        // Normalize URLs to ensure they end with '/' for correct join behavior
        let normalized_urls = fallback_base_urls
            .into_iter()
            .map(|url| {
                let s = url.as_str();
                if s.ends_with('/') {
                    url
                } else {
                    Url::parse(&format!("{}/", s)).unwrap_or(url)
                }
            })
            .collect();

        Self {
            cache: Cache::new(12960, 100000).unwrap(),
            fallback_base_urls: normalized_urls,
        }
    }

    /// Internal helper to resolve from a specific URL.
    /// Returns Ok(Some(value)) on success, Ok(None) if code not found in dict,
    /// Err for various failure modes.
    fn try_resolve_from_url(
        &self,
        base_url: &Url,
        code_space: &str,
        code: &str,
    ) -> Result<Option<String>, ResolveError> {
        let abs_url = base_url
            .join(code_space)
            .map_err(|_| ResolveError::UrlJoinFailed)?;

        let path = abs_url
            .to_file_path()
            .map_err(|_| ResolveError::NotFilePath)?;

        // Check cache first
        if let Some(dict) = self.cache.get(&path) {
            return Ok(dict.value().get(code).map(|d| d.value().to_string()));
        }

        // Try to open file
        let file = match std::fs::File::open(&path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(ResolveError::FileNotFound);
            }
            Err(e) => {
                return Err(ResolveError::IoError(e));
            }
        };

        let reader = std::io::BufReader::with_capacity(128 * 1024, file);
        let definitions = parse_dictionary(reader).map_err(ResolveError::ParseError)?;

        let v = definitions.get(code).map(|d| d.value().to_string());
        let cost = definitions.len() as i64;
        self.cache.insert(path, definitions, cost);
        self.cache.wait().unwrap();
        Ok(v)
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
        // Try primary path first
        match self.try_resolve_from_url(base_url, code_space, code) {
            Ok(result) => return Ok(result),
            Err(ResolveError::FileNotFound) => {
                // Continue to fallback
            }
            Err(ResolveError::UrlJoinFailed) => {
                return Err(ParseError::CodelistError(format!(
                    "failed to join url: {base_url:?} + {code_space:?}"
                )));
            }
            Err(ResolveError::NotFilePath) => {
                return Err(ParseError::CodelistError(format!(
                    "failed to convert url to file path: {base_url:?} + {code_space:?}"
                )));
            }
            Err(ResolveError::IoError(e)) => {
                return Err(ParseError::CodelistError(format!(
                    "IO error reading codelist: {e}"
                )));
            }
            Err(ResolveError::ParseError(e)) => {
                return Err(e);
            }
        }

        // Primary path file not found - try fallbacks
        if self.fallback_base_urls.is_empty() {
            let abs_url = base_url.join(code_space).ok();
            return Err(ParseError::CodelistError(format!(
                "codelist file not found: {:?}",
                abs_url
            )));
        }

        // Extract filename from code_space for fallback lookup
        let filename = Path::new(code_space)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or(code_space);

        let mut tried_paths = vec![base_url.join(code_space).ok()];

        for fallback_url in &self.fallback_base_urls {
            match self.try_resolve_from_url(fallback_url, filename, code) {
                Ok(result) => return Ok(result),
                Err(ResolveError::FileNotFound) => {
                    tried_paths.push(fallback_url.join(filename).ok());
                    continue;
                }
                Err(ResolveError::ParseError(e)) => return Err(e),
                Err(_) => continue,
            }
        }

        // All paths exhausted
        Err(ParseError::CodelistError(format!(
            "codelist file not found in any location. code_space: {}, tried: {:?}",
            code_space,
            tried_paths.into_iter().flatten().collect::<Vec<_>>()
        )))
    }
}
