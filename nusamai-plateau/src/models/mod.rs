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

impl TopLevelCityObject {
    pub fn name(&self) -> String {
        match self {
            TopLevelCityObject::Building(_) => "Building".to_string(),
            TopLevelCityObject::Road(_) => "Road".to_string(),
            TopLevelCityObject::Railway(_) => "Railway".to_string(),
            TopLevelCityObject::Track(_) => "Track".to_string(),
            TopLevelCityObject::Square(_) => "Square".to_string(),
            TopLevelCityObject::Bridge(_) => "Bridge".to_string(),
            TopLevelCityObject::CityFurniture(_) => "CityFurniture".to_string(),
            TopLevelCityObject::SolitaryVegetationObject(_) => "SolitaryVegetationObject".to_string(),
            TopLevelCityObject::PlantCover(_) => "PlantCover".to_string(),
            TopLevelCityObject::LandUse(_) => "LandUse".to_string(),
            TopLevelCityObject::Tunnel(_) => "Tunnel".to_string(),
            TopLevelCityObject::ReliefFeature(_) => "ReliefFeature".to_string(),
            TopLevelCityObject::WaterBody(_) => "WaterBody".to_string(),
            TopLevelCityObject::GenericCityObject(_) => "GenericCityObject".to_string(),
            TopLevelCityObject::CityObjectGroup(_) => "CityObjectGroup".to_string(),
            TopLevelCityObject::Waterway(_) => "Waterway".to_string(),
            TopLevelCityObject::OtherConstruction(_) => "OtherConstruction".to_string(),
            TopLevelCityObject::UndergroundBuilding(_) => "UndergroundBuilding".to_string(),
            TopLevelCityObject::Appurtenance(_) => "Appurtenance".to_string(),
            TopLevelCityObject::Cable(_) => "Cable".to_string(),
            TopLevelCityObject::Duct(_) => "Duct".to_string(),
            TopLevelCityObject::ElectricityCable(_) => "ElectricityCable".to_string(),
            TopLevelCityObject::Handhole(_) => "Handhole".to_string(),
            TopLevelCityObject::Manhole(_) => "Manhole".to_string(),
            TopLevelCityObject::OilGasChemicalsPipe(_) => "OilGasChemicalsPipe".to_string(),
            TopLevelCityObject::Pipe(_) => "Pipe".to_string(),
            TopLevelCityObject::SewerPipe(_) => "SewerPipe".to_string(),
            TopLevelCityObject::TelecommunicationsCable(_) => "TelecommunicationsCable".to_string(),
            TopLevelCityObject::ThermalPipe(_) => "ThermalPipe".to_string(),
            TopLevelCityObject::WaterPipe(_) => "WaterPipe".to_string(),
            TopLevelCityObject::Zone(_) => "Zone".to_string(),
            TopLevelCityObject::Agreement(_) => "Agreement".to_string(),
            TopLevelCityObject::AircraftNoiseControlZone(_) => "AircraftNoiseControlZone".to_string(),
            TopLevelCityObject::AreaClassification(_) => "AreaClassification".to_string(),
            TopLevelCityObject::CollectiveFacilitiesForReconstruction(_) => {
                "CollectiveFacilitiesForReconstruction".to_string()
            },
            TopLevelCityObject::CollectiveFacilitiesForReconstructionAndRevitalization(_) => {
                "CollectiveFacilitiesForReconstructionAndRevitalization".to_string()
            },
            TopLevelCityObject::CollectiveFacilitiesForTsunamiDisasterPrevention(_) => {
                "CollectiveFacilitiesForTsunamiDisasterPrevention".to_string()
            },
            TopLevelCityObject::CollectiveGovernmentAndPublicOfficeFacilities(_) => {
                "CollectiveGovernmentAndPublicOfficeFacilities".to_string()
            },
            TopLevelCityObject::CollectiveHousingFacilities(_) => "CollectiveHousingFacilities".to_string(),
            TopLevelCityObject::CollectiveUrbanDisasterPreventionFacilities(_) => {
                "CollectiveUrbanDisasterPreventionFacilities".to_string()
            },
            TopLevelCityObject::ConservationZoneForClustersOfTraditionalStructures(_) => {
                "ConservationZoneForClustersOfTraditionalStructures".to_string()
            },
            TopLevelCityObject::DisasterPreventionBlockImprovementProject(_) => {
                "DisasterPreventionBlockImprovementProject".to_string()
            },
            TopLevelCityObject::DisasterPreventionBlockImprovementZonePlan(_) => {
                "DisasterPreventionBlockImprovementZonePlan".to_string()
            },
            TopLevelCityObject::DistributionBusinessPark(_) => "DistributionBusinessPark".to_string(),
            TopLevelCityObject::DistributionBusinessZone(_) => "DistributionBusinessZone".to_string(),
            TopLevelCityObject::District(_) => "District".to_string(),
            TopLevelCityObject::DistrictDevelopmentPlan(_) => "DistrictDevelopmentPlan".to_string(),
            TopLevelCityObject::DistrictFacility(_) => "DistrictFacility".to_string(),
            TopLevelCityObject::DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan(_) => {
                "DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan".to_string()
            },
            TopLevelCityObject::DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict(_) => {
                "DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict".to_string()
            },
            TopLevelCityObject::DistrictPlan(_) => "DistrictPlan".to_string(),
            TopLevelCityObject::DistrictsAndZones(_) => "DistrictsAndZones".to_string(),
            TopLevelCityObject::EducationalAndCulturalFacility(_) => "EducationalAndCulturalFacility".to_string(),
            TopLevelCityObject::ExceptionalFloorAreaRateDistrict(_) => "ExceptionalFloorAreaRateDistrict".to_string(),
            TopLevelCityObject::FirePreventionDistrict(_) => "FirePreventionDistrict".to_string(),
            TopLevelCityObject::FireProtectionFacility(_) => "FireProtectionFacility".to_string(),
            TopLevelCityObject::FloodPreventionFacility(_) => "FloodPreventionFacility".to_string(),
            TopLevelCityObject::GlobalHubCityDevelopmentProject(_) => "GlobalHubCityDevelopmentProject".to_string(),
            TopLevelCityObject::GreenSpaceConservationDistrict(_) => "GreenSpaceConservationDistrict".to_string(),
            TopLevelCityObject::HeightControlDistrict(_) => "HeightControlDistrict".to_string(),
            TopLevelCityObject::HighLevelUseDistrict(_) => "HighLevelUseDistrict".to_string(),
            TopLevelCityObject::HighRiseResidentialAttractionDistrict(_) => {
                "HighRiseResidentialAttractionDistrict".to_string()
            },
            TopLevelCityObject::HistoricSceneryMaintenanceAndImprovementDistrictPlan(_) => {
                "HistoricSceneryMaintenanceAndImprovementDistrictPlan".to_string()
            },
            TopLevelCityObject::HousingControlArea(_) => "HousingControlArea".to_string(),
            TopLevelCityObject::IndustrialParkDevelopmentProject(_) => "IndustrialParkDevelopmentProject".to_string(),
            TopLevelCityObject::LandReadjustmentProject(_) => "LandReadjustmentProject".to_string(),
            TopLevelCityObject::LandReadjustmentPromotionArea(_) => "LandReadjustmentPromotionArea".to_string(),
            TopLevelCityObject::LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment(_) => {
                "LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment".to_string()
            },
            TopLevelCityObject::LandscapeZone(_) => "LandscapeZone".to_string(),
            TopLevelCityObject::MarketsSlaughterhousesCrematoria(_) => "MarketsSlaughterhousesCrematoria".to_string(),
            TopLevelCityObject::MedicalFacility(_) => "MedicalFacility".to_string(),
            TopLevelCityObject::NewHousingAndUrbanDevelopmentProject(_) => {
                "NewHousingAndUrbanDevelopmentProject".to_string()
            },
            TopLevelCityObject::NewUrbanInfrastructureProject(_) => "NewUrbanInfrastructureProject".to_string(),
            TopLevelCityObject::OpenSpaceForPublicUse(_) => "OpenSpaceForPublicUse".to_string(),
            TopLevelCityObject::ParkingPlaceDevelopmentZone(_) => "ParkingPlaceDevelopmentZone".to_string(),
            TopLevelCityObject::PortZone(_) => "PortZone".to_string(),
            TopLevelCityObject::PrivateUrbanRenewalProjectPlan(_) => "PrivateUrbanRenewalProjectPlan".to_string(),
            TopLevelCityObject::ProductiveGreenZone(_) => "ProductiveGreenZone".to_string(),
            TopLevelCityObject::ProjectPromotionArea(_) => "ProjectPromotionArea".to_string(),
            TopLevelCityObject::PromotionDistrict(_) => "PromotionDistrict".to_string(),
            TopLevelCityObject::QuasiUrbanPlanningArea(_) => "QuasiUrbanPlanningArea".to_string(),
            TopLevelCityObject::Regulation(_) => "Regulation".to_string(),
            TopLevelCityObject::ResidenceAttractionArea(_) => "ResidenceAttractionArea".to_string(),
            TopLevelCityObject::ResidentialBlockConstructionProject(_) => {
                "ResidentialBlockConstructionProject".to_string()
            },
            TopLevelCityObject::ResidentialBlockConstructionPromotionArea(_) => {
                "ResidentialBlockConstructionPromotionArea".to_string()
            },
            TopLevelCityObject::ResidentialEnvironmentImprovementDistrict(_) => {
                "ResidentialEnvironmentImprovementDistrict".to_string()
            },
            TopLevelCityObject::RoadsideDistrictFacility(_) => "RoadsideDistrictFacility".to_string(),
            TopLevelCityObject::RoadsideDistrictImprovementPlan(_) => {
                "RoadsideDistrictImprovementPlan".to_string()
            },
            TopLevelCityObject::RoadsideDistrictPlan(_) => "RoadsideDistrictPlan".to_string(),
            TopLevelCityObject::RuralDistrictFacility(_) => "RuralDistrictFacility".to_string(),
            TopLevelCityObject::RuralDistrictImprovementPlan(_) => "RuralDistrictImprovementPlan".to_string(),
            TopLevelCityObject::RuralDistrictPlan(_) => "RuralDistrictPlan".to_string(),
            TopLevelCityObject::SandControlFacility(_) => "SandControlFacility".to_string(),
            TopLevelCityObject::ScenicDistrict(_) => "ScenicDistrict".to_string(),
            TopLevelCityObject::ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities(_) => {
                "ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities".to_string()
            },
            TopLevelCityObject::ScheduledAreaForCollectiveHousingFacilities(_) => {
                "ScheduledAreaForCollectiveHousingFacilities".to_string()
            },
            TopLevelCityObject::ScheduledAreaForDistributionBusinessPark(_) => {
                "ScheduledAreaForDistributionBusinessPark".to_string()
            },
            TopLevelCityObject::ScheduledAreaForIndustrialParkDevelopmentProjects(_) => {
                "ScheduledAreaForIndustrialParkDevelopmentProjects".to_string()
            },
            TopLevelCityObject::ScheduledAreaForNewHousingAndUrbanDevelopmentProjects(_) => {
                "ScheduledAreaForNewHousingAndUrbanDevelopmentProjects".to_string()
            },
            TopLevelCityObject::ScheduledAreaForNewUrbanInfrastructureProjects(_) => {
                "ScheduledAreaForNewUrbanInfrastructureProjects".to_string()
            },
            TopLevelCityObject::ScheduledAreaForUrbanDevelopmentProject(_) => {
                "ScheduledAreaForUrbanDevelopmentProject".to_string()
            },
            TopLevelCityObject::SedimentDisasterProneArea(_) => "SedimentDisasterProneArea".to_string(),
            TopLevelCityObject::SnowProtectionFacility(_) => "SnowProtectionFacility".to_string(),
            TopLevelCityObject::SocialWelfareFacility(_) => "SocialWelfareFacility".to_string(),
            TopLevelCityObject::SpecialGreenSpaceConservationDistrict(_) => {
                "SpecialGreenSpaceConservationDistrict".to_string()
            },
            TopLevelCityObject::SpecialUrbanRenaissanceDistrict(_) => "SpecialUrbanRenaissanceDistrict".to_string(),
            TopLevelCityObject::SpecialUseAttractionDistrict(_) => "SpecialUseAttractionDistrict".to_string(),
            TopLevelCityObject::SpecialUseDistrict(_) => "SpecialUseDistrict".to_string(),
            TopLevelCityObject::SpecialUseRestrictionDistrict(_) => "SpecialUseRestrictionDistrict".to_string(),
            TopLevelCityObject::SpecialZoneForPreservationOfHistoricalLandscape(_) => {
                "SpecialZoneForPreservationOfHistoricalLandscape".to_string()
            },
            TopLevelCityObject::SpecifiedBlock(_) => "SpecifiedBlock".to_string(),
            TopLevelCityObject::SpecifiedBuildingZoneImprovementPlan(_) => {
                "SpecifiedBuildingZoneImprovementPlan".to_string()
            },
            TopLevelCityObject::SpecifiedDisasterPreventionBlockImprovementZone(_) => {
                "SpecifiedDisasterPreventionBlockImprovementZone".to_string()
            },
            TopLevelCityObject::SpecifiedUrgentUrbanRenewalArea(_) => "SpecifiedUrgentUrbanRenewalArea".to_string(),
            TopLevelCityObject::SupplyFacility(_) => "SupplyFacility".to_string(),
            TopLevelCityObject::TelecommunicationFacility(_) => "TelecommunicationFacility".to_string(),
            TopLevelCityObject::TideFacility(_) => "TideFacility".to_string(),
            TopLevelCityObject::TrafficFacility(_) => "TrafficFacility".to_string(),
            TopLevelCityObject::TreatmentFacility(_) => "TreatmentFacility".to_string(),
            TopLevelCityObject::TreePlantingDistrict(_) => "TreePlantingDistrict".to_string(),
            TopLevelCityObject::UnclassifiedBlankArea(_) => "UnclassifiedBlankArea".to_string(),
            TopLevelCityObject::UnclassifiedUseDistrict(_) => "UnclassifiedUseDistrict".to_string(),
            TopLevelCityObject::UnusedLandUsePromotionArea(_) => "UnusedLandUsePromotionArea".to_string(),
            TopLevelCityObject::UrbanDevelopmentProject(_) =>  "UrbanDevelopmentProject".to_string(),
            TopLevelCityObject::UrbanDisasterRecoveryPromotionArea(_) => "UrbanDisasterRecoveryPromotionArea".to_string(),
            TopLevelCityObject::UrbanFacility(_) => "UrbanFacility".to_string(),
            TopLevelCityObject::UrbanFacilityStipulatedByCabinetOrder(_) => {
                "UrbanFacilityStipulatedByCabinetOrder".to_string()
            },
            TopLevelCityObject::UrbanFunctionAttractionArea(_) => "UrbanFunctionAttractionArea".to_string(),
            TopLevelCityObject::UrbanPlanningArea(_) => "UrbanPlanningArea".to_string(),
            TopLevelCityObject::UrbanRedevelopmentProject(_) => "UrbanRedevelopmentProject".to_string(),
            TopLevelCityObject::UrbanRedevelopmentPromotionArea(_) => "UrbanRedevelopmentPromotionArea".to_string(),
            TopLevelCityObject::UrbanRenewalProject(_) => "UrbanRenewalProject".to_string(),
            TopLevelCityObject::UrgentUrbanRenewalArea(_) => "UrgentUrbanRenewalArea".to_string(),
            TopLevelCityObject::UseDistrict(_) => "UseDistrict".to_string(),
            TopLevelCityObject::ZoneWaterway(_) => "ZoneWaterway".to_string(),
            TopLevelCityObject::WindProtectionFacility(_) => "WindProtectionFacility".to_string(),
            TopLevelCityObject::ZonalDisasterPreventionFacility(_) => "ZonalDisasterPreventionFacility".to_string(),
            TopLevelCityObject::ZoneForPreservationOfHistoricalLandscape(_) => {
                "ZoneForPreservationOfHistoricalLandscape".to_string()
            },
            TopLevelCityObject::Unknown => "Unknown".to_string(),
        }
    }

