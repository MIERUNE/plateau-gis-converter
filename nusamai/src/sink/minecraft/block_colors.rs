use std::collections::HashMap;

pub fn get_typename_block() -> HashMap<&'static str, &'static str> {
    let mut typename_blocks: HashMap<&'static str, &'static str> = HashMap::new();

    typename_blocks.insert("bldg:Building", "iron_block");
    typename_blocks.insert("tran:Road", "gray_wool");
    typename_blocks.insert("tran:Railway", "granite");
    typename_blocks.insert("tran:Track", "stone_bricks");
    typename_blocks.insert("tran:Square", "smooth_stone");
    typename_blocks.insert("uro:Waterway", "cyan_stained_glass");
    typename_blocks.insert("luse:LandUse", "coarse_dirt");
    typename_blocks.insert("frn:CityFurniture", "quartz_block");
    typename_blocks.insert("veg:PlantCover", "moss_block");
    typename_blocks.insert("veg:SolitaryVegetationObject", "oak_leaves");
    typename_blocks.insert("wtr:WaterBody", "water");
    typename_blocks.insert("dem:ReliefFeature", "stone");
    typename_blocks.insert("brid:Bridge", "polished_andesite");
    typename_blocks.insert("tun:Tunnel", "cobblestone");
    typename_blocks.insert("urf:UseDistrict", "green_stained_glass");
    typename_blocks.insert("urf:FirePreventionDistrict", "red_stained_glass");
    typename_blocks.insert("urf:SedimentDisasterProneArea", "yellow_stained_glass");
    typename_blocks.insert("urf:Zone", "magenta_stained_glass");

    typename_blocks
}
