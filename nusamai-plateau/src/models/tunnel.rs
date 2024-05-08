use nusamai_citygml::{citygml_feature, citygml_property, CityGmlElement, Code, GYear};

use super::iur::uro;

#[citygml_feature(name = "tun:Tunnel")]
pub struct Tunnel {
    #[citygml(path = b"tun:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tun:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tun:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tun:yearOfConstruction")]
    pub year_of_construction: Option<GYear>,

    #[citygml(path = b"tun:yearOfDemolition")]
    pub year_of_demolition: Option<GYear>,

    #[citygml(path = b"tun:outerTunnelInstallation/tun:TunnelInstallation")]
    pub outer_tunnel_installation: Vec<TunnelInstallation>,

    #[citygml(path = b"tun:interiorTunnelInstallation/tun:IntTunnelInstallation")]
    pub interior_tunnel_installation: Vec<TunnelInstallation>,

    #[citygml(path = b"tun:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> tun:_BoundarySurface

    #[citygml(path = b"tun:interiorHollowSpace/tun:HollowSpace")]
    pub interior_hollow_space: Vec<HollowSpace>,

    #[citygml(path = b"tun:consistsOfTunnelPart/tun:TunnelPart")]
    pub consists_of_tunnel_part: Vec<TunnelPart>,

    #[citygml(path = b"uro:tunBaseAttribute/uro:ConstructionBaseAttribute")]
    pub tun_base_attribute: Vec<uro::ConstructionBaseAttribute>,

    #[citygml(path = b"uro:tunDataQualityAttribute/uro:ConstructionDataQualityAttribute")]
    #[citygml(path = b"uro:tunDataQualityAttribute/uro:DataQualityAttribute")]
    pub tun_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:tunDisasterRiskAttribute")]
    pub tun_disaster_risk_attribute: Vec<uro::DisasterRiskAttributeProperty>, // -> uro:DisasterRiskAttribute

    #[citygml(path = b"uro:tunDmAttribute")]
    pub tun_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:tunFacilityAttribute")]
    pub tun_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:tunFacilityIdAttribute")]
    pub tun_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:tunFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub tun_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:tunFunctionalAttribute/uro:TunnelFunctionalAttribute")]
    pub tun_functional_attribute: Vec<uro::TunnelFunctionalAttribute>,

    #[citygml(path = b"uro:tunKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub tun_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:tunRiskAssessmentAttribute/uro:ConstructionRiskAssessmentAttribute")]
    pub tun_risk_assessment_attribute: Vec<uro::ConstructionRiskAssessmentAttribute>,

    #[citygml(path = b"uro:tunStructureAttribute/uro:TunnelStructureAttribute")]
    pub tun_structure_attribute: Vec<uro::TunnelStructureAttribute>,
}

#[citygml_feature(name = "tun:TunnelPart")]
pub struct TunnelPart {
    #[citygml(path = b"tun:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tun:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tun:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tun:yearOfConstruction")]
    pub year_of_construction: Option<GYear>,

    #[citygml(path = b"tun:yearOfDemolition")]
    pub year_of_demolition: Option<GYear>,

    #[citygml(path = b"tun:outerTunnelInstallation/tun:TunnelInstallation")]
    pub outer_tunnel_installation: Vec<TunnelInstallation>,

    #[citygml(path = b"tun:interiorTunnelInstallation/tun:IntTunnelInstallation")]
    pub interior_tunnel_installation: Vec<TunnelInstallation>,

    #[citygml(path = b"tun:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> tun:_BoundarySurface

    #[citygml(path = b"tun:interiorHollowSpace/tun:HollowSpace")]
    pub interior_hollow_space: Vec<HollowSpace>,

    #[citygml(path = b"tun:consistsOfTunnelPart/tun:TunnelPart")]
    pub consists_of_tunnel_part: Vec<TunnelPart>,

    #[citygml(path = b"uro:tunBaseAttribute/uro:ConstructionBaseAttribute")]
    pub tun_base_attribute: Vec<uro::ConstructionBaseAttribute>,

    #[citygml(path = b"uro:tunDataQualityAttribute/uro:ConstructionDataQualityAttribute")]
    #[citygml(path = b"uro:tunDataQualityAttribute/uro:DataQualityAttribute")]
    pub tun_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:tunDisasterRiskAttribute")]
    pub tun_disaster_risk_attribute: Vec<uro::DisasterRiskAttributeProperty>, // -> uro:DisasterRiskAttribute

