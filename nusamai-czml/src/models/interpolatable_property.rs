use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InterpolatableProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub epoch: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_algorithm: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation_degree: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_extrapolation_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_extrapolation_duration: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward_extrapolation_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward_extrapolation_duration: Option<f64>,
}

impl Default for InterpolatableProperty {
    fn default() -> Self {
        Self {
            epoch: Default::default(),
            interpolation_algorithm: Some("LINEAR".to_string()),
            interpolation_degree: Some(1.0),
            forward_extrapolation_type: Some("NONE".to_string()),
            forward_extrapolation_duration: Some(0.0),
            backward_extrapolation_type: Some("NONE".to_string()),
            backward_extrapolation_duration: Some(0.0),
        }
    }
}
