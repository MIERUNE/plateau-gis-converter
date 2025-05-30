use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Rotation, Scale, Translation};

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeTransformation {
    #[serde(flatten)]
    pub value: NodeTransformationValueType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NodeTransformationValueType {
    Array(Vec<NodeTransformationProperties>),
    Object(Box<NodeTransformationProperties>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeTransformationProperties {
    pub translation: Option<Translation>,
    pub rotation: Option<Rotation>,
    pub scale: Option<Scale>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeTransformations {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<HashMap<String, NodeTransformation>>,
}
