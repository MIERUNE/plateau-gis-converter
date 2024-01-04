use crate::parser::ParseError;

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
