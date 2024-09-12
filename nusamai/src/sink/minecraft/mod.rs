//! Minecraft sink
mod block_colors;
mod level;
mod region;

use log::error;
use std::{io::Write, path::PathBuf, sync::Mutex};

use dda_voxelize::DdaVoxelizer;
use earcut::{utils3d::project3d_to_2d, Earcut};
use flate2::{write::GzEncoder, Compression};
use hashbrown::HashMap;
use rayon::prelude::*;

use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};
use nusamai_projection::etmerc::ExtendedTransverseMercatorProjection;

use crate::{
    get_parameter_value,
    option::use_lod_config,
    parameters::*,
    pipeline::{Feedback, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
    transformer,
    transformer::TransformerRegistry,
};

use block_colors::{DefaultBlockResolver, Voxel};
use level::{Data, Level};
use region::{write_anvil, BlockData, ChunkData, Position2D, RegionData, SectionData, WorldData};

use super::option::output_parameter;

pub struct MinecraftSinkProvider {}

impl DataSinkProvider for MinecraftSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            id_name: "minecraft".to_string(),
            name: "Minecraft".to_string(),
        }
    }

    fn sink_options(&self) -> Parameters {
        let mut params = Parameters::new();
        params.define(output_parameter());

        params
    }

    fn transformer_options(&self) -> TransformerRegistry {
        let mut settings: TransformerRegistry = TransformerRegistry::new();
        settings.insert(use_lod_config("max_lod"));

        settings
    }
    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);
        let transform_options = self.transformer_options();

        Box::<MinecraftSink>::new(MinecraftSink {
            output_path: output_path.as_ref().unwrap().into(),
            transform_settings: transform_options,
        })
    }
}

