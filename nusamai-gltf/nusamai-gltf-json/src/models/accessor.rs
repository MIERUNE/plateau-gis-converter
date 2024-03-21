use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u16)]
pub enum SparseIndicesComponentType {
    #[default]
    UnsignedByte = 5121,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u16)]
pub enum ComponentType {
    #[default]
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    Float = 5126,
}

/// An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `accessor.sparse.count`. Indices **MUST** strictly increase.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AccessorSparseIndices {
    /// The index of the buffer view with sparse indices. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined. The buffer view and the optional `byteOffset` **MUST** be aligned to the `componentType` byte length.
    pub buffer_view: u32,

    /// The offset relative to the start of the buffer view in bytes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub byte_offset: u32,

    /// The indices data type.
    pub component_type: SparseIndicesComponentType,
}

/// An object pointing to a buffer view containing the deviating accessor values. The number of elements is equal to `accessor.sparse.count` times number of components. The elements have the same component type as the base accessor. The elements are tightly packed. Data **MUST** be aligned following the same rules as the base accessor.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AccessorSparseValues {
    /// The index of the bufferView with sparse values. The referenced buffer view **MUST NOT** have its `target` or `byteStride` properties defined.
    pub buffer_view: u32,

    /// The offset relative to the start of the bufferView in bytes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub byte_offset: u32,
}

/// Sparse storage of accessor values that deviate from their initialization value.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AccessorSparse {
    /// Number of deviating accessor values stored in this sparse structure.
    pub count: u32,

    /// An object pointing to a buffer view containing the indices of deviating accessor values. The number of indices is equal to `count`. Indices **MUST** strictly increase.
    pub indices: AccessorSparseIndices,

    /// An object pointing to a buffer view containing the deviating accessor values.
    pub values: AccessorSparseValues,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccessorType {
    #[default]
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

/// Properties for an accessor. Accessors contain index or attribute data.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde[rename_all = "camelCase"]]
#[serde(deny_unknown_fields)]
pub struct Accessor {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The index of the buffer view with this accessor's data. Buffer view and accessor **MUST** have the same `byteStride`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<u32>,

    /// The offset relative to the start of the buffer view in bytes.  This **MUST** be a multiple of the size of the component datatype. This property **MUST NOT** be defined when `bufferView` is undefined.
    #[serde(default, skip_serializing_if = "is_default")]
    pub byte_offset: u32,

    /// The datatype of the accessor's components.  UNSIGNED_INT type **MUST NOT** be used for any accessor that is not referenced by `mesh.primitive.indices`.
    pub component_type: ComponentType,

    /// Specifies whether integer data values are normalized (`true`) to [0, 1] (for unsigned types) or to [-1, 1] (for signed types) when they are accessed. This property **MUST NOT** be set to `true` for accessors with `FLOAT` or `UNSIGNED_INT` component type.
    #[serde(default, skip_serializing_if = "is_default")]
    pub normalized: bool,

    /// The number of elements referenced by this accessor, not to be confused with the number of bytes or number of components.
    pub count: u32,

    /// Specifies if the accessor's elements are scalars, vectors, or matrices.
    #[serde(rename = "type")]
    pub type_: AccessorType,

    /// Maximum value of each component in this accessor.  Array elements **MUST** be treated as having the same data type as accessor's `componentType`. Both `min` and `max` arrays have the same length.  The length is determined by the value of the `type` property; it can be 1, 2, 3, 4, 9, or 16.
    ///
    /// `normalized` property has no effect on array values: they always correspond to the actual values stored in the buffer. When the accessor is sparse, this property **MUST** contain maximum values of accessor data with sparse substitution applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f64>>,

    /// Minimum value of each component in this accessor.  Array elements **MUST** be treated as having the same data type as accessor's `componentType`. Both `min` and `max` arrays have the same length.  The length is determined by the value of the `type` property; it can be 1, 2, 3, 4, 9, or 16.
    ///
    /// `normalized` property has no effect on array values: they always correspond to the actual values stored in the buffer. When the accessor is sparse, this property **MUST** contain minimum values of accessor data with sparse substitution applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f64>>,

    /// Sparse storage of elements that deviate from their initialization value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse: Option<AccessorSparse>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<AccessorExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessorExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
