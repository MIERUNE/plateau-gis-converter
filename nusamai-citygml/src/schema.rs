use std::collections::HashMap;

pub struct Schema {
    pub types: HashMap<String, StereoType>,
}

pub enum StereoType {
    DataType(DataTypeDef),
    FeatureType(FeatureTypeDef),
    BasicType(BasicTypeDef),
}

pub struct Attribute {
    pub name: String,
    pub ty: String,
    pub min_occurs: Option<u16>,
    pub max_occurs: Option<u16>,
}

pub struct DataTypeDef {
    /// name to type
    pub attributes: Vec<Attribute>,
}

pub struct FeatureTypeDef {
    /// name to type
    pub attributes: Vec<Attribute>,
}

pub struct BasicTypeDef {
    pub ty: BasicType,
    /// scalar (false) or array (true)
    pub multiple: bool,
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
