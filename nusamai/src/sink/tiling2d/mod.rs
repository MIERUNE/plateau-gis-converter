//! 2D Tiling sink (タイリングの実験をするための一時的なSink)
//!
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use rayon::prelude::*;

use crate::get_parameter_value;
use crate::parameters::*;
use crate::pipeline::{Feedback, Receiver};
use crate::sink::{DataSink, DataSinkProvider, SinkInfo};

use nusamai_citygml::object::CityObject;
use nusamai_geojson::conversion::multipolygon_to_geometry;
use nusamai_geometry::{MultiPolygon3, Polygon3};
use nusamai_mvt::webmercator::{lnglat_to_web_mercator, web_mercator_to_lnglat};

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

impl DataSink for Tiling2DSink {
    fn run(&mut self, upstream: Receiver, feedback: &mut Feedback) {
        let (sender, receiver) = std::sync::mpsc::sync_channel(100);

        rayon::join(
            || {
                // Convert CityObjects to Slicing objects

                let _ = upstream.into_iter().par_bridge().try_for_each_with(
                    sender,
                    |sender, parcel| {
                        if feedback.is_cancelled() {
                            return Err(());
                        }

                        let features = toplevel_cityobj_to_geojson_features(&parcel.cityobj);

                        for feature in features {
                            let Ok(bytes) = serde_json::to_vec(&feature) else {
                                // TODO: fatal error
                                return Err(());
                            };
                            if sender.send(bytes).is_err() {
                                println!("sink cancelled");
                                return Err(());
                            };
                        }
                        Ok(())
                    },
                );
            },
            || {
                // Write Slicing to a file

                // TODO: Handle output file path
                let mut file = File::create(&self.output_path).unwrap();
                let mut writer = BufWriter::new(&mut file);

                // Write the FeatureCollection header
                writer
                    .write_all(b"{\"type\":\"FeatureCollection\",\"features\":[")
                    .unwrap();

                // Write each Feature
                let mut iter = receiver.into_iter().peekable();
                while let Some(bytes) = iter.next() {
                    writer.write_all(&bytes).unwrap();
                    if iter.peek().is_some() {
                        writer.write_all(b",").unwrap();
                    };
                }

                // Write the FeautureCollection footer and EOL
                writer.write_all(b"]}\n").unwrap();
            },
        );
    }
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

fn slice_polygon(zoom: u32, poly: &Polygon3, out: &mut MultiPolygon3) {
    if poly.exterior().is_empty() {
        return;
    }

    // Slice along X-axis
    let (min_x, max_x) = poly
        .exterior()
        .iter()
        .fold((f64::MAX, f64::MIN), |(min_x, max_x), c| {
            (min_x.min(c[0]), max_x.max(c[0]))
        });
    let size_x = (max_x.ceil() - min_x.floor()) as usize;

    let mut x_sliced_polys: Vec<(u32, Polygon3)> = vec![Default::default(); size_x];
    {
        for (i, (xi, x_sliced_poly)) in x_sliced_polys.iter_mut().enumerate() {
            let k1 = (min_x + i as f64).floor();
            let k2 = (min_x + (i + 1) as f64).floor();
            *xi = k1 as u32;

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
                                let z = (b[2] - a[2]) * (k1 - a[0]) / (b[0] - a[0]) + a[2];
                                new_ring.extend([k1, y, z])
                            }
                        } else if a[0] > k2 {
                            if b[0] < k2 {
                                let y = (b[1] - a[1]) * (k2 - a[0]) / (b[0] - a[0]) + a[1];
                                let z = (b[2] - a[2]) * (k2 - a[0]) / (b[0] - a[0]) + a[2];
                                new_ring.extend([k2, y, z])
                            }
                        } else {
                            new_ring.extend(a)
                        }

                        if b[0] < k1 && a[0] >= k1 {
                            let y = (b[1] - a[1]) * (k1 - a[0]) / (b[0] - a[0]) + a[1];
                            let z = (b[2] - a[2]) * (k1 - a[0]) / (b[0] - a[0]) + a[2];
                            new_ring.extend([k1, y, z])
                        } else if b[0] > k2 && a[0] <= k2 {
                            let y = (b[1] - a[1]) * (k2 - a[0]) / (b[0] - a[0]) + a[1];
                            let z = (b[2] - a[2]) * (k2 - a[0]) / (b[0] - a[0]) + a[2];
                            new_ring.extend([k2, y, z])
                        }

                        Some(b)
                    })
                    .unwrap();

                if k1 <= last_a[0] && last_a[0] <= k2 {
                    new_ring.extend(last_a)
                }

                x_sliced_poly.add_ring(new_ring.chunks_exact(3).map(|c| [c[0], c[1], c[2]]));
            }
        }
    }

    // Slice along Y-axis
    for (xi, x_sliced_poly) in &x_sliced_polys {
        let (min_y, max_y) = x_sliced_poly
            .exterior()
            .iter()
            .fold((f64::MAX, f64::MIN), |(min_y, max_y), c| {
                (min_y.min(c[1]), max_y.max(c[1]))
            });
        let size_y = (max_y.ceil() - min_y.floor()) as usize;

        for i in 0..size_y {
            let k1 = (min_y + i as f64).floor();
            let k2 = (min_y + (i + 1) as f64).floor();
            let yi = k1 as u32;

            // todo?: check interior bbox to optimize

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
                                let z = (b[2] - a[2]) * (k1 - a[1]) / (b[1] - a[1]) + a[2];
                                new_ring.extend([x, k1, z])
                            }
                        } else if a[1] > k2 {
                            if b[1] < k2 {
                                let x = (b[0] - a[0]) * (k2 - a[1]) / (b[1] - a[1]) + a[0];
                                let z = (b[2] - a[2]) * (k2 - a[1]) / (b[1] - a[1]) + a[2];
                                new_ring.extend([x, k2, z])
                            }
                        } else {
                            new_ring.extend(a)
                        }

                        if b[1] < k1 && a[1] >= k1 {
                            let x = (b[0] - a[0]) * (k1 - a[1]) / (b[1] - a[1]) + a[0];
                            let z = (b[2] - a[2]) * (k1 - a[1]) / (b[1] - a[1]) + a[2];
                            new_ring.extend([x, k1, z])
                        } else if b[1] > k2 && a[1] <= k2 {
                            let x = (b[0] - a[0]) * (k2 - a[1]) / (b[1] - a[1]) + a[0];
                            let z = (b[2] - a[2]) * (k2 - a[1]) / (b[1] - a[1]) + a[2];
                            new_ring.extend([x, k2, z])
                        }

                        Some(b)
                    })
                    .unwrap();

                if k1 <= last_a[1] && last_a[1] <= k2 {
                    new_ring.extend(last_a)
                }

                println!("{}/{}/{}", zoom, xi, yi);

                let iter = new_ring.chunks_exact(3).map(|c| [c[0], c[1], c[2]]);
                match ri {
                    0 => out.add_exterior(iter),
                    _ => out.add_interior(iter),
                };
            }
        }
    }
}

