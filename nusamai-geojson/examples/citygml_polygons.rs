//! CityGMLファイル (.gml) からポリゴンを読み込んで .geojson 形式で出力するデモ   
//! nusamai-geometry/examples/citygml_polygons.rs を元にしています。
//!
//! 使用例:
//!
//! ```bash
//! cargo run --example citygml_polygons --release -- ~/path/to/PLATEAU/22203_numazu-shi_2021_citygml_4_op/udx/*/52385628_*_6697_op.gml
//! ````
//!
//! このXMLのパース方法は本格的なパーザで使うことを意図していません。

use clap::Parser;
use nusamai_geojson::{geojson_geometry_to_feature, nusamai_to_geojson_geometry};
use nusamai_geometry::{Geometry, MultiPolygon3};
use quick_xml::{
    events::Event,
    name::{Namespace, ResolveResult::Bound},
    reader::NsReader,
};

use std::fs;
use std::io::Write;
use thiserror::Error;

const GML_NS: Namespace = Namespace(b"http://www.opengis.net/gml");
const BUILDING_NS: Namespace = Namespace(b"http://www.opengis.net/citygml/building/2.0");
const CITYFURNITURE_NS: Namespace = Namespace(b"http://www.opengis.net/citygml/cityfurniture/2.0");
const TRANSPORTATION_NS: Namespace =
    Namespace(b"http://www.opengis.net/citygml/transportation/2.0");
const BRIDGE_NS: Namespace = Namespace(b"http://www.opengis.net/citygml/bridge/2.0");
const VEGETATION_NS: Namespace = Namespace(b"http://www.opengis.net/citygml/vegetation/2.0");

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("XML error: {0}")]
    XmlError(quick_xml::Error),
}

fn parse_polygon(
    reader: &mut NsReader<&[u8]>,
    mpoly: &mut MultiPolygon3,
    buf: &mut Vec<f64>,
) -> Result<(), ParseError> {
    let mut is_interior = false;
    let mut in_poslist = false;
    loop {
        match reader.read_resolved_event() {
            Ok((Bound(GML_NS), Event::Start(e))) => match e.local_name().as_ref() {
                b"posList" => in_poslist = true,
                b"exterior" => is_interior = false,
                b"interior" => is_interior = true,
                _ => (),
            },
            Ok((Bound(GML_NS), Event::End(e))) => match e.local_name().as_ref() {
                b"Polygon" => return Ok(()),
                b"posList" => in_poslist = false,
                _ => (),
            },
            Ok((_, Event::Text(e))) => {
                if !in_poslist {
                    continue;
                }
                let text = e.unescape().unwrap();
                buf.clear();
                buf.extend(
                    text.split_ascii_whitespace()
                        .map(|v| v.parse::<f64>().unwrap()),
                );
                if is_interior {
                    mpoly.add_interior(buf.chunks_exact(3).map(|c| [c[0], c[1], c[2]]));
                } else {
                    mpoly.add_exterior(buf.chunks_exact(3).map(|c| [c[0], c[1], c[2]]));
                }
            }
            Ok(_) => (),
            Err(e) => return Err(ParseError::XmlError(e)),
        }
    }
}

fn parse_lod_geometry(
    reader: &mut NsReader<&[u8]>,
    mpoly: &mut MultiPolygon3,
    buf: &mut Vec<f64>,
) -> Result<(), ParseError> {
    let mut depth = 0;
    loop {
        match reader.read_resolved_event() {
            Ok((Bound(GML_NS), Event::Start(e))) => match e.local_name().as_ref() {
                b"Polygon" => parse_polygon(reader, mpoly, buf)?,
                _ => depth += 1,
            },
            Ok((_, Event::Start(_))) => depth += 1,
            Ok((_, Event::End(_))) => match depth {
                0 => return Ok(()),
                _ => depth -= 1,
            },
            Ok(_) => (),
            Err(e) => return Err(ParseError::XmlError(e)),
        }
    }
}

