use nusamai_citygml::{citygml_data, CityGmlElement, Code};

#[citygml_data(name = "uro:CityFurnitureDetailAttribute")]
pub struct CityFurnitureDetailAttribute {
    #[citygml(path = b"uro:facilityType")]
    pub facility_type: Option<Code>,

    #[citygml(path = b"uro:description")]
    pub description: Option<String>,
}
