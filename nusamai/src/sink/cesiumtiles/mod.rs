//! 3D Tiles sink

use std::fs;
use std::io::{BufWriter, Write};
pub mod slice;
pub mod sort;
pub mod tiling;

use std::path::{Path, PathBuf};
use std::sync::mpsc;

use ahash::RandomState;
use byteorder::{ByteOrder, LittleEndian};
use earcut_rs::utils_3d::project3d_to_2d;
use earcut_rs::Earcut;
use ext_sort::{buffer::mem::MemoryLimitedBufferBuilder, ExternalSorter, ExternalSorterBuilder};
use indexmap::IndexSet;
use itertools::Itertools;
use nusamai_projection::cartesian::geographic_to_geocentric;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use nusamai_citygml::schema::Schema;
use nusamai_geometry::MultiPolygon;
use nusamai_mvt::tileid::TileIdMethod;

use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};
use crate::{get_parameter_value, transformer};
use slice::slice_cityobj_geoms;
use sort::BincodeExternalChunk;

pub struct CesiumTilesSinkProvider {}

impl DataSinkProvider for CesiumTilesSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "Vector Tiles (MVT)".to_string(),
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
        // TODO: min Zoom
        // TODO: max Zoom
        params
    }

    fn create(&self, params: &Parameters) -> Box<dyn DataSink> {
        let output_path = get_parameter_value!(params, "@output", FileSystemPath);

        Box::<CesiumTilesSink>::new(CesiumTilesSink {
            output_path: output_path.as_ref().unwrap().into(),
        })
    }
}

struct CesiumTilesSink {
    output_path: PathBuf,
}

#[derive(Serialize, Deserialize, deepsize::DeepSizeOf)]
struct SerializedSlicedFeature {
    tile_id: u64,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct SlicedFeature<'a> {
    geometry: MultiPolygon<'a, 3>,
    properties: nusamai_citygml::object::Value,
}

impl DataSink for CesiumTilesSink {
    fn make_transform_requirements(&self) -> transformer::Requirements {
        use transformer::RequirementItem;

        transformer::Requirements {
            mergedown: RequirementItem::Recommended(transformer::Mergedown::Full),
            ..Default::default()
        }
    }

    fn run(&mut self, upstream: Receiver, feedback: &Feedback, _schema: &Schema) {
        let (sender_sliced, receiver_sliced) = mpsc::sync_channel(2000);
        let (sender_sorted, receiver_sorted) = mpsc::sync_channel(2000);

        let tile_id_conv = TileIdMethod::Hilbert;

        // TODO: refactoring

        std::thread::scope(|s| {
            // Slicing geometry along the tile boundaries
            {
                let feedback = feedback.clone();
                s.spawn(move || {
                    geometry_slicing_stage(feedback, upstream, tile_id_conv, sender_sliced);
                });
            }

            // Sort features by tile_id (using external sorter)
            {
                s.spawn(move || {
                    feature_sorting_stage(receiver_sliced, sender_sorted);
                });
            }

            // Group sorted features and write them into tiles
            {
                let feedback = feedback.clone();
                let output_path = &self.output_path;
                s.spawn(move || {
                    // Run in a separate thread pool to avoid deadlocks
                    let pool = rayon::ThreadPoolBuilder::new()
                        .use_current_thread()
                        .build()
                        .unwrap();
                    pool.install(|| {
                        tile_writing_stage(output_path, feedback, receiver_sorted, tile_id_conv);
                    })
                });
            }
        });
    }
}

fn geometry_slicing_stage(
    feedback: Feedback,
    upstream: mpsc::Receiver<crate::pipeline::Parcel>,
    tile_id_conv: TileIdMethod,
    sender_sliced: mpsc::SyncSender<SerializedSlicedFeature>,
) {
    // Convert CityObjects to sliced features
    let _ = upstream.into_iter().par_bridge().try_for_each(|parcel| {
        if feedback.is_cancelled() {
            return Err(());
        }

        slice_cityobj_geoms(&parcel.entity, 7, 17, |(z, x, y), mpoly| {
            let feature = SlicedFeature {
                geometry: mpoly,
                properties: parcel.entity.root.clone(),
            };
            let bytes = bincode::serialize(&feature).unwrap();
            let sfeat = SerializedSlicedFeature {
                tile_id: tile_id_conv.zxy_to_id(z, x, y),
                body: bytes,
            };

            if sender_sliced.send(sfeat).is_err() {
                log::info!("sink cancelled");
                return Err(());
            };
            Ok(())
        })
    });
}

