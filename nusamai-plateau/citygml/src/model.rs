use crate::parser::{ParseError, SubTreeReader};
use std::io::BufRead;

pub trait CityGMLElement: Sized {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError>;
}

impl CityGMLElement for String {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.push_str(st.parse_text()?);
        Ok(())
    }
}

impl CityGMLElement for i32 {
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
}

impl<T: CityGMLElement + Default> CityGMLElement for Vec<T> {
    #[inline]
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let mut v: T = Default::default();
        <T as CityGMLElement>::parse(&mut v, st)?;
        self.push(v);
        Ok(())
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
