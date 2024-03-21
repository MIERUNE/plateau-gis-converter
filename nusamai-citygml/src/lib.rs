pub mod appearance;
pub mod attribute;
pub mod codelist;
pub mod geometry;
pub mod namespace;
pub mod object;
pub mod parser;
pub mod schema;
pub mod values;

pub use attribute::*;
pub use geometry::*;
pub use macros::*;
pub use namespace::*;
pub use object::Value;
pub use parser::*;
pub use values::*;

pub trait CityGmlElement: Sized {
    /// Parse a XML fragment into this element.
    fn parse<R: std::io::BufRead>(&mut self, st: &mut SubTreeReader<R>) -> Result<(), ParseError>;

    /// Convert this element to a `Value` object representation.
    fn into_object(self) -> Option<object::Value>;

    /// Gets the schema fragment of this element.
    fn collect_schema(schema: &mut schema::Schema) -> schema::Attribute;
}
