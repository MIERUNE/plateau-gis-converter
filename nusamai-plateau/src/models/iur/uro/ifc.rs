use nusamai_citygml::{
    citygml_data, citygml_property, CityGmlElement, Code, Date, DoubleList, GYear, Measure, Point,
    Uri,
};

use crate::models::core::Address;

// TODO?
type IfcElementCompositionEnum = String;
type IfcTransportElementTypeEnum = String;
type IfcInternalOrExternalEnum = String;
type IfcUnitEnum = String;

#[citygml_property(name = "uro:IfcCoordinateReferenceSystemSelectType")]
pub enum IfcCoordinateReferenceSystemSelectType {
    #[citygml(path = b"uro:crs")]
    CRS(IfcCoordinateReferenceSystemProperty),
    #[citygml(path = b"uro:context/uro:IfcGeometricRepresentationContext")]
    Context(IfcGeometricRepresentationContext),
}

#[citygml_property(name = "uro:IfcCoordinateReferenceSystemProperty")]
pub enum IfcCoordinateReferenceSystemProperty {
    #[citygml(path = b"uro:IfcCoordinateReferenceSystem")]
    IfcCoordinateReferenceSystem(IfcCoordinateReferenceSystem),
    #[citygml(path = b"uro:IfcProjectedCRS")]
    IfcProjectedCRS(IfcProjectedCRS),
}

#[citygml_data(name = "uro:IfcCoordinateReferenceSystem")]
pub struct IfcCoordinateReferenceSystem {
    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:geodeticDatum")]
    pub geodetic_datum: Option<String>,

    #[citygml(path = b"uro:verticalDatum")]
    pub vertical_datum: Option<String>,
}

#[citygml_data(name = "uro:IfcProjectedCRS")]
pub struct IfcProjectedCRS {
    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:geodeticDatum")]
    pub geodetic_datum: Option<String>,

    #[citygml(path = b"uro:verticalDatum")]
    pub vertical_datum: Option<String>,

    #[citygml(path = b"uro:mapUnit")]
    pub map_unit: Option<String>,

    #[citygml(path = b"uro:mapProjection")]
    pub map_projection: Option<String>,

    #[citygml(path = b"uro:mapZone")]
    pub map_zone: Option<String>,
}

#[citygml_data(name = "uro:IfcGeometricRepresentationContext")]
pub struct IfcGeometricRepresentationContext {
    #[citygml(path = b"uro:contextIdentifier")]
    pub context_identifier: Option<String>,

    #[citygml(path = b"uro:contextType")]
    pub context_type: Option<String>,

    #[citygml(path = b"uro:coordinateSpaceDimension")]
    pub coordinate_space_dimension: Option<i64>,

    #[citygml(path = b"uro:precision")]
    pub precision: Option<f64>,

    #[citygml(path = b"uro:worldCoordinateSystem/uro:IfcAxis2Placement3D", required)]
    pub world_coordinate_system: Option<IfcAxis2Placement3D>,

    #[citygml(path = b"uro:trueNorth")]
    pub true_north: Option<DoubleList>,
}

