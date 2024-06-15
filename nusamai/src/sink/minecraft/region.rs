use std::sync::{Arc, Mutex};
use std::{fs::File, path::Path};

use fastanvil::{Error, Region};
use fastnbt::{to_bytes, LongArray};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::pipeline::{PipelineError, Result};

pub type Position2D = [i32; 2];

#[derive(Deserialize, Serialize, Debug)]
struct BlockPosition([u8; 3]);

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockId {
    name_space: String,
    block_name: String,
}

impl BlockId {
    pub fn new(block_name: String) -> Self {
        BlockId {
            name_space: "minecraft".to_string(),
            block_name,
        }
    }

    #[inline]
    pub fn get_block_id(&self) -> String {
        format!("{}:{}", self.name_space, self.block_name)
    }

    #[inline]
    pub fn get_block_name(&self) -> String {
        self.block_name.clone()
    }
}

impl BlockPosition {
    // Check that the input is in the range 0~15
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        if x > 15 || y > 15 || z > 15 {
            panic!(
                "Invalid BlockPosition values: x={x}, y={y}, z={z}. The position values must be \
                 within the range of 0 to 15."
            );
        }
        BlockPosition([x, y, z])
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockData {
    position: BlockPosition,
    block_id: BlockId,
}

impl BlockData {
    pub fn new(x: u8, y: u8, z: u8, block_name: String) -> Self {
        let position = BlockPosition::new(x, y, z);
        let block_id = BlockId::new(block_name);
        BlockData { position, block_id }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SectionData {
    pub y: i32,
    pub blocks: Vec<BlockData>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ChunkData {
    pub position: Position2D,
    pub sections: Vec<SectionData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegionData {
    pub position: Position2D,
    pub chunks: Vec<ChunkData>,
}

pub type WorldData = Vec<RegionData>;

#[derive(Serialize, Deserialize, Debug)]
struct Chunk {
    /// The status of the chunk, such as whether it is fully generated or being generated.
    #[serde(rename = "Status")]
    status: String,
    /// The Z coordinate of the chunk (absolute value).
    #[serde(rename = "zPos")]
    z_pos: i32,
    /// The Y coordinate of the lowest section in the chunk.
    #[serde(rename = "yPos")]
    y_pos: i32,
    /// The X coordinate of the chunk (absolute value).
    #[serde(rename = "xPos")]
    x_pos: i32,
    /// A vector containing the sections that make up the chunk.
    sections: Vec<Section>,
    /// The version of the data format used to store this chunk.
    #[serde(rename = "DataVersion")]
    data_version: u32,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            status: "full".to_string(),
            z_pos: 0,
            y_pos: -4, // Lowest Y section position in the chunk (e.g., -4 in version 1.18 and later)
            x_pos: 0,
            sections: Vec::new(),
            data_version: 3105, // Java Edition 1.19
        }
    }
}

impl Chunk {
    fn new(x: i32, z: i32, sections: Vec<Section>) -> Self {
        Chunk {
            x_pos: x,
            z_pos: z,
            sections,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Section {
    block_states: BlockStates,
    biomes: Biomes,
    #[serde(rename = "Y")]
    y: i8,
}

impl Section {
    fn new(block_states: BlockStates, biomes: Biomes, y: i8) -> Self {
        Section {
            block_states,
            biomes,
            y,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Biomes {
    palette: Vec<String>,
}

impl Biomes {
    fn new(palette: Vec<String>) -> Self {
        Biomes { palette }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct BlockStates {
    palette: Vec<PaletteItem>,
    data: Option<LongArray>,
}

impl BlockStates {
    fn new(palette: Vec<PaletteItem>, data: Option<LongArray>) -> Self {
        BlockStates { palette, data }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PaletteItem {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Properties")]
    properties: Option<serde_json::Value>,
}

impl PaletteItem {
    fn new(name: String, properties: Option<serde_json::Value>) -> Self {
        PaletteItem { name, properties }
    }
}

pub fn write_anvil(region_data: &RegionData, file_path: &Path) -> Result<()> {
    let out_path = file_path.join(format!(
        "r.{}.{}.mca",
        region_data.position[0], region_data.position[1]
    ));
    let out_file = File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(out_path)
        .map_err(PipelineError::IoError)?;

    // Create a empty region object
    let empty_region = Arc::new(Mutex::new(Region::new(out_file).unwrap()));

    // (0..32).into_par_iter().for_each(|chunk_z| {
    //     (0..32).into_par_iter().for_each(|chunk_x| {
    //         let absolute_chunk_x = region_data.position[0] * 32 + chunk_x;
    //         let absolute_chunk_z = region_data.position[1] * 32 + chunk_z;

    //         let chunk_data = region_data
    //             .chunks
    //             .iter()
    //             .find(|c| c.position == [absolute_chunk_x, absolute_chunk_z]);

    //         let chunk = create_chunk_structure(absolute_chunk_x, absolute_chunk_z, chunk_data);

    //         let serialized_chunk = to_bytes(&chunk).unwrap();

    //         {
    //             // Write the chunk data to the region file
    //             let mut region = empty_region.lock().unwrap();
    //             region
    //                 .write_chunk(chunk_x as usize, chunk_z as usize, &serialized_chunk)
    //                 .unwrap();
    //         }
    //     });
    // });

    if let Err(e) = region_data.chunks.par_iter().try_for_each(|chunk_data| {
        let [absolute_chunk_x, absolute_chunk_z] = chunk_data.position;
        let chunk_x = absolute_chunk_x - region_data.position[0] * 32;
        let chunk_z = absolute_chunk_z - region_data.position[1] * 32;

        let chunk = create_chunk_structure(absolute_chunk_x, absolute_chunk_z, Some(chunk_data));
        let serialized_chunk = to_bytes(&chunk).unwrap();

        {
            // Write the chunk data to the region file
            let mut region = empty_region.lock().unwrap();
            region.write_chunk(chunk_x as usize, chunk_z as usize, &serialized_chunk)
        }
    }) {
        return match e {
            Error::IO(e) => Err(e.into()),
            e => Err(PipelineError::Other(e.to_string())),
        };
    };

    Ok(())
}

/// Calculate the number of bits and data size of a palette.
fn calculate_bits_and_size(palette_len: usize) -> (u32, u32) {
    let bits_per_block = match palette_len {
        0..=16 => 4,
        17..=2048 => (palette_len - 1).ilog2() + 1,
        2049.. => 12,
    };
    let data_size = (4096 * bits_per_block + 63) / 64;
    (bits_per_block, data_size)
}

/// Create sections
fn create_chunk_section(
    blocks: &[BlockData],
    palette: &mut Vec<PaletteItem>,
    section_y: i32,
) -> Section {
    // Calculate the number of bits and data size based on the size of the palette
    let (bits_per_block, data_size) = calculate_bits_and_size(palette.len());

    // Create a 1D array of 4096 elements and embed the PALETTE index in each block.
    let mut block_indices = vec![0; 4096];
    for block in blocks {
        let BlockPosition([x, y, z]) = block.position;

        // Calculate the index of the 1D array and store the index of the palette
        let index = (y as usize) * 256 + (z as usize) * 16 + (x as usize);
        let palette_index = palette
            .iter()
            .position(|b| b.name == block.block_id.get_block_id())
            .unwrap_or_else(|| {
                palette.push(PaletteItem::new(block.block_id.get_block_id(), None));
                palette.len() - 1
            });
        block_indices[index] = palette_index;
    }

    // Calculate the number of blocks stored in entry (i64) by BPE
    let blocks_per_entry = 64 / bits_per_block;

    // Divide 1D array by number of blocks
    let block_entries: Vec<&[usize]> = block_indices.chunks(blocks_per_entry as usize).collect();

    // Divide 1D arrays by the number of blocks.
    let mut data = Vec::with_capacity(data_size as usize);
    for entry in block_entries {
        let mut value: i64 = 0;
        for (i, &index) in entry.iter().enumerate() {
            value |= (index as i64) << (i * bits_per_block as usize);
        }
        data.push(value);
    }

    // Additional padding as required
    if data_size as usize > data.len() {
        let padding_size = data_size as usize - data.len();
        data.extend(std::iter::repeat(0).take(padding_size));
    }

    Section::new(
        BlockStates::new(palette.clone(), Some(LongArray::new(data))),
        Biomes::new(vec!["minecraft:the_void".to_string()]),
        section_y as i8,
    )
}

/// Create chunk structures
fn create_chunk_structure(chunk_x: i32, chunk_z: i32, chunk_data: Option<&ChunkData>) -> Chunk {
    let palette = vec![PaletteItem::new("minecraft:air".to_string(), None)];

    let sections: Vec<Section> = if let Some(chunk_data) = chunk_data {
        chunk_data
            .sections
            .iter()
            .map(|section| {
                let mut local_palette = palette.clone();

                // Register the block in the palette.
                for block in &section.blocks {
                    if !local_palette
                        .iter()
                        .any(|b| b.name == block.block_id.get_block_id())
                    {
                        let properties = if block.block_id.get_block_name() == "oak_leaves" {
                            Some(serde_json::json!({ "persistent": "true" }))
                        } else {
                            None
                        };
                        local_palette
                            .push(PaletteItem::new(block.block_id.get_block_id(), properties))
                    }
                }

                create_chunk_section(&section.blocks, &mut local_palette, section.y)
            })
            .collect()
    } else {
        // Create an empty section if there is no chunk_data
        Vec::new()
    };

    Chunk::new(chunk_x, chunk_z, sections)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_calculate_bits_and_size() {
        let test_cases = vec![
            (1, (4, 256)),
            (16, (4, 256)),
            (17, (5, 320)),
            (32, (5, 320)),
            (33, (6, 384)),
            (64, (6, 384)),
            (65, (7, 448)),
            (128, (7, 448)),
            (129, (8, 512)),
            (256, (8, 512)),
            (257, (9, 576)),
            (512, (9, 576)),
            (513, (10, 640)),
            (1024, (10, 640)),
            (1025, (11, 704)),
            (2048, (11, 704)),
            (2049, (12, 768)),
        ];

        for (palette_len, expected) in test_cases {
            assert_eq!(calculate_bits_and_size(palette_len), expected);
        }
    }

    #[test]
    fn test_full_chunk_section() {
        let mut blocks = Vec::new();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    blocks.push(BlockData::new(x, y, z, "stone".to_string()));
                }
            }
        }

        let mut palette = vec![PaletteItem::new("minecraft:air".to_string(), None)];

        let section_y = 0;

        let section = create_chunk_section(&blocks, &mut palette, section_y);

        assert_eq!(section.y, section_y as i8);

        assert_eq!(section.block_states.palette.len(), 2); // "minecraft:air"と"minecraft:stone"
        assert!(section.block_states.data.is_some());
        assert_eq!(section.biomes.palette.len(), 1);
        assert_eq!(section.biomes.palette[0], "minecraft:the_void");
    }

    #[test]

    fn test_palette_size_17() {
        let blocks = vec![
            BlockData::new(0, 0, 0, "white_wool".to_string()),
            BlockData::new(1, 0, 0, "light_gray_wool".to_string()),
            BlockData::new(2, 0, 0, "gray_wool".to_string()),
            BlockData::new(3, 0, 0, "black_wool".to_string()),
            BlockData::new(4, 0, 0, "brown_wool".to_string()),
            BlockData::new(5, 0, 0, "red_wool".to_string()),
            BlockData::new(6, 0, 0, "orange_wool".to_string()),
            BlockData::new(7, 0, 0, "yellow_wool".to_string()),
            BlockData::new(8, 0, 0, "lime_wool".to_string()),
            BlockData::new(9, 0, 0, "green_wool".to_string()),
            BlockData::new(10, 0, 0, "cyan_wool".to_string()),
            BlockData::new(11, 0, 0, "light_blue_wool".to_string()),
            BlockData::new(12, 0, 0, "blue_wool".to_string()),
            BlockData::new(13, 0, 0, "purple_wool".to_string()),
            BlockData::new(14, 0, 0, "magenta_wool".to_string()),
            BlockData::new(15, 0, 0, "pink_wool".to_string()),
        ];

        let mut palette = vec![PaletteItem::new("minecraft:air".to_string(), None)];

        let section_y = 0;
        let section = create_chunk_section(&blocks, &mut palette, section_y);
        assert_eq!(section.y, section_y as i8);
        assert_eq!(section.block_states.palette.len(), 17); // 17 colors of wool
        assert!(section.block_states.data.is_some());
        assert_eq!(section.biomes.palette.len(), 1);
        assert_eq!(section.biomes.palette[0], "minecraft:the_void");
    }

    #[test]
    fn test_create_chunk_structure() {
        let chunk_data = ChunkData {
            position: [0, 0],
            sections: vec![SectionData {
                y: 0,
                blocks: vec![BlockData::new(0, 0, 0, "stone".to_string())],
            }],
        };

        let chunk = create_chunk_structure(0, 0, Some(&chunk_data));

        assert_eq!(chunk.x_pos, 0);
        assert_eq!(chunk.z_pos, 0);
        assert_eq!(chunk.sections.len(), 1);
    }

    #[test]
    fn test_empty_chunk_structure() {
        let chunk = create_chunk_structure(0, 0, None);

        assert_eq!(chunk.x_pos, 0);
        assert_eq!(chunk.z_pos, 0);
        assert_eq!(chunk.sections.len(), 0);
    }

    #[test]
    fn test_create_region() {
        // Create a temporary directory for the test
        let dir = tempdir().unwrap();
        let file_path = dir.path();

        // Create dummy RegionData
        let region_data = create_dummy_region_data();

        // Call the create_region function
        let result = write_anvil(&region_data, file_path);
        assert!(result.is_ok());

        // Check that the file was created
        let out_path = PathBuf::from(format!(
            "{}/r.{}.{}.mca",
            file_path.display(),
            region_data.position[0],
            region_data.position[1]
        ));
        assert!(out_path.exists());

        // Read the file to ensure it was written correctly
        let mut file = File::open(out_path).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        assert!(!contents.is_empty());

        // Clean up the temporary directory
        dir.close().unwrap();
    }

    fn create_dummy_region_data() -> RegionData {
        // Create dummy chunks
        let chunks = vec![ChunkData {
            position: [0, 0],
            sections: vec![],
        }];

        // Create dummy RegionData
        RegionData {
            position: [0, 0],
            chunks,
        }
    }
}
