use citygml::{citygml_data, CityGMLElement, Code};

#[citygml_data(name = "uro:CityFurnitureDetailAttribute")]
pub struct CityFurnitureDetailAttribute {
    #[citygml(path = b"uro:facilityType")]
    pub facility_type: Option<Code>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
}

#[citygml_data(name = "uro:CityFurnitureDataQualityAttribute")]
pub struct CityFurnitureDataQualityAttribute {
    #[citygml(path = b"uro:srcScale")]
    pub src_scale: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc")]
    pub geometry_src_desc: Vec<Code>,

    #[citygml(path = b"uro:thematicSrcDesc")]
    pub thematic_src_desc: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDesc")]
    pub appearance_src_desc: Vec<Code>,

    #[citygml(path = b"uro:lodType")]
    pub lod_type: Option<String>, // TODO: uro:CityFurnitureLODType(enumerations)
}