#[citygml_property(name = "uro:IfcAttributeProperty")]
pub enum IfcAttributeProperty {
    #[citygml(path = b"uro:IfcBuilding")]
    IfcBuilding(IfcBuilding),
    #[citygml(path = b"uro:IfcBuildingElement")]
    IfcBuildingElement(IfcBuildingElement),
    #[citygml(path = b"uro:IfcBuildingStorey")]
    IfcBuildingStorey(IfcBuildingStorey),
    #[citygml(path = b"uro:IfcClassificationReference")]
    IfcClassificationReference(IfcClassificationReference),
    #[citygml(path = b"uro:IfcCoordinateReferenceSystem")]
    IfcCoordinateReferenceSystem(IfcCoordinateReferenceSystem),
    #[citygml(path = b"uro:IfcCurtainWall")]
    IfcCurtainWall(IfcCurtainWall),
    #[citygml(path = b"uro:IfcDoor")]
    IfcDoor(IfcDoor),
    #[citygml(path = b"uro:IfcFurnishingElement")]
    IfcFurnishingElement(IfcFurnishingElement),
    #[citygml(path = b"uro:IfcGroup")]
    IfcGroup(IfcGroup),
    #[citygml(path = b"uro:IfcMapConversion")]
    IfcMapConversion(IfcMapConversion),
    #[citygml(path = b"uro:IfcOpeningElement")]
    IfcOpeningElement(IfcOpeningElement),
    #[citygml(path = b"uro:IfcProject")]
    IfcProject(IfcProject),
    #[citygml(path = b"uro:IfcProjectedCRS")]
    IfcProjectedCRS(IfcProjectedCRS),
    #[citygml(path = b"uro:IfcPsetBuildingCommon")]
    IfcPsetBuildingCommon(IfcPsetBuildingCommon),
    #[citygml(path = b"uro:IfcPsetDoorCommon")]
    IfcPsetDoorCommon(IfcPsetDoorCommon),
    #[citygml(path = b"uro:IfcPsetOpeningElementCommon")]
    IfcPsetOpeningElementCommon(IfcPsetOpeningElementCommon),
    #[citygml(path = b"uro:IfcPsetSiteCommon")]
    IfcPsetSiteCommon(IfcPsetSiteCommon),
    #[citygml(path = b"uro:IfcPsetSpaceCommon")]
    IfcPsetSpaceCommon(IfcPsetSpaceCommon),
    #[citygml(path = b"uro:IfcPsetWindowCommon")]
    IfcPsetWindowCommon(IfcPsetWindowCommon),
    #[citygml(path = b"uro:IfcRoof")]
    IfcRoof(IfcRoof),
    #[citygml(path = b"uro:IfcSite")]
    IfcSite(IfcSite),
    #[citygml(path = b"uro:IfcSpace")]
    IfcSpace(IfcSpace),
    #[citygml(path = b"uro:IfcSpaceBaseQuantity")]
    IfcSpaceBaseQuantity(IfcSpaceBaseQuantity),
    #[citygml(path = b"uro:IfcWall")]
    IfcWall(IfcWall),
    #[citygml(path = b"uro:IfcWallStandardCase")]
    IfcWallStandardCase(IfcWallStandardCase),
    #[citygml(path = b"uro:IfcWindow")]
    IfcWindow(IfcWindow),
    #[citygml(path = b"uro:IfcZone")]
    IfcZone(IfcZone),
}

#[citygml_data(name = "uro:IfcBuilding")]
pub struct IfcBuilding {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:longName")]
    pub long_name: Option<String>,

    #[citygml(path = b"uro:compositionType")]
    pub composition_type: Option<IfcElementCompositionEnum>,

    #[citygml(path = b"uro:elevationOfRefHeight")]
    pub elevation_of_ref_height: Option<Measure>,

    #[citygml(path = b"uro:elevationOfTerrain")]
    pub elevation_of_terrain: Option<Measure>,

    #[citygml(path = b"uro:buildingAddress/core:Address")]
    pub building_address: Option<Address>,
}

#[citygml_data(name = "uro:IfcBuildingElement")]
pub struct IfcBuildingElement {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,

    #[citygml(path = b"uro:elementType")]
    pub element_type: Option<Code>,

    #[citygml(path = b"uro:predefinedType")]
    pub predefined_type: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:numberOfRiser")]
    pub number_of_riser: Option<i64>,

    #[citygml(path = b"uro:numberOfTreads")]
    pub number_of_treads: Option<i64>,

    #[citygml(path = b"uro:riserHeight")]
    pub riser_height: Option<Measure>,

    #[citygml(path = b"uro:treadLength")]
    pub tread_length: Option<Measure>,

    #[citygml(path = b"uro:operationType")]
    pub operation_type: Option<IfcTransportElementTypeEnum>,

    #[citygml(path = b"uro:capacityByWeight")]
    pub capacity_by_weight: Option<Measure>,

    #[citygml(path = b"uro:capacityByNumber")]
    pub capacity_by_number: Option<i64>,
}

#[citygml_data(name = "uro:IfcBuildingStorey")]
pub struct IfcBuildingStorey {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:longName")]
    pub long_name: Option<String>,

    #[citygml(path = b"uro:compositionType")]
    pub composition_type: Option<IfcElementCompositionEnum>,

    #[citygml(path = b"uro:elevation")]
    pub elevation: Option<Measure>,
}

#[citygml_data(name = "uro:IfcClassificationReference")]
pub struct IfcClassificationReference {
    #[citygml(path = b"uro:location")]
    pub location: Option<Uri>,

