use nusamai_citygml::{
    citygml_data, citygml_feature, citygml_property, CityGmlElement, Code, Date, GYear, Length,
    Measure, Uri,
};

use crate::models::iur::uro;

#[citygml_property(name = "urf:DistrictFacilityProperty")]
pub enum DistrictFacilityProperty {
    #[citygml(path = b"urf:DistrictFacility")]
    DistrictFacility(DistrictFacility),
    #[citygml(path = b"urf:RoadsideDistrictFacility")]
    RoadsideDistrictFacility(RoadsideDistrictFacility),
    #[citygml(path = b"urf:RuralDistrictFacility")]
    RuralDistrictFacility(RuralDistrictFacility),
}

#[citygml_property(name = "urf:DistrictDevelopmentPlanProperty")]
pub enum DistrictDevelopmentPlanProperty {
    #[citygml(path = b"urf:DistrictDevelopmentPlan")]
    DistrictDevelopmentPlan(DistrictDevelopmentPlan),
    #[citygml(path = b"urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan")]
    DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan(
        DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan,
    ),
    #[citygml(
        path = b"urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict"
    )]
    DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict(
        DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict,
    ),
    #[citygml(path = b"urf:RoadsideDistrictImprovementPlan")]
    RoadsideDistrictImprovementPlan(RoadsideDistrictImprovementPlan),
    #[citygml(path = b"urf:RuralDistrictImprovementPlan")]
    RuralDistrictImprovementPlan(RuralDistrictImprovementPlan),
    #[citygml(path = b"urf:SpecifiedBuildingZoneImprovementPlan")]
    SpecifiedBuildingZoneImprovementPlan(SpecifiedBuildingZoneImprovementPlan),
}

#[citygml_feature(name = "urf:Zone")]
pub struct Zone {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:Agreement")]
pub struct Agreement {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:applicableArea")]
    pub applicable_area: Option<Measure>,

    #[citygml(path = b"urf:expiration")]
    pub expiration: Option<Date>,
}

#[citygml_feature(name = "urf:AircraftNoiseControlZone")]
pub struct AircraftNoiseControlZone {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:AreaClassification")]
pub struct AreaClassification {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:population")]
    pub population: Option<i64>,
}

#[citygml_feature(name = "urf:CollectiveFacilitiesForReconstruction")]
pub struct CollectiveFacilitiesForReconstruction {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:housingFacilities")]
    pub housing_facilities: Option<String>,

    #[citygml(path = b"urf:supecificBusinessFacilities")]
    pub supecific_business_facilities: Option<String>,

    #[citygml(path = b"urf:publicFacilities")]
    pub public_facilities: Option<String>,

    #[citygml(path = b"urf:utilityFacilities")]
    pub utility_facilities: Option<String>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,

    #[citygml(path = b"urf:maximumFloorAreaRate")]
    pub maximum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumFloorAreaRate")]
    pub minimum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate")]
    pub maximum_building_coverage_rate: Option<f64>,
}

#[citygml_feature(name = "urf:CollectiveFacilitiesForReconstructionAndRevitalization")]
pub struct CollectiveFacilitiesForReconstructionAndRevitalization {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:housingFacilities")]
    pub housing_facilities: Option<String>,

    #[citygml(path = b"urf:supecificBusinessFacilities")]
    pub supecific_business_facilities: Option<String>,

    #[citygml(path = b"urf:publicFacilities")]
    pub public_facilities: Option<String>,

    #[citygml(path = b"urf:utilityFacilities")]
    pub utility_facilities: Option<String>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,

    #[citygml(path = b"urf:maximumFloorAreaRate")]
    pub maximum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumFloorAreaRate")]
    pub minimum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate")]
    pub maximum_building_coverage_rate: Option<f64>,
}

#[citygml_feature(name = "urf:CollectiveFacilitiesForTsunamiDisasterPrevention")]
pub struct CollectiveFacilitiesForTsunamiDisasterPrevention {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:housingFacilities")]
    pub housing_facilities: Option<String>,

    #[citygml(path = b"urf:supecificBusinessFacilities")]
    pub supecific_business_facilities: Option<String>,

    #[citygml(path = b"urf:publicFacilities")]
    pub public_facilities: Option<String>,

    #[citygml(path = b"urf:utilityFacilities")]
    pub utility_facilities: Option<String>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,

    #[citygml(path = b"urf:maximumFloorAreaRate")]
    pub maximum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumFloorAreaRate")]
    pub minimum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate")]
    pub maximum_building_coverage_rate: Option<f64>,
}

#[citygml_feature(name = "urf:CollectiveGovernmentAndPublicOfficeFacilities")]
pub struct CollectiveGovernmentAndPublicOfficeFacilities {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:buildingCoverageRate")]
    pub building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:floorAreaRate")]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:publicFacilitiesAllocationPolicy")]
    pub public_facilities_allocation_policy: Option<String>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:CollectiveHousingFacilities")]
pub struct CollectiveHousingFacilities {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:buildingCoverageRate")]
    pub building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:floorAreaRate")]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:numberOfLowRiseHousing")]
    pub number_of_low_rise_housing: Option<i64>,

    #[citygml(path = b"urf:numberOfMiddleRiseHousing")]
    pub number_of_middle_rise_housing: Option<i64>,

    #[citygml(path = b"urf:numberOfHighRiseHousing")]
    pub number_of_high_rise_housing: Option<i64>,

    #[citygml(path = b"urf:totalNumberOfHousing")]
    pub total_number_of_housing: Option<i64>,

    #[citygml(path = b"urf:publicFacilitiesAllocationPolicy")]
    pub public_facilities_allocation_policy: Option<String>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:CollectiveUrbanDisasterPreventionFacilities")]
pub struct CollectiveUrbanDisasterPreventionFacilities {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:specificUtilityAndPublicFacilities")]
    pub specific_utility_and_public_facilities: Option<String>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,

    #[citygml(path = b"urf:maximumFloorAreaRate")]
    pub maximum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumFloorAreaRate")]
    pub minimum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate")]
    pub maximum_building_coverage_rate: Option<f64>,
}

#[citygml_feature(name = "urf:ConservationZoneForClustersOfTraditionalStructures")]
pub struct ConservationZoneForClustersOfTraditionalStructures {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:DisasterPreventionBlockImprovementProject")]
pub struct DisasterPreventionBlockImprovementProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,