/// Create GeoJSON features from a TopLevelCityObject
/// Each feature for MultiPolygon, MultiLineString, and MultiPoint will be created (if it exists)
// TODO: Handle properties (`obj.root` -> `geojson::Feature.properties`)
// TODO: We may want to traverse the tree and create features for each semantic child in the future
pub fn toplevel_cityobj_to_geojson_features(obj: &CityObject) -> Vec<geojson::Feature> {
    let mut geojson_features: Vec<geojson::Feature> = vec![];
    let properties = extract_properties(&obj.root);

    if !obj.geometries.multipolygon.is_empty() {
        // sliceする
        let mpolys = &obj.geometries.multipolygon;
        let mut new_mpoly = MultiPolygon3::new();

        let zoom = 13;
        let zoom_scale = 2i32.pow(zoom) as f64;
        mpolys.iter().for_each(|poly| {
            let mut new_poly = Polygon3::new();
            poly.rings().for_each(|ring| {
                new_poly.add_ring(ring.iter().map(|c| {
                    let [lng, lat, height] = obj.geometries.vertices[c[0] as usize];
                    let (mx, my) = lnglat_to_web_mercator(lng, lat);
                    [mx * zoom_scale, my * zoom_scale, height]
                }))
            });
            slice_polygon(zoom, &new_poly, &mut new_mpoly);
        });

        new_mpoly.transform_inplace(|c| {
            let (mx, my, height) = (c[0] / zoom_scale, c[1] / zoom_scale, c[2]);
            let (lng, lat) = web_mercator_to_lnglat(mx, my);
            [lng, lat, height]
        });

        let geometry = multipolygon_to_geometry(&new_mpoly);
        geojson_features.push(geojson::Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: properties.clone(),
            foreign_members: None,
        });
    }

    // NOTE: Not supported (yet)
    // if !obj.geometries.multilinestring.is_empty() {
    //     let mls_geojson_geom = multilinestring_to_geojson_geometry(
    //         &obj.geometries.vertices,
    //         &obj.geometries.multilinestring,
    //     );
    //     let mls_geojson_feat = geojson::Feature {
    //         bbox: None,
    //         geometry: Some(mls_geojson_geom),
    //         id: None,
    //         properties: properties.clone(),
    //         foreign_members: None,
    //     };
    //     geojson_features.push(mls_geojson_feat);
    // }

    // if !obj.geometries.multipoint.is_empty() {
    //     let mpoint_geojson_geom =
    //         multipoint_to_geojson_geometry(&obj.geometries.vertices, &obj.geometries.multipoint);
    //     let mpoint_geojson_feat = geojson::Feature {
    //         bbox: None,
    //         geometry: Some(mpoint_geojson_geom),
    //         id: None,
    //         properties,
    //         foreign_members: None,
    //     };
    //     geojson_features.push(mpoint_geojson_feat);
    // }

    geojson_features
}
