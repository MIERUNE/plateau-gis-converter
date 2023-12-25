use citygml::{citygml_data, citygml_property, CityGMLElement, Code, GeometryRef, Measure};

#[citygml_property(name = "uro:DmAttributeProperty")]
pub enum DmAttributeProperty {
    #[citygml(path = b"uro:DmAnnotation")]
    DmAnnotation(DmAnnotation),
    #[citygml(path = b"uro:DmGeometricAttribute")]
    DmGeometricAttribute(DmGeometricAttribute),
}

#[citygml_data(name = "uro:DmAnnotation")]
pub struct DmAnnotation {
    #[citygml(path = b"uro:dmCode")]
    pub dm_code: Option<Code>,

    #[citygml(path = b"uro:meshCode")]
    pub mesh_code: Vec<Code>,

    #[citygml(path = b"uro:dmElement/uro:DmElement")]
    pub dm_element: Option<DmElement>,

    #[citygml(path = b"uro:geometryType")]
    pub geometry_type: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:label")]
    pub label: Option<Code>,

    #[citygml(path = b"uro:isVertical")]
    pub is_vertical: Option<bool>,

    #[citygml(path = b"uro:size")]
    pub size: Option<i64>,

    #[citygml(path = b"uro:orientation")]
    pub orientation: Option<i64>,

    #[citygml(path = b"uro:linewidth")]
    pub linewidth: Option<i64>,

    #[citygml(path = b"uro:spacing")]
    pub spacing: Option<i64>,
    // TODO:
    // #[citygml(path = b"uro:lod0anchorPoint")]
    // pub lod0anchorPoint: GeometryPropertyType
}

#[citygml_data(name = "uro:DmGeometricAttribute")]
pub struct DmGeometricAttribute {
    #[citygml(geom = b"uro")]
    pub geometry: GeometryRef,

    #[citygml(path = b"uro:dmCode")]
    pub dm_code: Option<Code>,

    #[citygml(path = b"uro:meshCode")]
    pub mesh_code: Vec<Code>,

    #[citygml(path = b"uro:dmElement/uro:DmElement")]
    pub dm_element: Option<DmElement>,

    #[citygml(path = b"uro:geometryType")]
    pub geometry_type: Option<Code>,

    #[citygml(path = b"uro:mapLevel")]
    pub map_level: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
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
    pub elevation: Option<Measure>,
    // TODO:
    // #[citygml(path = b"uro:lod0Geometry")]
    // pub lod0anchorPoint: GeometryPropertyType
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
    pub creation_date: Option<String>, // TODO: xs:gYearMonth

    #[citygml(path = b"uro:updateDate")]
    pub update_date: Option<String>, // TODO: xs:gYearMonth

    #[citygml(path = b"uro:terminationDate")]
    pub termination_date: Option<String>, // TODO: xs:gYearMonth

    #[citygml(path = b"uro:freeSpace")]
    pub free_space: Option<String>,
}
