use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::Translation;

#[derive(Serialize, Deserialize)]
pub struct NodeTransformation {
    #[serde(flatten)]
    pub value: NodeTransformationValueType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeTransformationValueType {
    Array(Vec<NodeTransformationProperties>),
    Object(NodeTransformationProperties),
}

#[derive(Serialize, Deserialize)]
pub struct NodeTransformationProperties {
    pub translation: Option<Translation>,
    pub rotation: Option<Rotation>,
    pub scale: Option<Scale>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeTransformations {
    #[serde(flatten)]
    pub transformations: HashMap<String, NodeTransformation>,
}
