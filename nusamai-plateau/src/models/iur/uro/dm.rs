use nusamai_citygml::{
    citygml_data, citygml_feature, citygml_property, CityGmlElement, Code, GYearMonth, Length,
};

#[citygml_property(name = "uro:DmAttributeProperty")]
#[allow(clippy::large_enum_variant)]
pub enum DmAttributeProperty {
    #[citygml(path = b"uro:DmAnnotation")]
    DmAnnotation(DmAnnotation),
    #[citygml(path = b"uro:DmGeometricAttribute")]
    DmGeometricAttribute(DmGeometricAttribute),
}

#[citygml_data(name = "uro:DmAnnotation")]
pub struct DmAnnotation {
    #[citygml(path = b"uro:dmCode", required)]
    pub dm_code: Option<Code>,

    #[citygml(path = b"uro:meshCode")]
    pub mesh_code: Option<Code>,

    #[citygml(path = b"uro:dmElement/uro:DmElement")]
    pub dm_element: Option<DmElement>,

    #[citygml(path = b"uro:geometryType", required)]
    pub geometry_type: Option<Code>,

    #[citygml(path = b"uro:shapeType", required)]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:label", required)]
    pub label: Option<String>,

    #[citygml(path = b"uro:isVertical", required)]
    pub is_vertical: Option<bool>,

    #[citygml(path = b"uro:size", required)]
    pub size: Option<i64>,

    #[citygml(path = b"uro:orientation", required)]
    pub orientation: Option<i64>,

    #[citygml(path = b"uro:linewidth", required)]
    pub linewidth: Option<i64>,

    #[citygml(path = b"uro:spacing", required)]
    pub spacing: Option<i64>,
}

#[citygml_feature(name = "uro:DmGeometricAttribute")]
pub struct DmGeometricAttribute {
    #[citygml(path = b"uro:dmCode", required)]
    pub dm_code: Option<Code>,

    #[citygml(path = b"uro:meshCode")]
    pub mesh_code: Option<Code>,

    #[citygml(path = b"uro:dmElement/uro:DmElement")]
    pub dm_element: Option<DmElement>,

    #[citygml(path = b"uro:geometryType", required)]
    pub geometry_type: Option<Code>,

    #[citygml(path = b"uro:mapLevel", required)]
    pub map_level: Option<Code>,

    #[citygml(path = b"uro:shapeType", required)]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:visibility")]
    pub visibility: Option<bool>,

    #[citygml(path = b"uro:is3d")]
    pub is3d: Option<bool>,

    #[citygml(path = b"uro:isInstallation")]
    pub is_installation: Option<bool>,

    #[citygml(path = b"uro:isEdited")]
    pub is_edited: Option<bool>,

    #[citygml(path = b"uro:isSupplementarySymbol")]
    pub is_supplementary_symbol: Option<bool>,

    #[citygml(path = b"uro:angle")]
    pub angle: Option<f64>,

    #[citygml(path = b"uro:elevation")]
    pub elevation: Option<Length>,
}
#[citygml_data(name = "uro:DmElement")]
pub struct DmElement {
    #[citygml(path = b"uro:locationType")]
    pub location_type: Option<Code>,

    #[citygml(path = b"uro:infoType")]
    pub info_type: Option<Code>,

    #[citygml(path = b"uro:elementKey")]
    pub element_key: Option<String>,

    #[citygml(path = b"uro:hierarchyLevel")]
    pub hierarchy_level: Option<String>,

    #[citygml(path = b"uro:dataType")]
    pub data_type: Option<Code>,

    #[citygml(path = b"uro:annotationType")]
    pub annotation_type: Option<Code>,

    #[citygml(path = b"uro:precisionType")]
    pub precision_type: Option<Code>,

    #[citygml(path = b"uro:dislocationType")]
    pub dislocation_type: Option<Code>,

    #[citygml(path = b"uro:breakType")]
    pub break_type: Option<Code>,

    #[citygml(path = b"uro:attributeValue")]
    pub attribute_value: Option<String>,

    #[citygml(path = b"uro:attributeType")]
    pub attribute_type: Option<Code>,

    #[citygml(path = b"uro:attributeValueType")]
    pub attribute_value_type: Option<String>,

    #[citygml(path = b"uro:creationDate")]
    pub creation_date: Option<GYearMonth>,

    #[citygml(path = b"uro:updateDate")]
    pub update_date: Option<GYearMonth>,

    #[citygml(path = b"uro:terminationDate")]
    pub termination_date: Option<GYearMonth>,

    #[citygml(path = b"uro:freeSpace")]
    pub free_space: Option<String>,
}
