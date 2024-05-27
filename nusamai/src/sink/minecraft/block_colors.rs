use std::collections::HashMap;

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
