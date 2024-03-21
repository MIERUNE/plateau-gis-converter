use nusamai_citygml::{citygml_data, citygml_property, CityGmlElement, Code};

use super::UserDefinedValue;

#[citygml_property(name = "uro:IndoorAttributeProperty")]
pub enum IndoorAttributeProperty {
    #[citygml(path = b"uro:IndoorFacilityAttribute")]
    IndoorFacilityAttribute(IndoorFacilityAttribute),
    #[citygml(path = b"uro:IndoorFurnishingAttribute")]
    IndoorFurnishingAttribute(IndoorFurnishingAttribute),
    #[citygml(path = b"uro:IndoorPublicTagAttribute")]
    IndoorPublicTagAttribute(IndoorPublicTagAttribute),
    #[citygml(path = b"uro:IndoorSpaceAttribute")]
    IndoorSpaceAttribute(IndoorSpaceAttribute),
    #[citygml(path = b"uro:IndoorStoreyAttribute")]
    IndoorStoreyAttribute(IndoorStoreyAttribute),
    #[citygml(path = b"uro:IndoorTacatileTileAttribute")]
    IndoorTacatileTileAttribute(IndoorTacatileTileAttribute),
    #[citygml(path = b"uro:IndoorUserDefinedAttribute")]
    IndoorUserDefinedAttribute(IndoorUserDefinedAttribute),
    #[citygml(path = b"uro:IndoorZoneAttribute")]
    IndoorZoneAttribute(IndoorZoneAttribute),
}

#[citygml_data(name = "uro:IndoorFacilityAttribute")]
pub struct IndoorFacilityAttribute {
    #[citygml(path = b"uro:source")]
    pub source: Option<Code>,

    #[citygml(path = b"uro:weekdayHours")]
    pub weekday_hours: Option<String>,

    #[citygml(path = b"uro:weekendHours")]
    pub weekend_hours: Option<String>,

    #[citygml(path = b"uro:phone")]
    pub phone: Option<String>,

    #[citygml(path = b"uro:website")]
    pub website: Option<String>,
}

#[citygml_data(name = "uro:IndoorFurnishingAttribute")]
pub struct IndoorFurnishingAttribute {
    #[citygml(path = b"uro:source")]
    pub source: Option<Code>,

    #[citygml(path = b"uro:floorId")]
    pub floor_id: Option<String>,
}

#[citygml_data(name = "uro:IndoorPublicTagAttribute")]
pub struct IndoorPublicTagAttribute {
    #[citygml(path = b"uro:source")]
    pub source: Option<Code>,

    #[citygml(path = b"uro:ucode")]
    pub ucode: Option<String>,
}

#[citygml_data(name = "uro:IndoorSpaceAttribute")]
pub struct IndoorSpaceAttribute {
    #[citygml(path = b"uro:source")]
    pub source: Option<Code>,

    #[citygml(path = b"uro:floorId")]
    pub floor_id: Option<String>,

    #[citygml(path = b"uro:isRestricted")]
    pub is_restricted: Option<bool>,

    #[citygml(path = b"uro:suite")]
    pub suite: Option<String>,

    #[citygml(path = b"uro:isPublic")]
    pub is_public: Option<bool>,

    #[citygml(path = b"uro:tollType")]
    pub toll_type: Option<Code>,
}

#[citygml_data(name = "uro:IndoorStoreyAttribute")]
pub struct IndoorStoreyAttribute {
    #[citygml(path = b"uro:source")]
    pub source: Option<Code>,

    #[citygml(path = b"uro:category")]
    pub category: Option<bool>,

    #[citygml(path = b"uro:ordinal")]
    pub ordinal: Option<f64>,
}

#[citygml_data(name = "uro:IndoorTacatileTileAttribute")]
pub struct IndoorTacatileTileAttribute {
    #[citygml(path = b"uro:source")]
    pub source: Option<Code>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:category")]
    pub category: Option<Code>,

    #[citygml(path = b"uro:roof")]
    pub roof: Option<String>,

    #[citygml(path = b"uro:floorId")]
    pub floor_id: Option<String>,
}

#[citygml_data(name = "uro:IndoorUserDefinedAttribute")]
pub struct IndoorUserDefinedAttribute {
    #[citygml(path = b"uro:source")]
    pub source: Option<Code>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:nominalValue/uro:UserDefinedValue")]
    pub nominal_value: Option<UserDefinedValue>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:unit")]
    pub unit: Option<String>,
}

#[citygml_data(name = "uro:IndoorZoneAttribute")]
pub struct IndoorZoneAttribute {
    #[citygml(path = b"uro:source")]
    pub source: Option<Code>,

    #[citygml(path = b"uro:floorId")]
    pub floor_id: Option<String>,
}
