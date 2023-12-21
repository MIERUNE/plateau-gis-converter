//! EXT_structural_metadata
//!
//! https://github.com/CesiumGS/glTF/tree/3d-tiles-next/extensions/2.0/Vendor/EXT_structural_metadata

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExtStructuralMetadata {
    /// An object defining classes and enums.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,

    /// The URI (or IRI) of the external schema file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_uri: Option<String>,

    /// An array of property table definitions, which may be referenced by index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_tables: Option<Vec<PropertyTable>>,

    /// An array of indexes of property textures in the root `EXT_structural_metadata` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_textures: Option<Vec<PropertyTexture>>,

    /// An array of indexes of property attributes in the root `EXT_structural_metadata` object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_attributes: Option<Vec<PropertyAttribute>>,
}

/// Schema in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub classes: HashMap<String, Class>,

    /// A dictionary, where each key is an enum ID and each value is an object defining the values for the enum.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub enums: HashMap<String, Enum>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// Class in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Class {
    /// The name of the class, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A dictionary, where each key is a property ID and each value is an object defining the property.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, ClassProperty>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// ElementType enumeration
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum ClassPropertyType {
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
}

/// ComponentType enumeration
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ClassPropertyComponentType {
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float32,
    Float64,
}

/// Class Property in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ClassProperty {
    /// The name of the property, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The element type.
    #[serde(rename = "type")]
    pub type_: ClassPropertyType,

    /// The datatype of the element's components. Only applicable to `SCALAR`, `VECN`, and `MATN` types."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<ClassPropertyComponentType>,

    /// Enum ID as declared in the `enums` dictionary. Required when `type` is `ENUM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<String>,

    /// Whether the property is an array. When `count` is defined the property is a fixed-length array. Otherwise the property is a variable-length array.
    #[serde(default)]
    pub array: bool,

    /// The number of array elements. May only be defined when `array` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    /// Specifies whether integer values are normalized. Only applicable to `SCALAR`, `VECN`, and `MATN` types with integer component types. For unsigned integer component types, values are normalized between `[0.0, 1.0]`. For signed integer component types, values are normalized between `[-1.0, 1.0]`. For all other component types, this property must be false.
    #[serde(default)]
    pub normalized: bool,

    /// An offset to apply to property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Value>, // definitions.schema.json#/definitions/numericValue

    /// A scale to apply to property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>, // definitions.schema.json#/definitions/numericValue

    /// Maximum allowed value for the property. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the maximum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Value>, // definitions.schema.json#/definitions/numericValue

    /// Minimum allowed value for the property. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the minimum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Value>, // definitions.schema.json#/definitions/numericValue

    /// If required, the property must be present in every entity conforming to the class. If not required, individual entities may include `noData` values, or the entire property may be omitted. As a result, `noData` has no effect on a required property. Client implementations may use required properties to make performance optimizations.
    #[serde(default)]
    pub required: bool,

    /// A `noData` value represents missing data — also known as a sentinel value — wherever it appears. `BOOLEAN` properties may not specify `noData` values. This is given as the plain property value, without the transforms from the `normalized`, `offset`, and `scale` properties. Must not be defined if `required` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_data: Option<Value>, // definitions.schema.json#/definitions/noDataValue

    /// A default value to use when encountering a `noData` value or an omitted property. The value is given in its final form, taking the effect of `normalized`, `offset`, and `scale` properties into account. Must not be defined if `required` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>, // definitions.schema.json#/definitions/anyValue

    /// An identifier that describes how this property should be interpreted. The semantic cannot be used by other properties in the class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic: Option<String>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// ValueType enumeration
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum EnumValueType {
    Int8,
    UInt8,
    Int16,
    #[default]
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
}

/// Enum in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Enum {
    /// The name of the enum, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The description of the enum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The type of the integer enum value.
    #[serde(default)]
    pub value_type: EnumValueType,

    /// An array of enum values.
    pub values: Vec<EnumValue>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// Enum Value in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EnumValue {
    /// The name of the enum value.
    pub name: String,

    /// The description of the enum value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The integer enum value.
    pub value: i32,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

/// Property Table in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PropertyTable {
    /// The name of the property table, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The class that property values conform to.
    pub class: String,

    /// The number of elements in each property array.
    pub count: u32,

    /// A dictionary, where each key corresponds to a property ID and each value is an object describing where property values are stored.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, PropertyTableProperty>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PropertyTexture {
    /// The name of the property attribute, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// The class that property values conform to. The value must be a class ID declared in the `classes` dictionary.
    class: String,

    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value is an object describing where property values are stored. Required properties must be included in this dictionary.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    properties: HashMap<String, PropertyTextureProperty>,
}

/// A texture containing property values.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyTextureProperty {
    /// Texture channels containing property values, identified by index. The values may be packed into multiple channels if a single channel does not have sufficient bit depth. The values are packed in little-endian order."
    #[serde(default = "default_channels")]
    pub channels: Vec<u32>,

    /// An offset to apply to property values. Only applicable when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`. Overrides the class property's `offset` if both are defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Value>, // FIXME:

    /// A scale to apply to property values. Only applicable when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`. Overrides the class property's `scale` if both are defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<Value>, // FIXME:

    /// Maximum value present in the property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the maximum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied."
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Value>, // FIXME:

    /// Minimum value present in the property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the minimum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Value>, // FIXME:
}

fn default_channels() -> Vec<u32> {
    vec![0]
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PropertyAttribute {
    /// The name of the property attribute, e.g. for display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    /// The class that property values conform to. The value must be a class ID declared in the `classes` dictionary.
    class: String,

    /// A dictionary, where each key corresponds to a property ID in the class' `properties` dictionary and each value is an object describing where property values are stored. Required properties must be included in this dictionary.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    properties: HashMap<String, PropertyAttributeProperty>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyAttributeProperty {
    /// The name of the attribute containing property values.
    attribute: String,

    /// An offset to apply to property values. Only applicable when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`. Overrides the class property's `offset` if both are defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Value>, // FIXME:

    /// A scale to apply to property values. Only applicable when the component type is `FLOAT32` or `FLOAT64`, or when the property is `normalized`. Overrides the class property's `scale` if both are defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<Value>, // FIXME:

    /// Maximum value present in the property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the maximum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Value>, // FIXME:

    /// Minimum value present in the property values. Only applicable to `SCALAR`, `VECN`, and `MATN` types. This is the minimum of all property values, after the transforms based on the `normalized`, `offset`, and `scale` properties have been applied."
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Value>, // FIXME:
}

/// OffsetType enumeration
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum OffsetType {
    UInt8,
    UInt16,
    #[default]
    UInt32,
    UInt64,
}

/// Property Table Property in EXT_structural_metadata
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PropertyTableProperty {
    /// The index of the buffer view containing property values.
    pub values: u32,

    /// The index of the buffer view containing offsets for variable-length arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_offsets: Option<u32>,

    /// The index of the buffer view containing offsets for strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_offsets: Option<u32>,

    /// The type of values in `arrayOffsets`.
    #[serde(default)]
    pub array_offset_type: OffsetType,

    /// The type of values in `stringOffsets`.
    #[serde(default)]
    pub string_offset_type: OffsetType,

    /// An offset to apply to property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Value>, // definitions.schema.json#/definitions/numericValue

    /// A scale to apply to property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Value>, // definitions.schema.json#/definitions/numericValue

    /// Maximum value present in the property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Value>, // definitions.schema.json#/definitions/numericValue

    /// Minimum value present in the property values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Value>, // definitions.schema.json#/definitions/numericValue

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}
