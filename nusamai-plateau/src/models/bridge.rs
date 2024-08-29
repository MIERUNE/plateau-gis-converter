use nusamai_citygml::{citygml_feature, citygml_property, CityGmlElement, Code, GYear};
use once_cell::sync::Lazy;

use crate::BoundedBy;

use super::{core::Address, iur::uro};

#[citygml_feature(name = "brid:Bridge")]
pub struct Bridge {
    #[citygml(path = b"brid:class")]
    pub class: Option<Code>,

    #[citygml(path = b"brid:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"brid:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"brid:yearOfConstruction")]
    pub year_of_construction: Option<GYear>,

    #[citygml(path = b"brid:yearOfDemolition")]
    pub year_of_demolition: Option<GYear>,

    #[citygml(path = b"brid:isMovable")]
    pub is_movable: Option<bool>,

    #[citygml(path = b"brid:outerBridgeConstruction/brid:BridgeConstructionElement")]
    pub outer_bridge_construction: Vec<BridgeConstructionElement>,

    #[citygml(path = b"brid:outerBridgeInstallation/brid:BridgeInstallation")]
    pub outer_bridge_installation: Vec<BridgeInstallation>,

    #[citygml(path = b"brid:interiorBridgeInstallation/brid:IntBridgeInstallation")]
    pub interior_bridge_installation: Vec<BridgeInstallation>,

    #[citygml(path = b"brid:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> brid:_BoundarySurface

    #[citygml(path = b"brid:interiorBridgeRoom/brid:BridgeRoom")]
    pub interior_bridge_room: Vec<BridgeRoom>,

    #[citygml(path = b"brid:consistsOfBridgePart/brid:BridgePart")]
    pub consists_of_bridge_part: Vec<BridgePart>,

    #[citygml(path = b"brid:address/core:Address")]
    pub address: Vec<Address>,

    #[citygml(path = b"uro:bridBaseAttribute/uro:ConstructionBaseAttribute")]
    pub brid_base_attribute: Vec<uro::ConstructionBaseAttribute>,

    #[citygml(path = b"uro:bridDataQualityAttribute/uro:ConstructionDataQualityAttribute")]
    #[citygml(path = b"uro:bridDataQualityAttribute/uro:DataQualityAttribute")]
    pub brid_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:bridDisasterRiskAttribute")]
    pub brid_disaster_risk_attribute: Vec<uro::DisasterRiskAttributeProperty>, // -> uro:DisasterRiskAttribute

    #[citygml(path = b"uro:bridDmAttribute")]
    pub brid_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:bridFacilityAttribute")]
    pub brid_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:bridFacilityIdAttribute")]
    pub brid_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:bridFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub brid_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:bridFunctionalAttribute/uro:BridgeFunctionalAttribute")]
    pub brid_functional_attribute: Vec<uro::BridgeFunctionalAttribute>,

    #[citygml(path = b"uro:bridKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub brid_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:bridRiskAssessmentAttribute/uro:ConstructionRiskAssessmentAttribute")]
    pub brid_risk_assessment_attribute: Vec<uro::ConstructionRiskAssessmentAttribute>,

    #[citygml(path = b"uro:bridStructureAttribute/uro:BridgeStructureAttribute")]
    pub brid_structure_attribute: Vec<uro::BridgeStructureAttribute>,
}

#[citygml_feature(name = "brid:BridgePart")]
pub struct BridgePart {
    #[citygml(path = b"brid:class")]
    pub class: Option<Code>,

    #[citygml(path = b"brid:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"brid:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"brid:yearOfConstruction")]
    pub year_of_construction: Option<GYear>,

    #[citygml(path = b"brid:yearOfDemolition")]
    pub year_of_demolition: Option<GYear>,

    #[citygml(path = b"brid:isMovable")]
    pub is_movable: Option<bool>,

    #[citygml(path = b"brid:outerBridgeConstruction/brid:BridgeConstructionElement")]
    pub outer_bridge_construction: Vec<BridgeConstructionElement>,

