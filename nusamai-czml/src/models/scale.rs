use serde::{Deserialize, Serialize};

use crate::{
    Cartesian3Value, Cartesian3ValueProperty, DeletableProperty, InterpolatableProperty,
    ReferenceValue, ReferenceValueProperty,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Scale {
    #[serde(flatten)]
    pub value: ScaleValueType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ScaleValueType {
    Array(Vec<ScaleProperties>),
    Object(Box<ScaleProperties>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScaleProperties {
    pub cartesian: Option<Cartesian3Value>,
    pub reference: Option<ReferenceValue>,
    #[serde(flatten)]
    pub interpolatable_property: Option<InterpolatableProperty>,
    #[serde(flatten)]
    pub deletable_property: Option<DeletableProperty>,
    #[serde(flatten)]
    pub distance_display_condition_value_property: Option<Cartesian3ValueProperty>,
    #[serde(flatten)]
    pub reference_value_property: Option<ReferenceValueProperty>,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            value: ScaleValueType::Object(Box::new(ScaleProperties {
                cartesian: Some(Cartesian3Value::Constant([1.0, 1.0, 1.0])),
                reference: None,
                interpolatable_property: None,
                deletable_property: None,
                distance_display_condition_value_property: None,
                reference_value_property: None,
            })),
        }
    }
}
