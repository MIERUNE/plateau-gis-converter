use nusamai_citygml::{citygml_feature, CityGmlElement, Code, GYear};

use super::iur::uro;

#[citygml_feature(name = "grp:CityObjectGroup")]
pub struct CityObjectGroup {
    #[citygml(path = b"grp:class")]
    pub class: Option<Code>,

    #[citygml(path = b"grp:function")]
    pub function: Vec<Code>,

    #[citygml(path = b"grp:usage")]
    pub usage: Vec<Code>,

    //
    // TODO: not implemented yet
    #[citygml(path = b"grp:groupMember")]
    pub group_member: Vec<CityObjectOrRef>,
    //
    // TODO: not implemented yet
    #[citygml(path = b"grp:parent")]
    pub parent: Option<CityObjectOrRef>,
    //
    // TODO:
    // #[citygml(path = b"grp:geometry")]
    // pub geometry: Option<GeometryProperty>, // -> gml:_Geometry
    //
    #[citygml(path = b"uro:fiscalYearOfPublication")]
    pub fiscal_year_of_publication: Vec<GYear>,

    #[citygml(path = b"uro:ifcBuildingStoreyAttribute")]
    pub ifc_building_storey_attribute: Vec<uro::IfcAttributeProperty>, // -> uro:IfcAttribute

    #[citygml(path = b"uro:indoorStoreyAttribute")]
    pub indoor_storey_attribute: Vec<uro::IndoorAttributeProperty>, // -> uro:IndoorAttribute

    #[citygml(path = b"uro:language")]
    pub language: Vec<Code>,
}

#[citygml_feature(name = "grp:_CityObjectOrRef")]
pub struct CityObjectOrRef {
    #[citygml(path = b"@xlink:href")]
    href: Option<String>,
}