    #[citygml(path = b"uro:itemReference")]
    pub item_reference: Option<Code>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:referenceSource/uro:IfcClassification")]
    pub reference_source: Option<IfcClassification>,
}

#[citygml_data(name = "uro:IfcCurtainWall")]
pub struct IfcCurtainWall {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,

    #[citygml(path = b"uro:elementType")]
    pub element_type: Option<Code>,

    #[citygml(path = b"uro:predefinedType")]
    pub predefined_type: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:numberOfRiser")]
    pub number_of_riser: Option<i64>,

    #[citygml(path = b"uro:numberOfTreads")]
    pub number_of_treads: Option<i64>,

    #[citygml(path = b"uro:riserHeight")]
    pub riser_height: Option<Measure>,

    #[citygml(path = b"uro:treadLength")]
    pub tread_length: Option<Measure>,

    #[citygml(path = b"uro:operationType")]
    pub operation_type: Option<IfcTransportElementTypeEnum>,

    #[citygml(path = b"uro:capacityByWeight")]
    pub capacity_by_weight: Option<Measure>,

    #[citygml(path = b"uro:capacityByNumber")]
    pub capacity_by_number: Option<i64>,
}

#[citygml_data(name = "uro:IfcDoor")]
pub struct IfcDoor {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,

    #[citygml(path = b"uro:elementType")]
    pub element_type: Option<Code>,

    #[citygml(path = b"uro:predefinedType")]
    pub predefined_type: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:numberOfRiser")]
    pub number_of_riser: Option<i64>,

    #[citygml(path = b"uro:numberOfTreads")]
    pub number_of_treads: Option<i64>,

    #[citygml(path = b"uro:riserHeight")]
    pub riser_height: Option<Measure>,

    #[citygml(path = b"uro:treadLength")]
    pub tread_length: Option<Measure>,

    #[citygml(path = b"uro:operationType")]
    pub operation_type: Option<IfcTransportElementTypeEnum>,

    #[citygml(path = b"uro:capacityByWeight")]
    pub capacity_by_weight: Option<Measure>,

    #[citygml(path = b"uro:capacityByNumber")]
    pub capacity_by_number: Option<i64>,

    #[citygml(path = b"uro:overallHeight")]
    pub overall_height: Option<Measure>,

    #[citygml(path = b"uro:overallWidth")]
    pub overall_width: Option<Measure>,
}

#[citygml_data(name = "uro:IfcFurnishingElement")]
pub struct IfcFurnishingElement {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,
}

#[citygml_data(name = "uro:IfcGroup")]
pub struct IfcGroup {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,
}

#[citygml_data(name = "uro:IfcMapConversion")]
pub struct IfcMapConversion {
    #[citygml(path = b"uro:sourceCRS")]
    pub source_crs: Option<IfcCoordinateReferenceSystemSelectType>,

    #[citygml(path = b"uro:targetCRS")]
    pub target_crs: Option<IfcCoordinateReferenceSystemProperty>, // -> uro:IfcCoordinateReferenceSystem

    #[citygml(path = b"uro:eastings")]
    pub eastings: Option<Measure>,

    #[citygml(path = b"uro:northings")]
    pub northings: Option<Measure>,

    #[citygml(path = b"uro:orthogonalHeight")]
    pub orthogonal_height: Option<Measure>,

    #[citygml(path = b"uro:xAxisAbscissa")]
    pub x_axis_abscissa: Option<f64>,

    #[citygml(path = b"uro:xAxisOrdinate")]
    pub x_axis_ordinate: Option<f64>,

    #[citygml(path = b"uro:scale")]
    pub scale: Option<f64>,
}

#[citygml_data(name = "uro:IfcOpeningElement")]
pub struct IfcOpeningElement {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,

    #[citygml(path = b"uro:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"uro:nominalVolume")]
    pub nominal_volume: Option<Measure>,
}

#[citygml_data(name = "uro:IfcProject")]
pub struct IfcProject {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:longName")]
    pub long_name: Option<String>,

    #[citygml(path = b"uro:phase")]
    pub phase: Option<String>,

    #[citygml(path = b"uro:representationContexts/uro:IfcGeometricRepresentationContext")]
    pub representation_contexts: Option<IfcGeometricRepresentationContext>,

    #[citygml(path = b"uro:unitsInContext/uro:IfcUnit")]
    pub units_in_context: Vec<IfcUnit>,
}

