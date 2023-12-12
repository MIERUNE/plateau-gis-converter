use crate::object::ObjectValue;
use crate::parser::{ParseError, SubTreeReader};
use crate::CityGMLElement;
pub use chrono::NaiveDate;
use std::io::BufRead;

impl CityGMLElement for String {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.push_str(st.parse_text()?);
        Ok(())
    }

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::String(self.as_ref()))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct URI(String);
impl CityGMLElement for URI {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.0.push_str(st.parse_text()?);
        Ok(())
    }

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::String(self.0.as_ref()))
    }
}

#[derive(Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Code {
    pub value: String,
    pub code: String,
    // pub code_space: Option<String>,
}
impl CityGMLElement for Code {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let code = st.parse_text()?.to_string();
        self.code = code.to_string();
        // TODO: values must be resolved with external codelists
        self.value = code;
        Ok(())
    }

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::Code(self))
    }
}

impl CityGMLElement for i64 {
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

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::Integer(*self))
    }
}

impl CityGMLElement for u64 {
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

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::Integer(*self as i64))
    }
}

impl CityGMLElement for f64 {
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

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::Double(*self))
    }
}

impl CityGMLElement for bool {
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

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::Boolean(*self))
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Measure {
    value: f64,
    // uom: Option<String>,
}

impl CityGMLElement for Measure {
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

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::Measure(self.value))
    }
}

impl CityGMLElement for NaiveDate {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.parse_text()?;
        match NaiveDate::parse_from_str(text, "%Y-%m-%d") {
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

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::Date(self))
    }
}

#[derive(Debug, Default)]
pub struct Point {
    // TODO
}

impl CityGMLElement for Point {
    #[inline]
    fn parse<R: BufRead>(&mut self, _st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        // TODO
        todo!();
    }

    fn objectify(&self) -> Option<ObjectValue> {
        Some(ObjectValue::Point(self))
    }
}

impl<T: CityGMLElement + Default + std::fmt::Debug> CityGMLElement for Option<T> {
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

    fn objectify(&self) -> Option<ObjectValue> {
        match self {
            Some(v) => v.objectify(),
            None => None,
        }
    }
}

impl<T: CityGMLElement + Default> CityGMLElement for Vec<T> {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let mut v: T = Default::default();
        <T as CityGMLElement>::parse(&mut v, st)?;
        self.push(v);
        Ok(())
    }

    fn objectify(&self) -> Option<ObjectValue> {
        if self.is_empty() {
            None
        } else {
            Some(ObjectValue::Array(
                self.iter().filter_map(|v| v.objectify()).collect(),
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