    #[citygml(path = b"uro:tunDmAttribute")]
    pub tun_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:tunFacilityAttribute")]
    pub tun_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:tunFacilityIdAttribute")]
    pub tun_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:tunFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub tun_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:tunFunctionalAttribute/uro:TunnelFunctionalAttribute")]
    pub tun_functional_attribute: Vec<uro::TunnelFunctionalAttribute>,

    #[citygml(path = b"uro:tunKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub tun_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:tunRiskAssessmentAttribute/uro:ConstructionRiskAssessmentAttribute")]
    pub tun_risk_assessment_attribute: Vec<uro::ConstructionRiskAssessmentAttribute>,

    #[citygml(path = b"uro:tunStructureAttribute/uro:TunnelStructureAttribute")]
    pub tun_structure_attribute: Vec<uro::TunnelStructureAttribute>,
}

#[citygml_feature(name = "tun:HollowSpace")]
pub struct HollowSpace {
    #[citygml(path = b"tun:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tun:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tun:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tun:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> tun:_BoundarySurface

    #[citygml(path = b"tun:interiorFurniture/tun:TunnelFurniture")]
    pub interior_furniture: Vec<TunnelFurniture>,

    #[citygml(path = b"tun:hollowSpaceInstallation/tun:IntTunnelInstallation")]
    pub hollow_space_installation: Vec<TunnelInstallation>,
}

#[citygml_feature(name = "tun:TunnelInstallation")]
pub struct TunnelInstallation {
    #[citygml(path = b"tun:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tun:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tun:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"tun:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> tun:_BoundarySurface
}

// Intentionally not used to facilitate transition to CityGML 3.0.
// #[citygml_feature(name = "tun:IntTunnelInstallation")]
// pub struct IntTunnelInstallation {
//     #[citygml(path = b"tun:class")]
//     pub class: Option<Code>,
//
//     #[citygml(path = b"tun:function")]
//     pub function: Vec<Code>,
//
//     #[citygml(path = b"tun:usage")]
//     pub usage: Vec<Code>,
//
//     #[citygml(path = b"tun:boundedBy")]
//     pub bounded_by: Vec<BoundarySurfaceProperty>, // -> tun:_BoundarySurface
// }

#[citygml_property(name = "tun:_BoundarySurfaceProperty")]
pub enum BoundarySurfaceProperty {
    #[citygml(path = b"tun:CeilingSurface")]
    CeilingSurface(CeilingSurface),
    #[citygml(path = b"tun:ClosureSurface")]
    ClosureSurface(ClosureSurface),
    #[citygml(path = b"tun:FloorSurface")]
    FloorSurface(FloorSurface),
    #[citygml(path = b"tun:GroundSurface")]
    GroundSurface(GroundSurface),
    #[citygml(path = b"tun:InteriorWallSurface")]
    InteriorWallSurface(InteriorWallSurface),
    #[citygml(path = b"tun:OuterCeilingSurface")]
    OuterCeilingSurface(OuterCeilingSurface),
    #[citygml(path = b"tun:OuterFloorSurface")]
    OuterFloorSurface(OuterFloorSurface),
    #[citygml(path = b"tun:RoofSurface")]
    RoofSurface(RoofSurface),
    #[citygml(path = b"tun:WallSurface")]
    WallSurface(WallSurface),
}

#[citygml_feature(name = "tun:CeilingSurface")]
pub struct CeilingSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_feature(name = "tun:ClosureSurface")]
pub struct ClosureSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_feature(name = "tun:FloorSurface")]
pub struct FloorSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_feature(name = "tun:GroundSurface")]
pub struct GroundSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_feature(name = "tun:InteriorWallSurface")]
pub struct InteriorWallSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_feature(name = "tun:OuterCeilingSurface")]
pub struct OuterCeilingSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_feature(name = "tun:OuterFloorSurface")]
pub struct OuterFloorSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_feature(name = "tun:RoofSurface")]
pub struct RoofSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_feature(name = "tun:WallSurface")]
pub struct WallSurface {
    #[citygml(path = b"tun:opening")]
    pub opening: Vec<OpeningProperty>, // -> tun:_Opening
}

#[citygml_property(name = "tun:_OpeningProperty")]
pub enum OpeningProperty {
    #[citygml(path = b"tun:Door")]
    Door(Door),
    #[citygml(path = b"tun:Window")]
    Window(Window),
}

#[citygml_feature(name = "tun:Door")]
pub struct Door {}

#[citygml_feature(name = "tun:Window")]
pub struct Window {}

#[citygml_feature(name = "tun:TunnelFurniture")]
pub struct TunnelFurniture {
    #[citygml(path = b"tun:class")]
    pub class: Option<Code>,

    #[citygml(path = b"tun:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"tun:usage")]
    pub usage: Vec<Code>,
}