#[citygml_data(name = "uro:IfcPsetBuildingCommon")]
pub struct IfcPsetBuildingCommon {
    #[citygml(path = b"uro:buildingId")]
    pub building_id: Option<String>,

    #[citygml(path = b"uro:isPermanentId")]
    pub is_permanent_id: Option<bool>,

    #[citygml(path = b"uro:mainFireUse")]
    pub main_fire_use: Option<String>,

    #[citygml(path = b"uro:ancillaryFireUse")]
    pub ancillary_fire_use: Option<String>,

    #[citygml(path = b"uro:sprinklerProtection")]
    pub sprinkler_protection: Option<bool>,

    #[citygml(path = b"uro:sprinklerProtectionAutomatic")]
    pub sprinkler_protection_automatic: Option<bool>,

    #[citygml(path = b"uro:occupancyType")]
    pub occupancy_type: Option<Code>,

    #[citygml(path = b"uro:grossPlannedArea")]
    pub gross_planned_area: Option<Measure>,

    #[citygml(path = b"uro:numberOfStoreys")]
    pub number_of_storeys: Option<i64>,

    #[citygml(path = b"uro:yearOfConstruction")]
    pub year_of_construction: Option<GYear>,

    #[citygml(path = b"uro:isLandmarked")]
    pub is_landmarked: Option<bool>,
}

#[citygml_data(name = "uro:IfcPsetDoorCommon")]
pub struct IfcPsetDoorCommon {
    #[citygml(path = b"uro:reference")]
    pub reference: Option<String>,

    #[citygml(path = b"uro:acousticRating")]
    pub acoustic_rating: Option<String>,

    #[citygml(path = b"uro:firerating")]
    pub firerating: Option<String>,

    #[citygml(path = b"uro:securityRating")]
    pub security_rating: Option<String>,

    #[citygml(path = b"uro:isExternal")]
    pub is_external: Option<bool>,

    #[citygml(path = b"uro:infiltration")]
    pub infiltration: Option<f64>,

    #[citygml(path = b"uro:thermalTransmittance")]
    pub thermal_transmittance: Option<f64>,

    #[citygml(path = b"uro:glazingAreaFraction")]
    pub glazing_area_fraction: Option<f64>,

    #[citygml(path = b"uro:handicapAccessible")]
    pub handicap_accessible: Option<bool>,

    #[citygml(path = b"uro:fireExit")]
    pub fire_exit: Option<bool>,

    #[citygml(path = b"uro:selfClosing")]
    pub self_closing: Option<bool>,

    #[citygml(path = b"uro:smokeStop")]
    pub smoke_stop: Option<bool>,
}

#[citygml_data(name = "uro:IfcPsetOpeningElementCommon")]
pub struct IfcPsetOpeningElementCommon {
    #[citygml(path = b"uro:reference")]
    pub reference: Option<String>,

    #[citygml(path = b"uro:purpose")]
    pub purpose: Option<String>,

    #[citygml(path = b"uro:fireExit")]
    pub fire_exit: Option<bool>,

    #[citygml(path = b"uro:protectedOpening")]
    pub protected_opening: Option<bool>,

    #[citygml(path = b"uro:parallelJambs")]
    pub parallel_jambs: Option<bool>,
}

#[citygml_data(name = "uro:IfcPsetSiteCommon")]
pub struct IfcPsetSiteCommon {
    #[citygml(path = b"uro:buildableArea")]
    pub buildable_area: Option<Measure>,

    #[citygml(path = b"uro:totalArea")]
    pub total_area: Option<Measure>,

    #[citygml(path = b"uro:buildingHeightLimit")]
    pub building_height_limit: Option<Measure>,
}

#[citygml_data(name = "uro:IfcPsetSpaceCommon")]
pub struct IfcPsetSpaceCommon {
    #[citygml(path = b"uro:reference")]
    pub reference: Option<String>,

    #[citygml(path = b"uro:category")]
    pub category: Option<String>,

    #[citygml(path = b"uro:floorCovering")]
    pub floor_covering: Option<String>,

    #[citygml(path = b"uro:wallCovering")]
    pub wall_covering: Option<String>,

    #[citygml(path = b"uro:ceilingCovering")]
    pub ceiling_covering: Option<String>,

    #[citygml(path = b"uro:skirtingBoard")]
    pub skirting_board: Option<String>,

    #[citygml(path = b"uro:grossPlannedArea")]
    pub gross_planned_area: Option<Measure>,