fn feature_sorting_stage(
    receiver_sliced: mpsc::Receiver<SerializedSlicedFeature>,
    sender_sorted: mpsc::SyncSender<(u64, Vec<SerializedSlicedFeature>)>,
) {
    let sorter: ExternalSorter<
        SerializedSlicedFeature,
        std::io::Error,
        MemoryLimitedBufferBuilder,
        BincodeExternalChunk<_>,
        // TODO: Implement an external sorter by ourselves?
    > = ExternalSorterBuilder::new()
        .with_tmp_dir(Path::new("./"))
        .with_buffer(MemoryLimitedBufferBuilder::new(200 * 1024 * 1024)) // TODO
        .with_threads_number(8) // TODO
        .build()
        .unwrap();
    let sorted = sorter
        .sort_by(receiver_sliced.into_iter().map(Ok), |a, b| {
            a.tile_id.cmp(&b.tile_id)
        })
        .unwrap();

    for (tile_id, ser_feats) in &sorted
        .map(Result::unwrap)
        .group_by(|ser_feat| ser_feat.tile_id)
    {
        let ser_feats: Vec<_> = ser_feats.collect();
        if sender_sorted.send((tile_id, ser_feats)).is_err() {
            log::info!("sink cancelled?");
            return;
        };
    }
}

fn tile_writing_stage(
    output_path: &Path,
    feedback: Feedback,
    receiver_sorted: mpsc::Receiver<(u64, Vec<SerializedSlicedFeature>)>,
    tile_id_conv: TileIdMethod,
) {
    let ellipsoid = nusamai_projection::ellipsoid::wgs84();

    let _ = receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|(tile_id, sfeats)| {
            if feedback.is_cancelled() {
                return Err(());
            }

            let mut earcutter = Earcut::new();
            let mut buf3d: Vec<f64> = Vec::new();
            let mut buf2d: Vec<f64> = Vec::new();
            let mut triangles_buf: Vec<u32> = Vec::new();
            let mut triangles = Vec::new();

            for ser_feat in sfeats {
                let mut feat: SlicedFeature = bincode::deserialize(&ser_feat.body).unwrap();

                feat.geometry.transform_inplace(|&[lng, lat, height]| {
                    let (x, y, z) = geographic_to_geocentric(&ellipsoid, lng, lat, height);
                    [x, z, -y]
                });

                for poly in &feat.geometry {
                    let num_outer = match poly.hole_indices().first() {
                        Some(&v) => v as usize,
                        None => poly.coords().len() / 3,
                    };

                    buf3d.clear();
                    buf3d.extend(poly.coords());

                    if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                        // earcut
                        earcutter.earcut(&buf2d, poly.hole_indices(), 2, &mut triangles_buf);
                    }

                    triangles.extend(triangles_buf.iter().map(|idx| {
                        [
                            buf3d[*idx as usize * 3],
                            buf3d[*idx as usize * 3 + 1],
                            buf3d[*idx as usize * 3 + 2],
                        ]
                    }));
                }
            }

            // calculate the centroid and min/max
            let mut pos_max = [f64::MIN; 3];
            let mut pos_min = [f64::MAX; 3];
            let mut translation = [0.; 3];

            for &[x, y, z] in &triangles {
                pos_min = [
                    f64::min(pos_min[0], x),
                    f64::min(pos_min[1], y),
                    f64::min(pos_min[2], z),
                ];
                pos_max = [
                    f64::max(pos_max[0], x),
                    f64::max(pos_max[1], y),
                    f64::max(pos_max[2], z),
                ];
            }
            // TODO: Use a library for 3d linalg
            translation[0] = (pos_max[0] + pos_min[0]) / 2.;
            translation[1] = (pos_max[1] + pos_min[1]) / 2.;
            translation[2] = (pos_max[2] + pos_min[2]) / 2.;
            pos_min[0] -= translation[0];
            pos_max[0] -= translation[0];
            pos_min[1] -= translation[1];
            pos_max[1] -= translation[1];
            pos_min[2] -= translation[2];
            pos_max[2] -= translation[2];

            // make vertices and indices
            let mut vertices: IndexSet<[u32; 3], RandomState> = IndexSet::default();
            let indices: Vec<_> = triangles
                .iter()
                .map(|&[x, y, z]| {
                    let (x, y, z) = (x - translation[0], y - translation[1], z - translation[2]);
                    let vbits = [
                        (x as f32).to_bits(),
                        (y as f32).to_bits(),
                        (z as f32).to_bits(),
                    ];
                    let (index, _) = vertices.insert_full(vbits);
                    index as u32
                })
                .collect();

            // print tile information
            let (zoom, x, y) = tile_id_conv.id_to_zxy(tile_id);
            let (min_y, max_y) = tiling::y_slice_range(zoom, y);
            let xs = tiling::x_step(zoom, y);
            let (min_x, max_x) = tiling::x_slice_range(zoom, x as i32, xs);
            println!(
                "tile: z={}, x={}, y={} (lng: {} -> {}, lat: {} -> {})",
                zoom, x, y, min_x, max_x, min_y, max_y
            );
            println!("{:?} {:?}", vertices.len(), indices.len());

            // write to file
            let path_glb = output_path.join(Path::new(&format!("{}/{}/{}.glb", zoom, x, y)));
            if let Some(dir) = path_glb.parent() {
                if let Err(e) = fs::create_dir_all(dir) {
                    panic!("Fatal error: {:?}", e); // FIXME
                }
            }
            write_gltf_glb(&path_glb, pos_min, pos_max, translation, vertices, indices);

            Ok(())
        });
}

