//! 2D Tiling sink (タイリングの実験をするための一時的なSink)
//!
use std::path;
use std::path::PathBuf;

use ext_sort::{buffer::mem::MemoryLimitedBufferBuilder, ExternalSorter, ExternalSorterBuilder};
use hashbrown::HashMap;
use itertools::Itertools;
use prost::Message;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use nusamai_citygml::object::CityObject;
use nusamai_geometry::{MultiPolygon, Polygon2};
use nusamai_mvt::tileid::TileIdMethod;
use nusamai_mvt::vector_tile;
use nusamai_mvt::webmercator::lnglat_to_web_mercator;

pub struct Tiling2DSinkProvider {}

impl DataSinkProvider for Tiling2DSinkProvider {
    fn info(&self) -> SinkInfo {
        SinkInfo {
            name: "GeoJSON".to_string(),
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

        Box::<Tiling2DSink>::new(Tiling2DSink {
            output_path: output_path.unwrap().into(),
        })
    }
}

pub struct Tiling2DSink {
    output_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, deepsize::DeepSizeOf)]
struct SerializedSlicedFeature {
    tile_id: u64,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

struct SlicedFeature {
    geometry: MultiPolygon<'static, 2, i16>,
    properties: nusamai_citygml::object::Value,
}

type TileZXY = (u8, u32, u32);

const GEOM_COMMAND_MOVE_TO: u32 = 1;
const GEOM_COMMAND_LINE_TO: u32 = 2;
const GEOM_COMMAND_CLOSE_PATH: u32 = 7;

const GEOM_COMMAND_MOVE_TO_WITH_COUNT1: u32 = 1 << 3 | GEOM_COMMAND_MOVE_TO;
const GEOM_COMMAND_CLOSE_PATH_WITH_COUNT1: u32 = 1 << 3 | GEOM_COMMAND_CLOSE_PATH;

// Pipeline
//
// ---
// Single: upstream
// Multiple: Feature -> Sliced Features
// Single: downstream
//   V
//   V
// Single: upstream
// Multiple: Feature -> Sliced Features
// Single: downstream
// ---
//

impl DataSink for Tiling2DSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(100);
        let (sender2, receiver2) = std::sync::mpsc::sync_channel(100);

        let tileid_conv = TileIdMethod::Hilbert;

        // TODO: refactoring

        std::thread::scope(|s| {
            // Splitting geometry along the tile boundaries
            let feedback2 = feedback.clone();
            s.spawn(move || {
                let feedback = feedback2;
                // Convert CityObjects to Sliced features
                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    (sender, Vec::new()),
                    |(sender, buf), parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        buf.clear();
                        cityobj_to_tiled_features(&parcel.cityobj, buf);

                        for ((z, x, y), feature) in buf {
                            let bytes = bincode::serialize(feature).unwrap();
                            let sfeat = SerializedSlicedFeature {
                                tile_id: tileid_conv.zxy_to_id(*z, *x, *y),
                                body: bytes,
                            };

                            if sender.send(sfeat).is_err() {
                                log::info!("sink cancelled");
                                return Err(());
                            };
                        }
                        Ok(())
                    },
                );
            });

            // Sort features by tile_id (using external sorter)
            s.spawn(move || {
                let sorter: ExternalSorter<
                    SerializedSlicedFeature,
                    std::io::Error,
                    MemoryLimitedBufferBuilder,
                    // TODO: Use Binpack instead of RMP ?
                > = ExternalSorterBuilder::new()
                    .with_tmp_dir(path::Path::new("./"))
                    .with_buffer(MemoryLimitedBufferBuilder::new(150 * 1024 * 1024)) // TODO
                    .with_threads_number(8) // TODO
                    .build()
                    .unwrap();

                let sorted = sorter
                    .sort_by(receiver.into_iter().map(Ok), |a, b| {
                        a.tile_id.cmp(&b.tile_id)
                    })
                    .unwrap();

                for (tile_id, sfeats) in &sorted.map(Result::unwrap).group_by(|sfeat| sfeat.tile_id)
                {
                    let sfeats: Vec<_> = sfeats.collect();
                    if sender2.send((tile_id, sfeats)).is_err() {
                        log::info!("sink cancelled?");
                        return;
                    };
                }
            });

