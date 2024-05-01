//! Utility Network
//!
//! - (uro:UtilityNetworkElement)
//!   - (uro:UtilityNode)
//!     - uro:Appurtenance
//!   - (uro:UtilityNodeContainer)
//!     - uro:Manhole
//!     - uro:Handhole
//!   - (uro:UtilityLink)
//!     - uro:Pipe
//!       - uro:WaterPipe
//!       - uro:SewerPipe
//!       - uro:OilGasChemicalsPipe
//!       - uro:ThermalPipe
//!     - uro:Cable
//!       - uro:ElectricityCable
//!       - uro:TelecommunicationsCable
//!     - uro:Duct

use nusamai_citygml::{
    citygml_data, citygml_feature, CityGmlElement, Code, GYear, Length, Measure, Point,
};

use super::*;

#[citygml_feature(name = "uro:Appurtenance")]
pub struct Appurtenance {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:previousLink")]
    pub previous_link: Vec<String>,

    #[citygml(path = b"uro:nextLink")]
    pub next_link: Vec<String>,

    #[citygml(path = b"uro:rotationAngle")]
    pub rotation_angle: Option<f64>,

    #[citygml(path = b"uro:appurtenanceType")]
    pub appurtenance_type: Option<Code>,
}

#[citygml_feature(name = "uro:Cable")]
pub struct Cable {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:columns")]
    pub columns: Option<i64>,

    #[citygml(path = b"uro:rows")]
    pub rows: Option<i64>,

    #[citygml(path = b"uro:cables")]
    pub cables: Option<i64>,
}

#[citygml_feature(name = "uro:Duct")]
pub struct Duct {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "uro:ElectricityCable")]
pub struct ElectricityCable {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:columns")]
    pub columns: Option<i64>,

    #[citygml(path = b"uro:rows")]
    pub rows: Option<i64>,

    #[citygml(path = b"uro:cables")]
    pub cables: Option<i64>,
}

#[citygml_feature(name = "uro:Handhole")]
pub struct Handhole {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:containerType")]
    pub container_type: Option<Code>,

    #[citygml(path = b"uro:innerDiameterLong")]
    pub inner_diameter_long: Option<Length>,

    #[citygml(path = b"uro:outerDiameterLong")]
    pub outer_diameter_long: Option<Length>,

    #[citygml(path = b"uro:innerDiameterShort")]
    pub inner_diameter_short: Option<Length>,

    #[citygml(path = b"uro:outerDiameterShort")]
    pub outer_diameter_short: Option<Length>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:appurtenance")]
    pub appurtenance: Vec<String>,

    #[citygml(path = b"uro:rotationAngle")]
    pub rotation_angle: Option<f64>,
}

#[citygml_feature(name = "uro:Manhole")]
pub struct Manhole {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:containerType")]
    pub container_type: Option<Code>,

    #[citygml(path = b"uro:innerDiameterLong")]
    pub inner_diameter_long: Option<Length>,

    #[citygml(path = b"uro:outerDiameterLong")]
    pub outer_diameter_long: Option<Length>,

    #[citygml(path = b"uro:innerDiameterShort")]
    pub inner_diameter_short: Option<Length>,

    #[citygml(path = b"uro:outerDiameterShort")]
    pub outer_diameter_short: Option<Length>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:appurtenance")]
    pub appurtenance: Vec<String>,

    #[citygml(path = b"uro:rotationAngle")]
    pub rotation_angle: Option<f64>,

    #[citygml(path = b"uro:elevation")]
    pub elevation: Option<Length>,
}

#[citygml_feature(name = "uro:OilGasChemicalsPipe")]
pub struct OilGasChemicalsPipe {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:innerDiameter")]
    pub inner_diameter: Option<Length>,

    #[citygml(path = b"uro:outerDiameter")]
    pub outer_diameter: Option<Length>,

    #[citygml(path = b"uro:sleeveType")]
    pub sleeve_type: Option<Code>,
}

#[citygml_feature(name = "uro:Pipe")]
pub struct Pipe {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:innerDiameter")]
    pub inner_diameter: Option<Length>,

    #[citygml(path = b"uro:outerDiameter")]
    pub outer_diameter: Option<Length>,

    #[citygml(path = b"uro:sleeveType")]
    pub sleeve_type: Option<Code>,
}

#[citygml_feature(name = "uro:SewerPipe")]
pub struct SewerPipe {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:innerDiameter")]
    pub inner_diameter: Option<Length>,

