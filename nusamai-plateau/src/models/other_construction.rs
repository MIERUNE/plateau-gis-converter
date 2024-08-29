//! uro:OtherConstruction (PLATEAU, CityGML 2.x)
//! con:OtherConstruction (CityGML 3.x)

use nusamai_citygml::{citygml_feature, citygml_property, CityGmlElement, Code, Date};
use once_cell::sync::Lazy;

use crate::BoundedBy;

use super::iur::uro;

type ConditionOfConstructionValue = String;

#[citygml_feature(name = "uro:OtherConstruction")]
pub struct OtherConstruction {
    #[citygml(path = b"uro:conditionOfConstruction")]
    pub condition_of_construction: Option<ConditionOfConstructionValue>,

    #[citygml(path = b"uro:dateOfConstruction")]
    pub date_of_construction: Option<Date>,

    #[citygml(path = b"uro:dateOfDemolition")]
    pub date_of_demolition: Option<Date>,

    #[citygml(path = b"uro:constructionEvent/uro:ConstructionEvent")]
    pub construction_event: Vec<uro::ConstructionEvent>,

    #[citygml(path = b"uro:elevation/uro:Elevation")]
    pub elevation: Vec<uro::Elevation>,

    #[citygml(path = b"uro:height/uro:Height")]
    pub height: Vec<uro::Height>,

    #[citygml(path = b"uro:occupancy/uro:Occupancy")]
    pub occupancy: Vec<uro::Occupancy>,

    #[citygml(path = b"uro:consFacilityTypeAttribute/uro:FacilityTypeAttribute")]
    pub cons_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:consFacilityIdAttribute")]
    pub cons_facility_id_attribute: Option<uro::FacilityIdAttributeProperty>, // -> uro:FacilityIdAttribute

    #[citygml(path = b"uro:consFacilityAttribute")]
    pub cons_facility_attribute: Vec<uro::FacilityAttributeProperty>, // -> uro:FacilityAttribute

    #[citygml(path = b"uro:consBaseAttribute/uro:ConstructionBaseAttribute")]
    pub cons_base_attribute: Option<uro::ConstructionBaseAttribute>,

    #[citygml(path = b"uro:consStructureAttribute")]
    pub cons_structure_attribute: Option<uro::ConstructionStructureAttributeProperty>, // -> uro:ConstructionStructureAttribute

    #[citygml(path = b"uro:consDisasterRiskAttribute")]
    pub cons_disaster_risk_attribute: Vec<uro::DisasterRiskAttributeProperty>, // -> uro:DisasterRiskAttribute

    #[citygml(path = b"uro:consDmAttribute")]
    pub cons_dm_attribute: Vec<uro::DmAttributeProperty>, // -> uro:DmAttribute

    #[citygml(path = b"uro:consDataQualityAttribute/uro:ConstructionDataQualityAttribute")]
    #[citygml(path = b"uro:consDataQualityAttribute/uro:DataQualityAttribute")]
    pub cons_data_quality_attribute: Option<uro::DataQualityAttribute>,

    #[citygml(path = b"uro:boundedBy")]
    pub bounded_by: Vec<BoundarySurfaceProperty>, // -> uro:_BoundarySurface

    #[citygml(path = b"uro:constructionInstallation/uro:ConstructionInstallation")]
    pub construction_installation: Vec<ConstructionInstallation>,

    #[citygml(path = b"uro:class")]
    pub class: Option<Code>,

    #[citygml(path = b"uro:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"uro:usage")]
    pub usage: Vec<Code>,
}

#[citygml_property(name = "uro:_BoundarySurfaceProperty")]
pub enum BoundarySurfaceProperty {
    #[citygml(path = b"uro:ClosureSurface")]
    ClosureSurface(ClosureSurface),
    #[citygml(path = b"uro:GroundSurface")]
    GroundSurface(GroundSurface),
    #[citygml(path = b"uro:OuterCeilingSurface")]
    OuterCeilingSurface(OuterCeilingSurface),
    #[citygml(path = b"uro:OuterFloorSurface")]
    OuterFloorSurface(OuterFloorSurface),
    #[citygml(path = b"uro:RoofSurface")]
    RoofSurface(RoofSurface),
    #[citygml(path = b"uro:WallSurface")]
    WallSurface(WallSurface),
}

#[citygml_feature(name = "uro:ClosureSurface")]
pub struct ClosureSurface {}

#[citygml_feature(name = "uro:GroundSurface")]
pub struct GroundSurface {}

#[citygml_feature(name = "uro:OuterCeilingSurface")]
pub struct OuterCeilingSurface {}

#[citygml_feature(name = "uro:OuterFloorSurface")]
pub struct OuterFloorSurface {}

#[citygml_feature(name = "uro:RoofSurface")]
pub struct RoofSurface {}

#[citygml_feature(name = "uro:WallSurface")]
pub struct WallSurface {}

#[citygml_feature(name = "uro:ConstructionInstallation")]
pub struct ConstructionInstallation {
    #[citygml(path = b"uro:class")]
    pub class: Option<Code>,

    #[citygml(path = b"uro:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"uro:usage")]
    pub usage: Vec<Code>,
}

#[allow(clippy::type_complexity)]
pub static OTHER_CONSTRUCTION_SURFACE_MAPPINGS: Lazy<
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
