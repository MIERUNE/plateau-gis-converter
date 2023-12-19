use super::iur::uro;
use citygml::{CityGMLElement, Code, Date, GeometryRef};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, CityGMLElement)]
pub struct CityFurniture {
    #[citygml(geom = b"frn")]
    pub geometries: GeometryRef,

    #[citygml(path = b"@gml:id")]
    id: Option<String>,

    #[citygml(path = b"gml:identifier")]
    identifier: Option<String>,

    #[citygml(path = b"gml:name")]
    name: Option<String>,

    #[citygml(path = b"gml:description")]
    description: Option<String>,

    #[citygml(path = b"core:creationDate")]
    creation_date: Option<Date>,

    #[citygml(path = b"core:terminationDate")]
    termination_date: Option<Date>,

    #[citygml(path = b"frn:class")]
    pub class: Option<Code>,

    #[citygml(path = b"frn:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"frn:usage")]
    pub usage: Vec<Code>,

    #[citygml(path = b"uro:cityFurnitureDetailAttribute")]
    pub city_furniture_detail_attribute: Vec<uro::CityFurnitureDetailAttribute>,

    #[citygml(
        path = b"uro:cityFurnitureDataQualityAttribute/uro:CityFurnitureDataQualityAttribute"
    )]
    pub city_furniture_data_quality_attribute: Option<uro::CityFurnitureDataQualityAttribute>,

    #[citygml(path = b"uro:frnFacilityTypeAttribute")]
    pub frn_facility_type_attribute: Vec<uro::FacilityTypeAttribute>,

    #[citygml(path = b"uro:frnFacilityIdAttribute")]
    pub frn_facility_id_attribute: Option<uro::FacilityIdAttribute>,

    #[citygml(path = b"uro:frnFacilityAttribute")]
    pub frn_facility_attribute: Vec<uro::FacilityAttributeProperty>,

    #[citygml(path = b"uro:frnDmAttribute")]
    pub frn_dm_attribute: Vec<uro::DmAttributeProperty>,
}