    #[citygml(path = b"uro:outerDiameter")]
    pub outer_diameter: Option<Length>,

    #[citygml(path = b"uro:sleeveType")]
    pub sleeve_type: Option<Code>,

    #[citygml(path = b"uro:slope")]
    pub slope: Option<Measure>,

    #[citygml(path = b"uro:invertElevationUpstream")]
    pub invert_elevation_upstream: Option<Length>,

    #[citygml(path = b"uro:invertElevationDownstream")]
    pub invert_elevation_downstream: Option<Length>,

    #[citygml(path = b"uro:flowDirection")]
    pub flow_direction: Option<bool>,
}

#[citygml_feature(name = "uro:TelecommunicationsCable")]
pub struct TelecommunicationsCable {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:columns")]
    pub columns: Option<i64>,

    #[citygml(path = b"uro:rows")]
    pub rows: Option<i64>,

    #[citygml(path = b"uro:cables")]
    pub cables: Option<i64>,
}

#[citygml_feature(name = "uro:ThermalPipe")]
pub struct ThermalPipe {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:innerDiameter")]
    pub inner_diameter: Option<Length>,

    #[citygml(path = b"uro:outerDiameter")]
    pub outer_diameter: Option<Length>,

    #[citygml(path = b"uro:sleeveType")]
    pub sleeve_type: Option<Code>,
}

#[citygml_feature(name = "uro:WaterPipe")]
pub struct WaterPipe {
    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute/uro:CityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    #[citygml(path = b"uro:frnDataQualityAttribute/uro:DataQualityAttribute")]
    pub frn_data_quality_attribute: Option<DataQualityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:frnFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub frn_key_value_pair_attribute: Vec<KeyValuePairAttribute>,

    #[citygml(path = b"uro:occupierType")]
    pub occupier_type: Option<Code>,

    #[citygml(path = b"uro:occupierName")]
    pub occupier_name: Option<Code>,

    #[citygml(path = b"uro:year")]
    pub year: Option<GYear>,

    #[citygml(path = b"uro:yearType")]
    pub year_type: Option<Code>,

    #[citygml(path = b"uro:administrator")]
    pub administrator: Option<Code>,

    #[citygml(path = b"uro:offsetDepth/uro:OffsetDepth")]
    pub offset_depth: Vec<OffsetDepth>,

    #[citygml(path = b"uro:thematicShape/uro:ThematicShape")]
    pub thematic_shape: Vec<ThematicShape>,

    #[citygml(path = b"uro:routeStartNode")]
    pub route_start_node: Option<String>,

    #[citygml(path = b"uro:startNode")]
    pub start_node: Option<String>,

    #[citygml(path = b"uro:routeEndNode")]
    pub route_end_node: Option<String>,

    #[citygml(path = b"uro:endNode")]
    pub end_node: Option<String>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,

    #[citygml(path = b"uro:maxWidth")]
    pub max_width: Option<Length>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:material")]
    pub material: Option<Code>,

    #[citygml(path = b"uro:lengthAttribute/uro:LengthAttribute")]
    pub length_attribute: Vec<LengthAttribute>,

    #[citygml(path = b"uro:innerDiameter")]
    pub inner_diameter: Option<Length>,

    #[citygml(path = b"uro:outerDiameter")]
    pub outer_diameter: Option<Length>,

    #[citygml(path = b"uro:sleeveType")]
    pub sleeve_type: Option<Code>,
}

#[citygml_data(name = "uro:ThematicShape")]
pub struct ThematicShape {
    #[citygml(path = b"uro:horizontalType")]
    pub horizontal_type: Option<Code>,

    #[citygml(path = b"uro:heightType")]
    pub height_type: Option<Code>,
    //
    // TODO
    // #[citygml(path = b"uro:shape", required)]
    // pub shape: Option<GeometryProperty>, // -> gml:_Geometry
}

#[citygml_data(name = "uro:LengthAttribute")]
pub struct LengthAttribute {
    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:mesureType")]
    pub mesure_type: Option<Code>,

    #[citygml(path = b"uro:phaseType")]
    pub phase_type: Option<Code>,
}

#[citygml_data(name = "uro:OffsetDepth")]
pub struct OffsetDepth {
    #[citygml(path = b"uro:pos")]
    pub pos: Option<Point>,

    #[citygml(path = b"uro:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:minDepth")]
    pub min_depth: Option<Length>,

    #[citygml(path = b"uro:maxDepth")]
    pub max_depth: Option<Length>,
}
