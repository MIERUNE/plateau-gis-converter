use url::Url;

use crate::parser::ParseError;

pub trait CodeResolver: Send + Sync {
    fn resolve(
        &self,
        base_url: &Url,
        code_space: &str,
        code: &str,
    ) -> Result<Option<String>, ParseError>;
}

pub struct NoopResolver {}

impl CodeResolver for NoopResolver {
    fn resolve(
        &self,
        _base_url: &Url,
        _code_space: &str,
        _code: &str,
    ) -> Result<Option<String>, ParseError> {
        Ok(None)
    }
}