fn write_gltf_glb(
    path: &Path,
    min: [f64; 3],
    max: [f64; 3],
    translation: [f64; 3],
    vertices: impl IntoIterator<Item = [u32; 3]>,
    indices: impl IntoIterator<Item = u32>,
) {
    use nusamai_gltf_json::*;

    println!("tr: {:?}", translation);

    let mut bin_content: Vec<u8> = Vec::new();

    let vertices_offset = bin_content.len();
    let mut buf = [0; 12];
    let mut vertices_count = 0;
    for v in vertices {
        LittleEndian::write_u32_into(&v, &mut buf);
        bin_content.write_all(&buf).unwrap();
        vertices_count += 1;
    }
    let vertices_len = bin_content.len() - vertices_offset;

    let indices_offset = bin_content.len();
    let mut indices_count = 0;
    for idx in indices {
        bin_content.write_all(&idx.to_le_bytes()).unwrap();
        indices_count += 1;
    }
    let indices_len = bin_content.len() - indices_offset;

    let gltf = Gltf {
        scenes: vec![Scene {
            nodes: Some(vec![0]),
            ..Default::default()
        }],
        nodes: vec![Node {
            mesh: Some(0),
            translation,
            ..Default::default()
        }],
        meshes: vec![Mesh {
            primitives: vec![MeshPrimitive {
                attributes: vec![("POSITION".to_string(), 0)].into_iter().collect(),
                indices: Some(1),
                mode: PrimitiveMode::Triangles,
                ..Default::default()
            }],
            ..Default::default()
        }],
        accessors: vec![
            Accessor {
                buffer_view: Some(0),
                component_type: ComponentType::Float,
                count: vertices_count,
                min: Some(min.to_vec()),
                max: Some(max.to_vec()),
                type_: AccessorType::Vec3,
                ..Default::default()
            },
            Accessor {
                buffer_view: Some(1),
                component_type: ComponentType::UnsignedInt,
                count: indices_count,
                type_: AccessorType::Scalar,
                ..Default::default()
            },
        ],
        buffer_views: vec![
            BufferView {
                byte_offset: vertices_offset as u32,
                byte_length: vertices_len as u32,
                target: Some(BufferViewTarget::ArrayBuffer),
                ..Default::default()
            },
            BufferView {
                byte_offset: indices_offset as u32,
                byte_length: indices_len as u32,
                target: Some(BufferViewTarget::ElementArrayBuffer),
                ..Default::default()
            },
        ],
        buffers: vec![Buffer {
            byte_length: bin_content.len() as u32,
            ..Default::default()
        }],
        ..Default::default()
    };

    {
        let mut json_content = serde_json::to_vec(&gltf).unwrap();

        // append padding
        json_content.extend(vec![0x20; (4 - (json_content.len() % 4)) % 4].iter());
        bin_content.extend(vec![0x0; (4 - (bin_content.len() % 4)) % 4].iter());

        let total_size = 12 + 8 + json_content.len() + 8 + bin_content.len();

        let mut file = std::fs::File::create(path).unwrap();
        let mut writer = BufWriter::new(&mut file);

        writer.write_all(b"glTF").unwrap(); // magic
        writer.write_all(&2u32.to_le_bytes()).unwrap(); // version: 2
        writer
            .write_all(&(total_size as u32).to_le_bytes())
            .unwrap(); // total size

        writer
            .write_all(&(json_content.len() as u32).to_le_bytes())
            .unwrap(); // json content
        writer.write_all(b"JSON").unwrap(); // chunk type
        writer.write_all(&json_content).unwrap(); // json content

        writer
            .write_all(&(bin_content.len() as u32).to_le_bytes())
            .unwrap(); // json content
        writer.write_all(b"BIN\0").unwrap(); // chunk type
        writer.write_all(&bin_content).unwrap(); // bin content

        writer.flush().unwrap();
    }
}