    pub fn id(&self) -> String {
        match self {
            TopLevelCityObject::Building(v) => v.id.clone(),
            TopLevelCityObject::Road(v) => v.id.clone(),
            TopLevelCityObject::Railway(v) => v.id.clone(),
            TopLevelCityObject::Track(v) => v.id.clone(),
            TopLevelCityObject::Square(v) => v.id.clone(),
            TopLevelCityObject::Bridge(v) => v.id.clone(),
            TopLevelCityObject::CityFurniture(v) => v.id.clone(),
            TopLevelCityObject::SolitaryVegetationObject(v) => v.id.clone(),
            TopLevelCityObject::PlantCover(v) => v.id.clone(),
            TopLevelCityObject::LandUse(v) => v.id.clone(),
            TopLevelCityObject::Tunnel(v) => v.id.clone(),
            TopLevelCityObject::ReliefFeature(v) => v.id.clone(),
            TopLevelCityObject::WaterBody(v) => v.id.clone(),
            TopLevelCityObject::GenericCityObject(v) => v.id.clone(),
            TopLevelCityObject::CityObjectGroup(v) => v.id.clone(),
            TopLevelCityObject::Waterway(v) => v.id.clone(),
            TopLevelCityObject::OtherConstruction(v) => v.id.clone(),
            TopLevelCityObject::UndergroundBuilding(v) => v.id.clone(),
            TopLevelCityObject::Appurtenance(v) => v.id.clone(),
            TopLevelCityObject::Cable(v) => v.id.clone(),
            TopLevelCityObject::Duct(v) => v.id.clone(),
            TopLevelCityObject::ElectricityCable(v) => v.id.clone(),
            TopLevelCityObject::Handhole(v) => v.id.clone(),
            TopLevelCityObject::Manhole(v) => v.id.clone(),
            TopLevelCityObject::OilGasChemicalsPipe(v) => v.id.clone(),
            TopLevelCityObject::Pipe(v) => v.id.clone(),
            TopLevelCityObject::SewerPipe(v) => v.id.clone(),
            TopLevelCityObject::TelecommunicationsCable(v) => v.id.clone(),
            TopLevelCityObject::ThermalPipe(v) => v.id.clone(),
            TopLevelCityObject::WaterPipe(v) => v.id.clone(),
            TopLevelCityObject::Zone(v) => v.id.clone(),
            TopLevelCityObject::Agreement(v) => v.id.clone(),
            TopLevelCityObject::AircraftNoiseControlZone(v) => v.id.clone(),
            TopLevelCityObject::AreaClassification(v) => v.id.clone(),
            TopLevelCityObject::CollectiveFacilitiesForReconstruction(v) => v.id.clone(),
            TopLevelCityObject::CollectiveFacilitiesForReconstructionAndRevitalization(v) => v.id.clone(),
            TopLevelCityObject::CollectiveFacilitiesForTsunamiDisasterPrevention(v) => v.id.clone(),
            TopLevelCityObject::CollectiveGovernmentAndPublicOfficeFacilities(v) => v.id.clone(),
            TopLevelCityObject::CollectiveHousingFacilities(v) => v.id.clone(),
            TopLevelCityObject::CollectiveUrbanDisasterPreventionFacilities(v) => v.id.clone(),
            TopLevelCityObject::ConservationZoneForClustersOfTraditionalStructures(v) => v.id.clone(),
            TopLevelCityObject::DisasterPreventionBlockImprovementProject(v) => v.id.clone(),
            TopLevelCityObject::DisasterPreventionBlockImprovementZonePlan(v) => v.id.clone(),
            TopLevelCityObject::DistributionBusinessPark(v) => v.id.clone(),
            TopLevelCityObject::DistributionBusinessZone(v) => v.id.clone(),
            TopLevelCityObject::District(v) => v.id.clone(),
            TopLevelCityObject::DistrictDevelopmentPlan(v) => v.id.clone(),
            TopLevelCityObject::DistrictFacility(v) => v.id.clone(),
            TopLevelCityObject::DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan(v) => v.id.clone(),
            TopLevelCityObject::DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict(v) => v.id.clone(),
            TopLevelCityObject::DistrictPlan(v) => v.id.clone(),
            TopLevelCityObject::DistrictsAndZones(v) => v.id.clone(),
            TopLevelCityObject::EducationalAndCulturalFacility(v) => v.id.clone(),
            TopLevelCityObject::ExceptionalFloorAreaRateDistrict(v) => v.id.clone(),
            TopLevelCityObject::FirePreventionDistrict(v) => v.id.clone(),
            TopLevelCityObject::FireProtectionFacility(v) => v.id.clone(),
            TopLevelCityObject::FloodPreventionFacility(v) => v.id.clone(),
            TopLevelCityObject::GlobalHubCityDevelopmentProject(v) => v.id.clone(),
            TopLevelCityObject::GreenSpaceConservationDistrict(v) => v.id.clone(),
            TopLevelCityObject::HeightControlDistrict(v) => v.id.clone(),
            TopLevelCityObject::HighLevelUseDistrict(v) => v.id.clone(),
            TopLevelCityObject::HighRiseResidentialAttractionDistrict(v) => v.id.clone(),
            TopLevelCityObject::HistoricSceneryMaintenanceAndImprovementDistrictPlan(v) => v.id.clone(),
            TopLevelCityObject::HousingControlArea(v) => v.id.clone(),
            TopLevelCityObject::IndustrialParkDevelopmentProject(v) => v.id.clone(),
            TopLevelCityObject::LandReadjustmentProject(v) => v.id.clone(),
            TopLevelCityObject::LandReadjustmentPromotionArea(v) => v.id.clone(),
            TopLevelCityObject::LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment(v) => v.id.clone(),
            TopLevelCityObject::LandscapeZone(v) => v.id.clone(),
            TopLevelCityObject::MarketsSlaughterhousesCrematoria(v) => v.id.clone(),
            TopLevelCityObject::MedicalFacility(v) => v.id.clone(),
            TopLevelCityObject::NewHousingAndUrbanDevelopmentProject(v) => v.id.clone(),
            TopLevelCityObject::NewUrbanInfrastructureProject(v) => v.id.clone(),
            TopLevelCityObject::OpenSpaceForPublicUse(v) => v.id.clone(),
            TopLevelCityObject::ParkingPlaceDevelopmentZone(v) => v.id.clone(),
            TopLevelCityObject::PortZone(v) => v.id.clone(),
            TopLevelCityObject::PrivateUrbanRenewalProjectPlan(v) => v.id.clone(),
            TopLevelCityObject::ProductiveGreenZone(v) => v.id.clone(),
            TopLevelCityObject::ProjectPromotionArea(v) => v.id.clone(),
            TopLevelCityObject::PromotionDistrict(v) => v.id.clone(),
            TopLevelCityObject::QuasiUrbanPlanningArea(v) => v.id.clone(),
            TopLevelCityObject::Regulation(v) => v.id.clone(),
            TopLevelCityObject::ResidenceAttractionArea(v) => v.id.clone(),
            TopLevelCityObject::ResidentialBlockConstructionProject(v) => v.id.clone(),
            TopLevelCityObject::ResidentialBlockConstructionPromotionArea(v) => v.id.clone(),
            TopLevelCityObject::ResidentialEnvironmentImprovementDistrict(v) => v.id.clone(),
            TopLevelCityObject::RoadsideDistrictFacility(v) => v.id.clone(),
            TopLevelCityObject::RoadsideDistrictImprovementPlan(v) => v.id.clone(),
            TopLevelCityObject::RoadsideDistrictPlan(v) => v.id.clone(),
            TopLevelCityObject::RuralDistrictFacility(v) => v.id.clone(),
            TopLevelCityObject::RuralDistrictImprovementPlan(v) => v.id.clone(),
            TopLevelCityObject::RuralDistrictPlan(v) => v.id.clone(),
            TopLevelCityObject::SandControlFacility(v) => v.id.clone(),
            TopLevelCityObject::ScenicDistrict(v) => v.id.clone(),
            TopLevelCityObject::ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities(v) => v.id.clone(),
            TopLevelCityObject::ScheduledAreaForCollectiveHousingFacilities(v) => v.id.clone(),
            TopLevelCityObject::ScheduledAreaForDistributionBusinessPark(v) => v.id.clone(),
            TopLevelCityObject::ScheduledAreaForIndustrialParkDevelopmentProjects(v) => v.id.clone(),
            TopLevelCityObject::ScheduledAreaForNewHousingAndUrbanDevelopmentProjects(v) => v.id.clone(),
            TopLevelCityObject::ScheduledAreaForNewUrbanInfrastructureProjects(v) => v.id.clone(),
            TopLevelCityObject::ScheduledAreaForUrbanDevelopmentProject(v) => v.id.clone(),
            TopLevelCityObject::SedimentDisasterProneArea(v) => v.id.clone(),
            TopLevelCityObject::SnowProtectionFacility(v) => v.id.clone(),
            TopLevelCityObject::SocialWelfareFacility(v) => v.id.clone(),
            TopLevelCityObject::SpecialGreenSpaceConservationDistrict(v) => v.id.clone(),
            TopLevelCityObject::SpecialUrbanRenaissanceDistrict(v) => v.id.clone(),
            TopLevelCityObject::SpecialUseAttractionDistrict(v) => v.id.clone(),
            TopLevelCityObject::SpecialUseDistrict(v) => v.id.clone(),
            TopLevelCityObject::SpecialUseRestrictionDistrict(v) => v.id.clone(),
            TopLevelCityObject::SpecialZoneForPreservationOfHistoricalLandscape(v) => v.id.clone(),
            TopLevelCityObject::SpecifiedBlock(v) => v.id.clone(),
            TopLevelCityObject::SpecifiedBuildingZoneImprovementPlan(v) => v.id.clone(),
            TopLevelCityObject::SpecifiedDisasterPreventionBlockImprovementZone(v) => v.id.clone(),
            TopLevelCityObject::SpecifiedUrgentUrbanRenewalArea(v) => v.id.clone(),
            TopLevelCityObject::SupplyFacility(v) => v.id.clone(),
            TopLevelCityObject::TelecommunicationFacility(v) => v.id.clone(),
            TopLevelCityObject::TideFacility(v) => v.id.clone(),
            TopLevelCityObject::TrafficFacility(v) => v.id.clone(),
            TopLevelCityObject::TreatmentFacility(v) => v.id.clone(),
            TopLevelCityObject::TreePlantingDistrict(v) => v.id.clone(),
            TopLevelCityObject::UnclassifiedBlankArea(v) => v.id.clone(),
            TopLevelCityObject::UnclassifiedUseDistrict(v) => v.id.clone(),
            TopLevelCityObject::UnusedLandUsePromotionArea(v) => v.id.clone(),
            TopLevelCityObject::UrbanDevelopmentProject(v) => v.id.clone(),
            TopLevelCityObject::UrbanDisasterRecoveryPromotionArea(v) => v.id.clone(),
            TopLevelCityObject::UrbanFacility(v) => v.id.clone(),
            TopLevelCityObject::UrbanFacilityStipulatedByCabinetOrder(v) => v.id.clone(),
            TopLevelCityObject::UrbanFunctionAttractionArea(v) => v.id.clone(),
            TopLevelCityObject::UrbanPlanningArea(v) => v.id.clone(),
            TopLevelCityObject::UrbanRedevelopmentProject(v) => v.id.clone(),
            TopLevelCityObject::UrbanRedevelopmentPromotionArea(v) => v.id.clone(),
            TopLevelCityObject::UrbanRenewalProject(v) => v.id.clone(),
            TopLevelCityObject::UrgentUrbanRenewalArea(v) => v.id.clone(),
            TopLevelCityObject::UseDistrict(v) => v.id.clone(),
            TopLevelCityObject::ZoneWaterway(v) => v.id.clone(),
            TopLevelCityObject::WindProtectionFacility(v) => v.id.clone(),
            TopLevelCityObject::ZonalDisasterPreventionFacility(v) => v.id.clone(),
            TopLevelCityObject::ZoneForPreservationOfHistoricalLandscape(v) => v.id.clone(),
            TopLevelCityObject::Unknown => todo!(),
        }
    }