    #[citygml(path = b"uro:netPlannedArea")]
    pub net_planned_area: Option<Measure>,

    #[citygml(path = b"uro:publiclyAccessible")]
    pub publicly_accessible: Option<bool>,

    #[citygml(path = b"uro:handicapAccessible")]
    pub handicap_accessible: Option<bool>,

    #[citygml(path = b"uro:concealedFlooring")]
    pub concealed_flooring: Option<bool>,

    #[citygml(path = b"uro:concealedCeiling")]
    pub concealed_ceiling: Option<bool>,
}

#[citygml_data(name = "uro:IfcPsetWindowCommon")]
pub struct IfcPsetWindowCommon {
    #[citygml(path = b"uro:reference")]
    pub reference: Option<String>,

    #[citygml(path = b"uro:acousticRating")]
    pub acoustic_rating: Option<String>,

    #[citygml(path = b"uro:fireRating")]
    pub fire_rating: Option<String>,

    #[citygml(path = b"uro:securityRating")]
    pub security_rating: Option<String>,

    #[citygml(path = b"uro:isExternal")]
    pub is_external: Option<bool>,

    #[citygml(path = b"uro:infiltration")]
    pub infiltration: Option<f64>,

    #[citygml(path = b"uro:thermalTransmittance")]
    pub thermal_transmittance: Option<f64>,

    #[citygml(path = b"uro:glazingAreaFraction")]
    pub glazing_area_fraction: Option<f64>,

    #[citygml(path = b"uro:smokeStop")]
    pub smoke_stop: Option<bool>,
}

#[citygml_data(name = "uro:IfcRoof")]
pub struct IfcRoof {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,

    #[citygml(path = b"uro:elementType")]
    pub element_type: Option<Code>,

    #[citygml(path = b"uro:predefinedType")]
    pub predefined_type: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:numberOfRiser")]
    pub number_of_riser: Option<i64>,

    #[citygml(path = b"uro:numberOfTreads")]
    pub number_of_treads: Option<i64>,

    #[citygml(path = b"uro:riserHeight")]
    pub riser_height: Option<Measure>,

    #[citygml(path = b"uro:treadLength")]
    pub tread_length: Option<Measure>,

    #[citygml(path = b"uro:operationType")]
    pub operation_type: Option<IfcTransportElementTypeEnum>,

    #[citygml(path = b"uro:capacityByWeight")]
    pub capacity_by_weight: Option<Measure>,

    #[citygml(path = b"uro:capacityByNumber")]
    pub capacity_by_number: Option<i64>,
}

#[citygml_data(name = "uro:IfcSite")]
pub struct IfcSite {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:longName")]
    pub long_name: Option<String>,

    #[citygml(path = b"uro:compositionType")]
    pub composition_type: Option<IfcElementCompositionEnum>,

    #[citygml(path = b"uro:refLongitude")]
    pub ref_longitude: Option<f64>,

    #[citygml(path = b"uro:refLatitude")]
    pub ref_latitude: Option<f64>,

    #[citygml(path = b"uro:refElevation")]
    pub ref_elevation: Option<Measure>,

    #[citygml(path = b"uro:landTitleNumber")]
    pub land_title_number: Option<String>,

    #[citygml(path = b"uro:siteAddress/core:Address")]
    pub site_address: Option<Address>,
}

#[citygml_data(name = "uro:IfcSpace")]
pub struct IfcSpace {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:longName")]
    pub long_name: Option<String>,

    #[citygml(path = b"uro:compositionType")]
    pub composition_type: Option<IfcElementCompositionEnum>,

    #[citygml(path = b"uro:interiorOrExteriorSpace")]
    pub interior_or_exterior_space: Option<IfcInternalOrExternalEnum>,

    #[citygml(path = b"uro:elevationWithFlooring")]
    pub elevation_with_flooring: Option<Measure>,
}

#[citygml_data(name = "uro:IfcSpaceBaseQuantity")]
pub struct IfcSpaceBaseQuantity {
    #[citygml(path = b"uro:nominalHeight")]
    pub nominal_height: Option<Measure>,

    #[citygml(path = b"uro:clearHeight")]
    pub clear_height: Option<Measure>,

    #[citygml(path = b"uro:finishCeilingHeight")]
    pub finish_ceiling_height: Option<Measure>,

