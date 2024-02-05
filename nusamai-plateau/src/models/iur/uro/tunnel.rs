use nusamai_citygml::{citygml_data, CityGmlElement, Code, Length, Measure};

#[citygml_data(name = "uro:TunnelFunctionalAttribute")]
pub struct TunnelFunctionalAttribute {
    #[citygml(path = b"uro:directionType")]
    pub direction_type: Option<Code>,

    #[citygml(path = b"uro:userType")]
    pub user_type: Option<Code>,
}

#[citygml_data(name = "uro:TunnelStructureAttribute")]
pub struct TunnelStructureAttribute {
    #[citygml(path = b"uro:tunnelType")]
    pub tunnel_type: Option<Code>,

    #[citygml(path = b"uro:tunnelSubtype")]
    pub tunnel_subtype: Option<Code>,

    #[citygml(path = b"uro:mouthType")]
    pub mouth_type: Vec<Code>,

    #[citygml(path = b"uro:length")]
    pub length: Option<Length>,

    #[citygml(path = b"uro:width")]
    pub width: Option<Length>,

    #[citygml(path = b"uro:area")]
    pub area: Option<Measure>,

    #[citygml(path = b"uro:innerHeight")]
    pub inner_height: Option<Length>,

    #[citygml(path = b"uro:effectiveHeight")]
    pub effective_height: Option<Length>,

    #[citygml(path = b"uro:slopeType")]
    pub slope_type: Option<Code>,
}
