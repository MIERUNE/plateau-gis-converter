use nusamai_citygml::{citygml_data, citygml_property, CityGMLElement, Code, Length, Measure};

#[citygml_data(name = "uro:ConstructionDataQualityAttribute")]
pub struct ConstructionDataQualityAttribute {
    #[citygml(path = b"uro:srcScale")]
    pub src_scale: Vec<Code>,

    #[citygml(path = b"uro:geometrySrcDesc")]
    pub geometry_src_desc: Vec<Code>,

    #[citygml(path = b"uro:thematicSrcDesc")]
    pub thematic_src_desc: Vec<Code>,

    #[citygml(path = b"uro:appearanceSrcDesc")]
    pub appearance_src_desc: Vec<Code>,

    #[citygml(path = b"uro:dataAcquisition")]
    pub data_acquisition: Option<String>,

    #[citygml(path = b"uro:photoScale")]
    pub photo_scale: Option<i64>,

    #[citygml(path = b"uro:lod1HeightType")]
    pub lod1_height_type: Option<Code>,

    #[citygml(path = b"uro:lodType")]
    pub lod_type: Vec<Code>,
}

#[citygml_property(name = "uro:ConstructionStructureAttributeProperty")]
pub enum ConstructionStructureAttributeProperty {
    #[citygml(path = b"uro:ConstructionStructureAttribute")]
    ConstructionStructureAttribute(ConstructionStructureAttribute),
    #[citygml(path = b"uro:DamAttribute")]
    DamAttribute(DamAttribute),
    #[citygml(path = b"uro:EmbankmentAttribute")]
    EmbankmentAttribute(EmbankmentAttribute),
}

#[citygml_data(name = "uro:ConstructionStructureAttribute")]
pub struct ConstructionStructureAttribute {
    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:volume")]
    pub volume: Option<Measure>,
}

#[citygml_data(name = "uro:DamAttribute")]
pub struct DamAttribute {
    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:volume")]
    pub volume: Option<Measure>,

    #[citygml(path = b"uro:damCode")]
    pub dam_code: Option<Code>,

    #[citygml(path = b"uro:totalWaterStorage")]
    pub total_water_storage: Option<Measure>,
}

#[citygml_data(name = "uro:EmbankmentAttribute")]
pub struct EmbankmentAttribute {
    #[citygml(path = b"uro:structureType")]
    pub structure_type: Option<Code>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:depth")]
    pub depth: Option<Length>,

    #[citygml(path = b"uro:volume")]
    pub volume: Option<Measure>,

    #[citygml(path = b"uro:mainPartLength")]
    pub main_part_length: Option<Length>,

    #[citygml(path = b"uro:ceilingHeight")]
    pub ceiling_height: Option<Length>,

    #[citygml(path = b"uro:waveDissipatorLength")]
    pub wave_dissipator_length: Option<Length>,
}
