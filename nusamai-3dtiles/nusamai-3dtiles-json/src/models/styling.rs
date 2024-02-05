//! Tileset style files

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde_json::Value;

/// A 3D Tiles style.
#[derive(Serialize, Deserialize, Debug)]
#[serde(default, rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct Style {
    /// A dictionary object of `expression` strings mapped to a variable name key that may be referenced throughout the style. If an expression references a defined variable, it is replaced with the evaluated result of the corresponding expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defines: Option<HashMap<String, Expression>>, // default: None

    pub show: BooleanExpressionOrConditions, // default: "true"

    /// A `color expression` or `conditions` property which determines the color blended with the feature's intrinsic color.
    pub color: ColorExpressionOrConditions, // default: ColorExpression("color('#FFFFFF')")

    /// A `meta` object which determines the values of non-visual properties of the feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,

    /// A `number expression` or `conditions` property which determines the size of the points in pixels.
    pub point_size: NumberExpressionOrConditions, // default: 1.0,

    /// Dictionary object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, Value>>,

    /// Application-specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            defines: None,
            show: BooleanExpressionOrConditions::BooleanExpression(Value::Bool(true)),
            color: ColorExpressionOrConditions::ColorExpression("color('#FFFFFF')".to_string()),
            meta: None,
            point_size: NumberExpressionOrConditions::NumberExpression(
                serde_json::Number::from_f64(1.0).into(),
            ),
            extensions: None,
            extra: None,
        }
    }
}

/// A valid 3D Tiles style expression. Details are described in the 3D Tiles Styling specification.
type Expression = String;

/// A boolean or string with a 3D Tiles style expression that evaluates to a boolean. Details are described in the 3D Tiles Styling specification.
type BooleanExpression = Value; // String | bool

/// 3D Tiles style `expression` that evaluates to a Color. Details are described in the 3D Tiles Styling specification.
type ColorExpression = String;

/// 3D Tiles style expression that evaluates to a number. Details are described in the 3D Tiles Styling specification.
type NumberExpression = Value; // float | str

/// A series of property names and the `expression` to evaluate for the value of that property."""
type Meta = HashMap<String, Expression>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BooleanExpressionOrConditions {
    BooleanExpression(BooleanExpression),
    Conditions(Conditions),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NumberExpressionOrConditions {
    NumberExpression(NumberExpression),
    Conditions(Conditions),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ColorExpressionOrConditions {
    ColorExpression(ColorExpression),
    Conditions(Conditions),
}

/// A series of conditions evaluated in order, like a series of if...else statements that result in an expression being evaluated.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Conditions {
    /// A series of boolean conditions evaluated in order. For the first one that evaluates to true, its value, the 'result' (which is also an expression), is evaluated and returned. Result expressions shall all be the same type. If no condition evaluates to true, the result is `undefined`. When conditions is `undefined`, `null`, or an empty object, the result is `undefined`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<[Expression; 2]>>,
}
