//! ポリゴンをタイル状にスライスしてGeoJSONとして出力するデモ
//!
//! 使用例:
//!
//! ```bash
//! cargo run --example slice --release -- ~/path/to/PLATEAU/22203_numazu-shi_2021_citygml_4_op/udx/*/52385628_*_6697_op.gml
//! ````
//!
//! This example converts a CityGML file to GeoJSON and outputs it to a file

use clap::Parser;
use nusamai_citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use nusamai_geometry::{MultiPolygon3, Polygon3};
use nusamai_plateau::TopLevelCityObject;
use std::fs;
use std::io::BufRead;
use std::io::BufWriter;

use nusamai_geojson::conversion::{
    multilinestring_to_geojson_geometry, multipoint_to_geojson_geometry,
};

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    filename: String,
}

fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<Vec<TopLevelCityObject>, ParseError> {
    let mut cityobjs: Vec<TopLevelCityObject> = vec![];

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: nusamai_plateau::models::TopLevelCityObject = Default::default();
            cityobj.parse(st)?;
            let geometries = st.collect_geometries();

            if let Some(root) = cityobj.into_object() {
                let obj = TopLevelCityObject { root, geometries };
                cityobjs.push(obj);
            }

            Ok(())
        }
        b"gml:boundedBy" | b"app:appearanceMember" => {
            st.skip_current_element()?;
            Ok(())
        }
        other => Err(ParseError::SchemaViolation(format!(
            "Unrecognized element {}",
            String::from_utf8_lossy(other)
        ))),
    }) {
        Ok(_) => Ok(cityobjs),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}

// --------------------

fn slice_polygon(poly: &Polygon3, out: &mut MultiPolygon3) {
    const STEP: f64 = 5.0 / 1000.0;
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
    let size_x = ((max_x / STEP).ceil() - (min_x / STEP).floor()) as usize;
    let mut x_sliced_polys: Vec<Polygon3> = vec![Polygon3::new(); size_x];
    {
        for (i, x_sliced_poly) in x_sliced_polys.iter_mut().enumerate() {
            let k1 = (min_x / STEP + i as f64).floor() * STEP;
            let k2 = (min_x / STEP + (i + 1) as f64).floor() * STEP;

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
    for x_sliced_poly in &x_sliced_polys {
        let (min_y, max_y) = x_sliced_poly
            .exterior()
            .iter()
            .fold((f64::MAX, f64::MIN), |(min_y, max_y), c| {
                (min_y.min(c[1]), max_y.max(c[1]))
            });
        let size_y = ((max_y / STEP).ceil() - (min_y / STEP).floor()) as usize;

        for i in 0..size_y {
            let k1 = (min_y / STEP + i as f64).floor() * STEP;
            let k2 = (min_y / STEP + (i + 1) as f64).floor() * STEP;

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
pub fn toplevel_cityobj_to_geojson_features(obj: &TopLevelCityObject) -> Vec<geojson::Feature> {
    let mut geojson_features: Vec<geojson::Feature> = vec![];

    if !obj.geometries.multipolygon.is_empty() {
        // sliceする
        let mpolys = &obj.geometries.multipolygon;
        let mut new_mpoly = MultiPolygon3::new();
        mpolys.iter().for_each(|poly| {
            let mut new_poly = Polygon3::new();
            poly.rings().for_each(|r| {
                new_poly.add_ring(r.iter().map(|c| obj.geometries.vertices[c[0] as usize]))
            });
            slice_polygon(&new_poly, &mut new_mpoly);
        });

        println!("mpoly: {:?}", mpolys.len());
        println!("split mpoly: {:?}", new_mpoly.len());

        let mpoly = new_mpoly
            .iter()
            .map(|poly| {
                poly.rings()
                    .map(|c| c.iter().map(|v| vec![v[1], v[0], v[2]]).collect())
                    .collect::<Vec<_>>()
            })
            .collect();

        let mpoly_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(geojson::Geometry {
                bbox: None,
                value: geojson::Value::MultiPolygon(mpoly),
                foreign_members: None,
            }),
            id: None,
            properties: None,
            foreign_members: None,
        };
        geojson_features.push(mpoly_geojson_feat);
    }

    if !obj.geometries.multilinestring.is_empty() {
        let mls_geojson_geom = multilinestring_to_geojson_geometry(
            &obj.geometries.vertices,
            &obj.geometries.multilinestring,
        );
        let mls_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mls_geojson_geom),
            id: None,
            properties: None,
            foreign_members: None,
        };
        geojson_features.push(mls_geojson_feat);
    }

    if !obj.geometries.multipoint.is_empty() {
        let mpoint_geojson_geom =
            multipoint_to_geojson_geometry(&obj.geometries.vertices, &obj.geometries.multipoint);
        let mpoint_geojson_feat = geojson::Feature {
            bbox: None,
            geometry: Some(mpoint_geojson_geom),
            id: None,
            properties: None,
            foreign_members: None,
        };
        geojson_features.push(mpoint_geojson_feat);
    }

    geojson_features
}

fn main() {
    let args = Args::parse();

    let reader = std::io::BufReader::new(std::fs::File::open(args.filename).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);

    let context = nusamai_citygml::ParseContext::default();
    let cityobjs = match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(items) => items,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    let geojson_features: Vec<geojson::Feature> = cityobjs
        .iter()
        .flat_map(toplevel_cityobj_to_geojson_features)
        .collect();

    let geojson_feature_collection = geojson::FeatureCollection {
        bbox: None,
        features: geojson_features,
        foreign_members: None,
    };
    let geojson = geojson::GeoJson::from(geojson_feature_collection);

    let mut file = fs::File::create("out.geojson").unwrap();
    let mut writer = BufWriter::new(&mut file);
    serde_json::to_writer(&mut writer, &geojson).unwrap();
}
