//! Minecraft sink

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use hashbrown::HashMap;
use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};

use nusamai_plateau::Entity;
use rayon::prelude::*;

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
};

use earcut::{utils3d::project3d_to_2d, Earcut};
use nusamai_projection::etmerc::ExtendedTransverseMercatorProjection;

use nusamai_voxelize::{DdaVoxelizer, MeshVoxelizer, Voxel};

use fastanvil::Region;
use fastnbt::{to_bytes, LongArray};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use nusamai_citygml::object::{Map, Object};

pub struct MinecraftSinkProvider {}

impl DataSinkProvider for MinecraftSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "minecraft".to_string(),
            name: "Minecraft".to_string(),
        }
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(
            "@output".into(),
            ParameterEntry {
                description: "Output file path".into(),
                required: true,
                parameter: ParameterType::FileSystemPath(FileSystemPathParameter {
                    value: None,
                    must_exist: false,
                }),
            },
        );
        params
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);

        Box::<MinecraftSink>::new(MinecraftSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

pub struct MinecraftSink {
    output_path: PathBuf,
}

pub struct BoundingVolume {
    pub min_lng: f64,
    pub max_lng: f64,
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_height: f64,
    pub max_height: f64,
}

impl BoundingVolume {
    fn update(&mut self, other: &Self) {
        self.min_lng = self.min_lng.min(other.min_lng);
        self.max_lng = self.max_lng.max(other.max_lng);
        self.min_lat = self.min_lat.min(other.min_lat);
        self.max_lat = self.max_lat.max(other.max_lat);
        self.min_height = self.min_height.min(other.min_height);
        self.max_height = self.max_height.max(other.max_height);
    }
}

impl Default for BoundingVolume {
    fn default() -> Self {
        Self {
            min_lng: f64::MAX,
            max_lng: f64::MIN,
            min_lat: f64::MAX,
            max_lat: f64::MIN,
            min_height: f64::MAX,
            max_height: f64::MIN,
        }
    }
}

mod block_colors;

use block_colors::get_block_colors;

type PositionXYZ = [u8; 3];
type PositionXZ = [i32; 2];

#[derive(Deserialize, Serialize, Debug)]
struct BlockSchema {
    position: PositionXYZ,
    name: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct SectionSchema {
    y: i32,
    blocks: Vec<BlockSchema>,
}
#[derive(Deserialize, Serialize, Debug)]
struct ChunkSchema {
    position: PositionXZ,
    sections: Vec<SectionSchema>,
}

#[derive(Deserialize, Serialize, Debug)]
struct RegionSchema {
    position: PositionXZ,
    chunks: Vec<ChunkSchema>,
}

type WorldSchema = Vec<RegionSchema>;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Chunk {
    Status: String, // The status of the chunk, such as whether it is fully generated or being generated.
    zPos: i32,      // The Z coordinate of the chunk (absolute value).
    yPos: i32,      // The Y coordinate of the lowest section in the chunk.
    xPos: i32,      // The X coordinate of the chunk (absolute value).
    sections: Vec<Section>, // A vector containing the sections that make up the chunk.
    DataVersion: u32, // The version of the data format used to store this chunk.
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Section {
    block_states: Blockstates,
    biomes: Biomes,
    Y: i8,
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
    properties: Option<Value>,
}

impl DataSink for MinecraftSink {
    fn make_requirements(&self) -> DataRequirements {
        DataRequirements {
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        let mut world_data = WorldSchema::new();
        let mut region_map: HashMap<PositionXZ, usize> = HashMap::new();
        let mut chunk_map: HashMap<(PositionXZ, PositionXZ), usize> = HashMap::new();
        let mut section_map: HashMap<(PositionXZ, PositionXZ, i32), usize> = HashMap::new();

        // 境界の計算
        let mut local_bvol = BoundingVolume::default();
        let parcels: Vec<crate::pipeline::Parcel> = upstream.into_iter().collect();

        parcels.iter().for_each(|parcel| {
            let entity = &parcel.entity;

            let geom_store = entity.geometry_store.read().unwrap();
            let mut parcel_bvol = BoundingVolume::default();

            // バウンディングボリュームの計算
            geom_store.vertices.iter().for_each(|v| {
                parcel_bvol.min_lng = parcel_bvol.min_lng.min(v[0]);
                parcel_bvol.max_lng = parcel_bvol.max_lng.max(v[0]);
                parcel_bvol.min_lat = parcel_bvol.min_lat.min(v[1]);
                parcel_bvol.max_lat = parcel_bvol.max_lat.max(v[1]);
                parcel_bvol.min_height = parcel_bvol.min_height.min(v[2]);
                parcel_bvol.max_height = parcel_bvol.max_height.max(v[2]);
            });

            local_bvol.update(&parcel_bvol);
        });
        let (ra, rb) = rayon::join(
            || {
                parcels
                    .into_par_iter()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let entity = parcel.entity;

                        let geom_store = entity.geometry_store.read().unwrap();

                        let mut earcutter = Earcut::<f32>::new();
                        let mut buf3d: Vec<[f32; 3]> = Vec::new();
                        let mut buf2d: Vec<[f32; 2]> = Vec::new();
                        let mut index_buf: Vec<u32> = Vec::new();

                        let mut voxelizer = DdaVoxelizer {
                            voxels: HashMap::new(),
                        };

                        // 中心点の計算
                        let center_lng = (local_bvol.min_lng + local_bvol.max_lng) / 2.0;
                        let center_lat = (local_bvol.min_lat + local_bvol.max_lat) / 2.0;
                        let min_height = local_bvol.min_height;

                        let projection = ExtendedTransverseMercatorProjection::new(
                            center_lng,
                            center_lat,
                            1.0,
                            &nusamai_projection::ellipsoid::wgs84(),
                        );

                        let vertices: Vec<_> = geom_store
                            .vertices
                            .iter()
                            .map(|v| match projection.project_forward(v[0], v[1], v[2]) {
                                Ok((x, y, z)) => [x, z - min_height - 64 as f64, y * -1 as f64],
                                Err(e) => {
                                    println!("変換エラー: {:?}", e);
                                    // エラーの場合は無効な座標を返す
                                    [f64::NAN, f64::NAN, f64::NAN]
                                }
                            })
                            .collect();

                        for (idx_poly) in geom_store.multipolygon.iter() {
                            let poly = idx_poly.transform(|idx| vertices[*idx as usize]);
                            let num_outer = match poly.hole_indices().first() {
                                Some(&v) => v as usize,
                                None => poly.raw_coords().len(),
                            };

                            buf3d.clear();
                            buf3d.extend(
                                poly.raw_coords()
                                    .iter()
                                    .map(|v| [v[0] as f32, v[1] as f32, v[2] as f32]),
                            );

                            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                                earcutter.earcut(
                                    buf2d.iter().cloned(),
                                    poly.hole_indices(),
                                    &mut index_buf,
                                );
                                for indx in index_buf.chunks_exact(3) {
                                    voxelizer.add_triangle(&[
                                        buf3d[indx[0] as usize],
                                        buf3d[indx[1] as usize],
                                        buf3d[indx[2] as usize],
                                    ]);
                                }
                            }
                        }

                        let occupied_voxels = voxelizer.finalize();

                        if sender.send(occupied_voxels).is_err() {
                            return Err(PipelineError::Canceled);
                        };

                        Ok(())
                    })
            },
            || {
                let block_colors = get_block_colors();

                receiver.into_iter().for_each(|feature| {
                    feature.iter().for_each(|(key, voxel)| {
                        // 最も近いブロックを見つける
                        let mut min_distance = f64::MAX;
                        let mut block_name = "minecraft:stone";

                        for (name, color) in block_colors.iter() {
                            let distance = (color.0 as f64 - voxel.color[0] as f64).powi(2)
                                + (color.1 as f64 - voxel.color[1] as f64).powi(2)
                                + (color.2 as f64 - voxel.color[2] as f64).powi(2);

                            if distance < min_distance {
                                min_distance = distance;
                                block_name = name;
                            }
                        }

                        // ピクセル座標から、中心のピクセル座標を減算
                        let adjusted_x = key[0];
                        let adjusted_y = key[1];
                        let adjusted_z = key[2];

                        // ピクセル座標からリージョン座標を計算
                        let region_x = adjusted_x.div_euclid(512);
                        let region_z = adjusted_z.div_euclid(512);

                        // ピクセル座標座標からチャンク座標を計算
                        let chunk_x = adjusted_x.div_euclid(16);
                        let chunk_z = adjusted_z.div_euclid(16);

                        // DEMの値からセクションのyレベルを計算
                        let section_y = (adjusted_y + 64) / 16 - 4;

                        // ブロックのスキーマを作成
                        let block_data = BlockSchema {
                            position: [
                                adjusted_x.rem_euclid(16) as u8,
                                adjusted_y.rem_euclid(16) as u8,
                                adjusted_z.rem_euclid(16) as u8,
                            ],
                            name: block_name.to_string(),
                        };

                        // リージョン、チャンク、セクションのインデックスを取得
                        let region_pos = [region_x, region_z];
                        let chunk_pos = [chunk_x, chunk_z];

                        let region_index = *region_map.entry(region_pos).or_insert_with(|| {
                            world_data.push(RegionSchema {
                                position: region_pos,
                                chunks: Vec::new(),
                            });
                            world_data.len() - 1
                        });

                        let chunk_index =
                            *chunk_map.entry((region_pos, chunk_pos)).or_insert_with(|| {
                                world_data[region_index].chunks.push(ChunkSchema {
                                    position: chunk_pos,
                                    sections: Vec::new(),
                                });
                                world_data[region_index].chunks.len() - 1
                            });

                        let section_index = *section_map
                            .entry((region_pos, chunk_pos, section_y))
                            .or_insert_with(|| {
                                world_data[region_index].chunks[chunk_index].sections.push(
                                    SectionSchema {
                                        y: section_y,
                                        blocks: Vec::new(),
                                    },
                                );
                                world_data[region_index].chunks[chunk_index].sections.len() - 1
                            });

                        world_data[region_index].chunks[chunk_index].sections[section_index]
                            .blocks
                            .push(block_data);
                    });
                });

                let mut file_path = self.output_path.clone();
                file_path.push("region");
                std::fs::create_dir_all(&file_path)?;

                let _ = world_data.iter().try_for_each(|region| -> Result<()> {
                    feedback.ensure_not_canceled()?;

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
                        .unwrap();

                    let new_region = Arc::new(Mutex::new(Region::new(out_file).unwrap()));

                    (0..32).into_par_iter().for_each(|chunk_z| {
                        (0..32).into_par_iter().for_each(|chunk_x| {
                            // チャンクの絶対座標を計算
                            let absolute_chunk_x = region.position[0] * 32 + chunk_x;
                            let absolute_chunk_z = region.position[1] * 32 + chunk_z;

                            let chunk_data = region
                                .chunks
                                .iter()
                                .find(|c| c.position == [absolute_chunk_x, absolute_chunk_z]);

                            let chunk = create_chunk_structure(
                                absolute_chunk_x,
                                absolute_chunk_z,
                                chunk_data,
                            );

                            let ser = to_bytes(&chunk).unwrap(); // 適切なシリアライズ関数を想定

                            let mut region = new_region.lock().unwrap();
                            region
                                .write_chunk(chunk_x as usize, chunk_z as usize, &ser)
                                .unwrap();
                        });
                    });

                    Ok(())
                });

                Ok(())
            },
        );

        match ra {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.fatal_error(error),
        }
        match rb {
            Ok(_) | Err(PipelineError::Canceled) => {}
            Err(error) => feedback.fatal_error(error),
        }

        Ok(())
    }
}

// パレットのビット数とデータサイズを計算する関数
fn calculate_bits_and_size(palette_len: usize) -> (usize, usize) {
    let bits_per_block = match palette_len {
        1..=16 => 4,
        17..=32 => 5,
        33..=64 => 6,
        65..=128 => 7,
        129..=256 => 8,
        257..=512 => 9,
        513..=1024 => 10,
        1025..=2048 => 11,
        _ => 12,
    };
    let data_size = (4096 * bits_per_block + 63) / 64;
    (bits_per_block, data_size)
}

// セクションを作成する関数
fn create_chunk_section(
    blocks: &[BlockSchema],
    palette: &mut Vec<PaletteItem>,
    section_y: i32,
) -> Section {
    // パレットのサイズに基づいてビット数とデータサイズを計算
    let (bits_per_block, data_size) = calculate_bits_and_size(palette.len());

    // 4096要素の1次元配列を作成し、ブロックごとにpaletteのindexを埋め込む
    let mut block_indices = vec![0; 4096];
    for block in blocks {
        let [x, y, z] = block.position;

        // 1次元配列のインデックスを計算し、パレットのインデックスを格納
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

    // BPEにより、entry（i64）に格納されるブロック数を算出する
    let blocks_per_entry = 64 / bits_per_block;

    // 1次元配列をブロック数で分割する
    let block_entries: Vec<&[usize]> = block_indices.chunks(blocks_per_entry).collect();

    // 分割後、ビッグエンディアンでi64に変換する
    let mut data = Vec::with_capacity(data_size);
    for entry in block_entries {
        let mut value: i64 = 0;
        for (i, &index) in entry.iter().enumerate() {
            value |= (index as i64) << (i * bits_per_block);
        }
        data.push(value);
    }

    // パディングを追加（必要な場合のみ）
    if data_size > data.len() {
        let padding_size = data_size - data.len();
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
        Y: section_y as i8,
    }
}

// チャンクの構造を作成する関数
fn create_chunk_structure(chunk_x: i32, chunk_z: i32, chunk_data: Option<&ChunkSchema>) -> Chunk {
    let mut palette = vec![PaletteItem {
        name: "minecraft:air".to_string(),
        properties: None,
    }];

    let sections: Vec<Section> = if let Some(chunk_data) = chunk_data {
        chunk_data
            .sections
            .iter()
            .map(|section| {
                let mut local_palette = palette.clone();

                // ブロックをパレットに登録
                for block in &section.blocks {
                    if !local_palette.iter().any(|b| b.name == block.name) {
                        local_palette.push(PaletteItem {
                            name: block.name.clone(),
                            properties: None,
                        });
                    }
                }

                create_chunk_section(&section.blocks, &mut local_palette, section.y)
            })
            .collect()
    } else {
        // chunk_dataがない場合は空のセクションを作成
        Vec::new()
    };

    Chunk {
        Status: "full".to_string(),
        zPos: chunk_z,
        yPos: -4,
        xPos: chunk_x,
        sections,
        DataVersion: 3105, // Java Edition 1.19
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
