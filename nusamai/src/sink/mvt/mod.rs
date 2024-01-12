//! Mapbox Vector Tile (MVT) sink

use std::fs;
use std::path;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use ext_sort::{buffer::mem::MemoryLimitedBufferBuilder, ExternalSorter, ExternalSorterBuilder};
use hashbrown::HashMap;
use itertools::Itertools;
use nusamai_mvt::geometry::GeometryEncoder;
use prost::Message;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use nusamai_citygml::object::CityObject;
use nusamai_geometry::{MultiPolygon, Polygon2};
use nusamai_mvt::{
    tileid::TileIdMethod, vector_tile, webmercator::lnglat_to_web_mercator, TileZXY,
};

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

pub struct MVTSinkProvider {}

impl DataSinkProvider for MVTSinkProvider {
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

        Box::<MVTSink>::new(MVTSink {
            output_path: output_path.unwrap().into(),
        })
    }
}

struct MVTSink {
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
    geometry: MultiPolygon<'a, 2, i16>,
    properties: nusamai_citygml::object::Value,
}

impl DataSink for MVTSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
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

            // Group sorted features and write them into MVT tiles
            {
                let feedback = feedback.clone();
                let output_path = &self.output_path;
                s.spawn(move || {
                    // Run in a separate thread pool to avoid deadlocks
                    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
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
    // Convert CityObjects to Sliced features
    let _ = upstream.into_iter().par_bridge().try_for_each(|parcel| {
        if feedback.is_cancelled() {
            return Err(());
        }

        slice_cityobj_geoms(&parcel.cityobj, 7, 15, |(z, x, y), mpoly| {
            let feature = SlicedFeature {
                geometry: mpoly,
                properties: parcel.cityobj.root.clone(),
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
        // TODO: Use Binpack instead of RMP ?
        // TODO: Implement an external sorter by ourselves?
    > = ExternalSorterBuilder::new()
        .with_tmp_dir(path::Path::new("./"))
        .with_buffer(MemoryLimitedBufferBuilder::new(200 * 1024 * 1024)) // TODO
        .with_threads_number(12) // TODO
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
    let _ = receiver_sorted
        .into_iter()
        .par_bridge()
        .try_for_each(|(tile_id, sfeats)| {
            if feedback.is_cancelled() {
                return Err(());
            }
            let (zoom, x, y) = tile_id_conv.id_to_zxy(tile_id);
            let extent = 4096;
            let mut features = Vec::new();

            for sfeat in sfeats {
                let feat: SlicedFeature = bincode::deserialize(&sfeat.body).unwrap();
                let mpoly = feat.geometry;

                // encode geometry
                let mut geom_enc = GeometryEncoder::new();
                for poly in &mpoly {
                    let exterior = poly.exterior();
                    if poly.exterior().is_ccw() {
                        geom_enc.add_ring(exterior.into_iter().map(|c| [c[0], c[1]]));
                    }
                }

                let geometry = geom_enc.into_vec();
                if !geometry.is_empty() {
                    features.push(vector_tile::tile::Feature {
                        id: None,
                        tags: vec![],
                        r#type: Some(vector_tile::tile::GeomType::Polygon as i32),
                        geometry,
                    });
                }
            }

            // skip if no features
            if features.is_empty() {
                return Ok(());
            }

            let layer = vector_tile::tile::Layer {
                version: 2,
                name: "dummy-layer".to_string(),
                features,
                keys: vec![],
                values: vec![],
                extent: Some(extent),
            };
            let tile = vector_tile::Tile {
                layers: vec![layer],
            };

            let path = output_path.join(path::Path::new(&format!("{}/{}/{}.pbf", zoom, x, y)));

            if let Some(dir) = path.parent() {
                if let Err(e) = fs::create_dir_all(dir) {
                    panic!("Fatal error: {:?}", e); // FIXME
                }
            }
            let bytes = tile.encode_to_vec();
            fs::write(&path, &bytes).unwrap();

            log::info!(
                "Wrote a tile: {:?} ({:?} KB)",
                &path,
                bytesize::to_string(bytes.len() as u64, true)
            );

            Ok(())
        });
}

fn slice_cityobj_geoms(
    obj: &CityObject,
    min_z: u8,
    max_z: u8,
    f: impl Fn(TileZXY, MultiPolygon<2, i16>) -> Result<(), ()>,
) -> Result<(), ()> {
    assert!(max_z > min_z, "max_z must be greater than min_z");

    if obj.geometries.multipolygon.is_empty() {
        return Ok(());
    }

    let idx_mpoly = &obj.geometries.multipolygon;
    let mut tiled_mpolys = HashMap::new();

    let extent = 4096;
    idx_mpoly.iter().for_each(|idx_poly| {
        let poly = idx_poly.transform(|c| {
            let [lng, lat, _height] = obj.geometries.vertices[c[0] as usize];
            let (mx, my) = lnglat_to_web_mercator(lng, lat);
            [mx, my]
        });

        // Accept only front-facing polygons
        if !poly.exterior().is_cw() {
            return;
        }
        debug_assert!(poly.exterior().ring_area() > 0.0);

        for zoom in min_z..=max_z {
            let zoom_scale = 2i32.pow(zoom as u32) as f64;
            let scaled_poly = poly.transform(|c| [c[0] * zoom_scale, c[1] * zoom_scale]);
            slice_polygon(zoom, extent, 80, &scaled_poly, &mut tiled_mpolys);
        }
    });

    for ((z, x, y), mpoly) in tiled_mpolys {
        if mpoly.iter().any(|poly| poly.area() > 0.0) {
            f((z, x, y), mpoly)?;
        }
    }

    Ok(())
    // TODO: linestring, point
}

// fn extract_properties(tree: &nusamai_citygml::object::Value) -> Option<geojson::JsonObject> {
//     match &tree {
//         feat @ nusamai_citygml::Value::Feature(_) => match feat.to_attribute_json() {
//             serde_json::Value::Object(map) => Some(map),
//             _ => unreachable!(),
//         },
//         _ => panic!("Root value type must be Feature, but found {:?}", tree),
//     }
// }

fn slice_polygon(
    zoom: u8,
    extent: u32,
    buffer: u32,
    poly: &Polygon2,
    out: &mut HashMap<(u8, u32, u32), MultiPolygon<2, i16>>,
) {
    if poly.exterior().is_empty() {
        return;
    }

    let buf_width = buffer as f64 / extent as f64;

    // Slice along X-axis
    let x_range = {
        let (min_x, max_x) = poly
            .exterior()
            .iter()
            .fold((f64::MAX, f64::MIN), |(min_x, max_x), c| {
                (min_x.min(c[0]), max_x.max(c[0]))
            });
        min_x.floor() as u32..max_x.ceil() as u32
    };

    let mut x_sliced_polys = Vec::with_capacity(x_range.len());

    for xi in x_range.clone() {
        let k1 = xi as f64 - buf_width;
        let k2 = (xi + 1) as f64 + buf_width;
        let mut x_sliced_poly = Polygon2::new();

        // todo?: check interior bbox to optimize

        for mut ring in poly.rings() {
            if ring.coords().is_empty() {
                continue;
            }
            ring.reverse_ring_inplace();
            let mut new_ring = Vec::with_capacity(ring.coords().len());

            let last_a = ring
                .iter_closed()
                .fold(None, |a, b| {
                    let Some(a) = a else { return Some(b) };

                    if a[0] < k1 {
                        if b[0] > k1 {
                            let y = (b[1] - a[1]) * (k1 - a[0]) / (b[0] - a[0]) + a[1];
                            // let z = (b[2] - a[2]) * (k1 - a[0]) / (b[0] - a[0]) + a[2];
                            new_ring.extend([k1, y])
                        }
                    } else if a[0] > k2 {
                        if b[0] < k2 {
                            let y = (b[1] - a[1]) * (k2 - a[0]) / (b[0] - a[0]) + a[1];
                            // let z = (b[2] - a[2]) * (k2 - a[0]) / (b[0] - a[0]) + a[2];
                            new_ring.extend([k2, y])
                        }
                    } else {
                        new_ring.extend(a)
                    }

                    if b[0] < k1 && a[0] >= k1 {
                        let y = (b[1] - a[1]) * (k1 - a[0]) / (b[0] - a[0]) + a[1];
                        // let z = (b[2] - a[2]) * (k1 - a[0]) / (b[0] - a[0]) + a[2];
                        new_ring.extend([k1, y])
                    } else if b[0] > k2 && a[0] <= k2 {
                        let y = (b[1] - a[1]) * (k2 - a[0]) / (b[0] - a[0]) + a[1];
                        // let z = (b[2] - a[2]) * (k2 - a[0]) / (b[0] - a[0]) + a[2];
                        new_ring.extend([k2, y])
                    }

                    Some(b)
                })
                .unwrap();

            if k1 <= last_a[0] && last_a[0] <= k2 {
                new_ring.extend(last_a)
            }

            assert!(new_ring[0] == new_ring[new_ring.len() - 2]);
            assert!(new_ring[1] == new_ring[new_ring.len() - 1]);

            x_sliced_poly.add_ring(new_ring.chunks_exact(2).map(|c| [c[0], c[1]]));
        }

        x_sliced_polys.push(x_sliced_poly);
    }

    // Slice along Y-axis
    for (xi, x_sliced_poly) in x_range.zip(x_sliced_polys.iter()) {
        let y_range = {
            let (min_y, max_y) = x_sliced_poly
                .exterior()
                .iter()
                .fold((f64::MAX, f64::MIN), |(min_y, max_y), c| {
                    (min_y.min(c[1]), max_y.max(c[1]))
                });
            min_y.floor() as u32..max_y.ceil() as u32
        };

        for yi in y_range {
            let k1 = yi as f64 - buf_width;
            let k2 = (yi + 1) as f64 + buf_width;

            // todo?: check interior bbox to optimize

            let tile_mpoly = out
                .entry((zoom, xi, yi))
                .or_insert_with(MultiPolygon::<2, i16>::new);

            for (ri, ring) in x_sliced_poly.rings().enumerate() {
                if ring.coords().is_empty() {
                    continue;
                }

                let mut new_ring = Vec::with_capacity(ring.coords().len());

                let last_a = ring
                    .iter_closed()
                    .fold(None, |a, b| {
                        let Some(a) = a else { return Some(b) };

                        if a[1] < k1 {
                            if b[1] > k1 {
                                let x = (b[0] - a[0]) * (k1 - a[1]) / (b[1] - a[1]) + a[0];
                                // let z = (b[2] - a[2]) * (k1 - a[1]) / (b[1] - a[1]) + a[2];
                                new_ring.extend([x, k1])
                            }
                        } else if a[1] > k2 {
                            if b[1] < k2 {
                                let x = (b[0] - a[0]) * (k2 - a[1]) / (b[1] - a[1]) + a[0];
                                // let z = (b[2] - a[2]) * (k2 - a[1]) / (b[1] - a[1]) + a[2];
                                new_ring.extend([x, k2])
                            }
                        } else {
                            new_ring.extend(a)
                        }

                        if b[1] < k1 && a[1] >= k1 {
                            let x = (b[0] - a[0]) * (k1 - a[1]) / (b[1] - a[1]) + a[0];
                            // let z = (b[2] - a[2]) * (k1 - a[1]) / (b[1] - a[1]) + a[2];
                            new_ring.extend([x, k1])
                        } else if b[1] > k2 && a[1] <= k2 {
                            let x = (b[0] - a[0]) * (k2 - a[1]) / (b[1] - a[1]) + a[0];
                            // let z = (b[2] - a[2]) * (k2 - a[1]) / (b[1] - a[1]) + a[2];
                            new_ring.extend([x, k2])
                        }

                        Some(b)
                    })
                    .unwrap();

                if k1 <= last_a[1] && last_a[1] <= k2 {
                    new_ring.extend(last_a)
                }

                let iter = new_ring.chunks_exact(2).map(|c| {
                    let (x, y) = (c[0], c[1]);
                    let tx = (((x - xi as f64) * (extent as f64)) + 0.5) as i16;
                    let ty = (((y - yi as f64) * (extent as f64)) + 0.5) as i16;
                    [tx, ty]
                });

                match ri {
                    0 => tile_mpoly.add_exterior(iter),
                    _ => tile_mpoly.add_interior(iter),
                };
            }
        }
    }
}