            // Tile writer
            let feedback2 = feedback.clone();
            s.spawn(move || {
                let feedback = feedback2;
                // Run in a separate thread pool to avoid deadlocks
                let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
                pool.install(|| {
                    let _ = receiver2
                        .into_iter()
                        .par_bridge()
                        .try_for_each(|(tile_id, sfeats)| {
                            if feedback.is_cancelled() {
                                return Err(());
                            }
                            let (zoom, x, y) = tileid_conv.id_to_zxy(tile_id);
                            let extent = 4096;
                            let mut features = Vec::new();

                            for sfeat in sfeats {
                                let feat: SlicedFeature =
                                    bincode::deserialize(&sfeat.body).unwrap();

                                let mpoly = feat.geometry;
                                let mut prev_x = 0;
                                let mut prev_y = 0;

                                // encode geometry
                                // TODO: Refactor this as GeometryEncoder.
                                let mut encoded_geom: Vec<u32> = Vec::new();
                                for poly in &mpoly {
                                    let exterior = poly.exterior();
                                    if exterior.ring_area() == 0.0 || poly.exterior().is_ccw() {
                                        continue;
                                    }

                                    let mut iter = exterior.into_iter();
                                    let &[first_x, first_y] = iter.next().unwrap() else {
                                        unreachable!("polygon must be 2D");
                                    };
                                    let dx = (first_x - prev_x) as i32;
                                    let dy = (first_y - prev_y) as i32;
                                    (prev_x, prev_y) = (first_x, first_y);

                                    // move to
                                    encoded_geom.push(GEOM_COMMAND_MOVE_TO_WITH_COUNT1);
                                    encoded_geom.push(((dx << 1) ^ (dx >> 31)) as u32);
                                    encoded_geom.push(((dy << 1) ^ (dy >> 31)) as u32);

                                    // line to
                                    encoded_geom.push(GEOM_COMMAND_LINE_TO); // length will be updated later
                                    let lineto_cmd_pos = encoded_geom.len();
                                    let mut count = 0;
                                    for coord in iter {
                                        let &[x, y] = coord else {
                                            unreachable!("polygon must be 2D")
                                        };
                                        let dx = (x - prev_x) as i32;
                                        let dy = (y - prev_y) as i32;
                                        (prev_x, prev_y) = (x, y);

                                        if dx != 0 || dy != 0 {
                                            encoded_geom.push(((dx << 1) ^ (dx >> 31)) as u32);
                                            encoded_geom.push(((dy << 1) ^ (dy >> 31)) as u32);
                                            count += 1;
                                        }
                                    }
                                    assert!(count >= 2);
                                    encoded_geom[lineto_cmd_pos] =
                                        GEOM_COMMAND_LINE_TO | count << 3;

                                    // close path
                                    encoded_geom.push(GEOM_COMMAND_CLOSE_PATH_WITH_COUNT1);
                                }

                                if !encoded_geom.is_empty() {
                                    let encoded_geom = vec![];

                                    features.push(vector_tile::tile::Feature {
                                        id: None,
                                        tags: vec![],
                                        r#type: Some(vector_tile::tile::GeomType::Polygon as i32),
                                        geometry: encoded_geom,
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
                                features: features,
                                keys: vec![],
                                values: vec![],
                                extent: Some(extent),
                            };
                            let tile = vector_tile::Tile {
                                layers: vec![layer],
                            };

                            let path = self
                                .output_path
                                .join(path::Path::new(&format!("{}/{}/{}.pbf", zoom, x, y)));
                            log::info!("Writing a tile: {:?}", path);

                            if let Some(dir) = path.parent() {
                                if let Err(e) = fs::create_dir_all(dir) {
                                    panic!("Fatal error: {:?}", e); // FIXME
                                }
                            }

                            fs::write(path, tile.encode_to_vec()).unwrap();
                            Ok(())
                        });
                });
            });
        });
    }
}

fn cityobj_to_tiled_features(obj: &CityObject, out: &mut Vec<(TileZXY, SlicedFeature)>) {
    // let mut geojson_features: Vec<geojson::Feature> = vec![];
    // let properties = extract_properties(&obj.root);

    if !obj.geometries.multipolygon.is_empty() {
        // sliceする
        let mpolys = &obj.geometries.multipolygon;

        let mut tiled_mpolys = HashMap::new();

        let extent = 4096;
        let zoom = 14;
        let zoom_scale = 2i32.pow(zoom) as f64;
        mpolys.iter().for_each(|poly| {
            let mut new_poly = Polygon2::new();
            poly.rings().for_each(|ring| {
                new_poly.add_ring(ring.iter().map(|c| {
                    let [lng, lat, _height] = obj.geometries.vertices[c[0] as usize];
                    let (mx, my) = lnglat_to_web_mercator(lng, lat);
                    [mx * zoom_scale, my * zoom_scale]
                }))
            });
            slice_polygon(zoom as u8, extent, 80, &new_poly, &mut tiled_mpolys);
        });

        for ((z, x, y), mpoly) in tiled_mpolys {
            out.push((
                (z, x, y),
                SlicedFeature {
                    geometry: mpoly,
                    properties: obj.root.clone(),
                },
            ));
        }
    }
    // TODO: linestring, point
}

fn extract_properties(tree: &nusamai_citygml::object::Value) -> Option<geojson::JsonObject> {
    match &tree {
        feat @ nusamai_citygml::Value::Feature(_) => match feat.to_attribute_json() {
            serde_json::Value::Object(map) => Some(map),
            _ => unreachable!(),
        },
        _ => panic!("Root value type must be Feature, but found {:?}", tree),
    }
}

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

        for ring in poly.rings() {
            if ring.coords().is_empty() {
                continue;
            }
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
                    let x = c[0];
                    let y = c[1];
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
