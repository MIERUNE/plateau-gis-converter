//! Configuration mechanism for Sink, Source, and other components.

use std::collections::HashMap;

use indexmap::map::Entry;
use indexmap::IndexMap;
use serde::Serialize;
use thiserror::Error;

#[macro_export]
macro_rules! get_parameter_value {
    ($params:ident, $key:literal, $param_type:ident) => {{
        let entry = $params.get($key).unwrap();
        let ParameterType::$param_type(inner) = &entry.parameter else {
            unreachable!();
        };
        inner.value.as_ref()
    }};
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("required parameter is not provided")]
    RequiredValueNotProvided,
    #[error("invalid value: {0}")]
    InvalidValue(String),
    #[error("Unkown parameter: {0}")]
    UnknownParameter(String),
}

#[derive(Debug, Default, Serialize)]
pub struct Parameters {
    items: IndexMap<String, ParameterEntry>,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            items: IndexMap::new(),
        }
    }
}

impl Parameters {
    pub fn define(&mut self, key: String, item: ParameterEntry) {
        match self.items.entry(key) {
            Entry::Occupied(entry) => {
                panic!("Configuration key={} already exists", entry.key())
            }
            Entry::Vacant(entry) => {
                entry.insert(item);
            }
        }
    }

    /// Returns a reference to the parameter entry corresponding to the key.
    pub fn get(&self, key: &str) -> Option<&ParameterEntry> {
        self.items.get(key)
    }

    /// Returns a mutable reference to the parameter entry corresponding to the key.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut ParameterEntry> {
        self.items.get_mut(key)
    }

    pub fn update_values_with_str<'a>(
        &mut self,
        iter: impl IntoIterator<Item = &'a (String, String)>,
    ) -> Result<(), HashMap<String, Error>> {
        let mut errors = HashMap::new();
        for (key, value) in iter {
            if let Some(entry) = self.items.get_mut(key) {
                if let Err(e) = entry.update_value_with_str(value) {
                    errors.insert(key.into(), e);
                }
            } else {
                errors.insert(key.into(), Error::UnknownParameter(key.into()));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn validate(&mut self) -> Result<(), HashMap<String, Error>> {
        let errors: HashMap<String, Error> = self
            .items
            .iter()
            .flat_map(|(key, value)| {
                if let Err(e) = value.validate() {
                    Some((key.clone(), e))
                } else {
                    None
                }
            })
            .collect();

        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ParameterEntry {
    pub description: String,
    pub required: bool,
    pub parameter: ParameterType,
}

impl ParameterEntry {
    pub fn validate(&self) -> Result<(), Error> {
        match &self.parameter {
            ParameterType::FileSystemPath(p) => p.validate(self.required),
            ParameterType::String(p) => p.validate(self.required),
        }
    }

    /// Updates the parameter value with the given string.
    pub fn update_value_with_str(&mut self, s: &str) -> Result<(), Error> {
        match &mut self.parameter {
            ParameterType::FileSystemPath(p) => p.update_value_with_str(s),
            ParameterType::String(p) => p.update_value_with_str(s),
        }
    }

    /// Updates the parameter value with the JSON Value.
    pub fn update_value_with_json(&mut self, v: &serde_json::Value) -> Result<(), Error> {
        match &mut self.parameter {
            ParameterType::FileSystemPath(p) => p.update_value_with_json(v),
            ParameterType::String(p) => p.update_value_with_json(v),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ParameterType {
    FileSystemPath(FileSystemPathParameter),
    String(StringParameter),
    // and so on ...
}

#[derive(Debug, Serialize)]
pub struct FileSystemPathParameter {
    pub value: Option<std::path::PathBuf>,
    pub must_exist: bool,
}

impl FileSystemPathParameter {
    pub fn validate(&self, required: bool) -> Result<(), Error> {
        if required && self.value.is_none() {
            return Err(Error::RequiredValueNotProvided);
        }
        if let Some(v) = &self.value {
            if self.must_exist && !v.exists() {
                return Err(Error::InvalidValue("Specified path does not exist.".into()));
            }
        }
        Ok(())
    }

    pub fn update_value_with_str(&mut self, s: &str) -> Result<(), Error> {
        self.value = Some(std::path::PathBuf::from(s));
        Ok(())
    }

    pub fn update_value_with_json(&mut self, v: &serde_json::Value) -> Result<(), Error> {
        if let serde_json::Value::String(s) = v {
            self.value = Some(std::path::PathBuf::from(s));
            Ok(())
        } else {
            Err(Error::InvalidValue("Invalid value type.".into()))
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StringParameter {
    pub value: Option<String>,
}

impl StringParameter {
    pub fn validate(&self, required: bool) -> Result<(), Error> {
        if required {
            match &self.value {
                Some(v) if !v.is_empty() => {}
                _ => {
                    return Err(Error::RequiredValueNotProvided);
                }
            }
        }
        Ok(())
    }

    pub fn update_value_with_str(&mut self, s: &str) -> Result<(), Error> {
        self.value = Some(s.into());
        Ok(())
    }

    pub fn update_value_with_json(&mut self, v: &serde_json::Value) -> Result<(), Error> {
        if let serde_json::Value::String(s) = v {
            self.value = Some(s.clone());
            Ok(())
        } else {
            Err(Error::InvalidValue("Invalid value type.".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let mut params = Parameters::new();
        params.define(
            "test".into(),
            ParameterEntry {
                description: "test".into(),
                required: true,
                parameter: ParameterType::String(StringParameter { value: None }),
            },
        );
        params.define(
            "test2".into(),
            ParameterEntry {
                description: "test2".into(),
                required: false,
                parameter: ParameterType::String(StringParameter { value: None }),
            },
        );

        let result = params.validate();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors.contains_key("test"));
    }
}
