//! Configuration facility for Sink, Source, and other components.

use std::collections::HashMap;

use indexmap::{map::Entry, IndexMap};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[macro_export]
macro_rules! get_parameter_value {
    ($params:ident, $key:literal, $param_type:ident) => {{
        let entry = $params.get($key).unwrap();
        let ParameterType::$param_type(inner) = &entry.parameter else {
            unreachable!();
        };
        &inner.value
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

#[derive(Debug, Default, Serialize, Deserialize)]
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

pub struct ParameterDefinition {
    pub key: String,
    pub entry: ParameterEntry,
}

impl Parameters {
    pub fn define(&mut self, definition: ParameterDefinition) {
        match self.items.entry(definition.key) {
            Entry::Occupied(entry) => {
                panic!("Configuration key={} already exists", entry.key())
            }
            Entry::Vacant(entry) => {
                entry.insert(definition.entry);
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterEntry {
    pub description: String,
    pub required: bool,
    pub parameter: ParameterType,
    pub label: Option<String>,
}

impl ParameterEntry {
    pub fn validate(&self) -> Result<(), Error> {
        match &self.parameter {
            ParameterType::FileSystemPath(p) => p.validate(self.required),
            ParameterType::String(p) => p.validate(self.required),
            ParameterType::Boolean(p) => p.validate(self.required),
            ParameterType::Integer(p) => p.validate(self.required),
        }
    }

    /// Updates the parameter value with a string.
    pub fn update_value_with_str(&mut self, s: &str) -> Result<(), Error> {
        match &mut self.parameter {
            ParameterType::FileSystemPath(p) => p.update_value_with_str(s),
            ParameterType::String(p) => p.update_value_with_str(s),
            ParameterType::Boolean(p) => p.update_value_with_str(s),
            ParameterType::Integer(p) => p.update_value_with_str(s),
        }
    }

    /// Updates the parameter value with the JSON Value.
    pub fn update_value_with_json(&mut self, v: &serde_json::Value) -> Result<(), Error> {
        match &mut self.parameter {
            ParameterType::FileSystemPath(p) => p.update_value_with_json(v),
            ParameterType::String(p) => p.update_value_with_json(v),
            ParameterType::Boolean(p) => p.update_value_with_json(v),
            ParameterType::Integer(p) => p.update_value_with_json(v),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ParameterType {
    FileSystemPath(FileSystemPathParameter),
    String(StringParameter),
    Boolean(BooleanParameter),
    Integer(IntegerParameter),
    // and so on ...
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
            Err(Error::InvalidValue("Value must be a string.".into()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanParameter {
    pub value: Option<bool>,
}

impl BooleanParameter {
    pub fn validate(&self, required: bool) -> Result<(), Error> {
        if required {
            match &self.value {
                Some(_) => Ok(()),
                _ => Err(Error::RequiredValueNotProvided),
            }
        } else {
            Ok(())
        }
    }

    pub fn update_value_with_str(&mut self, s: &str) -> Result<(), Error> {
        let Ok(v) = s.parse::<bool>() else {
            return Err(Error::InvalidValue("Value must be a boolean.".into()));
        };
        self.value = Some(v);
        Ok(())
    }

    pub fn update_value_with_json(&mut self, v: &serde_json::Value) -> Result<(), Error> {
        if let &serde_json::Value::Bool(s) = v {
            self.value = Some(s);
            Ok(())
        } else {
            Err(Error::InvalidValue("Value must be a boolean.".into()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegerParameter {
    pub value: Option<i64>,
    pub min: Option<i64>,
    pub max: Option<i64>,
}

impl IntegerParameter {
    pub fn validate(&self, required: bool) -> Result<(), Error> {
        match self.value {
            Some(v) => {
                if let Some(min) = self.min {
                    if v < min {
                        return Err(Error::InvalidValue(format!(
                            "Value must be greater than or equal to {}.",
                            min
                        )));
                    }
                }
                if let Some(max) = self.max {
                    if v > max {
                        return Err(Error::InvalidValue(format!(
                            "Value must be less than or equal to {}.",
                            max
                        )));
                    }
                }
                Ok(())
            }
            None => {
                if required {
                    return Err(Error::RequiredValueNotProvided);
                }
                Ok(())
            }
        }
    }

    pub fn update_value_with_str(&mut self, s: &str) -> Result<(), Error> {
        let Ok(v) = s.parse::<i64>() else {
            return Err(Error::InvalidValue("Value must be an integer.".into()));
        };
        self.value = Some(v);
        Ok(())
    }

    pub fn update_value_with_json(&mut self, v: &serde_json::Value) -> Result<(), Error> {
        if let serde_json::Value::Number(n) = v {
            let Some(v) = n.as_i64() else {
                return Err(Error::InvalidValue("Value must be an integer.".into()));
            };
            self.value = Some(v);
            Ok(())
        } else {
            Err(Error::InvalidValue("Value must be an integer.".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let mut params = Parameters::new();
        params.define(ParameterDefinition {
            key: "test".into(),
            entry: ParameterEntry {
                description: "test".into(),
                required: true,
                parameter: ParameterType::String(StringParameter { value: None }),
                label: None,
            },
        });
        params.define(ParameterDefinition {
            key: "test2".into(),
            entry: ParameterEntry {
                description: "test2".into(),
                required: false,
                parameter: ParameterType::String(StringParameter { value: None }),
                label: Some("test2".into()),
            },
        });

        let result = params.validate();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors.contains_key("test"));
    }

    #[test]
    fn boolean() {
        let mut params = Parameters::new();
        params.define(ParameterDefinition {
            key: "test".into(),
            entry: ParameterEntry {
                description: "test".into(),
                required: true,
                parameter: ParameterType::Boolean(BooleanParameter { value: None }),
                label: None,
            },
        });
        params.define(ParameterDefinition {
            key: "test2".into(),
            entry: ParameterEntry {
                description: "test2".into(),
                required: false,
                parameter: ParameterType::Boolean(BooleanParameter { value: None }),
                label: Some("test2".into()),
            },
        });

        // validation should fail
        let result = params.validate();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors.contains_key("test"));

        // set proper values
        params
            .update_values_with_str(&vec![("test".into(), "true".into())])
            .unwrap();

        let result = params.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn interger() {
        let mut params = Parameters::new();
        params.define(ParameterDefinition {
            key: "test".into(),
            entry: ParameterEntry {
                description: "test".into(),
                required: true,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: None,
                    min: Some(4),
                    max: Some(10),
                }),
                label: None,
            },
        });
        params.define(ParameterDefinition {
            key: "test2".into(),
            entry: ParameterEntry {
                description: "test2".into(),
                required: false,
                parameter: ParameterType::Integer(IntegerParameter {
                    value: None,
                    min: Some(4),
                    max: Some(10),
                }),
                label: Some("test2".into()),
            },
        });

        // validation should fail
        let result = params.validate();
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors.contains_key("test"));

        // set invalid value
        params
            .update_values_with_str(&vec![("test".into(), "3".into())])
            .unwrap();
        let result = params.validate();
        assert!(result.is_err());

        // set proper value
        params
            .update_values_with_str(&vec![("test".into(), "4".into())])
            .unwrap();
        let result = params.validate();
        assert!(result.is_ok());

        // set invalid value
        params
            .update_values_with_str(&vec![("test2".into(), "11".into())])
            .unwrap();
        let result = params.validate();
        assert!(result.is_err());
    }
}
