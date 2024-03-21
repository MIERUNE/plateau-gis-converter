use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AnimationSamplerInterpolation {
    #[default]
    Linear,
    Step,
    CubicSpline,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AnimationChannelTargetPath {
    #[default]
    Translation,
    Rotation,
    Scale,
    Weights,
}

/// The descriptor of the animated property.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnimationChannelTarget {
    /// The index of the node to animate. When undefined, the animated object **MAY** be defined by an extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<u32>,
    /// The name of the node's TRS property to animate, or the "weights" of the Morph Targets it instantiates.
    pub path: AnimationChannelTargetPath,
}

/// An animation channel combines an animation sampler with a target property being animated.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnimationChannel {
    /// The index of a sampler in this animation used to compute the value for the target, e.g., a node's translation, rotation, or scale (TRS).
    pub sampler: u32,
    /// The descriptor of the animated property.
    pub target: AnimationChannelTarget,
}

/// An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnimationSampler {
    /// The index of an accessor containing keyframe timestamps. The accessor **MUST** be of scalar type with floating-point components.
    pub input: u32,
    /// Interpolation algorithm.
    #[serde(default, skip_serializing_if = "is_default")]
    pub interpolation: AnimationSamplerInterpolation,
    /// The index of an accessor, containing keyframe output values.
    pub output: u32,
}

/// A keyframe animation.
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Animation {
    /// The user-defined name of this object.  This is not necessarily unique, e.g., an accessor and a buffer could have the same name, or two accessors could even have the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// An array of animation channels. An animation channel combines an animation sampler with a target property being animated.
    pub channels: Vec<AnimationChannel>,

    /// An array of animation samplers. An animation sampler combines timestamps with a sequence of output values and defines an interpolation algorithm.
    pub samplers: Vec<AnimationSampler>,

    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<AnimationExtensions>,

    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnimationExtensions {
    #[serde(flatten)]
    others: HashMap<String, Value>,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