    #[citygml(path = b"urf:disasterPreventionPublicFacilityAllocation", required)]
    pub disaster_prevention_public_facility_allocation: Option<String>,

    #[citygml(path = b"urf:otherPublicFacilityAllocation", required)]
    pub other_public_facility_allocation: Option<String>,

    #[citygml(path = b"urf:developmentPlan", required)]
    pub development_plan: Option<String>,
}

#[citygml_feature(name = "urf:DisasterPreventionBlockImprovementZonePlan")]
pub struct DisasterPreventionBlockImprovementZonePlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:objectives")]
    pub objectives: Option<String>,

    #[citygml(path = b"urf:policy")]
    pub policy: Option<String>,

    #[citygml(path = b"urf:districtDevelopmentPlan")]
    pub district_development_plan: Vec<DistrictDevelopmentPlanProperty>, // -> urf:DistrictDevelopmentPlan

    #[citygml(path = b"urf:promotionDistrict/urf:PromotionDistrict")]
    pub promotion_district: Vec<PromotionDistrict>,

    #[citygml(path = b"urf:zonalDisasterPreventionFacilitiesAllocation")]
    pub zonal_disaster_prevention_facilities_allocation: Option<String>,

    #[citygml(path = b"urf:specifiedZonalDisasterPreventionFacilitiesAllocation")]
    pub specified_zonal_disaster_prevention_facilities_allocation: Option<String>,

    #[citygml(path = b"urf:zonalDisasterPreventionFacilities/urf:ZonalDisasterPreventionFacility")]
    pub zonal_disaster_prevention_facilities: Vec<ZonalDisasterPreventionFacility>,
}

#[citygml_feature(name = "urf:DistributionBusinessPark")]
pub struct DistributionBusinessPark {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:distributionBusinessPark")]
    pub distribution_business_park: Option<String>,

    #[citygml(path = b"urf:publicAndUtilityFacilities")]
    pub public_and_utility_facilities: Option<String>,

    #[citygml(path = b"urf:buildingCoverageRate")]
    pub building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:floorAreaRate")]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,

    #[citygml(path = b"urf:setbackSize")]
    pub setback_size: Option<String>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:DistributionBusinessZone")]
pub struct DistributionBusinessZone {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:guidelinePublicationDate")]
    pub guideline_publication_date: Option<Date>,
}

#[citygml_feature(name = "urf:District")]
pub struct District {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:useRestrictions")]
    pub use_restrictions: Option<String>,

    #[citygml(path = b"urf:maximumFloorAreaRate")]
    pub maximum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumFloorAreaRate")]
    pub minimum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate")]
    pub maximum_building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:minimumBuildingCoverageRate")]
    pub minimum_building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:minimumSiteArea")]
    pub minimum_site_area: Option<Measure>,

    #[citygml(path = b"urf:minimumBuildingArea")]
    pub minimum_building_area: Option<Measure>,

    #[citygml(path = b"urf:minimumGroundHeight")]
    pub minimum_ground_height: Option<Length>,

    #[citygml(path = b"urf:setbackSize")]
    pub setback_size: Option<String>,

    #[citygml(path = b"urf:structurePlacementRestrictions")]
    pub structure_placement_restrictions: Option<String>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumFloorHeight")]
    pub minimum_floor_height: Option<Length>,

    #[citygml(path = b"urf:buildingDesignRestriction")]
    pub building_design_restriction: Option<String>,

    #[citygml(path = b"urf:minimumGreeningRate")]
    pub minimum_greening_rate: Option<f64>,

    #[citygml(path = b"urf:fenceGuideline")]
    pub fence_guideline: Option<String>,

    #[citygml(path = b"urf:restrictionsForFireProtection")]
    pub restrictions_for_fire_protection: Option<String>,

    #[citygml(path = b"urf:restrictionsForNoiseProtection")]
    pub restrictions_for_noise_protection: Option<String>,

    #[citygml(path = b"urf:minimumFrontageRate")]
    pub minimum_frontage_rate: Option<f64>,
}

#[citygml_feature(name = "urf:DistrictDevelopmentPlan")]
pub struct DistrictDevelopmentPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:districtFacilitiesAllocation")]
    pub district_facilities_allocation: Option<String>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:urbanGreenSpaceConservation")]
    pub urban_green_space_conservation: Option<String>,

    #[citygml(path = b"urf:activityRestrictionInFarmland")]
    pub activity_restriction_in_farmland: Option<String>,

    #[citygml(path = b"urf:landuseRestrictions")]
    pub landuse_restrictions: Option<String>,

    #[citygml(path = b"urf:districtFacility")]
    pub district_facility: Vec<DistrictFacilityProperty>, // -> urf:DistrictFacility

    #[citygml(path = b"urf:district/urf:District")]
    pub district: Vec<District>,
}

#[citygml_feature(name = "urf:DistrictFacility")]
pub struct DistrictFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(
    name = "urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan"
)]
pub struct DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:districtFacilitiesAllocation")]
    pub district_facilities_allocation: Option<String>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:urbanGreenSpaceConservation")]
    pub urban_green_space_conservation: Option<String>,

    #[citygml(path = b"urf:activityRestrictionInFarmland")]
    pub activity_restriction_in_farmland: Option<String>,

    #[citygml(path = b"urf:landuseRestrictions")]
    pub landuse_restrictions: Option<String>,

    #[citygml(path = b"urf:districtFacility")]
    pub district_facility: Vec<DistrictFacilityProperty>, // -> urf:DistrictFacility

    #[citygml(path = b"urf:district/urf:District")]
    pub district: Vec<District>,
}

#[citygml_feature(
    name = "urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict"
)]
pub struct DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:districtFacilitiesAllocation")]
    pub district_facilities_allocation: Option<String>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:urbanGreenSpaceConservation")]
    pub urban_green_space_conservation: Option<String>,

    #[citygml(path = b"urf:activityRestrictionInFarmland")]
    pub activity_restriction_in_farmland: Option<String>,

    #[citygml(path = b"urf:landuseRestrictions")]
    pub landuse_restrictions: Option<String>,

    #[citygml(path = b"urf:districtFacility")]
    pub district_facility: Vec<DistrictFacilityProperty>, // -> urf:DistrictFacility

    #[citygml(path = b"urf:district/urf:District")]
    pub district: Vec<District>,
}

