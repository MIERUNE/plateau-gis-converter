use crate::pipeline::{PipelineError, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path, path::PathBuf};

use fastanvil::Region;
use fastnbt::{to_bytes, LongArray};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub type Position2D = [i32; 2];
#[derive(Deserialize, Serialize, Debug)]
struct BlockPosition([u8; 3]);

pub struct BlockId {
    name_space: String,
    block_id: String,
}

impl BlockId {
    pub fn get_id() -> Result<Self> {
        Ok(BlockId {
            name_space: "minecraft".to_string(),
            block_id: "stone".to_string(),
        })
    }

    pub fn new(name_space: String, block_id: String) -> Result<Self> {
        Ok(BlockId {
            name_space,
            block_id,
        })
    }

    // デフォルト値を返す
    pub fn default() -> Self {
        BlockId {
            name_space: "minecraft".to_string(),
            block_id: "stone".to_string(),
        }
    }
}

impl BlockPosition {
    // Check that the input is in the range 0~15
    pub fn new(x: u8, y: u8, z: u8) -> Result<Self> {
        if x > 15 || y > 15 || z > 15 {
            Err(PipelineError::Other(format!(
                "Invalid BlockPosition values: x={}, y={}, z={}. The position values must be within the range of 0 to 15.",
                x, y, z
            )))
        } else {
            Ok(BlockPosition([x, y, z]))
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockSchema {
    position: BlockPosition,
    name: String,
}

impl BlockSchema {
    pub fn new(x: u8, y: u8, z: u8, name: String) -> Result<Self> {
        let position = BlockPosition::new(x, y, z)?;
        Ok(BlockSchema { position, name })
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SectionSchema {
    pub y: i32,
    pub blocks: Vec<BlockSchema>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ChunkSchema {
    pub position: Position2D,
    pub sections: Vec<SectionSchema>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegionSchema {
    pub position: Position2D,
    pub chunks: Vec<ChunkSchema>,
}

pub type WorldSchema = Vec<RegionSchema>;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Chunk {
    #[serde(rename = "Status")]
    status: String, // The status of the chunk, such as whether it is fully generated or being generated.
    #[serde(rename = "zPos")]
    z_pos: i32, // The Z coordinate of the chunk (absolute value).
    #[serde(rename = "yPos")]
    y_pos: i32, // The Y coordinate of the lowest section in the chunk.
    #[serde(rename = "xPos")]
    x_pos: i32, // The X coordinate of the chunk (absolute value).
    sections: Vec<Section>, // A vector containing the sections that make up the chunk.
    #[serde(rename = "DataVersion")]
    data_version: u32, // The version of the data format used to store this chunk.
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Section {
    block_states: Blockstates,
    biomes: Biomes,
    #[serde(rename = "Y")]
    y: i8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Biomes {
    palette: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Blockstates {
    palette: Vec<PaletteItem>,
    data: Option<LongArray>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PaletteItem {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Properties")]
    properties: Option<serde_json::Value>,
}

pub fn write_region(region: &RegionSchema, file_path: &Path) -> Result<()> {
    let out_path = PathBuf::from(format!(
        "{}/r.{}.{}.mca",
        file_path.display(),
        region.position[0],
        region.position[1]
    ));

    let out_file = File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(out_path)
        .map_err(PipelineError::IoError)?;

    let new_region = Arc::new(Mutex::new(Region::new(out_file).unwrap()));

    (0..32).into_par_iter().for_each(|chunk_z| {
        (0..32).into_par_iter().for_each(|chunk_x| {
            // Calculate absolute coordinates of chunks
            let absolute_chunk_x = region.position[0] * 32 + chunk_x;
            let absolute_chunk_z = region.position[1] * 32 + chunk_z;

            let chunk_data = region
                .chunks
                .iter()
                .find(|c| c.position == [absolute_chunk_x, absolute_chunk_z]);

            let chunk = create_chunk_structure(absolute_chunk_x, absolute_chunk_z, chunk_data);

            let ser = to_bytes(&chunk).unwrap();

            let mut region = new_region.lock().unwrap();
            region
                .write_chunk(chunk_x as usize, chunk_z as usize, &ser)
                .unwrap();
        });
    });

    Ok(())
}

// Function to calculate the number of bits and data size of a palette.
fn calculate_bits_and_size(palette_len: usize) -> (u32, u32) {
    let bits_per_block = match palette_len {
        0..=16 => 4,
        17..=2048 => (palette_len - 1).ilog2() + 1,
        2049.. => 12,
    };
    let data_size = (4096 * bits_per_block + 63) / 64;
    (bits_per_block, data_size)
}

// Functions to create sections
fn create_chunk_section(
    blocks: &[BlockSchema],
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
            .position(|b| b.name == block.name)
            .unwrap_or_else(|| {
                palette.push(PaletteItem {
                    name: block.name.clone(),
                    properties: None,
                });
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

    Section {
        block_states: Blockstates {
            palette: palette.clone(),
            data: Some(LongArray::new(data)),
        },
        biomes: Biomes {
            palette: vec!["minecraft:the_void".to_string()],
        },
        y: section_y as i8,
    }
}

// Functions to create chunk structures
fn create_chunk_structure(chunk_x: i32, chunk_z: i32, chunk_data: Option<&ChunkSchema>) -> Chunk {
    let palette = vec![PaletteItem {
        name: "minecraft:air".to_string(),
        properties: None,
    }];

    let sections: Vec<Section> = if let Some(chunk_data) = chunk_data {
        chunk_data
            .sections
            .iter()
            .map(|section| {
                let mut local_palette = palette.clone();

                // Register the block in the palette.
                for block in &section.blocks {
                    if !local_palette.iter().any(|b| b.name == block.name) {
                        local_palette.push(PaletteItem {
                            name: block.name.clone(),
                            // Set the persistent property for oak leaves.
                            properties: if block.name == "minecraft:oak_leaves" {
                                Some(serde_json::json!({ "persistent": "true" }))
                            } else {
                                None
                            },
                        });
                    }
                }

                create_chunk_section(&section.blocks, &mut local_palette, section.y)
            })
            .collect()
    } else {
        // Create an empty section if there is no chunk_data
        Vec::new()
    };

    Chunk {
        status: "full".to_string(),
        z_pos: chunk_z,
        y_pos: -4, // Lowest Y section position in the chunk (e.g., -4 in version 1.18 and later)
        x_pos: chunk_x,
        sections,
        data_version: 3105, // Java Edition 1.19
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
