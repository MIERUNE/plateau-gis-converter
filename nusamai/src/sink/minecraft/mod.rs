//! Minecraft sink
mod region_writer;
use log::error;
use region_writer::{
    write_region, BlockSchema, ChunkSchema, Position2D, RegionSchema, SectionSchema, WorldSchema,
};

mod block_colors;
use block_colors::{get_block_colors, get_typename_colors};
use std::path::PathBuf;

use hashbrown::HashMap;
use nusamai_citygml::{
    object::{ObjectStereotype, Value},
    schema::Schema,
    GeometryType,
};
use rayon::prelude::*;

use crate::{
    get_parameter_value,
    parameters::*,
    pipeline::{Feedback, PipelineError, Receiver, Result},
    sink::{DataRequirements, DataSink, DataSinkProvider, SinkInfo},
};

use earcut::{utils3d::project3d_to_2d, Earcut};
use nusamai_projection::etmerc::ExtendedTransverseMercatorProjection;

use nusamai_voxelize::{DdaVoxelizer, MeshVoxelizer};

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

impl DataSink for MinecraftSink {
    fn make_requirements(&self) -> DataRequirements {
        DataRequirements {
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1000);

        let mut world_data = WorldSchema::new();
        let mut region_map: HashMap<Position2D, usize> = HashMap::new();
        let mut chunk_map: HashMap<(Position2D, Position2D), usize> = HashMap::new();
        let mut section_map: HashMap<(Position2D, Position2D, i32), usize> = HashMap::new();

        let block_colors = get_block_colors();

        let mut local_bvol = BoundingVolume::default();

        // FIXME: Collecting all features in memory to calculate the bounding volume, which is not scalable.
        let parcels: Vec<crate::pipeline::Parcel> = upstream.into_iter().collect();

        parcels.iter().for_each(|parcel| {
            let entity = &parcel.entity;

            let geom_store = entity.geometry_store.read().unwrap();
            let mut parcel_bvol = BoundingVolume::default();

            // Calculation of bounding volume
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

        // Calculation of centre coordinates
        let center_lng = (local_bvol.min_lng + local_bvol.max_lng) / 2.0;
        let center_lat = (local_bvol.min_lat + local_bvol.max_lat) / 2.0;

        let projection = ExtendedTransverseMercatorProjection::new(
            center_lng,
            center_lat,
            0.9999,
            &nusamai_projection::ellipsoid::grs80(),
        );

        let (ra, rb) = rayon::join(
            || {
                parcels
                    .into_par_iter()
                    .try_for_each_with(sender, |sender, parcel| {
                        feedback.ensure_not_canceled()?;

                        let entity = parcel.entity;

                        let mut rgb = [255, 255, 255];
                        let typename_colors = get_typename_colors();

                        let Value::Object(obj) = &entity.root else {
                            error!("The root value is not an object");
                            return Ok(());
                        };

                        let typename = &obj.typename.as_ref();

                        match typename_colors.get(typename) {
                            Some(color) => {
                                rgb = *color;
                            }
                            _ => {
                                feedback.info(format!(
                                    "No color found for typename '{}'. Using white color.",
                                    typename
                                ));
                            }
                        }
                        let ObjectStereotype::Feature { geometries, .. } = &obj.stereotype else {
                            return Ok(());
                        };

                        let geom_store = entity.geometry_store.read().unwrap();

                        let mut earcutter = Earcut::<f32>::new();
                        let mut buf3d: Vec<[f32; 3]> = Vec::new();
                        let mut buf2d: Vec<[f32; 2]> = Vec::new();
                        let mut index_buf: Vec<u32> = Vec::new();

                        let vertices: Vec<_> = geom_store
                            .vertices
                            .iter()
                            .map(|v| match projection.project_forward(v[0], v[1], v[2]) {
                                // To match the Minecraft coordinate system, the y-coordinate is multiplied by -1 and replaced with z
                                Ok((x, y, z)) => [x, z, y * -1.0],
                                Err(e) => {
                                    println!("conversion error: {:?}", e);
                                    [f64::NAN, f64::NAN, f64::NAN]
                                }
                            })
                            .collect();

                        let mut voxelizer = DdaVoxelizer {
                            voxels: HashMap::new(),
                        };

                        geometries.iter().for_each(|entry| match entry.ty {
                            GeometryType::Solid
                            | GeometryType::Surface
                            | GeometryType::Triangle => {
                                for idx_poly in geom_store.multipolygon.iter_range(
                                    entry.pos as usize..(entry.pos + entry.len) as usize,
                                ) {
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
                            }
                            GeometryType::Curve => {
                                // TODO: implement
                            }
                            GeometryType::Point => {
                                // TODO: implement
                            }
                        });

                        let occupied_voxels = voxelizer.finalize();

                        if sender.send(occupied_voxels).is_err() {
                            return Err(PipelineError::Canceled);
                        };

                        Ok(())
                    })
            },
            || {
                receiver.into_iter().for_each(|feature| {
                    feature.iter().for_each(|(key, voxel)| {
                        let mut block_name = "minecraft:white_wool";

                        // Find the block with the closest color to the voxel color
                        let mut min_distance = f64::MAX;
                        for (name, color) in &block_colors {
                            let distance = ((color[0] as f64 - voxel.color[0] as f64).powi(2)
                                + (color[1] as f64 - voxel.color[1] as f64).powi(2)
                                + (color[2] as f64 - voxel.color[2] as f64).powi(2))
                            .sqrt();
                            if distance < min_distance {
                                min_distance = distance;
                                block_name = name;
                            }
                        }

                        let [x, y, z] = key;

                        // Calculate region coordinates from x,y coordinates
                        let region_x = x.div_euclid(512);
                        let region_z = z.div_euclid(512);

                        // Calculate chunk coordinates from x,y coordinates
                        let chunk_x = x.div_euclid(16);
                        let chunk_z = z.div_euclid(16);

                        // Calculate the y-level of the section from the y-coordinate.
                        let section_y = (y + 64) / 16 - 4;

                        // Create BlockSchema
                        // Coordinates relative to the blocks within a section (0-15)
                        let block_data = BlockSchema::new(
                            x.rem_euclid(16) as u8,
                            y.rem_euclid(16) as u8,
                            z.rem_euclid(16) as u8,
                            block_name.to_string(),
                        );

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
                            .push(block_data.unwrap());
                    });
                });

                let mut file_path = self.output_path.clone();
                file_path.push("region");
                std::fs::create_dir_all(&file_path)?;

                let _ = world_data.iter().try_for_each(|region| -> Result<()> {
                    feedback.ensure_not_canceled()?;

                    write_region(region, &file_path)?;

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