pub struct MinecraftSink {
    output_path: PathBuf,
    transform_settings: TransformerRegistry,
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

impl DataSink for MinecraftSink {
    fn make_requirements(&mut self, properties: TransformerRegistry) -> DataRequirements {
        let default_requirements = DataRequirements {
            tree_flattening: transformer::TreeFlatteningSpec::Flatten {
                feature: transformer::FeatureFlatteningOption::AllExceptThematicSurfaces,
                data: transformer::DataFlatteningOption::None,
                object: transformer::ObjectFlatteningOption::None,
            },
            ..Default::default()
        };

        for config in properties.configs.iter() {
            let _ = &self.transform_settings.update_transformer(config.clone());
        }

        self.transform_settings.build(default_requirements)
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let mut world_data = WorldData::new();
        let mut region_map: HashMap<Position2D, usize> = HashMap::new();
        let mut chunk_map: HashMap<(Position2D, Position2D), usize> = HashMap::new();
        let mut section_map: HashMap<(Position2D, Position2D, i32), usize> = HashMap::new();

        let mut global_bvol = BoundingVolume::default();

        // FIXME: Collecting all features in memory to calculate the bounding volume, which is not scalable.
        let parcels: Vec<crate::pipeline::Parcel> = upstream.into_iter().collect();

        parcels.iter().for_each(|parcel| {
            let entity = &parcel.entity;

            let geom_store = entity.geometry_store.read().unwrap();
            let mut local_bvol = BoundingVolume::default();

            // Calculation of bounding volume
            geom_store.vertices.iter().for_each(|v| {
                local_bvol.min_lng = local_bvol.min_lng.min(v[0]);
                local_bvol.max_lng = local_bvol.max_lng.max(v[0]);
                local_bvol.min_lat = local_bvol.min_lat.min(v[1]);
                local_bvol.max_lat = local_bvol.max_lat.max(v[1]);
                local_bvol.min_height = local_bvol.min_height.min(v[2]);
                local_bvol.max_height = local_bvol.max_height.max(v[2]);
            });

            global_bvol.update(&local_bvol);
        });

        log::info!("start voxelizing...");

        // Calculation of centre coordinates
        let center_lng = (global_bvol.min_lng + global_bvol.max_lng) / 2.0;
        let center_lat = (global_bvol.min_lat + global_bvol.max_lat) / 2.0;

        let projection = ExtendedTransverseMercatorProjection::new(
            center_lng,
            center_lat,
            0.9999,
            &nusamai_projection::ellipsoid::grs80(),
        );

        let typename_block = DefaultBlockResolver::new();

        let voxelizer = Mutex::new(DdaVoxelizer::<Voxel>::new());

        // TODO: Scalable par-region processing with external sorting (map-reduce).

        parcels.into_par_iter().try_for_each(|parcel| {
            feedback.ensure_not_canceled()?;

            let entity = parcel.entity;

            let Value::Object(obj) = &entity.root else {
                error!("The root value is not an object");
                return Result::<()>::Ok(());
            };

            let Some(voxel) = typename_block.resolve(&obj.typename) else {
                return Ok(());
            };

            let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype else {
                return Ok(());
            };

            let geom_store = entity.geometry_store.read().unwrap();

            geometries.par_iter().for_each(|entry| match entry.ty {
                GeometryType::Solid | GeometryType::Surface | GeometryType::Triangle => {
                    let mut earcutter = Earcut::<f32>::new();
                    let mut buf3d: Vec<[f32; 3]> = Vec::new();
                    let mut buf2d: Vec<[f32; 2]> = Vec::new();
                    let mut index_buf: Vec<u32> = Vec::new();

                    for idx_poly in geom_store
                        .multipolygon
                        .iter_range(entry.pos as usize..(entry.pos + entry.len) as usize)
                    {
                        let poly = idx_poly.transform(|idx| geom_store.vertices[*idx as usize]);
                        let num_outer = match poly.hole_indices().first() {
                            Some(&v) => v as usize,
                            None => poly.raw_coords().len(),
                        };

                        buf3d.clear();
                        buf3d.extend(poly.raw_coords().iter().map(|v| {
                            match projection.project_forward(v[0], v[1], v[2]) {
                                // To match the Minecraft coordinate system, the y-coordinate is multiplied by -1 and replaced with z
                                Ok((x, y, mut z)) => {
                                    // Set the minimum altitude to 0 by subtracting the minimum altitude of the bounding volume.
                                    z -= global_bvol.min_height.min(v[2]);
                                    [x as f32, z as f32, -y as f32]
                                }
                                Err(e) => {
                                    println!("conversion error: {:?}", e);
                                    [f32::NAN, f32::NAN, f32::NAN]
                                }
                            }
                        }));

                        if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                            earcutter.earcut(
                                buf2d.iter().cloned(),
                                poly.hole_indices(),
                                &mut index_buf,
                            );
                            {
                                let mut voxelizer = voxelizer.lock().unwrap();
                                for indx in index_buf.chunks_exact(3) {
                                    voxelizer.add_triangle(
                                        &[
                                            buf3d[indx[0] as usize],
                                            buf3d[indx[1] as usize],
                                            buf3d[indx[2] as usize],
                                        ],
                                        &|previous_value, _, _| match previous_value {
                                            None => voxel.clone(),
                                            Some(prev) => {
                                                if voxel.priority > prev.priority {
                                                    voxel.clone()
                                                } else {
                                                    prev.clone()
                                                }
                                            }
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
                GeometryType::Curve => {
                    // TODO: implement
                }
                GeometryType::Point => {
                    // TODO: implement
                }
            });

            Ok(())
        })?;

        let voxels = voxelizer.into_inner().unwrap().finalize();

        voxels.iter().for_each(|(pos, voxel)| {
            // TODO:Process for determining blocks from voxel colors.

            let [x, y, z] = pos;

            // Calculate region coordinates from x,y coordinates
            let region_x = x.div_euclid(512);
            let region_z = z.div_euclid(512);

            // Calculate chunk coordinates from x,y coordinates
            let chunk_x = x.div_euclid(16);
            let chunk_z = z.div_euclid(16);

            // Calculate the y-level of the section from the y-coordinate.
            let section_y = (y + 64) / 16 - 4;

            // Create BlockData
            // Coordinates relative to the blocks within a section (0-15)
            let block_data = BlockData::new(
                x.rem_euclid(16) as u8,
                y.rem_euclid(16) as u8,
                z.rem_euclid(16) as u8,
                voxel.block_name.to_string(),
            );

            let region_pos = [region_x, region_z];
            let chunk_pos = [chunk_x, chunk_z];

            let region_index = *region_map.entry(region_pos).or_insert_with(|| {
                world_data.push(RegionData {
                    position: region_pos,
                    chunks: Vec::new(),
                });
                world_data.len() - 1
            });

            let chunk_index = *chunk_map.entry((region_pos, chunk_pos)).or_insert_with(|| {
                world_data[region_index].chunks.push(ChunkData {
                    position: chunk_pos,
                    sections: Vec::new(),
                });
                world_data[region_index].chunks.len() - 1
            });

            let section_index = *section_map
                .entry((region_pos, chunk_pos, section_y))
                .or_insert_with(|| {
                    world_data[region_index].chunks[chunk_index]
                        .sections
                        .push(SectionData {
                            y: section_y,
                            blocks: Vec::new(),
                        });
                    world_data[region_index].chunks[chunk_index].sections.len() - 1
                });

            world_data[region_index].chunks[chunk_index].sections[section_index]
                .blocks
                .push(block_data);
        });

        let mut file_path = self.output_path.clone();
        file_path.push("region");
        std::fs::create_dir_all(&file_path)?;

        world_data.iter().try_for_each(|region| -> Result<()> {
            feedback.ensure_not_canceled()?;

            write_anvil(region, &file_path)?;

            Ok(())
        })?;

        // write level.dat
        let dir_name = self.output_path.file_name().unwrap().to_string_lossy();

        // Set the entered directory name as the level name
        let data = Data {
            level_name: Some(dir_name.to_string()),
            ..Default::default()
        };

        let level_dat_file = std::fs::File::create(self.output_path.join("level.dat"))?;
        let mut encoder = GzEncoder::new(level_dat_file, Compression::fast());

        let bytes = fastnbt::to_bytes(&Level { data }).unwrap();
        encoder.write_all(&bytes)?;

        Ok(())
    }
}
