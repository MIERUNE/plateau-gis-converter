pub mod appearance;
pub mod bridge;
pub mod building;
pub mod city_furniture;
pub mod cityobjectgroup;
pub mod core;
pub mod generics;
pub mod iur;
pub mod landuse;
pub mod other_construction;
pub mod relief;
pub mod transportation;
pub mod tunnel;
pub mod vegetation;
pub mod waterbody;

pub use bridge::Bridge;
pub use building::Building;
pub use city_furniture::CityFurniture;
pub use cityobjectgroup::CityObjectGroup;
pub use generics::GenericCityObject;
pub use iur::{urf, uro};
pub use landuse::LandUse;
use nusamai_citygml::{citygml_property, CityGmlElement};
pub use other_construction::OtherConstruction;
pub use relief::ReliefFeature;
pub use transportation::{Railway, Road, Square, Track, Waterway};
pub use tunnel::Tunnel;
pub use vegetation::{PlantCover, SolitaryVegetationObject};
pub use waterbody::WaterBody;

#[citygml_property(name = "_:TopLevelFeatureProperty")]
pub enum TopLevelCityObject {
    //
    // CityGML 2.0 standard
    //
    #[citygml(path = b"bldg:Building")]
    Building(Building),
    #[citygml(path = b"tran:Road")]
    Road(Road),
    #[citygml(path = b"tran:Railway")]
    Railway(Railway),
    #[citygml(path = b"tran:Track")]
    Track(Track),
    #[citygml(path = b"tran:Square")]
    Square(Square),
    #[citygml(path = b"brid:Bridge")]
    Bridge(Bridge),
    #[citygml(path = b"frn:CityFurniture")]
    CityFurniture(CityFurniture),
    #[citygml(path = b"veg:SolitaryVegetationObject")]
    SolitaryVegetationObject(SolitaryVegetationObject),
    #[citygml(path = b"veg:PlantCover")]
    PlantCover(PlantCover),
    #[citygml(path = b"luse:LandUse")]
    LandUse(LandUse),
    #[citygml(path = b"tun:Tunnel")]
    Tunnel(Tunnel),
    #[citygml(path = b"dem:ReliefFeature")]
    ReliefFeature(ReliefFeature),
    #[citygml(path = b"wtr:WaterBody")]
    WaterBody(WaterBody),
    #[citygml(path = b"gen:GenericCityObject")]
    GenericCityObject(GenericCityObject),
    #[citygml(path = b"grp:CityObjectGroup")]
    CityObjectGroup(CityObjectGroup),
    //
    // CityGML 3.0 standard preview
    //
    #[citygml(path = b"uro:Waterway")]
    Waterway(Waterway),
    #[citygml(path = b"uro:OtherConstruction")]
    OtherConstruction(OtherConstruction),
    //
    // i-UR urban objects
    //
    #[citygml(path = b"uro:UndergroundBuilding")]
    UndergroundBuilding(uro::UndergroundBuilding),
    #[citygml(path = b"uro:Appurtenance")]
    Appurtenance(uro::Appurtenance),
    #[citygml(path = b"uro:Cable")]
    Cable(uro::Cable),
    #[citygml(path = b"uro:Duct")]
    Duct(uro::Duct),
    #[citygml(path = b"uro:ElectricityCable")]
    ElectricityCable(uro::ElectricityCable),
    #[citygml(path = b"uro:Handhole")]
    Handhole(uro::Handhole),
    #[citygml(path = b"uro:Manhole")]
    Manhole(uro::Manhole),
    #[citygml(path = b"uro:OilGasChemicalsPipe")]
    OilGasChemicalsPipe(uro::OilGasChemicalsPipe),
    #[citygml(path = b"uro:Pipe")]
    Pipe(uro::Pipe),
    #[citygml(path = b"uro:SewerPipe")]
    SewerPipe(uro::SewerPipe),
    #[citygml(path = b"uro:TelecommunicationsCable")]
    TelecommunicationsCable(uro::TelecommunicationsCable),
    #[citygml(path = b"uro:ThermalPipe")]
    ThermalPipe(uro::ThermalPipe),
    #[citygml(path = b"uro:WaterPipe")]
    WaterPipe(uro::WaterPipe),
    //
    // i-UR urban functions
    //
    #[citygml(path = b"urf:Zone")]
    Zone(urf::Zone),
    #[citygml(path = b"urf:Agreement")]
    Agreement(urf::Agreement),
    #[citygml(path = b"urf:AircraftNoiseControlZone")]
    AircraftNoiseControlZone(urf::AircraftNoiseControlZone),
    #[citygml(path = b"urf:AreaClassification")]
    AreaClassification(urf::AreaClassification),
    #[citygml(path = b"urf:CollectiveFacilitiesForReconstruction")]
    CollectiveFacilitiesForReconstruction(urf::CollectiveFacilitiesForReconstruction),
    #[citygml(path = b"urf:CollectiveFacilitiesForReconstructionAndRevitalization")]
    CollectiveFacilitiesForReconstructionAndRevitalization(
        urf::CollectiveFacilitiesForReconstructionAndRevitalization,
    ),
    #[citygml(path = b"urf:CollectiveFacilitiesForTsunamiDisasterPrevention")]
    CollectiveFacilitiesForTsunamiDisasterPrevention(
        urf::CollectiveFacilitiesForTsunamiDisasterPrevention,
    ),
    #[citygml(path = b"urf:CollectiveGovernmentAndPublicOfficeFacilities")]
    CollectiveGovernmentAndPublicOfficeFacilities(
        urf::CollectiveGovernmentAndPublicOfficeFacilities,
    ),
    #[citygml(path = b"urf:CollectiveHousingFacilities")]
    CollectiveHousingFacilities(urf::CollectiveHousingFacilities),
    #[citygml(path = b"urf:CollectiveUrbanDisasterPreventionFacilities")]
    CollectiveUrbanDisasterPreventionFacilities(urf::CollectiveUrbanDisasterPreventionFacilities),
    #[citygml(path = b"urf:ConservationZoneForClustersOfTraditionalStructures")]
    ConservationZoneForClustersOfTraditionalStructures(
        urf::ConservationZoneForClustersOfTraditionalStructures,
    ),
    #[citygml(path = b"urf:DisasterPreventionBlockImprovementProject")]
    DisasterPreventionBlockImprovementProject(urf::DisasterPreventionBlockImprovementProject),
    #[citygml(path = b"urf:DisasterPreventionBlockImprovementZonePlan")]
    DisasterPreventionBlockImprovementZonePlan(urf::DisasterPreventionBlockImprovementZonePlan),
    #[citygml(path = b"urf:DistributionBusinessPark")]
    DistributionBusinessPark(urf::DistributionBusinessPark),
    #[citygml(path = b"urf:DistributionBusinessZone")]
    DistributionBusinessZone(urf::DistributionBusinessZone),
    #[citygml(path = b"urf:District")]
    District(urf::District),
    #[citygml(path = b"urf:DistrictDevelopmentPlan")]
    DistrictDevelopmentPlan(urf::DistrictDevelopmentPlan),
    #[citygml(path = b"urf:DistrictFacility")]
    DistrictFacility(urf::DistrictFacility),
    #[citygml(path = b"urf:DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan")]
    DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan(
        urf::DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan,
    ),
    #[citygml(
        path = b"urf:DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict"
    )]
    DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict(
        urf::DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict,
    ),
    #[citygml(path = b"urf:DistrictPlan")]
    DistrictPlan(urf::DistrictPlan),
    #[citygml(path = b"urf:DistrictsAndZones")]
    DistrictsAndZones(urf::DistrictsAndZones),
    #[citygml(path = b"urf:EducationalAndCulturalFacility")]
    EducationalAndCulturalFacility(urf::EducationalAndCulturalFacility),
    #[citygml(path = b"urf:ExceptionalFloorAreaRateDistrict")]
    ExceptionalFloorAreaRateDistrict(urf::ExceptionalFloorAreaRateDistrict),
    #[citygml(path = b"urf:FirePreventionDistrict")]
    FirePreventionDistrict(urf::FirePreventionDistrict),
    #[citygml(path = b"urf:FireProtectionFacility")]
    FireProtectionFacility(urf::FireProtectionFacility),
    #[citygml(path = b"urf:FloodPreventionFacility")]
    FloodPreventionFacility(urf::FloodPreventionFacility),
    #[citygml(path = b"urf:GlobalHubCityDevelopmentProject")]
    GlobalHubCityDevelopmentProject(urf::GlobalHubCityDevelopmentProject),
    #[citygml(path = b"urf:GreenSpaceConservationDistrict")]
    GreenSpaceConservationDistrict(urf::GreenSpaceConservationDistrict),
    #[citygml(path = b"urf:HeightControlDistrict")]
    HeightControlDistrict(urf::HeightControlDistrict),
    #[citygml(path = b"urf:HighLevelUseDistrict")]
    HighLevelUseDistrict(urf::HighLevelUseDistrict),
    #[citygml(path = b"urf:HighRiseResidentialAttractionDistrict")]
    HighRiseResidentialAttractionDistrict(urf::HighRiseResidentialAttractionDistrict),
    #[citygml(path = b"urf:HistoricSceneryMaintenanceAndImprovementDistrictPlan")]
    HistoricSceneryMaintenanceAndImprovementDistrictPlan(
        urf::HistoricSceneryMaintenanceAndImprovementDistrictPlan,
    ),
    #[citygml(path = b"urf:HousingControlArea")]
    HousingControlArea(urf::HousingControlArea),
    #[citygml(path = b"urf:IndustrialParkDevelopmentProject")]
    IndustrialParkDevelopmentProject(urf::IndustrialParkDevelopmentProject),
    #[citygml(path = b"urf:LandReadjustmentProject")]
    LandReadjustmentProject(urf::LandReadjustmentProject),
    #[citygml(path = b"urf:LandReadjustmentPromotionArea")]
    LandReadjustmentPromotionArea(urf::LandReadjustmentPromotionArea),
    #[citygml(path = b"urf:LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment")]
    LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment(
        urf::LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment,
    ),
    #[citygml(path = b"urf:LandscapeZone")]
    LandscapeZone(urf::LandscapeZone),
    #[citygml(path = b"urf:MarketsSlaughterhousesCrematoria")]
    MarketsSlaughterhousesCrematoria(urf::MarketsSlaughterhousesCrematoria),
    #[citygml(path = b"urf:MedicalFacility")]
    MedicalFacility(urf::MedicalFacility),
    #[citygml(path = b"urf:NewHousingAndUrbanDevelopmentProject")]
    NewHousingAndUrbanDevelopmentProject(urf::NewHousingAndUrbanDevelopmentProject),
    #[citygml(path = b"urf:NewUrbanInfrastructureProject")]
    NewUrbanInfrastructureProject(urf::NewUrbanInfrastructureProject),
    #[citygml(path = b"urf:OpenSpaceForPublicUse")]
    OpenSpaceForPublicUse(urf::OpenSpaceForPublicUse),
    #[citygml(path = b"urf:ParkingPlaceDevelopmentZone")]
    ParkingPlaceDevelopmentZone(urf::ParkingPlaceDevelopmentZone),
    #[citygml(path = b"urf:PortZone")]
    PortZone(urf::PortZone),
    #[citygml(path = b"urf:PrivateUrbanRenewalProjectPlan")]
    PrivateUrbanRenewalProjectPlan(urf::PrivateUrbanRenewalProjectPlan),
    #[citygml(path = b"urf:ProductiveGreenZone")]
    ProductiveGreenZone(urf::ProductiveGreenZone),
    #[citygml(path = b"urf:ProjectPromotionArea")]
    ProjectPromotionArea(urf::ProjectPromotionArea),
    #[citygml(path = b"urf:PromotionDistrict")]
    PromotionDistrict(urf::PromotionDistrict),
    #[citygml(path = b"urf:QuasiUrbanPlanningArea")]
    QuasiUrbanPlanningArea(urf::QuasiUrbanPlanningArea),
    #[citygml(path = b"urf:Regulation")]
    Regulation(urf::Regulation),
    #[citygml(path = b"urf:ResidenceAttractionArea")]
    ResidenceAttractionArea(urf::ResidenceAttractionArea),
    #[citygml(path = b"urf:ResidentialBlockConstructionProject")]
    ResidentialBlockConstructionProject(urf::ResidentialBlockConstructionProject),
    #[citygml(path = b"urf:ResidentialBlockConstructionPromotionArea")]
    ResidentialBlockConstructionPromotionArea(urf::ResidentialBlockConstructionPromotionArea),
    #[citygml(path = b"urf:ResidentialEnvironmentImprovementDistrict")]
    ResidentialEnvironmentImprovementDistrict(urf::ResidentialEnvironmentImprovementDistrict),
    #[citygml(path = b"urf:RoadsideDistrictFacility")]
    RoadsideDistrictFacility(urf::RoadsideDistrictFacility),
    #[citygml(path = b"urf:RoadsideDistrictImprovementPlan")]
    RoadsideDistrictImprovementPlan(urf::RoadsideDistrictImprovementPlan),
    #[citygml(path = b"urf:RoadsideDistrictPlan")]
    RoadsideDistrictPlan(urf::RoadsideDistrictPlan),
    #[citygml(path = b"urf:RuralDistrictFacility")]
    RuralDistrictFacility(urf::RuralDistrictFacility),
    #[citygml(path = b"urf:RuralDistrictImprovementPlan")]
    RuralDistrictImprovementPlan(urf::RuralDistrictImprovementPlan),
    #[citygml(path = b"urf:RuralDistrictPlan")]
    RuralDistrictPlan(urf::RuralDistrictPlan),
    #[citygml(path = b"urf:SandControlFacility")]
    SandControlFacility(urf::SandControlFacility),
    #[citygml(path = b"urf:ScenicDistrict")]
    ScenicDistrict(urf::ScenicDistrict),
    #[citygml(path = b"urf:ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities")]
    ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities(
        urf::ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities,
    ),
    #[citygml(path = b"urf:ScheduledAreaForCollectiveHousingFacilities")]
    ScheduledAreaForCollectiveHousingFacilities(urf::ScheduledAreaForCollectiveHousingFacilities),
    #[citygml(path = b"urf:ScheduledAreaForDistributionBusinessPark")]
    ScheduledAreaForDistributionBusinessPark(urf::ScheduledAreaForDistributionBusinessPark),
    #[citygml(path = b"urf:ScheduledAreaForIndustrialParkDevelopmentProjects")]
    ScheduledAreaForIndustrialParkDevelopmentProjects(
        urf::ScheduledAreaForIndustrialParkDevelopmentProjects,
    ),
    #[citygml(path = b"urf:ScheduledAreaForNewHousingAndUrbanDevelopmentProjects")]
    ScheduledAreaForNewHousingAndUrbanDevelopmentProjects(
        urf::ScheduledAreaForNewHousingAndUrbanDevelopmentProjects,
    ),
    #[citygml(path = b"urf:ScheduledAreaForNewUrbanInfrastructureProjects")]
    ScheduledAreaForNewUrbanInfrastructureProjects(
        urf::ScheduledAreaForNewUrbanInfrastructureProjects,
    ),
    #[citygml(path = b"urf:ScheduledAreaForUrbanDevelopmentProject")]
    ScheduledAreaForUrbanDevelopmentProject(urf::ScheduledAreaForUrbanDevelopmentProject),
    #[citygml(path = b"urf:SedimentDisasterProneArea")]
    SedimentDisasterProneArea(urf::SedimentDisasterProneArea),
    #[citygml(path = b"urf:SnowProtectionFacility")]
    SnowProtectionFacility(urf::SnowProtectionFacility),
    #[citygml(path = b"urf:SocialWelfareFacility")]
    SocialWelfareFacility(urf::SocialWelfareFacility),
    #[citygml(path = b"urf:SpecialGreenSpaceConservationDistrict")]
    SpecialGreenSpaceConservationDistrict(urf::SpecialGreenSpaceConservationDistrict),
    #[citygml(path = b"urf:SpecialUrbanRenaissanceDistrict")]
    SpecialUrbanRenaissanceDistrict(urf::SpecialUrbanRenaissanceDistrict),
    #[citygml(path = b"urf:SpecialUseAttractionDistrict")]
    SpecialUseAttractionDistrict(urf::SpecialUseAttractionDistrict),
    #[citygml(path = b"urf:SpecialUseDistrict")]
    SpecialUseDistrict(urf::SpecialUseDistrict),
    #[citygml(path = b"urf:SpecialUseRestrictionDistrict")]
    SpecialUseRestrictionDistrict(urf::SpecialUseRestrictionDistrict),
    #[citygml(path = b"urf:SpecialZoneForPreservationOfHistoricalLandscape")]
    SpecialZoneForPreservationOfHistoricalLandscape(
        urf::SpecialZoneForPreservationOfHistoricalLandscape,
    ),
    #[citygml(path = b"urf:SpecifiedBlock")]
    SpecifiedBlock(urf::SpecifiedBlock),
    #[citygml(path = b"urf:SpecifiedBuildingZoneImprovementPlan")]
    SpecifiedBuildingZoneImprovementPlan(urf::SpecifiedBuildingZoneImprovementPlan),
    #[citygml(path = b"urf:SpecifiedDisasterPreventionBlockImprovementZone")]
    SpecifiedDisasterPreventionBlockImprovementZone(
        urf::SpecifiedDisasterPreventionBlockImprovementZone,
    ),
    #[citygml(path = b"urf:SpecifiedUrgentUrbanRenewalArea")]
    SpecifiedUrgentUrbanRenewalArea(urf::SpecifiedUrgentUrbanRenewalArea),
    #[citygml(path = b"urf:SupplyFacility")]
    SupplyFacility(urf::SupplyFacility),
    #[citygml(path = b"urf:TelecommunicationFacility")]
    TelecommunicationFacility(urf::TelecommunicationFacility),
    #[citygml(path = b"urf:TideFacility")]
    TideFacility(urf::TideFacility),
    #[citygml(path = b"urf:TrafficFacility")]
    TrafficFacility(urf::TrafficFacility),
    #[citygml(path = b"urf:TreatmentFacility")]
    TreatmentFacility(urf::TreatmentFacility),
    #[citygml(path = b"urf:TreePlantingDistrict")]
    TreePlantingDistrict(urf::TreePlantingDistrict),
    #[citygml(path = b"urf:UnclassifiedBlankArea")]
    UnclassifiedBlankArea(urf::UnclassifiedBlankArea),
    #[citygml(path = b"urf:UnclassifiedUseDistrict")]
    UnclassifiedUseDistrict(urf::UnclassifiedUseDistrict),
    #[citygml(path = b"urf:UnusedLandUsePromotionArea")]
    UnusedLandUsePromotionArea(urf::UnusedLandUsePromotionArea),
    #[citygml(path = b"urf:UrbanDevelopmentProject")]
    UrbanDevelopmentProject(urf::UrbanDevelopmentProject),
    #[citygml(path = b"urf:UrbanDisasterRecoveryPromotionArea")]
    UrbanDisasterRecoveryPromotionArea(urf::UrbanDisasterRecoveryPromotionArea),
    #[citygml(path = b"urf:UrbanFacility")]
    UrbanFacility(urf::UrbanFacility),
    #[citygml(path = b"urf:UrbanFacilityStipulatedByCabinetOrder")]
    UrbanFacilityStipulatedByCabinetOrder(urf::UrbanFacilityStipulatedByCabinetOrder),
    #[citygml(path = b"urf:UrbanFunctionAttractionArea")]
    UrbanFunctionAttractionArea(urf::UrbanFunctionAttractionArea),
    #[citygml(path = b"urf:UrbanPlanningArea")]
    UrbanPlanningArea(urf::UrbanPlanningArea),
    #[citygml(path = b"urf:UrbanRedevelopmentProject")]
    UrbanRedevelopmentProject(urf::UrbanRedevelopmentProject),
    #[citygml(path = b"urf:UrbanRedevelopmentPromotionArea")]
    UrbanRedevelopmentPromotionArea(urf::UrbanRedevelopmentPromotionArea),
    #[citygml(path = b"urf:UrbanRenewalProject")]
    UrbanRenewalProject(urf::UrbanRenewalProject),
    #[citygml(path = b"urf:UrgentUrbanRenewalArea")]
    UrgentUrbanRenewalArea(urf::UrgentUrbanRenewalArea),
    #[citygml(path = b"urf:UseDistrict")]
    UseDistrict(urf::UseDistrict),
    #[citygml(path = b"urf:Waterway")]
    ZoneWaterway(urf::Waterway),
    #[citygml(path = b"urf:WindProtectionFacility")]
    WindProtectionFacility(urf::WindProtectionFacility),
    #[citygml(path = b"urf:ZonalDisasterPreventionFacility")]
    ZonalDisasterPreventionFacility(urf::ZonalDisasterPreventionFacility),
    #[citygml(path = b"urf:ZoneForPreservationOfHistoricalLandscape")]
    ZoneForPreservationOfHistoricalLandscape(urf::ZoneForPreservationOfHistoricalLandscape),
}
