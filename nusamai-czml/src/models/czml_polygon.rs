use serde::{Deserialize, Serialize};

use crate::{
    ArcType, ArcTypeValue, ClassificationType, ClassificationTypeValue, Color, ColorProperties,
    CzmlBoolean, CzmlDouble, CzmlInteger, DistanceDisplayCondition, HeightReference,
    HeightReferenceValue, Material, MaterialProperties, PositionList, PositionListOfLists,
    RgbaValue, ShadowMode, ShadowModeValue, SolidColorMaterial,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CzmlPolygon {
    #[serde(default = "default_show", skip_serializing_if = "is_default_show")]
    pub show: CzmlBoolean,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<PositionList>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub holes: Option<PositionListOfLists>,

    #[serde(
        default = "default_arc_type",
        skip_serializing_if = "is_default_arc_type"
    )]
    pub arc_type: ArcType,

    #[serde(default = "default_height", skip_serializing_if = "is_default_height")]
    pub height: CzmlDouble,

    #[serde(
        default = "default_height_reference",
        skip_serializing_if = "is_default_height_reference"
    )]
    pub height_reference: HeightReference,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extruded_height: Option<CzmlDouble>,

    #[serde(
        default = "default_extruded_height_reference",
        skip_serializing_if = "is_default_extruded_height_reference"
    )]
    pub extruded_height_reference: HeightReference,

    #[serde(
        default = "default_st_rotation",
        skip_serializing_if = "is_default_st_rotation"
    )]
    pub st_rotation: CzmlDouble,

    #[serde(
        default = "default_granularity",
        skip_serializing_if = "is_default_granularity"
    )]
    pub granularity: CzmlDouble,

    #[serde(default = "default_fill", skip_serializing_if = "is_default_fill")]
    pub fill: CzmlBoolean,

    #[serde(
        default = "default_material",
        skip_serializing_if = "is_default_material"
    )]
    pub material: Material,

    #[serde(
        default = "default_outline",
        skip_serializing_if = "is_default_outline"
    )]
    pub outline: CzmlBoolean,

    #[serde(
        default = "default_outline_color",
        skip_serializing_if = "is_default_outline_color"
    )]
    pub outline_color: Color,

    #[serde(
        default = "default_outline_width",
        skip_serializing_if = "is_default_outline_width"
    )]
    pub outline_width: CzmlDouble,

    #[serde(
        default = "default_per_position_height",
        skip_serializing_if = "is_default_per_position_height"
    )]
    pub per_position_height: CzmlBoolean,

    #[serde(
        default = "default_close_top",
        skip_serializing_if = "is_default_close_top"
    )]
    pub close_top: CzmlBoolean,

    #[serde(
        default = "default_close_bottom",
        skip_serializing_if = "is_default_close_bottom"
    )]
    pub close_bottom: CzmlBoolean,

    #[serde(
        default = "default_shadows",
        skip_serializing_if = "is_default_shadows"
    )]
    pub shadows: ShadowMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_display_condition: Option<DistanceDisplayCondition>,

    #[serde(
        default = "default_classification_type",
        skip_serializing_if = "is_default_classification_type"
    )]
    pub classification_type: ClassificationType,

    #[serde(
        default = "default_z_index",
        skip_serializing_if = "is_default_z_index"
    )]
    pub z_index: CzmlInteger,
}

fn default_show() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn is_default_show(show: &CzmlBoolean) -> bool {
    *show == default_show()
}

fn default_arc_type() -> ArcType {
    ArcType::String(ArcTypeValue::Geodesic)
}

fn is_default_arc_type(arc_type: &ArcType) -> bool {
    *arc_type == default_arc_type()
}

fn default_height() -> CzmlDouble {
    CzmlDouble::Double(0.0)
}

fn is_default_height(height: &CzmlDouble) -> bool {
    *height == default_height()
}

fn default_height_reference() -> HeightReference {
    HeightReference::String(HeightReferenceValue::None)
}

fn is_default_height_reference(height_reference: &HeightReference) -> bool {
    *height_reference == default_height_reference()
}

fn default_extruded_height_reference() -> HeightReference {
    HeightReference::String(HeightReferenceValue::None)
}

fn is_default_extruded_height_reference(extruded_height_reference: &HeightReference) -> bool {
    *extruded_height_reference == default_extruded_height_reference()
}