    pub fn description(&self) -> Option<String> {
        match self {
            TopLevelCityObject::Building(v) => v.description.clone(),
            TopLevelCityObject::Road(v) => v.description.clone(),
            TopLevelCityObject::Railway(v) => v.description.clone(),
            TopLevelCityObject::Track(v) => v.description.clone(),
            TopLevelCityObject::Square(v) => v.description.clone(),
            TopLevelCityObject::Bridge(v) => v.description.clone(),
            TopLevelCityObject::CityFurniture(v) => v.description.clone(),
            TopLevelCityObject::SolitaryVegetationObject(v) => v.description.clone(),
            TopLevelCityObject::PlantCover(v) => v.description.clone(),
            TopLevelCityObject::LandUse(v) => v.description.clone(),
            TopLevelCityObject::Tunnel(v) => v.description.clone(),
            TopLevelCityObject::ReliefFeature(v) => v.description.clone(),
            TopLevelCityObject::WaterBody(v) => v.description.clone(),
            TopLevelCityObject::GenericCityObject(v) => v.description.clone(),
            TopLevelCityObject::CityObjectGroup(v) => v.description.clone(),
            TopLevelCityObject::Waterway(v) => v.description.clone(),
            TopLevelCityObject::OtherConstruction(v) => v.description.clone(),
            TopLevelCityObject::UndergroundBuilding(v) => v.description.clone(),
            TopLevelCityObject::Appurtenance(v) => v.description.clone(),
            TopLevelCityObject::Cable(v) => v.description.clone(),
            TopLevelCityObject::Duct(v) => v.description.clone(),
            TopLevelCityObject::ElectricityCable(v) => v.description.clone(),
            TopLevelCityObject::Handhole(v) => v.description.clone(),
            TopLevelCityObject::Manhole(v) => v.description.clone(),
            TopLevelCityObject::OilGasChemicalsPipe(v) => v.description.clone(),
            TopLevelCityObject::Pipe(v) => v.description.clone(),
            TopLevelCityObject::SewerPipe(v) => v.description.clone(),
            TopLevelCityObject::TelecommunicationsCable(v) => v.description.clone(),
            TopLevelCityObject::ThermalPipe(v) => v.description.clone(),
            TopLevelCityObject::WaterPipe(v) => v.description.clone(),
            TopLevelCityObject::Zone(v) => v.description.clone(),
            TopLevelCityObject::Agreement(v) => v.description.clone(),
            TopLevelCityObject::AircraftNoiseControlZone(v) => v.description.clone(),
            TopLevelCityObject::AreaClassification(v) => v.description.clone(),
            TopLevelCityObject::CollectiveFacilitiesForReconstruction(v) => v.description.clone(),
            TopLevelCityObject::CollectiveFacilitiesForReconstructionAndRevitalization(v) => v.description.clone(),
            TopLevelCityObject::CollectiveFacilitiesForTsunamiDisasterPrevention(v) => v.description.clone(),
            TopLevelCityObject::CollectiveGovernmentAndPublicOfficeFacilities(v) => v.description.clone(),
            TopLevelCityObject::CollectiveHousingFacilities(v) => v.description.clone(),
            TopLevelCityObject::CollectiveUrbanDisasterPreventionFacilities(v) => v.description.clone(),
            TopLevelCityObject::ConservationZoneForClustersOfTraditionalStructures(v) => v.description.clone(),
            TopLevelCityObject::DisasterPreventionBlockImprovementProject(v) => v.description.clone(),
            TopLevelCityObject::DisasterPreventionBlockImprovementZonePlan(v) => v.description.clone(),
            TopLevelCityObject::DistributionBusinessPark(v) => v.description.clone(),
            TopLevelCityObject::DistributionBusinessZone(v) => v.description.clone(),
            TopLevelCityObject::District(v) => v.description.clone(),
            TopLevelCityObject::DistrictDevelopmentPlan(v) => v.description.clone(),
            TopLevelCityObject::DistrictFacility(v) => v.description.clone(),
            TopLevelCityObject::DistrictImprovementPlanForDisasterPreventionBlockImprovementZonePlan(v) => v.description.clone(),
            TopLevelCityObject::DistrictImprovementPlanForHistoricSceneryMaintenanceAndImprovementDistrict(v) => v.description.clone(),
            TopLevelCityObject::DistrictPlan(v) => v.description.clone(),
            TopLevelCityObject::DistrictsAndZones(v) => v.description.clone(),
            TopLevelCityObject::EducationalAndCulturalFacility(v) => v.description.clone(),
            TopLevelCityObject::ExceptionalFloorAreaRateDistrict(v) => v.description.clone(),
            TopLevelCityObject::FirePreventionDistrict(v) => v.description.clone(),
            TopLevelCityObject::FireProtectionFacility(v) => v.description.clone(),
            TopLevelCityObject::FloodPreventionFacility(v) => v.description.clone(),
            TopLevelCityObject::GlobalHubCityDevelopmentProject(v) => v.description.clone(),
            TopLevelCityObject::GreenSpaceConservationDistrict(v) => v.description.clone(),
            TopLevelCityObject::HeightControlDistrict(v) => v.description.clone(),
            TopLevelCityObject::HighLevelUseDistrict(v) => v.description.clone(),
            TopLevelCityObject::HighRiseResidentialAttractionDistrict(v) => v.description.clone(),
            TopLevelCityObject::HistoricSceneryMaintenanceAndImprovementDistrictPlan(v) => v.description.clone(),
            TopLevelCityObject::HousingControlArea(v) => v.description.clone(),
            TopLevelCityObject::IndustrialParkDevelopmentProject(v) => v.description.clone(),
            TopLevelCityObject::LandReadjustmentProject(v) => v.description.clone(),
            TopLevelCityObject::LandReadjustmentPromotionArea(v) => v.description.clone(),
            TopLevelCityObject::LandReadjustmentPromotionAreasForCoreBusinessUrbanDevelopment(v) => v.description.clone(),
            TopLevelCityObject::LandscapeZone(v) => v.description.clone(),
            TopLevelCityObject::MarketsSlaughterhousesCrematoria(v) => v.description.clone(),
            TopLevelCityObject::MedicalFacility(v) => v.description.clone(),
            TopLevelCityObject::NewHousingAndUrbanDevelopmentProject(v) => v.description.clone(),
            TopLevelCityObject::NewUrbanInfrastructureProject(v) => v.description.clone(),
            TopLevelCityObject::OpenSpaceForPublicUse(v) => v.description.clone(),
            TopLevelCityObject::ParkingPlaceDevelopmentZone(v) => v.description.clone(),
            TopLevelCityObject::PortZone(v) => v.description.clone(),
            TopLevelCityObject::PrivateUrbanRenewalProjectPlan(v) => v.description.clone(),
            TopLevelCityObject::ProductiveGreenZone(v) => v.description.clone(),
            TopLevelCityObject::ProjectPromotionArea(v) => v.description.clone(),
            TopLevelCityObject::PromotionDistrict(v) => v.description.clone(),
            TopLevelCityObject::QuasiUrbanPlanningArea(v) => v.description.clone(),
            TopLevelCityObject::Regulation(v) => v.description.clone(),
            TopLevelCityObject::ResidenceAttractionArea(v) => v.description.clone(),
            TopLevelCityObject::ResidentialBlockConstructionProject(v) => v.description.clone(),
            TopLevelCityObject::ResidentialBlockConstructionPromotionArea(v) => v.description.clone(),
            TopLevelCityObject::ResidentialEnvironmentImprovementDistrict(v) => v.description.clone(),
            TopLevelCityObject::RoadsideDistrictFacility(v) => v.description.clone(),
            TopLevelCityObject::RoadsideDistrictImprovementPlan(v) => v.description.clone(),
            TopLevelCityObject::RoadsideDistrictPlan(v) => v.description.clone(),
            TopLevelCityObject::RuralDistrictFacility(v) => v.description.clone(),
            TopLevelCityObject::RuralDistrictImprovementPlan(v) => v.description.clone(),
            TopLevelCityObject::RuralDistrictPlan(v) => v.description.clone(),
            TopLevelCityObject::SandControlFacility(v) => v.description.clone(),
            TopLevelCityObject::ScenicDistrict(v) => v.description.clone(),
            TopLevelCityObject::ScheduledAreaForCollectiveGovernmentAndPublicOfficeFacilities(v) => v.description.clone(),
            TopLevelCityObject::ScheduledAreaForCollectiveHousingFacilities(v) => v.description.clone(),
            TopLevelCityObject::ScheduledAreaForDistributionBusinessPark(v) => v.description.clone(),
            TopLevelCityObject::ScheduledAreaForIndustrialParkDevelopmentProjects(v) => v.description.clone(),
            TopLevelCityObject::ScheduledAreaForNewHousingAndUrbanDevelopmentProjects(v) => v.description.clone(),
            TopLevelCityObject::ScheduledAreaForNewUrbanInfrastructureProjects(v) => v.description.clone(),
            TopLevelCityObject::ScheduledAreaForUrbanDevelopmentProject(v) => v.description.clone(),
            TopLevelCityObject::SedimentDisasterProneArea(v) => v.description.clone(),
            TopLevelCityObject::SnowProtectionFacility(v) => v.description.clone(),
            TopLevelCityObject::SocialWelfareFacility(v) => v.description.clone(),
            TopLevelCityObject::SpecialGreenSpaceConservationDistrict(v) => v.description.clone(),
            TopLevelCityObject::SpecialUrbanRenaissanceDistrict(v) => v.description.clone(),
            TopLevelCityObject::SpecialUseAttractionDistrict(v) => v.description.clone(),
            TopLevelCityObject::SpecialUseDistrict(v) => v.description.clone(),
            TopLevelCityObject::SpecialUseRestrictionDistrict(v) => v.description.clone(),
            TopLevelCityObject::SpecialZoneForPreservationOfHistoricalLandscape(v) => v.description.clone(),
            TopLevelCityObject::SpecifiedBlock(v) => v.description.clone(),
            TopLevelCityObject::SpecifiedBuildingZoneImprovementPlan(v) => v.description.clone(),
            TopLevelCityObject::SpecifiedDisasterPreventionBlockImprovementZone(v) => v.description.clone(),
            TopLevelCityObject::SpecifiedUrgentUrbanRenewalArea(v) => v.description.clone(),
            TopLevelCityObject::SupplyFacility(v) => v.description.clone(),
            TopLevelCityObject::TelecommunicationFacility(v) => v.description.clone(),
            TopLevelCityObject::TideFacility(v) => v.description.clone(),
            TopLevelCityObject::TrafficFacility(v) => v.description.clone(),
            TopLevelCityObject::TreatmentFacility(v) => v.description.clone(),
            TopLevelCityObject::TreePlantingDistrict(v) => v.description.clone(),
            TopLevelCityObject::UnclassifiedBlankArea(v) => v.description.clone(),
            TopLevelCityObject::UnclassifiedUseDistrict(v) => v.description.clone(),
            TopLevelCityObject::UnusedLandUsePromotionArea(v) => v.description.clone(),
            TopLevelCityObject::UrbanDevelopmentProject(v) => v.description.clone(),
            TopLevelCityObject::UrbanDisasterRecoveryPromotionArea(v) => v.description.clone(),
            TopLevelCityObject::UrbanFacility(v) => v.description.clone(),
            TopLevelCityObject::UrbanFacilityStipulatedByCabinetOrder(v) => v.description.clone(),
            TopLevelCityObject::UrbanFunctionAttractionArea(v) => v.description.clone(),
            TopLevelCityObject::UrbanPlanningArea(v) => v.description.clone(),
            TopLevelCityObject::UrbanRedevelopmentProject(v) => v.description.clone(),
            TopLevelCityObject::UrbanRedevelopmentPromotionArea(v) => v.description.clone(),
            TopLevelCityObject::UrbanRenewalProject(v) => v.description.clone(),
            TopLevelCityObject::UrgentUrbanRenewalArea(v) => v.description.clone(),
            TopLevelCityObject::UseDistrict(v) => v.description.clone(),
            TopLevelCityObject::ZoneWaterway(v) => v.description.clone(),
            TopLevelCityObject::WindProtectionFacility(v) => v.description.clone(),
            TopLevelCityObject::ZonalDisasterPreventionFacility(v) => v.description.clone(),
            TopLevelCityObject::ZoneForPreservationOfHistoricalLandscape(v) => v.description.clone(),
            TopLevelCityObject::Unknown => todo!(),
        }
    }
}
