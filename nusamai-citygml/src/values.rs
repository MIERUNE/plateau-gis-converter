use crate::object::Value;
use crate::parser::{ParseError, SubTreeReader};
use crate::{CityGMLElement, ElementType};
pub use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::BufRead;

pub type Date = chrono::NaiveDate;

impl CityGMLElement for String {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.push_str(st.parse_text()?);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::String(self))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct URI(pub String);
impl fmt::Display for URI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CityGMLElement for URI {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.0.push_str(st.parse_text()?);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::String(self.0))
    }
}

#[derive(Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Code {
    pub value: String,
    pub code: String,
    // pub code_space: Option<String>,
}
impl CityGMLElement for Code {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let code_space = st.find_codespace_attr();
        let code = st.parse_text()?.to_string();
        self.code = code.clone();

        if let Some(code_space) = code_space {
            if let Some(base_url) = st.context().source_url() {
                match st
                    .context()
                    .code_resolver()
                    .resolve(base_url, &code_space, &code)
                {
                    Ok(Some(v)) => {
                        self.value = v;
                        return Ok(());
                    }
                    Ok(None) => {}
                    Err(_) => {
                        return Err(ParseError::InvalidValue(format!(
                            "Failed to resolve code: {} {}",
                            code_space, code
                        )));
                    }
                }
            }
        }
        self.value = code;
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Code(self))
    }
}

impl CityGMLElement for i64 {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected an integer, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Integer(self))
    }
}

impl CityGMLElement for u64 {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected an integer, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Integer(self as i64))
    }
}

impl CityGMLElement for f64 {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected a floating point number, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Double(self))
    }
}

impl CityGMLElement for bool {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?.trim();
        match text {
            "1" | "true" | "True" | "TRUE" => {
                *self = true;
                Ok(())
            }
            "0" | "false" | "False" | "FALSE" => {
                *self = false;
                Ok(())
            }
            _ => Err(ParseError::InvalidValue(format!(
                "Expected a boolean value, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Boolean(self))
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Measure {
    pub value: f64,
    // pub uom: Option<String>,
}

impl CityGMLElement for Measure {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match text.parse() {
            Ok(v) => {
                self.value = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected a floating point number, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Measure(self.value))
    }
}

impl CityGMLElement for Date {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match Date::parse_from_str(text, "%Y-%m-%d") {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(_) => Err(ParseError::InvalidValue(format!(
                "Expected a date in the format YYYY-MM-DD, got {}",
                text
            ))),
        }
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Date(self))
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Point {
    // TODO
}

impl CityGMLElement for Point {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, _st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        // TODO
        todo!();
    }

    fn into_object(self) -> Option<Value> {
        Some(Value::Point(self))
    }
}

impl<T: CityGMLElement + Default + std::fmt::Debug> CityGMLElement for Option<T> {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        if self.is_some() {
            return Err(ParseError::SchemaViolation(format!(
                "{} must not occur two or more times.",
                String::from_utf8_lossy(st.current_path()),
            )));
        }
        let mut v: T = Default::default();
        T::parse(&mut v, st)?;
        *self = Some(v);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        match self {
            Some(v) => v.into_object(),
            None => None,
        }
    }
}

impl<T: CityGMLElement + Default> CityGMLElement for Vec<T> {
    const ELEMENT_TYPE: ElementType = ElementType::BasicType;

    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let mut v: T = Default::default();
        <T as CityGMLElement>::parse(&mut v, st)?;
        self.push(v);
        Ok(())
    }

    fn into_object(self) -> Option<Value> {
        if self.is_empty() {
            None
        } else {
            Some(Value::Array(
                self.into_iter().filter_map(|v| v.into_object()).collect(),
            ))
        }
    }
}

pub trait CityGMLAttribute: Sized {
    fn parse_attr_value(value: &str) -> Result<Self, ParseError>;
}

impl CityGMLAttribute for String {
    #[inline]
    fn parse_attr_value(value: &str) -> Result<Self, ParseError> {
        Ok(value.to_string())
    }
}

impl<T: CityGMLAttribute> CityGMLAttribute for Option<T> {
    #[inline]
    fn parse_attr_value(value: &str) -> Result<Self, ParseError> {
        Ok(Some(<T as CityGMLAttribute>::parse_attr_value(value)?))
    }
}
