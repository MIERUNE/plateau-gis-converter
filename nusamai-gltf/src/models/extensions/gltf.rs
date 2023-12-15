use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Gltf {
    #[serde(rename = "EXT_structural_metadata")]
    pub ext_structural_metadata: ExtStructuralMetadata,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExtStructuralMetadata {
    /// A dictionary object, where each key is the ID of the schema and each value is an object defining the schema.
    pub schemas: Schema,
    pub propertyTables: Vec<PropertyTable>,
}

/// Schema in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    /// Unique identifier for the schema.
    pub id: String,

    /// The name of the schema, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Application-specific version of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// A dictionary, where each key is a class ID and each value is an object defining the class.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub classes: HashMap<String, Class>,

    /// A dictionary, where each key is an enum ID and each value is an object defining the values for the enum.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub enums: HashMap<String, EnumMetadata>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// Class in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Class {
    /// The name of the class, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A dictionary, where each key is a property ID and each value is an object defining the property.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, ClassProperty>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// ElementType enumeration
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum ElementType {
    #[default]
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
    String,
    Boolean,
    Enum,
    // Add other types as needed
}

/// ComponentType enumeration
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ComponentType {
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Float32,
    Float64,
    // Add other types as needed
}

/// Class Property in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClassProperty {
    /// The name of the property, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The element type.
    pub element_type: ElementType,

    /// The datatype of the element's components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<ComponentType>,

    /// Enum ID as declared in the `enums` dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<String>,

    /// Whether the property is an array.
    #[serde(default)]
    pub array: bool,

    /// The number of array elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Specifies whether integer values are normalized.
    #[serde(default)]
    pub normalized: bool,

    /// An offset to apply to property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Value>,

    /// A scale to apply to property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>,

    /// Maximum allowed value for the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Value>,

    /// Minimum allowed value for the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Value>,

    /// If required, the property must be present in every entity.
    #[serde(default)]
    pub required: bool,

    /// A `noData` value represents missing data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_data: Option<Value>,

    /// A default value to use when encountering a `noData` value or an omitted property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,

    /// An identifier that describes how this property should be interpreted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic: Option<String>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// ValueType enumeration
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum ValueType {
    #[default]
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    // Add other types as needed
}

/// Enum in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EnumMetadata {
    /// The name of the enum, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the enum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The type of the integer enum value.
    #[serde(default = "default_value_type")]
    pub value_type: ValueType,

    /// An array of enum values.
    pub values: Vec<EnumValue>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

fn default_value_type() -> ValueType {
    ValueType::Uint16
}

/// Enum Value in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EnumValue {
    /// The name of the enum value.
    pub name: String,

    /// The description of the enum value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The integer enum value.
    pub value: i32, // Adjust the integer type if necessary

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// Property Table in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PropertyTable {
    /// The name of the property table, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The class that property values conform to.
    pub class: String,

    /// The number of elements in each property array.
    pub count: u32,

    /// A dictionary, where each key corresponds to a property ID and each value is an object describing where property values are stored.
    pub properties: HashMap<String, PropertyTableProperty>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// OffsetType enumeration
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum OffsetType {
    #[default]
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    // Add other types as needed
}

/// Property Table Property in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PropertyTableProperty {
    /// The index of the buffer view containing property values.
    pub values: Value, // Adjust the type if necessary

    /// The index of the buffer view containing offsets for variable-length arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_offsets: Option<Value>, // Adjust the type if necessary

    /// The index of the buffer view containing offsets for strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_offsets: Option<Value>, // Adjust the type if necessary

    /// The type of values in `arrayOffsets`.
    #[serde(default = "default_offset_type")]
    pub array_offset_type: OffsetType,

    /// The type of values in `stringOffsets`.
    #[serde(default = "default_offset_type")]
    pub string_offset_type: OffsetType,

    /// An offset to apply to property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Value>, // Adjust the type if necessary

    /// A scale to apply to property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>, // Adjust the type if necessary

    /// Maximum value present in the property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Value>, // Adjust the type if necessary

    /// Minimum value present in the property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Value>, // Adjust the type if necessary

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Value>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

fn default_offset_type() -> OffsetType {
    OffsetType::Uint32
}
