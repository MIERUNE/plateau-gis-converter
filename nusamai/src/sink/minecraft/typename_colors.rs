use std::collections::HashMap;

pub fn get_typename_colors() -> HashMap<&'static str, [u8; 3]> {
    let mut typename_colors: HashMap<&str, [u8; 3]> = HashMap::new();

    typename_colors.insert("bldg:Building", [220, 220, 220]);
    typename_colors.insert("tran:Road", [62, 68, 71]);
    typename_colors.insert("tran:Railway", [149, 103, 85]);
    typename_colors.insert("tran:Track", [122, 121, 122]);
    typename_colors.insert("tran:Square", [158, 158, 158]);
    typename_colors.insert("uro:Waterway", [75, 127, 153]);
    typename_colors.insert("luse:LandUse", [119, 85, 59]);
    typename_colors.insert("frn:CityFurniture", [235, 229, 222]);
    typename_colors.insert("veg:PlantCover", [89, 109, 45]);
    typename_colors.insert("veg:SolitaryVegetationObject", [0, 255, 0]);
    typename_colors.insert("wtr:WaterBody", [0, 0, 255]);
    typename_colors.insert("dem:ReliefFeature", [125, 125, 125]);
    typename_colors.insert("brid:Bridge", [132, 134, 133]);
    typename_colors.insert("tun:Tunnel", [127, 127, 127]);
    typename_colors.insert("urf:UseDistrict", [102, 127, 50]);
    typename_colors.insert("urf:FirePreventionDistrict", [53, 50, 50]);
    typename_colors.insert("urf:SedimentDisasterProneArea", [229, 229, 50]);
    typename_colors.insert("urf:Zone", [178, 75, 215]);
    typename_colors.insert("uro:UnderGroundBuilding", [255, 255, 255]);
    typename_colors.insert("uro:OtherConstruction", [255, 255, 255]);
    // typename_colors.insert("gen", [0, 0, 15]);
    // typename_colors.insert("uro", [0, 0, 16]);

    typename_colors
}
