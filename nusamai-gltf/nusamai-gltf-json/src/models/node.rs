use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` **MUST** contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node **MAY** have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), `matrix` **MUST NOT** be present.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde[rename_all = "camelCase"]]
#[serde(deny_unknown_fields)]
pub struct Node {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The index of the camera referenced by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<u32>,

    /// The indices of this node's children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<u32>>,

    /// The index of the skin referenced by this node. When a skin is referenced by a node within a scene, all joints used by the skin **MUST** belong to the same scene. When defined, `mesh` **MUST** also be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<u32>,

    /// A floating-point 4x4 transformation matrix stored in column-major order.
    #[serde(default = "default_matrix", skip_serializing_if = "is_default_matrix")]
    pub matrix: [f64; 16],

    /// The index of the mesh in this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<u32>,

    /// The node's unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
    #[serde(
        default = "default_rotation",
        skip_serializing_if = "is_default_rotation"
    )]
    pub rotation: [f64; 4],

    /// The node's non-uniform scale, given as the scaling factors along the x, y, and z axes.
    #[serde(default = "default_scale", skip_serializing_if = "is_default_scale")]
    pub scale: [f64; 3],

    /// The node's translation along the x, y, and z axes.
    #[serde(
        default = "default_translation",
        skip_serializing_if = "is_default_translation"
    )]
    pub translation: [f64; 3],

    /// The weights of the instantiated morph target. The number of array elements **MUST** match the number of morph targets of the referenced mesh. When defined, `mesh` **MUST** also be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<NodeExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            name: None,
            camera: None,
            children: None,
            skin: None,
            matrix: default_matrix(),
            mesh: None,
            rotation: default_rotation(),
            scale: default_scale(),
            translation: default_translation(),
            weights: None,
            extensions: None,
            extras: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NodeExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

fn default_matrix() -> [f64; 16] {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

fn is_default_matrix(matrix: &[f64; 16]) -> bool {
    *matrix == default_matrix()
}

fn default_rotation() -> [f64; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn is_default_rotation(rotation: &[f64; 4]) -> bool {
    *rotation == default_rotation()
}

fn default_scale() -> [f64; 3] {
    [1.0, 1.0, 1.0]
}

fn is_default_scale(rotation: &[f64; 3]) -> bool {
    *rotation == default_scale()
}

fn default_translation() -> [f64; 3] {
    [0., 0., 0.]
}

fn is_default_translation(translation: &[f64; 3]) -> bool {
    *translation == default_translation()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let node: Node = Default::default();
        assert_eq!(
            node.matrix,
            [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(node.translation, [0.0, 0.0, 0.0]);
        assert_eq!(node.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(node.scale, [1.0, 1.0, 1.0]);

        assert_eq!(serde_json::to_string(&node).unwrap(), "{}");

        let node: Node = serde_json::from_str("{}").unwrap();
        assert_eq!(
            node.matrix,
            [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(node.translation, [0.0, 0.0, 0.0]);
        assert_eq!(node.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(node.scale, [1.0, 1.0, 1.0]);
    }
}
