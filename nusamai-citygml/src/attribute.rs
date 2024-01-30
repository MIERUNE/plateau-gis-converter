use crate::parser::ParseError;

pub trait CityGmlAttribute: Sized {
    fn parse_attr_value(value: &str) -> Result<Self, ParseError>;
}

impl CityGmlAttribute for String {
    #[inline]
    fn parse_attr_value(value: &str) -> Result<Self, ParseError> {
        Ok(value.to_string())
    }
}

impl<T: CityGmlAttribute> CityGmlAttribute for Option<T> {
    #[inline]
    fn parse_attr_value(value: &str) -> Result<Self, ParseError> {
        Ok(Some(<T as CityGmlAttribute>::parse_attr_value(value)?))
    }
}