#[citygml_feature(name = "urf:DistrictPlan")]
pub struct DistrictPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:objectives")]
    pub objectives: Option<String>,

    #[citygml(path = b"urf:policy")]
    pub policy: Option<String>,

    #[citygml(path = b"urf:districtDevelopmentPlan")]
    pub district_development_plan: Vec<DistrictDevelopmentPlanProperty>, // -> urf:DistrictDevelopmentPlan

    #[citygml(path = b"urf:promotionDistrict/urf:PromotionDistrict")]
    pub promotion_district: Vec<PromotionDistrict>,

    #[citygml(path = b"urf:facilityAllocation")]
    pub facility_allocation: Option<String>,

    #[citygml(path = b"urf:landUsePolicy")]
    pub land_use_policy: Option<String>,
}

#[citygml_feature(name = "urf:DistrictsAndZones")]
pub struct DistrictsAndZones {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:EducationalAndCulturalFacility")]
pub struct EducationalAndCulturalFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,
}

#[citygml_feature(name = "urf:ExceptionalFloorAreaRateDistrict")]
pub struct ExceptionalFloorAreaRateDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:buildingHeightLimits")]
    pub building_height_limits: Option<Length>,
}

#[citygml_feature(name = "urf:FirePreventionDistrict")]
pub struct FirePreventionDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:FireProtectionFacility")]
pub struct FireProtectionFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:FloodPreventionFacility")]
pub struct FloodPreventionFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:GlobalHubCityDevelopmentProject")]
pub struct GlobalHubCityDevelopmentProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:implementationBody")]
    pub implementation_body: Option<String>,

    #[citygml(path = b"urf:implementationPeriod")]
    pub implementation_period: Option<String>,

    #[citygml(path = b"urf:plan")]
    pub plan: Option<String>,
}

#[citygml_feature(name = "urf:GreenSpaceConservationDistrict")]
pub struct GreenSpaceConservationDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:HeightControlDistrict")]
pub struct HeightControlDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,
}

#[citygml_feature(name = "urf:HighLevelUseDistrict")]
pub struct HighLevelUseDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:maximumFloorAreaRate", required)]
    pub maximum_floor_area_rate: Vec<f64>,

    #[citygml(path = b"urf:minimumFloorAreaRate", required)]
    pub minimum_floor_area_rate: Vec<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate", required)]
    pub maximum_building_coverage_rate: Vec<f64>,

    #[citygml(path = b"urf:minimumBuildingArea", required)]
    pub minimum_building_area: Vec<Measure>,

    #[citygml(path = b"urf:setbackSize")]
    pub setback_size: Option<String>,
}

#[citygml_feature(name = "urf:HighRiseResidentialAttractionDistrict")]
pub struct HighRiseResidentialAttractionDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:floorAreaRate", required)]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate")]
    pub maximum_building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:minimumSiteArea")]
    pub minimum_site_area: Option<Measure>,
}

#[citygml_feature(name = "urf:HistoricSceneryMaintenanceAndImprovementDistrictPlan")]
pub struct HistoricSceneryMaintenanceAndImprovementDistrictPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:objectives")]
    pub objectives: Option<String>,

    #[citygml(path = b"urf:policy")]
    pub policy: Option<String>,

    #[citygml(path = b"urf:districtDevelopmentPlan")]
    pub district_development_plan: Vec<DistrictDevelopmentPlanProperty>, // -> urf:DistrictDevelopmentPlan

    #[citygml(path = b"urf:promotionDistrict/urf:PromotionDistrict")]
    pub promotion_district: Vec<PromotionDistrict>,

    #[citygml(path = b"urf:landUsePolicy")]
    pub land_use_policy: Option<String>,
}

#[citygml_feature(name = "urf:HousingControlArea")]
pub struct HousingControlArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:IndustrialParkDevelopmentProject")]
pub struct IndustrialParkDevelopmentProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,

    #[citygml(path = b"urf:publicFacilityAllocation", required)]
    pub public_facility_allocation: Option<String>,

    #[citygml(path = b"urf:residentialLandUsePlan", required)]
    pub residential_land_use_plan: Option<String>,
}

#[citygml_feature(name = "urf:LandReadjustmentProject")]
pub struct LandReadjustmentProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,

    #[citygml(path = b"urf:publicFacilityAllocation", required)]
    pub public_facility_allocation: Option<String>,

    #[citygml(path = b"urf:buildingLotDevelopment", required)]
    pub building_lot_development: Option<String>,
}

#[citygml_feature(name = "urf:LandReadjustmentPromotionArea")]
pub struct LandReadjustmentPromotionArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:developmentPolicy")]
    pub development_policy: Option<String>,

    #[citygml(path = b"urf:publicFacilitiesPlans")]
    pub public_facilities_plans: Option<String>,
}

#[citygml_feature(name = "urf:LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment")]
pub struct LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:developmentPolicy")]
    pub development_policy: Option<String>,

    #[citygml(path = b"urf:publicFacilitiesPlans")]
    pub public_facilities_plans: Option<String>,
}

#[citygml_feature(name = "urf:LandscapeZone")]
pub struct LandscapeZone {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:buildingDesignRestriction")]
    pub building_design_restriction: Option<String>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,

    #[citygml(path = b"urf:setbackSize")]
    pub setback_size: Option<String>,

    #[citygml(path = b"urf:minimumSiteArea")]
    pub minimum_site_area: Option<Measure>,
}

#[citygml_feature(name = "urf:MarketsSlaughterhousesCrematoria")]
pub struct MarketsSlaughterhousesCrematoria {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,
}

#[citygml_feature(name = "urf:MedicalFacility")]
pub struct MedicalFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,
}

#[citygml_feature(name = "urf:NewHousingAndUrbanDevelopmentProject")]
pub struct NewHousingAndUrbanDevelopmentProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,

    #[citygml(path = b"urf:housing", required)]
    pub housing: Option<String>,

    #[citygml(path = b"urf:publicFacilityAllocation", required)]
    pub public_facility_allocation: Option<String>,

    #[citygml(path = b"urf:residentialLandUsePlan", required)]
    pub residential_land_use_plan: Option<String>,
}

