use std::collections::HashMap;

pub fn get_block_for_typename() -> HashMap<&'static str, &'static str> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_typename_block() {
        let typename_blocks = get_block_for_typename();
        assert_eq!(typename_blocks.get("bldg:Building"), Some(&"iron_block"));
        assert_eq!(typename_blocks.get("tran:Road"), Some(&"gray_wool"));
        assert_eq!(typename_blocks.get("tran:Railway"), Some(&"granite"));
        assert_eq!(typename_blocks.get("tran:Track"), Some(&"stone_bricks"));
        assert_eq!(typename_blocks.get("tran:Square"), Some(&"smooth_stone"));
        assert_eq!(
            typename_blocks.get("uro:Waterway"),
            Some(&"cyan_stained_glass")
        );
        assert_eq!(typename_blocks.get("luse:LandUse"), Some(&"coarse_dirt"));
        assert_eq!(
            typename_blocks.get("frn:CityFurniture"),
            Some(&"quartz_block")
        );
        assert_eq!(typename_blocks.get("veg:PlantCover"), Some(&"moss_block"));
        assert_eq!(
            typename_blocks.get("veg:SolitaryVegetationObject"),
            Some(&"oak_leaves")
        );
        assert_eq!(typename_blocks.get("wtr:WaterBody"), Some(&"water"));
        assert_eq!(typename_blocks.get("dem:ReliefFeature"), Some(&"stone"));
        assert_eq!(
            typename_blocks.get("brid:Bridge"),
            Some(&"polished_andesite")
        );
        assert_eq!(typename_blocks.get("tun:Tunnel"), Some(&"cobblestone"));
        assert_eq!(
            typename_blocks.get("urf:UseDistrict"),
            Some(&"green_stained_glass")
        );
        assert_eq!(
            typename_blocks.get("urf:FirePreventionDistrict"),
            Some(&"red_stained_glass")
        );
        assert_eq!(
            typename_blocks.get("urf:SedimentDisasterProneArea"),
            Some(&"yellow_stained_glass")
        );
        assert_eq!(
            typename_blocks.get("urf:Zone"),
            Some(&"magenta_stained_glass")
        );
    }
}