    #[citygml(path = b"brid:outerBridgeInstallation/brid:BridgeInstallation")]
    pub outer_bridge_installation: Vec<BridgeInstallation>,

    #[citygml(path = b"brid:interiorBridgeInstallation/brid:IntBridgeInstallation")]
    pub interior_bridge_installation: Vec<BridgeInstallation>,

    #[citygml(path = b"brid:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> brid:_BoundarySurface

    #[citygml(path = b"brid:interiorBridgeRoom/brid:BridgeRoom")]
    pub interior_bridge_room: Vec<BridgeRoom>,

    #[citygml(path = b"brid:consistsOfBridgePart/brid:BridgePart")]
    pub consists_of_bridge_part: Vec<BridgePart>,

    #[citygml(path = b"brid:address/core:Address")]
    pub address: Vec<Address>,

    #[citygml(path = b"uro:bridBaseAttribute/uro:ConstructionBaseAttribute")]
    pub brid_base_attribute: Vec<uro::ConstructionBaseAttribute>,

    #[citygml(path = b"uro:bridDataQualityAttribute/uro:ConstructionDataQualityAttribute")]
    #[citygml(path = b"uro:bridDataQualityAttribute/uro:DataQualityAttribute")]
    pub brid_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:bridDisasterRiskAttribute")]
    pub brid_disaster_risk_attribute: Vec<uro::DisasterRiskAttributeProperty>, // -> uro:DisasterRiskAttribute

    #[citygml(path = b"uro:bridDmAttribute")]
    pub brid_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:bridFacilityAttribute")]
    pub brid_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:bridFacilityIdAttribute")]
    pub brid_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:bridFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub brid_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:bridFunctionalAttribute/uro:BridgeFunctionalAttribute")]
    pub brid_functional_attribute: Vec<uro::BridgeFunctionalAttribute>,

    #[citygml(path = b"uro:bridKeyValuePairAttribute/uro:KeyValuePairAttribute")]
    pub brid_key_value_pair_attribute: Vec<uro::KeyValuePairAttribute>,

    #[citygml(path = b"uro:bridRiskAssessmentAttribute/uro:ConstructionRiskAssessmentAttribute")]
    pub brid_risk_assessment_attribute: Vec<uro::ConstructionRiskAssessmentAttribute>,

    #[citygml(path = b"uro:bridStructureAttribute/uro:BridgeStructureAttribute")]
    pub brid_structure_attribute: Vec<uro::BridgeStructureAttribute>,
}

#[citygml_feature(name = "brid:BridgeConstructionElement")]
pub struct BridgeConstructionElement {
    #[citygml(path = b"brid:class")]
    pub class: Option<Code>,

    #[citygml(path = b"brid:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"brid:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"brid:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> brid:_BoundarySurface
}

#[citygml_feature(name = "brid:BridgeRoom")]
pub struct BridgeRoom {
    #[citygml(path = b"brid:class")]
    pub class: Option<Code>,

    #[citygml(path = b"brid:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"brid:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"brid:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> brid:_BoundarySurface

    #[citygml(path = b"brid:interiorFurniture/brid:BridgeFurniture")]
    pub interior_furniture: Vec<BridgeFurniture>,

    #[citygml(path = b"brid:bridgeRoomInstallation/brid:IntBridgeInstallation")]
    pub bridge_room_installation: Vec<BridgeInstallation>,
}
#[citygml_feature(name = "brid:RoofSurface")]
pub struct RoofSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[citygml_feature(name = "brid:WallSurface")]
pub struct WallSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[citygml_feature(name = "brid:GroundSurface")]
pub struct GroundSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[citygml_feature(name = "brid:OuterCeilingSurface")]
pub struct OuterCeilingSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[citygml_feature(name = "brid:OuterFloorSurface")]
pub struct OuterFloorSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[citygml_feature(name = "brid:ClosureSurface")]
pub struct ClosureSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[citygml_feature(name = "brid:InteriorWallSurface")]
pub struct InteriorWallSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[citygml_feature(name = "brid:BridgeInstallation")]
pub struct BridgeInstallation {
    #[citygml(path = b"brid:class")]
    pub class: Option<Code>,