#[citygml_feature(name = "urf:NewUrbanInfrastructureProject")]
pub struct NewUrbanInfrastructureProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,

    #[citygml(path = b"urf:landForCentralPublicFacilities", required)]
    pub land_for_central_public_facilities: Option<String>,

    #[citygml(path = b"urf:districtsAllocation", required)]
    pub districts_allocation: Option<String>,

    #[citygml(path = b"urf:landUsePlan", required)]
    pub land_use_plan: Option<String>,
}

#[citygml_feature(name = "urf:OpenSpaceForPublicUse")]
pub struct OpenSpaceForPublicUse {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:parkAttribute/urf:ParkAttribute")]
    pub park_attribute: Option<ParkAttribute>,
}

#[citygml_feature(name = "urf:ParkingPlaceDevelopmentZone")]
pub struct ParkingPlaceDevelopmentZone {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:PortZone")]
pub struct PortZone {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:floorAreaRate")]
    pub floor_area_rate: Option<f64>,
}

#[citygml_feature(name = "urf:PrivateUrbanRenewalProjectPlan")]
pub struct PrivateUrbanRenewalProjectPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:developer")]
    pub developer: Option<String>,

    #[citygml(path = b"urf:plan")]
    pub plan: Option<String>,
}

#[citygml_feature(name = "urf:ProductiveGreenZone")]
pub struct ProductiveGreenZone {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:zoneNumber")]
    pub zone_number: Option<String>,

    #[citygml(path = b"urf:specification")]
    pub specification: Option<Code>,
}

#[citygml_feature(name = "urf:ProjectPromotionArea")]
pub struct ProjectPromotionArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:developmentPolicy")]
    pub development_policy: Option<String>,

    #[citygml(path = b"urf:publicFacilitiesPlans")]
    pub public_facilities_plans: Option<String>,
}

#[citygml_feature(name = "urf:PromotionDistrict")]
pub struct PromotionDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:QuasiUrbanPlanningArea")]
pub struct QuasiUrbanPlanningArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:population")]
    pub population: Option<i64>,

    #[citygml(path = b"urf:cityArea")]
    pub city_area: Option<Measure>,

    #[citygml(path = b"urf:cityPopulation")]
    pub city_population: Option<i64>,
}

#[citygml_feature(name = "urf:Regulation")]
pub struct Regulation {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:ResidenceAttractionArea")]
pub struct ResidenceAttractionArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:ResidentialBlockConstructionProject")]
pub struct ResidentialBlockConstructionProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,

    #[citygml(path = b"urf:publicFacilityAllocation", required)]
    pub public_facility_allocation: Option<String>,

    #[citygml(path = b"urf:developmentPlan")]
    pub development_plan: Option<String>,

    #[citygml(path = b"urf:siteArea")]
    pub site_area: Option<Measure>,

    #[citygml(path = b"urf:totalFloorArea")]
    pub total_floor_area: Option<Measure>,
}

#[citygml_feature(name = "urf:ResidentialBlockConstructionPromotionArea")]
pub struct ResidentialBlockConstructionPromotionArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:developmentPolicy")]
    pub development_policy: Option<String>,

    #[citygml(path = b"urf:publicFacilitiesPlans")]
    pub public_facilities_plans: Option<String>,
}

#[citygml_feature(name = "urf:ResidentialEnvironmentImprovementDistrict")]
pub struct ResidentialEnvironmentImprovementDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:useToBeInduced")]
    pub use_to_be_induced: Option<String>,

    #[citygml(path = b"urf:maximumFloorAreaRate")]
    pub maximum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate")]
    pub maximum_building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<String>,

    #[citygml(path = b"urf:setbackSize")]
    pub setback_size: Option<String>,

    #[citygml(path = b"urf:otherRestrictions")]
    pub other_restrictions: Option<String>,
}

#[citygml_feature(name = "urf:RoadsideDistrictFacility")]
pub struct RoadsideDistrictFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:RoadsideDistrictImprovementPlan")]
pub struct RoadsideDistrictImprovementPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:districtFacilitiesAllocation")]
    pub district_facilities_allocation: Option<String>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:urbanGreenSpaceConservation")]
    pub urban_green_space_conservation: Option<String>,

    #[citygml(path = b"urf:activityRestrictionInFarmland")]
    pub activity_restriction_in_farmland: Option<String>,

    #[citygml(path = b"urf:landuseRestrictions")]
    pub landuse_restrictions: Option<String>,

    #[citygml(path = b"urf:districtFacility")]
    pub district_facility: Vec<DistrictFacilityProperty>, // -> urf:DistrictFacility

    #[citygml(path = b"urf:district/urf:District")]
    pub district: Vec<District>,

    #[citygml(path = b"urf:roadsideDistrictFacilitiesAllocation")]
    pub roadside_district_facilities_allocation: Option<String>,
}

#[citygml_feature(name = "urf:RoadsideDistrictPlan")]
pub struct RoadsideDistrictPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:objectives")]
    pub objectives: Option<String>,

    #[citygml(path = b"urf:policy")]
    pub policy: Option<String>,

    #[citygml(path = b"urf:districtDevelopmentPlan")]
    pub district_development_plan: Vec<DistrictDevelopmentPlanProperty>, // -> urf:DistrictDevelopmentPlan

    #[citygml(path = b"urf:promotionDistrict/urf:PromotionDistrict")]
    pub promotion_district: Vec<PromotionDistrict>,

    #[citygml(path = b"urf:facilitiesAllocation")]
    pub facilities_allocation: Option<String>,

    #[citygml(path = b"urf:landUsePolicy")]
    pub land_use_policy: Option<String>,
}

#[citygml_feature(name = "urf:RuralDistrictFacility")]
pub struct RuralDistrictFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:RuralDistrictImprovementPlan")]
pub struct RuralDistrictImprovementPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:districtFacilitiesAllocation")]
    pub district_facilities_allocation: Option<String>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:urbanGreenSpaceConservation")]
    pub urban_green_space_conservation: Option<String>,

    #[citygml(path = b"urf:activityRestrictionInFarmland")]
    pub activity_restriction_in_farmland: Option<String>,

    #[citygml(path = b"urf:landuseRestrictions")]
    pub landuse_restrictions: Option<String>,

    #[citygml(path = b"urf:districtFacility")]
    pub district_facility: Vec<DistrictFacilityProperty>, // -> urf:DistrictFacility

    #[citygml(path = b"urf:district/urf:District")]
    pub district: Vec<District>,

    #[citygml(path = b"urf:ruralDistrictFacilitiesAllocation")]
    pub rural_district_facilities_allocation: Option<String>,
}

