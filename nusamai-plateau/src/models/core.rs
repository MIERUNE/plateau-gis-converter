use nusamai_citygml::{citygml_data, CityGmlElement};

#[citygml_data(name = "core:Address")]
#[citygml(allow_extra)]
pub struct Address {
    #[citygml(path = b"core:xalAddress/xAL:AddressDetails")]
    country: Vec<AddressDetail>,
}

#[citygml_data(name = "xAL:AddressDetails")]
#[citygml(allow_extra)]
pub struct AddressDetail {
    #[citygml(path = b"xAL:Country")]
    country: Option<Country>,
}

#[citygml_data(name = "xAL:Country")]
#[citygml(allow_extra)]
pub struct Country {
    #[citygml(path = b"xAL:CountryName")]
    name: Option<String>,
    #[citygml(path = b"xAL:Locality/xAL:LocalityName")]
    locality_name: Option<String>,
}
