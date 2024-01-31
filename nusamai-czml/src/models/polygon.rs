use serde::{Deserialize, Serialize};

use crate::{
    ArcType, ClassificationType, Color, CzmlBoolean, CzmlDouble, CzmlInteger,
    DistanceDisplayCondition, HeightReference, Material, PositionList, PositionListOfLists,
    RgbaValue, ShadowMode,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Polygon {
    #[serde(default = "default_show")]
    pub show: CzmlBoolean,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<PositionList>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub holes: Option<PositionListOfLists>,

    #[serde(default = "default_arc_type")]
    pub arc_type: ArcType,

    #[serde(default = "default_height")]
    pub height: CzmlDouble,

    #[serde(default = "default_height_reference")]
    pub height_reference: HeightReference,

    pub extruded_height: Option<CzmlDouble>,

    #[serde(default = "default_extruded_height_reference")]
    pub extruded_height_reference: HeightReference,

    #[serde(default = "default_st_rotation")]
    pub st_rotation: CzmlDouble,

    #[serde(default = "default_granularity")]
    pub granularity: CzmlDouble,

    #[serde(default = "default_fill")]
    pub fill: CzmlBoolean,

    #[serde(default = "default_material")]
    pub material: Material,

    #[serde(default = "default_outline")]
    pub outline: CzmlBoolean,

    #[serde(default = "default_outline_color")]
    pub outline_color: Color,

    #[serde(default = "default_outline_width")]
    pub outline_width: CzmlDouble,

    #[serde(default = "default_per_position_height")]
    pub per_position_height: CzmlBoolean,

    #[serde(default = "default_close_top")]
    pub close_top: CzmlBoolean,

    #[serde(default = "default_close_bottom")]
    pub close_bottom: CzmlBoolean,

    #[serde(default = "default_shadows")]
    pub shadows: ShadowMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_display_condition: Option<DistanceDisplayCondition>,

    #[serde(default = "default_classification_type")]
    pub classification_type: ClassificationType,

    #[serde(default = "default_z_index")]
    pub z_index: CzmlInteger,
}

fn default_show() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn default_arc_type() -> ArcType {
    ArcType::Object(ArcType::ArcTypeObject::Geodesic)
}

fn default_height() -> CzmlDouble {
    CzmlDouble::Double(0.0)
}

fn default_height_reference() -> HeightReference {
    HeightReference::Object(HeightReference::HeightReferenceObject::None)
}

fn default_extruded_height_reference() -> HeightReference {
    HeightReference::Object(HeightReference::HeightReferenceObject::None)
}

fn default_st_rotation() -> CzmlDouble {
    CzmlDouble::Double(0.0)
}

fn default_granularity() -> CzmlDouble {
    CzmlDouble::Double(0.0174532)
}

fn default_fill() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn default_material() -> Material {
    Material::Object(Material::MaterialObject::SolidColorMaterial(
        crate::SolidColorMaterial {
            color: Color::ColorObject(crate::Color {
                rgba: RgbaValue::Constant([0, 0, 0, 255]),
            }),
        },
    ))
}

fn default_outline() -> CzmlBoolean {
    CzmlBoolean::Boolean(false)
}

fn default_outline_color() -> Color {
    Color::ColorObject(crate::Color {
        rgba: RgbaValue::Constant([0, 0, 0, 255]),
    })
}

fn default_outline_width() -> CzmlDouble {
    CzmlDouble::Double(1.0)
}

fn default_per_position_height() -> CzmlBoolean {
    CzmlBoolean::Boolean(false)
}

fn default_close_top() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn default_close_bottom() -> CzmlBoolean {
    CzmlBoolean::Boolean(true)
}

fn default_shadows() -> ShadowMode {
    ShadowMode::Object(ShadowMode::ShadowModeObject::Disabled)
}

fn default_classification_type() -> ClassificationType {
    ClassificationType::Object(ClassificationType::ClassificationTypeObject::Both)
}

fn default_z_index() -> CzmlInteger {
    CzmlInteger::Integer(0)
}

impl Default for Polygon {
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
        let polygon: Polygon = serde_json::from_str("{}").unwrap();
    }

    #[test]
    fn test_default_serialize() {
        let polygon = Polygon::default();
        let json = serde_json::to_string(&model).unwrap();
    }
}