#[citygml_feature(name = "urf:RuralDistrictPlan")]
pub struct RuralDistrictPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:objectives")]
    pub objectives: Option<String>,

    #[citygml(path = b"urf:policy")]
    pub policy: Option<String>,

    #[citygml(path = b"urf:districtDevelopmentPlan")]
    pub district_development_plan: Vec<DistrictDevelopmentPlanProperty>, // -> urf:DistrictDevelopmentPlan

    #[citygml(path = b"urf:promotionDistrict/urf:PromotionDistrict")]
    pub promotion_district: Vec<PromotionDistrict>,
}

#[citygml_feature(name = "urf:SandControlFacility")]
pub struct SandControlFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:ScenicDistrict")]
pub struct ScenicDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:buildingCoverageRate")]
    pub building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:buildingHeightLimits")]
    pub building_height_limits: Option<Length>,

    #[citygml(path = b"urf:wallSetbackDistanceWithRoad")]
    pub wall_setback_distance_with_road: Option<Length>,

    #[citygml(path = b"urf:wallSetbackDistanceWithAdjoiningLand")]
    pub wall_setback_distance_with_adjoining_land: Option<Length>,
}

#[citygml_feature(name = "urf:ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities")]
pub struct ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor", required)]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:ScheduledAreaForCollectiveHousingFacilities")]
pub struct ScheduledAreaForCollectiveHousingFacilities {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor", required)]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:ScheduledAreaForDistributionBusinessPark")]
pub struct ScheduledAreaForDistributionBusinessPark {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor", required)]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:ScheduledAreaForIndustrialParkDevelopmentProjects")]
pub struct ScheduledAreaForIndustrialParkDevelopmentProjects {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor", required)]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:ScheduledAreaForNewHousingAndUrbanDevelopmentProjects")]
pub struct ScheduledAreaForNewHousingAndUrbanDevelopmentProjects {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor", required)]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:ScheduledAreaForNewUrbanInfrastructureProjects")]
pub struct ScheduledAreaForNewUrbanInfrastructureProjects {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor", required)]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:ScheduledAreaForUrbanDevelopmentProject")]
pub struct ScheduledAreaForUrbanDevelopmentProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor", required)]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:SedimentDisasterProneArea")]
pub struct SedimentDisasterProneArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:disasterType", required)]
    pub disaster_type: Option<Code>,

    #[citygml(path = b"urf:areaType", required)]
    pub area_type: Option<Code>,

    #[citygml(path = b"urf:zoneNumber", required)]
    pub zone_number: Option<String>,

    #[citygml(path = b"urf:zoneName", required)]
    pub zone_name: Option<String>,

    #[citygml(path = b"urf:status")]
    pub status: Option<Code>,
}

#[citygml_feature(name = "urf:SnowProtectionFacility")]
pub struct SnowProtectionFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:SocialWelfareFacility")]
pub struct SocialWelfareFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,
}

#[citygml_feature(name = "urf:SpecialGreenSpaceConservationDistrict")]
pub struct SpecialGreenSpaceConservationDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:requirement")]
    pub requirement: Option<Code>,
}

#[citygml_feature(name = "urf:SpecialUrbanRenaissanceDistrict")]
pub struct SpecialUrbanRenaissanceDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:useToBeInduced", required)]
    pub use_to_be_induced: Option<String>,

    #[citygml(path = b"urf:maximumFloorAreaRate", required)]
    pub maximum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumFloorAreaRate", required)]
    pub minimum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingCoverageRate", required)]
    pub maximum_building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:minimumBuildingArea", required)]
    pub minimum_building_area: Option<Measure>,

    #[citygml(path = b"urf:maximumBuildingHeight", required)]
    pub maximum_building_height: Option<String>,

    #[citygml(path = b"urf:setbackSize", required)]
    pub setback_size: Option<String>,

    #[citygml(path = b"urf:otherRestrictions")]
    pub other_restrictions: Option<String>,
}

#[citygml_feature(name = "urf:SpecialUseAttractionDistrict")]
pub struct SpecialUseAttractionDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:useToBeInduced", required)]
    pub use_to_be_induced: Option<String>,

    #[citygml(path = b"urf:maximumFloorAreaRate", required)]
    pub maximum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumFloorAreaRate")]
    pub minimum_floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumBuildingArea")]
    pub minimum_building_area: Option<Measure>,

    #[citygml(path = b"urf:maximumBuildingHeight")]
    pub maximum_building_height: Option<String>,

    #[citygml(path = b"urf:otherRestrictions")]
    pub other_restrictions: Option<String>,
}

#[citygml_feature(name = "urf:SpecialUseDistrict")]
pub struct SpecialUseDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:otherRestrictions")]
    pub other_restrictions: Option<String>,
}

#[citygml_feature(name = "urf:SpecialUseRestrictionDistrict")]
pub struct SpecialUseRestrictionDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:otherRestrictions")]
    pub other_restrictions: Option<String>,
}

#[citygml_feature(name = "urf:SpecialZoneForPreservationOfHistoricalLandscape")]
pub struct SpecialZoneForPreservationOfHistoricalLandscape {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:SpecifiedBlock")]
pub struct SpecifiedBlock {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:floorAreaRate", required)]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:maximumBuildingHeight", required)]
    pub maximum_building_height: Option<Length>,

    #[citygml(path = b"urf:setbackSize", required)]
    pub setback_size: Option<String>,
}

#[citygml_feature(name = "urf:SpecifiedBuildingZoneImprovementPlan")]
pub struct SpecifiedBuildingZoneImprovementPlan {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:districtFacilitiesAllocation")]
    pub district_facilities_allocation: Option<String>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:urbanGreenSpaceConservation")]
    pub urban_green_space_conservation: Option<String>,

    #[citygml(path = b"urf:activityRestrictionInFarmland")]
    pub activity_restriction_in_farmland: Option<String>,

    #[citygml(path = b"urf:landuseRestrictions")]
    pub landuse_restrictions: Option<String>,

    #[citygml(path = b"urf:districtFacility")]
    pub district_facility: Vec<DistrictFacilityProperty>, // -> urf:DistrictFacility

    #[citygml(path = b"urf:district/urf:District")]
    pub district: Vec<District>,
}

