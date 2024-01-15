use std::collections::HashMap;

pub struct Schema {
    pub types: HashMap<String, TypeDef>,
}

pub struct TypeDef {
    pub name: String,
    pub ty: Type,
    pub min_occurs: Option<u16>,
    pub max_occurs: Option<u16>,
}

pub enum Type {
    BasicType(BasicType),
    ComplexType(ComplexType),
}

pub struct ComplexType {
    /// name to type mapping
    pub attributes: Vec<TypeDef>,
}

pub enum BasicType {
    String,
    Integer,
    NonNegativeInteger,
    Double,
    Boolean,
    URI,
    Date,
    DataTime,
    Measure,
}
