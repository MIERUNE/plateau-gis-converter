use crate::parser::{ParseError, SubTreeReader};
use std::io::BufRead;

pub trait CityGMLModel: Sized {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError>;
}

impl CityGMLModel for String {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        self.push_str(st.expect_text()?);
        Ok(())
    }
}

impl CityGMLModel for i32 {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.expect_text()?;
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

impl CityGMLModel for f64 {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let text = st.expect_text()?;
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

impl<T: CityGMLModel + Default> CityGMLModel for Option<T> {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        if self.is_some() {
            return Err(ParseError::SchemaViolation(
                "This attribute must not occur two or more times.".into(),
            ));
        }
        let mut v: T = Default::default();
        T::parse(&mut v, st)?;
        *self = Some(v);
        Ok(())
    }
}

impl<T: CityGMLModel + Default> CityGMLModel for Vec<T> {
    fn parse<R: BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError> {
        let mut v: T = Default::default();
        T::parse(&mut v, st)?;
        self.push(v);
        Ok(())
    }
}