#[citygml_feature(name = "urf:SpecifiedDisasterPreventionBlockImprovementZone")]
pub struct SpecifiedDisasterPreventionBlockImprovementZone {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:minimumSiteArea", required)]
    pub minimum_site_area: Option<Measure>,

    #[citygml(path = b"urf:setbackSize")]
    pub setback_size: Option<String>,

    #[citygml(path = b"urf:minimumFrontageRate")]
    pub minimum_frontage_rate: Option<f64>,

    #[citygml(path = b"urf:minimumBuildingHeight")]
    pub minimum_building_height: Option<Length>,
}

#[citygml_feature(name = "urf:SpecifiedUrgentUrbanRenewalArea")]
pub struct SpecifiedUrgentUrbanRenewalArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:developmentPolicy")]
    pub development_policy: Option<String>,

    #[citygml(path = b"urf:privateProject/urf:PrivateUrbanRenewalProjectPlan")]
    pub private_project: Vec<PrivateUrbanRenewalProjectPlan>,

    #[citygml(path = b"urf:specifiedArea/urf:SpecifiedUrgentUrbanRenewalArea")]
    pub specified_area: Vec<SpecifiedUrgentUrbanRenewalArea>,

    #[citygml(path = b"urf:specialDistrict/urf:SpecialUrbanRenaissanceDistrict")]
    pub special_district: Vec<SpecialUrbanRenaissanceDistrict>,

    #[citygml(path = b"urf:developmentProject/urf:GlobalHubCityDevelopmentProject")]
    pub development_project: Vec<GlobalHubCityDevelopmentProject>,
}

#[citygml_feature(name = "urf:SupplyFacility")]
pub struct SupplyFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:waterWorksAttribute/urf:WaterWorksAttribute")]
    pub water_works_attribute: Option<WaterWorksAttribute>,
}

#[citygml_feature(name = "urf:TelecommunicationFacility")]
pub struct TelecommunicationFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:TideFacility")]
pub struct TideFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:TrafficFacility")]
pub struct TrafficFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:startLocation")]
    pub start_location: Option<String>,

    #[citygml(path = b"urf:endLocation")]
    pub end_location: Option<String>,

    #[citygml(path = b"urf:viaLocations")]
    pub via_locations: Option<String>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,

    #[citygml(path = b"urf:urbanRoadAttribute/urf:UrbanRoadAttribute")]
    pub urban_road_attribute: Option<UrbanRoadAttribute>,

    #[citygml(
        path = b"urf:urbanRapidTransitRailroadAttribute/urf:UrbanRapidTransitRailroadAttribute"
    )]
    pub urban_rapid_transit_railroad_attribute: Option<UrbanRapidTransitRailroadAttribute>,

    #[citygml(path = b"urf:parkingPlaceAttribute/urf:ParkingPlaceAttribute")]
    pub parking_place_attribute: Option<ParkingPlaceAttribute>,

    #[citygml(path = b"urf:vehicleTerminalAttribute/urf:VehicleTerminalAttribute")]
    pub vehicle_terminal_attribute: Option<VehicleTerminalAttribute>,
}

#[citygml_feature(name = "urf:TreatmentFacility")]
pub struct TreatmentFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:sewerSystemsAttribute/urf:SewerSystemAttribute")]
    pub sewer_systems_attribute: Option<SewerSystemAttribute>,
}

#[citygml_feature(name = "urf:TreePlantingDistrict")]
pub struct TreePlantingDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:minimumGreeningRate", required)]
    pub minimum_greening_rate: Option<f64>,
}

#[citygml_feature(name = "urf:UnclassifiedBlankArea")]
pub struct UnclassifiedBlankArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:UnclassifiedUseDistrict")]
pub struct UnclassifiedUseDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:UnusedLandUsePromotionArea")]
pub struct UnusedLandUsePromotionArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:UrbanDevelopmentProject")]
pub struct UrbanDevelopmentProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,
}

#[citygml_feature(name = "urf:UrbanDisasterRecoveryPromotionArea")]
pub struct UrbanDisasterRecoveryPromotionArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:expirationDate", required)]
    pub expiration_date: Option<Date>,

    #[citygml(path = b"urf:emergencyRecoveryPolicy")]
    pub emergency_recovery_policy: Option<String>,

    #[citygml(path = b"urf:plannedProjectType")]
    pub planned_project_type: Option<Code>,
}

#[citygml_feature(name = "urf:UrbanFacility")]
pub struct UrbanFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,
}

#[citygml_feature(name = "urf:UrbanFacilityStipulatedByCabinetOrder")]
pub struct UrbanFacilityStipulatedByCabinetOrder {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:UrbanFunctionAttractionArea")]
pub struct UrbanFunctionAttractionArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,
}

#[citygml_feature(name = "urf:UrbanPlanningArea")]
pub struct UrbanPlanningArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaClassification", required)]
    pub area_classification: Option<Code>,

    #[citygml(path = b"urf:reasonForAreaClassification")]
    pub reason_for_area_classification: Option<String>,

    #[citygml(path = b"urf:policyForAreaClassification")]
    pub policy_for_area_classification: Option<String>,

    #[citygml(path = b"urf:purposeForUrbanPlan")]
    pub purpose_for_urban_plan: Option<String>,

    #[citygml(path = b"urf:policyForUrbanPlanDecision")]
    pub policy_for_urban_plan_decision: Option<String>,

    #[citygml(path = b"urf:population")]
    pub population: Option<i64>,

    #[citygml(path = b"urf:cityArea")]
    pub city_area: Option<Measure>,

    #[citygml(path = b"urf:cityPopulation")]
    pub city_population: Option<i64>,
}

#[citygml_feature(name = "urf:UrbanRedevelopmentProject")]
pub struct UrbanRedevelopmentProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,

    #[citygml(path = b"urf:publicFacilityAllocation", required)]
    pub public_facility_allocation: Option<String>,

    #[citygml(path = b"urf:developmentPlan", required)]
    pub development_plan: Option<String>,

    #[citygml(path = b"urf:housingTarget")]
    pub housing_target: Option<String>,

    #[citygml(path = b"urf:siteArea")]
    pub site_area: Option<Measure>,

    #[citygml(path = b"urf:totalFloorArea")]
    pub total_floor_area: Option<Measure>,

    #[citygml(path = b"urf:numberOfHousing")]
    pub number_of_housing: Option<i64>,
}

