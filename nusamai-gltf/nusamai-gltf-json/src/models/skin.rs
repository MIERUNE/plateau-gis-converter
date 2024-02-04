use serde::{Deserialize, Serialize};

/// Joints and matrices defining a skin.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde[rename_all = "camelCase"]]
#[serde(deny_unknown_fields)]
pub struct Skin {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The index of the accessor containing the floating-point 4x4 inverse-bind matrices. Its `accessor.count` property **MUST** be greater than or equal to the number of elements of the `joints` array. When undefined, each matrix is a 4x4 identity matrix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse_bind_matrices: Option<u32>,

    /// The index of the node used as a skeleton root. The node **MUST** be the closest common root of the joints hierarchy or a direct or indirect parent node of the closest common root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeleton: Option<u32>,

    /// Indices of skeleton nodes, used as joints in this skin.
    pub joints: Vec<u32>,
}