fn default_st_rotation() -> CzmlDouble {
    CzmlDouble::Double(0.0)
}

fn is_default_st_rotation(st_rotation: &CzmlDouble) -> bool {
    *st_rotation == default_st_rotation()
}

fn default_granularity() -> CzmlDouble {
    CzmlDouble::Double(0.0174532)
}

fn is_default_granularity(granularity: &CzmlDouble) -> bool {
    *granularity == default_granularity()
}

fn default_fill() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn is_default_fill(fill: &CzmlBoolean) -> bool {
    *fill == default_fill()
}

fn default_material() -> Material {
    Material::Object(Box::new(MaterialProperties {
        solid_color: Some(SolidColorMaterial {
            color: Color::Object(Box::new(ColorProperties {
                rgba: Some(RgbaValue::Constant([255, 255, 255, 255])),
                ..Default::default()
            })),
        }),
        ..Default::default()
    }))
}

fn is_default_material(material: &Material) -> bool {
    *material == default_material()
}

fn default_outline() -> CzmlBoolean {
    CzmlBoolean::Boolean(false)
}

fn is_default_outline(outline: &CzmlBoolean) -> bool {
    *outline == default_outline()
}

fn default_outline_color() -> Color {
    Color::Object(Box::new(ColorProperties {
        rgba: Some(RgbaValue::Constant([0, 0, 0, 255])),
        ..Default::default()
    }))
}

fn is_default_outline_color(outline_color: &Color) -> bool {
    *outline_color == default_outline_color()
}

fn default_outline_width() -> CzmlDouble {
    CzmlDouble::Double(1.0)
}

fn is_default_outline_width(outline_width: &CzmlDouble) -> bool {
    *outline_width == default_outline_width()
}

fn default_per_position_height() -> CzmlBoolean {
    CzmlBoolean::Boolean(false)
}

fn is_default_per_position_height(per_position_height: &CzmlBoolean) -> bool {
    *per_position_height == default_per_position_height()
}

fn default_close_top() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn is_default_close_top(close_top: &CzmlBoolean) -> bool {
    *close_top == default_close_top()
}

fn default_close_bottom() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn is_default_close_bottom(close_bottom: &CzmlBoolean) -> bool {
    *close_bottom == default_close_bottom()
}

fn default_shadows() -> ShadowMode {
    ShadowMode::String(ShadowModeValue::Disabled)
}

fn is_default_shadows(shadows: &ShadowMode) -> bool {
    *shadows == default_shadows()
}

fn default_classification_type() -> ClassificationType {
    ClassificationType::String(ClassificationTypeValue::Both)
}

fn is_default_classification_type(classification_type: &ClassificationType) -> bool {
    *classification_type == default_classification_type()
}

fn default_z_index() -> CzmlInteger {
    CzmlInteger::Integer(0)
}

fn is_default_z_index(z_index: &CzmlInteger) -> bool {
    *z_index == default_z_index()
}

impl Default for CzmlPolygon {
    fn default() -> Self {
        Self {
            show: default_show(),
            positions: None,
            holes: None,
            arc_type: default_arc_type(),
            height: default_height(),
            height_reference: default_height_reference(),
            extruded_height: None,
            extruded_height_reference: default_extruded_height_reference(),
            st_rotation: default_st_rotation(),
            granularity: default_granularity(),
            fill: default_fill(),
            material: default_material(),
            outline: default_outline(),
            outline_color: default_outline_color(),
            outline_width: default_outline_width(),
            per_position_height: default_per_position_height(),
            close_top: default_close_top(),
            close_bottom: default_close_bottom(),
            shadows: default_shadows(),
            distance_display_condition: None,
            classification_type: default_classification_type(),
            z_index: default_z_index(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_deserialize() {
        let polygon: CzmlPolygon = serde_json::from_str("{}").unwrap();
        assert_eq!(polygon.show, CzmlBoolean::Boolean(true));
    }

    #[test]
    fn test_default_serialize() {
        let polygon = CzmlPolygon::default();
        assert_eq!(polygon.show, CzmlBoolean::Boolean(true));

        let json = serde_json::to_string(&polygon).unwrap();
        assert_eq!(json, r#"{}"#)
    }
}
