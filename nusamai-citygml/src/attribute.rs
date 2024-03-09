use crate::parser::{ParseContext, ParseError};

pub trait CityGmlAttribute: Sized {
    fn parse_attribute_value(value: &str, st: &mut ParseContext) -> Result<Self, ParseError>;
}

impl CityGmlAttribute for String {
    fn parse_attribute_value(value: &str, _st: &mut ParseContext) -> Result<Self, ParseError> {
        Ok(value.to_string())
    }
}

impl<T: CityGmlAttribute> CityGmlAttribute for Option<T> {
    fn parse_attribute_value(value: &str, st: &mut ParseContext) -> Result<Self, ParseError> {
        Ok(Some(<T as CityGmlAttribute>::parse_attribute_value(
            value, st,
        )?))
    }
}