#[citygml_feature(name = "urf:UrbanRedevelopmentPromotionArea")]
pub struct UrbanRedevelopmentPromotionArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:developmentPolicy")]
    pub development_policy: Option<String>,

    #[citygml(path = b"urf:publicFacilitiesPlans")]
    pub public_facilities_plans: Option<String>,

    #[citygml(path = b"urf:publicFacilities", required)]
    pub public_facilities: Option<String>,

    #[citygml(path = b"urf:unitArea", required)]
    pub unit_area: Option<String>,
}

#[citygml_feature(name = "urf:UrbanRenewalProject")]
pub struct UrbanRenewalProject {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:scheduledExecutor")]
    pub scheduled_executor: Option<String>,

    #[citygml(path = b"urf:storeysAboveGround")]
    pub storeys_above_ground: Option<u64>,

    #[citygml(path = b"urf:storeysBelowGround")]
    pub storeys_below_ground: Option<u64>,

    #[citygml(path = b"urf:setbackSize")]
    pub setback_size: Option<String>,

    #[citygml(path = b"urf:floorAreaRate")]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:buildingUsage")]
    pub building_usage: Option<String>,

    #[citygml(path = b"urf:siteArea")]
    pub site_area: Option<Measure>,
}

#[citygml_feature(name = "urf:UrgentUrbanRenewalArea")]
pub struct UrgentUrbanRenewalArea {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:developmentPolicy")]
    pub development_policy: Option<String>,

    #[citygml(path = b"urf:privateProject/urf:PrivateUrbanRenewalProjectPlan")]
    pub private_project: Vec<PrivateUrbanRenewalProjectPlan>,

    #[citygml(path = b"urf:specifiedArea/urf:SpecifiedUrgentUrbanRenewalArea")]
    pub specified_area: Vec<SpecifiedUrgentUrbanRenewalArea>,

    #[citygml(path = b"urf:specialDistrict/urf:SpecialUrbanRenaissanceDistrict")]
    pub special_district: Vec<SpecialUrbanRenaissanceDistrict>,
}

#[citygml_feature(name = "urf:UseDistrict")]
pub struct UseDistrict {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,

    #[citygml(path = b"urf:floorAreaRate", required)]
    pub floor_area_rate: Option<f64>,

    #[citygml(path = b"urf:minimumSiteArea")]
    pub minimum_site_area: Option<Measure>,

    #[citygml(path = b"urf:buildingCoverageRate")]
    pub building_coverage_rate: Option<f64>,

    #[citygml(path = b"urf:wallSetbackDistance")]
    pub wall_setback_distance: Option<String>,

    #[citygml(path = b"urf:buildingHeightLimits")]
    pub building_height_limits: Option<Length>,

    #[citygml(path = b"urf:buildingRestrictions")]
    pub building_restrictions: Option<String>,

    #[citygml(path = b"urf:otherRestrictions")]
    pub other_restrictions: Option<String>,

    #[citygml(path = b"urf:setbackRestrictions")]
    pub setback_restrictions: Option<String>,

    #[citygml(path = b"urf:frontRoadRestrictions")]
    pub front_road_restrictions: Option<String>,

    #[citygml(path = b"urf:adjacentLandRestrictions")]
    pub adjacent_land_restrictions: Option<String>,

    #[citygml(path = b"urf:northDirectionRestrictions")]
    pub north_direction_restrictions: Option<String>,

    #[citygml(path = b"urf:shadeRegulation")]
    pub shade_regulation: Option<String>,
}

#[citygml_feature(name = "urf:Waterway")]
pub struct Waterway {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:startLocation")]
    pub start_location: Option<String>,

    #[citygml(path = b"urf:endLocation")]
    pub end_location: Option<String>,

    #[citygml(path = b"urf:structure")]
    pub structure: Option<Code>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:WindProtectionFacility")]
pub struct WindProtectionFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:number")]
    pub number: Option<String>,

    #[citygml(path = b"urf:threeDimensionalExtent/urf:ThreeDimensionalExtent")]
    pub three_dimensional_extent: Vec<ThreeDimensionalExtent>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:width")]
    pub width: Option<Length>,
}

#[citygml_feature(name = "urf:ZonalDisasterPreventionFacility")]
pub struct ZonalDisasterPreventionFacility {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:facilityType", required)]
    pub facility_type: Option<Code>,
}

#[citygml_feature(name = "urf:ZoneForPreservationOfHistoricalLandscape")]
pub struct ZoneForPreservationOfHistoricalLandscape {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:keyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"urf:dataQualityAttribute/uro:DataQualityAttribute")]
    pub data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"urf:boundary/urf:Boundary")]
    pub boundary: Vec<Boundary>,

    #[citygml(path = b"urf:location")]
    pub location: Option<String>,

    #[citygml(path = b"urf:urbanParkAttribute/urf:UrbanParkAttribute")]
    pub urban_park_attribute: Option<UrbanParkAttribute>,

    #[citygml(path = b"urf:areaInTotal")]
    pub area_in_total: Option<Measure>,
}

#[citygml_feature(name = "urf:ThreeDimensionalExtent")]
pub struct ThreeDimensionalExtent {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:validFrom")]
    pub urf_valid_from: Option<Date>,

    #[citygml(path = b"urf:validFromType")]
    pub valid_from_type: Option<Code>,

    #[citygml(path = b"urf:enactmentFiscalYear")]
    pub enactment_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:validTo")]
    pub urf_valid_to: Option<Date>,

    #[citygml(path = b"urf:validToType")]
    pub valid_to_type: Option<Code>,

    #[citygml(path = b"urf:expirationFiscalYear")]
    pub expiration_fiscal_year: Option<GYear>,

    #[citygml(path = b"urf:legalGrounds")]
    pub legal_grounds: Option<String>,

    #[citygml(path = b"urf:custodian")]
    pub custodian: Option<String>,

    #[citygml(path = b"urf:notificationNumber")]
    pub notification_number: Option<String>,

    #[citygml(path = b"urf:finalNotificationDate")]
    pub final_notification_date: Option<Date>,

    #[citygml(path = b"urf:finalNotificationNumber")]
    pub final_notification_number: Option<String>,

    #[citygml(path = b"urf:urbanPlanType")]
    pub urban_plan_type: Option<Code>,