    #[citygml(path = b"brid:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"brid:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"brid:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> brid:_BoundarySurface
}

// Intentionally not used to facilitate transition to CityGML 3.0.
// #[citygml_feature(name = "brid:IntBridgeInstallation")]
// pub struct IntBridgeInstallation {
//     #[citygml(path = b"brid:class")]
//     pub class: Option<Code>,
//
//     #[citygml(path = b"brid:function")]
//     pub function: Vec<Code>,
//
//     #[citygml(path = b"brid:usage")]
//     pub usage: Vec<Code>,
//
//     #[citygml(path = b"brid:boundedBy")]
//     pub bounded_by: Vec<BoundarySurfaceProperty>, // -> brid:_BoundarySurface
// }

#[citygml_feature(name = "brid:BridgeFurniture")]
pub struct BridgeFurniture {
    #[citygml(path = b"brid:class")]
    pub class: Option<Code>,

    #[citygml(path = b"brid:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"brid:usage")]
    pub usage: Vec<Code>,
}

#[citygml_property(name = "brid:_OpeningProperty")]
pub enum OpeningProperty {
    #[citygml(path = b"brid:Door")]
    Door(Door),
    #[citygml(path = b"brid:Window")]
    Window(Window),
}

#[citygml_feature(name = "brid:Door")]
pub struct Door {
    #[citygml(path = b"brid:address/core:Address")]
    pub address: Vec<Address>,
}

#[citygml_feature(name = "brid:Window")]
pub struct Window {}

#[citygml_property(name = "brid:_BoundarySurfaceProperty")]
pub enum BoundarySurfaceProperty {
    #[citygml(path = b"brid:CeilingSurface")]
    CeilingSurface(CeilingSurface),
    #[citygml(path = b"brid:ClosureSurface")]
    ClosureSurface(ClosureSurface),
    #[citygml(path = b"brid:FloorSurface")]
    FloorSurface(FloorSurface),
    #[citygml(path = b"brid:GroundSurface")]
    GroundSurface(GroundSurface),
    #[citygml(path = b"brid:InteriorWallSurface")]
    InteriorWallSurface(InteriorWallSurface),
    #[citygml(path = b"brid:OuterCeilingSurface")]
    OuterCeilingSurface(OuterCeilingSurface),
    #[citygml(path = b"brid:OuterFloorSurface")]
    OuterFloorSurface(OuterFloorSurface),
    #[citygml(path = b"brid:RoofSurface")]
    RoofSurface(RoofSurface),
    #[citygml(path = b"brid:WallSurface")]
    WallSurface(WallSurface),
}

#[citygml_feature(name = "brid:CeilingSurface")]
pub struct CeilingSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[citygml_feature(name = "brid:FloorSurface")]
pub struct FloorSurface {
    #[citygml(path = b"brid:opening")]
    pub opening: Vec<OpeningProperty>, // -> brid:_Opening
}

#[allow(clippy::type_complexity)]
pub static BRIDGE_SURFACE_MAPPINGS: Lazy<
    Box<dyn Fn(&BoundarySurfaceProperty) -> Option<BoundedBy> + Send + Sync>,
> = Lazy::new(|| {
    let result = |bounded: &BoundarySurfaceProperty| match bounded {
        BoundarySurfaceProperty::RoofSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::WallSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::GroundSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::ClosureSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::FloorSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::CeilingSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::InteriorWallSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::OuterCeilingSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::OuterFloorSurface(v) => Some(BoundedBy {
            id: v.id.clone(),
            geometry_refs: v.geometries.clone(),
        }),
        BoundarySurfaceProperty::Unknown => None,
    };
    Box::new(result) as Box<dyn Fn(&BoundarySurfaceProperty) -> Option<BoundedBy> + Send + Sync>
});
