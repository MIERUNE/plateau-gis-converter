use crate::parser::{ParseContext, ParseError};

pub trait CityGMLAttribute: Sized {
    fn parse_attribute_value(value: &str, st: &mut ParseContext) -> Result<Self, ParseError>;
}

impl CityGMLAttribute for String {
    #[inline]
    fn parse_attribute_value(value: &str, _st: &mut ParseContext) -> Result<Self, ParseError> {
        Ok(value.to_string())
    }
}

impl<T: CityGMLAttribute> CityGMLAttribute for Option<T> {
    #[inline]
    fn parse_attribute_value(value: &str, st: &mut ParseContext) -> Result<Self, ParseError> {
        Ok(Some(<T as CityGMLAttribute>::parse_attribute_value(
            value, st,
        )?))
    }
}
