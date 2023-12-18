use super::iur::city_furniture::{CityFurnitureDataQualityAttribute, CityFurnitureDetailAttribute};
use super::iur::urban_object::{
    DmAttribute, FacilityAttribute, FacilityIdAttribute, FacilityTypeAttribute,
};
use citygml::{CityGMLElement, Code, GeometryRef};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct CityFurniture {
    #[citygml(geom = b"frn")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<CityFurnitureDetailAttribute>,

    #[citygml(path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute")]
    pub city_furniture_data_quality_attribute: Option<CityFurnitureDataQualityAttribute>,

    #[citygml(path = b"uro:frnFacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<FacilityIdAttribute>,

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<FacilityAttribute>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<DmAttribute>,
}