    #[citygml(path = b"uro:grossPerimeter")]
    pub gross_perimeter: Option<Measure>,

    #[citygml(path = b"uro:netPerimeter")]
    pub net_perimeter: Option<Measure>,

    #[citygml(path = b"uro:grossCeilingArea")]
    pub gross_ceiling_area: Option<Measure>,

    #[citygml(path = b"uro:grossFloorArea")]
    pub gross_floor_area: Option<Measure>,

    #[citygml(path = b"uro:netCeilingArea")]
    pub net_ceiling_area: Option<Measure>,

    #[citygml(path = b"uro:netFloorArea")]
    pub net_floor_area: Option<Measure>,

    #[citygml(path = b"uro:grossWallArea")]
    pub gross_wall_area: Option<Measure>,

    #[citygml(path = b"uro:netWallArea")]
    pub net_wall_area: Option<Measure>,

    #[citygml(path = b"uro:grossVolume")]
    pub gross_volume: Option<Measure>,

    #[citygml(path = b"uro:netVolume")]
    pub net_volume: Option<Measure>,
}

#[citygml_data(name = "uro:IfcWall")]
pub struct IfcWall {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,

    #[citygml(path = b"uro:elementType")]
    pub element_type: Option<Code>,

    #[citygml(path = b"uro:predefinedType")]
    pub predefined_type: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:numberOfRiser")]
    pub number_of_riser: Option<i64>,

    #[citygml(path = b"uro:numberOfTreads")]
    pub number_of_treads: Option<i64>,

    #[citygml(path = b"uro:riserHeight")]
    pub riser_height: Option<Measure>,

    #[citygml(path = b"uro:treadLength")]
    pub tread_length: Option<Measure>,

    #[citygml(path = b"uro:operationType")]
    pub operation_type: Option<IfcTransportElementTypeEnum>,

    #[citygml(path = b"uro:capacityByWeight")]
    pub capacity_by_weight: Option<Measure>,

    #[citygml(path = b"uro:capacityByNumber")]
    pub capacity_by_number: Option<i64>,

    #[citygml(path = b"uro:nominalLength")]
    pub nominal_length: Option<Measure>,

    #[citygml(path = b"uro:nominalWidth")]
    pub nominal_width: Option<Measure>,

    #[citygml(path = b"uro:nominalHeight")]
    pub nominal_height: Option<Measure>,

    #[citygml(path = b"uro:grossFootPrintArea")]
    pub gross_foot_print_area: Option<Measure>,

    #[citygml(path = b"uro:netFootPrintArea")]
    pub net_foot_print_area: Option<Measure>,

    #[citygml(path = b"uro:grossSideArea")]
    pub gross_side_area: Option<Measure>,

    #[citygml(path = b"uro:netSideArea")]
    pub net_side_area: Option<Measure>,

    #[citygml(path = b"uro:grossSideAreaLeft")]
    pub gross_side_area_left: Option<Measure>,

    #[citygml(path = b"uro:netSideAreaLeft")]
    pub net_side_area_left: Option<Measure>,

    #[citygml(path = b"uro:grossSideAreaRight")]
    pub gross_side_area_right: Option<Measure>,

    #[citygml(path = b"uro:netSideAreaRight")]
    pub net_side_area_right: Option<Measure>,

    #[citygml(path = b"uro:grossVolume")]
    pub gross_volume: Option<Measure>,

    #[citygml(path = b"uro:netVolume")]
    pub net_volume: Option<Measure>,
}

#[citygml_data(name = "uro:IfcWallStandardCase")]
pub struct IfcWallStandardCase {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,

    #[citygml(path = b"uro:elementType")]
    pub element_type: Option<Code>,

    #[citygml(path = b"uro:predefinedType")]
    pub predefined_type: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:numberOfRiser")]
    pub number_of_riser: Option<i64>,

    #[citygml(path = b"uro:numberOfTreads")]
    pub number_of_treads: Option<i64>,

    #[citygml(path = b"uro:riserHeight")]
    pub riser_height: Option<Measure>,

    #[citygml(path = b"uro:treadLength")]
    pub tread_length: Option<Measure>,

    #[citygml(path = b"uro:operationType")]
    pub operation_type: Option<IfcTransportElementTypeEnum>,

    #[citygml(path = b"uro:capacityByWeight")]
    pub capacity_by_weight: Option<Measure>,

