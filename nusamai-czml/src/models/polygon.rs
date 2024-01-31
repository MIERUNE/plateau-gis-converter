use serde::{Deserialize, Serialize};

use crate::{
    ArcType, Color, CzmlBoolean, CzmlDouble, DistanceDisplayCondition, HeightReference,
    PositionList, PositionListOfLists, ShadowMode,
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
    pub z_index: Integer,
}
