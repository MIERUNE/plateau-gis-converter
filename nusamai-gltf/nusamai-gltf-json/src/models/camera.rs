use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An orthographic camera containing properties to create an orthographic projection matrix.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CameraOrthographic {
    /// The floating-point horizontal magnification of the view. This value MUST NOT be equal to zero. This value SHOULD NOT be negative.
    pub xmag: f64,

    /// The floating-point vertical magnification of the view. This value MUST NOT be equal to zero. This value SHOULD NOT be negative.
    pub ymag: f64,

    /// The floating-point distance to the far clipping plane. This value MUST NOT be equal to zero. zfar MUST be greater than znear.
    pub zfar: f64,

    /// The floating-point distance to the near clipping plane.
    pub znear: f64,
}

/// A perspective camera containing properties to create a perspective projection matrix.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CameraPerspective {
    /// The floating-point aspect ratio of the field of view. When undefined, the aspect ratio of the rendering viewport MUST be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f64>,

    /// The floating-point vertical field of view in radians. This value SHOULD be less than π.
    pub yfov: f64,

    /// The floating-point distance to the far clipping plane. When defined, zfar MUST be greater than znear. If zfar is undefined, client implementations SHOULD use infinite projection matrix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zfar: Option<f64>,

    /// The floating-point distance to the near clipping plane.
    pub znear: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CameraType {
    #[default]
    Perspective,
    Orthographic,
}

/// A camera's projection.  A node MAY reference a camera to apply a transform to place the camera in the scene.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Camera {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// An orthographic camera containing properties to create an orthographic projection matrix. This property MUST NOT be defined when perspective is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orthographic: Option<CameraOrthographic>,

    /// A perspective camera containing properties to create a perspective projection matrix. This property MUST NOT be defined when orthographic is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perspective: Option<CameraPerspective>,

    /// Specifies if the camera uses a perspective or orthographic projection.  Based on this, either the camera's perspective or orthographic property MUST be defined.
    #[serde(rename = "type")]
    pub type_: CameraType,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<CameraExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CameraExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}