    #[citygml(path = b"uro:capacityByNumber")]
    pub capacity_by_number: Option<i64>,

    #[citygml(path = b"uro:nominalLength")]
    pub nominal_length: Option<Measure>,

    #[citygml(path = b"uro:nominalWidth")]
    pub nominal_width: Option<Measure>,

    #[citygml(path = b"uro:nominalHeight")]
    pub nominal_height: Option<Measure>,

    #[citygml(path = b"uro:grossFootPrintArea")]
    pub gross_foot_print_area: Option<Measure>,

    #[citygml(path = b"uro:netFootPrintArea")]
    pub net_foot_print_area: Option<Measure>,

    #[citygml(path = b"uro:grossSideArea")]
    pub gross_side_area: Option<Measure>,

    #[citygml(path = b"uro:netSideArea")]
    pub net_side_area: Option<Measure>,

    #[citygml(path = b"uro:grossSideAreaLeft")]
    pub gross_side_area_left: Option<Measure>,

    #[citygml(path = b"uro:netSideAreaLeft")]
    pub net_side_area_left: Option<Measure>,

    #[citygml(path = b"uro:grossSideAreaRight")]
    pub gross_side_area_right: Option<Measure>,

    #[citygml(path = b"uro:netSideAreaRight")]
    pub net_side_area_right: Option<Measure>,

    #[citygml(path = b"uro:grossVolume")]
    pub gross_volume: Option<Measure>,

    #[citygml(path = b"uro:netVolume")]
    pub net_volume: Option<Measure>,
}

#[citygml_data(name = "uro:IfcWindow")]
pub struct IfcWindow {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,

    #[citygml(path = b"uro:tag")]
    pub tag: Option<String>,

    #[citygml(path = b"uro:elementType")]
    pub element_type: Option<Code>,

    #[citygml(path = b"uro:predefinedType")]
    pub predefined_type: Option<Code>,

    #[citygml(path = b"uro:shapeType")]
    pub shape_type: Option<Code>,

    #[citygml(path = b"uro:numberOfRiser")]
    pub number_of_riser: Option<i64>,

    #[citygml(path = b"uro:numberOfTreads")]
    pub number_of_treads: Option<i64>,

    #[citygml(path = b"uro:riserHeight")]
    pub riser_height: Option<Measure>,

    #[citygml(path = b"uro:treadLength")]
    pub tread_length: Option<Measure>,

    #[citygml(path = b"uro:operationType")]
    pub operation_type: Option<IfcTransportElementTypeEnum>,

    #[citygml(path = b"uro:capacityByWeight")]
    pub capacity_by_weight: Option<Measure>,

    #[citygml(path = b"uro:capacityByNumber")]
    pub capacity_by_number: Option<i64>,

    #[citygml(path = b"uro:overallHeight")]
    pub overall_height: Option<Measure>,

    #[citygml(path = b"uro:overallWidth")]
    pub overall_width: Option<Measure>,
}

#[citygml_data(name = "uro:IfcZone")]
pub struct IfcZone {
    #[citygml(path = b"uro:globalId")]
    pub global_id: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,

    #[citygml(path = b"uro:objectType")]
    pub object_type: Option<String>,
}

#[citygml_data(name = "uro:IfcUnit")]
pub struct IfcUnit {
    #[citygml(path = b"uro:dimensions")]
    pub dimensions: Option<i64>,

    #[citygml(path = b"uro:unitType")]
    pub unit_type: Option<IfcUnitEnum>,

    #[citygml(path = b"uro:perfix")]
    pub perfix: Option<String>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,
}

#[citygml_data(name = "uro:IfcClassification")]
pub struct IfcClassification {
    #[citygml(path = b"uro:source")]
    pub source: Option<String>,

    #[citygml(path = b"uro:edition")]
    pub edition: Option<String>,

    #[citygml(path = b"uro:editionDate")]
    pub edition_date: Option<Date>,

    #[citygml(path = b"uro:name")]
    pub name: Option<String>,
}

#[citygml_data(name = "uro:IfcAxis2Placement3D")]
pub struct IfcAxis2Placement3D {
    #[citygml(path = b"uro:location/gml:Point")]
    pub location: Option<Point>,

    #[citygml(path = b"uro:axis")]
    pub axis: Option<DoubleList>,

    #[citygml(path = b"uro:refDirection")]
    pub ref_direction: Option<DoubleList>,
}
