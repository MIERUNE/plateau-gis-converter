pub mod codelist;
pub mod geometry;
pub mod namespace;
pub mod object;
pub mod parser;
pub mod values;

pub use geometry::*;
pub use macros::*;
pub use namespace::*;
pub use parser::*;
pub use values::*;

pub use object::Value;

pub enum ElementType {
    BasicType,
    FeatureType,
    DataType,
    PropertyType,
}

pub trait CityGMLElement: Sized {
    const ELEMENT_TYPE: ElementType;

    /// Parse a XML fragment into this element.
    fn parse<R: std::io::BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError>;

    /// Convert this element to a `Value` object representation.
    fn into_object(self) -> Option<object::Value>;
}