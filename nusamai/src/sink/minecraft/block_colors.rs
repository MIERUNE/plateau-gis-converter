use std::collections::HashMap;

#[derive(Clone)]
pub struct Voxel {
    pub block_name: &'static str,
    pub priority: u8,
}

impl Voxel {
    fn new(block_name: &'static str, priority: u8) -> Voxel {
        Voxel {
            block_name,
            priority,
        }
    }
}

pub struct DefaultBlockResolver {
    typename_blocks: HashMap<&'static str, Voxel>,
}

impl DefaultBlockResolver {
    pub fn new() -> DefaultBlockResolver {
        let mut typename_blocks: HashMap<&'static str, Voxel> = HashMap::new();

        typename_blocks.insert(
            "bldg:BuildingInstallation",
            Voxel::new("light_gray_concrete", 40),
        );
        typename_blocks.insert("bldg:OuterFloorSurface", Voxel::new("white_wool", 40));
        typename_blocks.insert("bldg:OuterCeilingSurface", Voxel::new("white_wool", 40));
        typename_blocks.insert("bldg:CeilingSurface", Voxel::new("white_wool", 20));
        typename_blocks.insert("bldg:FloorSurface", Voxel::new("white_wool", 20));
        typename_blocks.insert("bldg:WallSurface", Voxel::new("iron_block", 20));

        typename_blocks.insert("tran:Road", Voxel::new("gray_wool", 10));
        typename_blocks.insert("tran:Railway", Voxel::new("granite", 10));
        typename_blocks.insert("tran:Track", Voxel::new("stone_bricks", 10));
        typename_blocks.insert("tran:Square", Voxel::new("smooth_stone", 10));

        typename_blocks.insert("veg:PlantCover", Voxel::new("moss_block", 10));
        typename_blocks.insert("veg:SolitaryVegetationObject", Voxel::new("oak_leaves", 10));

        typename_blocks.insert("uro:Waterway", Voxel::new("cyan_stained_glass", 10));
        typename_blocks.insert("luse:LandUse", Voxel::new("coarse_dirt", 10));
        typename_blocks.insert("dem:ReliefFeature", Voxel::new("stone", 10));
        typename_blocks.insert("urf:UseDistrict", Voxel::new("green_stained_glass", 1));
        typename_blocks.insert(
            "urf:FirePreventionDistrict",
            Voxel::new("red_stained_glass", 1),
        );
        typename_blocks.insert(
            "urf:SedimentDisasterProneArea",
            Voxel::new("yellow_stained_glass", 1),
        );
        typename_blocks.insert("urf:Zone", Voxel::new("magenta_stained_glass", 1));

        DefaultBlockResolver { typename_blocks }
    }

    pub fn resolve(&self, typename: &str) -> Option<Voxel> {
        let (prefix, local_name) = typename.split_once(':')?;

        match local_name {
            "ClosureSurface" => return None,
            "InteriorWallSurface" => return None,
            "Window" | "Door" => return Some(Voxel::new("cyan_stained_glass", 100)),
            _ => {}
        };

        if let Some(voxel) = self.typename_blocks.get(typename) {
            return Some(voxel.clone());
        }

        match prefix {
            "bldg" => return Some(Voxel::new("iron_block", 30)),
            "brid" => return Some(Voxel::new("polished_andesite", 30)),
            "tun" => return Some(Voxel::new("cobblestone", 20)),
            "frn" => return Some(Voxel::new("quartz_block", 20)),
            "tran" => return Some(Voxel::new("gray_wool", 10)),
            "wtr" => return Some(Voxel::new("water", 10)),
            _ => {}
        }

        Some(Voxel::new("white_wool", 0))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_get_typename_block() {
//         let typename_blocks = get_block_for_typename();
//         assert_eq!(typename_blocks.get("bldg:Building"), Some(&"iron_block"));
//         assert_eq!(typename_blocks.get("tran:Road"), Some(&"gray_wool"));
//         assert_eq!(typename_blocks.get("tran:Railway"), Some(&"granite"));
//         assert_eq!(typename_blocks.get("tran:Track"), Some(&"stone_bricks"));
//         assert_eq!(typename_blocks.get("tran:Square"), Some(&"smooth_stone"));
//         assert_eq!(
//             typename_blocks.get("uro:Waterway"),
//             Some(&"cyan_stained_glass")
//         );
//         assert_eq!(typename_blocks.get("luse:LandUse"), Some(&"coarse_dirt"));
//         assert_eq!(
//             typename_blocks.get("frn:CityFurniture"),
//             Some(&"quartz_block")
//         );
//         assert_eq!(typename_blocks.get("veg:PlantCover"), Some(&"moss_block"));
//         assert_eq!(
//             typename_blocks.get("veg:SolitaryVegetationObject"),
//             Some(&"oak_leaves")
//         );
//         assert_eq!(typename_blocks.get("wtr:WaterBody"), Some(&"water"));
//         assert_eq!(typename_blocks.get("dem:ReliefFeature"), Some(&"stone"));
//         assert_eq!(
//             typename_blocks.get("brid:Bridge"),
//             Some(&"polished_andesite")
//         );
//         assert_eq!(typename_blocks.get("tun:Tunnel"), Some(&"cobblestone"));
//         assert_eq!(
//             typename_blocks.get("urf:UseDistrict"),
//             Some(&"green_stained_glass")
//         );
//         assert_eq!(
//             typename_blocks.get("urf:FirePreventionDistrict"),
//             Some(&"red_stained_glass")
//         );
//         assert_eq!(
//             typename_blocks.get("urf:SedimentDisasterProneArea"),
//             Some(&"yellow_stained_glass")
//         );
//         assert_eq!(
//             typename_blocks.get("urf:Zone"),
//             Some(&"magenta_stained_glass")
//         );
//     }
// }
