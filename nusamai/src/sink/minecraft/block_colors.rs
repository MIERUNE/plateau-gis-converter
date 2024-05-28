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

    typename_colors
}

pub fn get_block_colors() -> HashMap<&'static str, [u8; 3]> {
    let mut block_colors: HashMap<&str, [u8; 3]> = HashMap::new();

    block_colors.insert("minecraft:iron_block", [220, 220, 220]); // bldg:Building
    block_colors.insert("minecraft:gray_wool", [62, 68, 71]); // tran:Road
    block_colors.insert("minecraft:granite", [149, 103, 85]); // tran:Railway
    block_colors.insert("minecraft:stone_bricks", [122, 121, 122]); // tran:Track
    block_colors.insert("minecraft:smooth_stone", [158, 158, 158]); // tran:Square
    block_colors.insert("minecraft:cyan_stained_glass", [75, 127, 153]); // uro:Waterway
    block_colors.insert("minecraft:coarse_dirt", [119, 85, 59]); // luse:LandUse
    block_colors.insert("minecraft:quartz_block", [235, 229, 222]); // frn:CityFurniture
    block_colors.insert("minecraft:moss_block", [89, 109, 45]); // veg:PlantCover
    block_colors.insert("minecraft:oak_leaves", [0, 255, 0]); // veg:SolitaryVegetationObject
    block_colors.insert("minecraft:water", [0, 0, 255]); // wtr:WaterBody
    block_colors.insert("minecraft:stone", [125, 125, 125]); // dem:ReliefFeature
    block_colors.insert("minecraft:polished_andesite", [132, 134, 133]); // brid:Bridge
    block_colors.insert("minecraft:cobblestone", [127, 127, 127]); // tun:Tunnel
    block_colors.insert("minecraft:green_stained_glass", [102, 127, 50]); // urf:UseDistrict
    block_colors.insert("minecraft:red_stained_glass", [53, 50, 50]); // urf:FirePreventionDistrict
    block_colors.insert("minecraft:yellow_stained_glass", [229, 229, 50]); // urf:SedimentDisasterProneArea
    block_colors.insert("minecraft:magenta_stained_glass", [178, 75, 215]); // urf:Zone
    block_colors.insert("minecraft:white_wool", [255, 255, 255]); // Undefined geographical type

    block_colors
}
