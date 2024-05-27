use std::collections::HashMap;

pub fn get_typename_colors() -> HashMap<&'static str, [u8; 3]> {
    let mut typename_colors: HashMap<&str, [u8; 3]> = HashMap::new();

    typename_colors.insert("bldg:Building", [220, 220, 220]); // minecraft:iron_block
    typename_colors.insert("tran:Road", [62, 68, 71]); // minecraft:gray_wool
    typename_colors.insert("tran:Railway", [149, 103, 85]); // minecraft:granite
    typename_colors.insert("tran:Track", [122, 121, 122]); // minecraft:stone_bricks
    typename_colors.insert("tran:Square", [158, 158, 158]); // minecraft:smooth_stone
    typename_colors.insert("uro:Waterway", [75, 127, 153]); // minecraft:cyan_stained_glass
    typename_colors.insert("luse:LandUse", [119, 85, 59]); // minecraft:coarse_dirt
    typename_colors.insert("frn:CityFurniture", [235, 229, 222]); // minecraft:quartz_block
    typename_colors.insert("veg:PlantCover", [89, 109, 45]); // minecraft:moss_block
    typename_colors.insert("veg:SolitaryVegetationObject", [0, 255, 0]); // minecraft:oak_leaves
    typename_colors.insert("wtr:WaterBody", [0, 0, 255]); // minecraft:water
    typename_colors.insert("dem:ReliefFeature", [125, 125, 125]); // minecraft:stone
    typename_colors.insert("brid:Bridge", [132, 134, 133]); // minecraft:polished_andesite
    typename_colors.insert("tun:Tunnel", [127, 127, 127]); // minecraft:cobblestone
    typename_colors.insert("urf:UseDistrict", [102, 127, 50]); // minecraft:green_stained_glass
    typename_colors.insert("urf:FirePreventionDistrict", [53, 50, 50]); // minecraft:red_stained_glass
    typename_colors.insert("urf:SedimentDisasterProneArea", [229, 229, 50]); // minecraft:yellow_stained_glass
    typename_colors.insert("urf:Zone", [178, 75, 215]); // minecraft:magenta_stained_glass
    typename_colors.insert("uro:UnderGroundBuilding", [255, 255, 255]); // minecraft:minecraft:white_wool
    typename_colors.insert("uro:OtherConstruction", [255, 255, 255]); // minecraft:minecraft:white_wool
                                                                      // typename_colors.insert("gen", [0, 0, 15]);
                                                                      // typename_colors.insert("uro", [0, 0, 16]);

    typename_colors
}