fn parse_cityobj(
    reader: &mut NsReader<&[u8]>,
    buf: &mut Vec<f64>,
) -> Result<MultiPolygon3<'static>, ParseError> {
    let mut mpoly = MultiPolygon3::new();
    let mut depth = 0;
    let mut max_lod = 0;
    loop {
        let ev = reader.read_resolved_event();
        match ev {
            Ok((
                Bound(
                    BUILDING_NS | CITYFURNITURE_NS | TRANSPORTATION_NS | VEGETATION_NS | BRIDGE_NS,
                ),
                Event::Start(e),
            )) => match e.local_name().as_ref() {
                b"lod4Geometry" | b"lod4MultiSurface" => {
                    if max_lod < 4 {
                        max_lod = 4;
                        mpoly.clear();
                    }
                    if max_lod == 4 {
                        parse_lod_geometry(reader, &mut mpoly, buf)?;
                    } else {
                        depth += 1;
                    }
                }
                b"lod3Geometry" | b"lod3MultiSurface" => {
                    if max_lod < 3 {
                        max_lod = 3;
                        mpoly.clear();
                    }
                    if max_lod == 3 {
                        parse_lod_geometry(reader, &mut mpoly, buf)?;
                    } else {
                        depth += 1;
                    }
                }
                b"lod2Geometry" | b"lod2MultiSurface" => {
                    if max_lod < 2 {
                        max_lod = 2;
                        mpoly.clear();
                    }
                    if max_lod == 2 {
                        parse_lod_geometry(reader, &mut mpoly, buf)?;
                    } else {
                        depth += 1;
                    }
                }
                b"lod1Solid" | b"lod1MultiSurface" => {
                    if max_lod < 1 {
                        max_lod = 1;
                        mpoly.clear();
                    }
                    if max_lod == 1 {
                        parse_lod_geometry(reader, &mut mpoly, buf)?;
                    } else {
                        depth += 1;
                    }
                }
                _ => depth += 1,
            },
            Ok((_, Event::Start(_))) => depth += 1,
            Ok((_, Event::End(_))) => match depth {
                0 => return Ok(mpoly),
                _ => depth -= 1,
            },
            Ok(_) => (),
            Err(e) => return Err(ParseError::XmlError(e)),
        }
    }
}

fn parse_body(reader: &mut NsReader<&[u8]>) -> Result<Vec<MultiPolygon3<'static>>, ParseError> {
    let mut mpolys: Vec<MultiPolygon3> = Vec::new();
    let mut buf: Vec<f64> = Vec::new();
    loop {
        match reader.read_resolved_event() {
            Ok((_, Event::Eof)) => return Ok(mpolys),
            Ok((
                Bound(
                    BUILDING_NS | CITYFURNITURE_NS | TRANSPORTATION_NS | VEGETATION_NS | BRIDGE_NS,
                ),
                Event::Start(e),
            )) => match e.local_name().as_ref() {
                b"Building"
                | b"CityFurniture"
                | b"Road"
                | b"Bridge"
                | b"SolitaryVegetationObject"
                | b"PlantCover" => mpolys.push(parse_cityobj(reader, &mut buf)?),
                _ => (),
            },
            Ok(_) => (),
            Err(e) => return Err(ParseError::XmlError(e)),
        }
    }
}

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    filenames: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut all_mpolys = Vec::new();

    for filename in args.filenames {
        let xml = fs::read_to_string(filename).unwrap();
        let mut reader = NsReader::from_str(&xml);
        reader.trim_text(true);
        match parse_body(&mut reader) {
            Ok(mpolys) => {
                println!(
                    "features={features} polygons={polygons}",
                    features = mpolys.len(),
                    polygons = mpolys.iter().flatten().count()
                );
                all_mpolys.extend(mpolys);
            }
            Err(e) => match e {
                ParseError::XmlError(e) => {
                    println!("Error at position {}: {:?}", reader.buffer_position(), e)
                }
            },
        };
    }

    // NOTE: この時点で MultiPolygon にジオメトリデータが詰め込まれている状態
    //
    // ここから先は geojson 形式での出力を行う。

    let geojson_features = all_mpolys
        .iter()
        .map(|poly| nusamai_to_geojson_geometry(&Geometry::MultiPolygon::<3, f64>(poly.to_owned())))
        .map(geojson_geometry_to_feature);

    let geojson_feature_collection = geojson::FeatureCollection {
        bbox: None,
        features: geojson_features.collect(),
        foreign_members: None,
    };
    let geojson = geojson::GeoJson::from(geojson_feature_collection);

    let mut file = fs::File::create("out.geojson").unwrap();
    file.write_all(geojson.to_string().as_bytes()).unwrap();
}