    #[citygml(path = b"urf:areaClassificationType")]
    pub area_classification_type: Option<Code>,

    #[citygml(path = b"urf:nominalArea")]
    pub nominal_area: Option<Measure>,

    #[citygml(path = b"urf:prefecture")]
    pub prefecture: Option<Code>,

    #[citygml(path = b"urf:city")]
    pub city: Option<Code>,

    #[citygml(path = b"urf:reference")]
    pub reference: Option<Uri>,

    #[citygml(path = b"urf:reason")]
    pub reason: Option<String>,

    #[citygml(path = b"urf:note")]
    pub note: Option<String>,

    #[citygml(path = b"urf:surveyYear")]
    pub survey_year: Option<GYear>,

    #[citygml(path = b"urf:minimumDistance")]
    pub minimum_distance: Option<Length>,

    #[citygml(path = b"urf:maximumLoad")]
    pub maximum_load: Option<Measure>,
}

#[citygml_data(name = "urf:UrbanRoadAttribute")]
pub struct UrbanRoadAttribute {
    #[citygml(path = b"urf:routeTypeNumber")]
    pub route_type_number: Option<Code>,

    #[citygml(path = b"urf:routeSizeNumber")]
    pub route_size_number: Option<Code>,

    #[citygml(path = b"urf:routeSerialNumber")]
    pub route_serial_number: Option<String>,

    #[citygml(path = b"urf:roadType")]
    pub road_type: Option<Code>,

    #[citygml(path = b"urf:numberOfLanes")]
    pub number_of_lanes: Option<i64>,

    #[citygml(path = b"urf:roadStructure")]
    pub road_structure: Option<String>,

    #[citygml(path = b"urf:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"urf:crossType")]
    pub cross_type: Option<Code>,

    #[citygml(path = b"urf:trafficPlazas")]
    pub traffic_plazas: Option<Code>,

    #[citygml(path = b"urf:structuralDetails/urf:StructureDetails")]
    pub structural_details: Vec<StructureDetails>,
}

#[citygml_data(name = "urf:ParkAttribute")]
pub struct ParkAttribute {
    #[citygml(path = b"urf:parkTypeNumber")]
    pub park_type_number: Option<Code>,

    #[citygml(path = b"urf:parkSizeNumber")]
    pub park_size_number: Option<Code>,

    #[citygml(path = b"urf:parkSerialNumber")]
    pub park_serial_number: Option<String>,
}

#[citygml_data(name = "urf:WaterWorksAttribute")]
pub struct WaterWorksAttribute {
    #[citygml(path = b"urf:startLocation")]
    pub start_location: Option<String>,

    #[citygml(path = b"urf:endLocation")]
    pub end_location: Option<String>,
}

#[citygml_data(name = "urf:StructureDetails")]
pub struct StructureDetails {
    #[citygml(path = b"urf:startLocation", required)]
    pub start_location: Option<String>,

    #[citygml(path = b"urf:endLocation", required)]
    pub end_location: Option<String>,

    #[citygml(path = b"urf:viaLocations")]
    pub via_locations: Option<String>,

    #[citygml(path = b"urf:length")]
    pub length: Option<Length>,

    #[citygml(path = b"urf:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"urf:minimumWidth")]
    pub minimum_width: Option<Length>,

    #[citygml(path = b"urf:maximumWidth")]
    pub maximum_width: Option<Length>,

    #[citygml(path = b"urf:standardWidth")]
    pub standard_width: Option<Length>,

    #[citygml(path = b"urf:crossType")]
    pub cross_type: Option<Code>,
}

#[citygml_data(name = "urf:SewerSystemAttribute")]
pub struct SewerSystemAttribute {
    #[citygml(path = b"urf:startLocation")]
    pub start_location: Option<String>,

    #[citygml(path = b"urf:endLocation")]
    pub end_location: Option<String>,

    #[citygml(path = b"urf:systemType")]
    pub system_type: Option<Code>,

    #[citygml(path = b"urf:drainageArea")]
    pub drainage_area: Option<String>,
}

#[citygml_data(name = "urf:UrbanRapidTransitRailroadAttribute")]
pub struct UrbanRapidTransitRailroadAttribute {
    #[citygml(path = b"urf:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"urf:crossType")]
    pub cross_type: Option<Code>,

    #[citygml(path = b"urf:structuralDetails/urf:StructureDetails")]
    pub structural_details: Vec<StructureDetails>,
}

#[citygml_data(name = "urf:ParkingPlaceAttribute")]
pub struct ParkingPlaceAttribute {
    #[citygml(path = b"urf:storeysAboveGround", required)]
    pub storeys_above_ground: Option<u64>,

    #[citygml(path = b"urf:storeysBelowGround", required)]
    pub storeys_below_ground: Option<u64>,
}

#[citygml_data(name = "urf:UrbanParkAttribute")]
pub struct UrbanParkAttribute {
    #[citygml(path = b"urf:parkCode", required)]
    pub park_code: Option<Code>,

    #[citygml(path = b"urf:startFrom", required)]
    pub start_from: Option<Date>,

    #[citygml(path = b"urf:breakdownOfNominalArea/urf:BreakdownOfNominalArea")]
    pub breakdown_of_nominal_area: Vec<BreakdownOfNominalArea>,
}

#[citygml_data(name = "urf:VehicleTerminalAttribute")]
pub struct VehicleTerminalAttribute {
    #[citygml(path = b"urf:terminalType", required)]
    pub terminal_type: Option<Code>,
}

#[citygml_data(name = "urf:BreakdownOfNominalArea")]
pub struct BreakdownOfNominalArea {
    #[citygml(path = b"urf:breakdown", required)]
    pub breakdown: Option<String>,

    #[citygml(path = b"urf:areaInSquareMeter", required)]
    pub area_in_square_meter: Option<Measure>,
}

#[citygml_data(name = "urf:Boundary")]
pub struct Boundary {
    #[citygml(path = b"urf:class")]
    pub class: Option<Code>,

    #[citygml(path = b"urf:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"urf:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"urf:offset")]
    pub offset: Option<Length>,

    #[citygml(path = b"urf:offsetDirection")]
    pub offset_direction: Option<String>,
    // TODO
    // #[citygml(path = b"urf:location/gml:MultiCurve")]
    // pub location: Option<MultiCurve>,
}
