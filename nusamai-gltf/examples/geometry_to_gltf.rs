//! CityGMLファイル (.gml) からポリゴンを読み込んで .geojson 形式で出力するデモ
//! nusamai-geometry/examples/citygml_polygons.rs を元にしています。
//!
//! 使用例:
//!
//! ```bash
//! cargo run --example geometry_to_gltf --release -- ~/path/to/PLATEAU/22203_numazu-shi_2021_citygml_4_op/udx/*/52385628_*_6697_op.gml
//! ````
//!
//! このXMLのパース方法は本格的なパーザで使うことを意図していません。

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use clap::Parser;
use indexmap::IndexSet;
use nusamai_geometry::MultiPolygon3;
use nusamai_gltf::*;
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
                    mpoly.add_interior(buf.chunks_exact(3).map(|c| [c[1], c[0], c[2]]));
                // lon, lat, height
                } else {
                    mpoly.add_exterior(buf.chunks_exact(3).map(|c| [c[1], c[0], c[2]]));
                    // lon, lat, height
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
    // ここから先は glTF 形式での出力を行う。

    let mut features: Vec<MultiPolygon3> = Vec::new();

    for feature in all_mpolys.iter() {
        // mpolyが1つのfeature（建物）で、複数のpolygon（面）を持っている
        println!("\npolygon length {:?}", feature.len());

        for polygon in feature.iter() {
            println!("・exterior {:?}", polygon.exterior());
            println!("・interiors");
            for item in polygon.interiors() {
                println!("interior {:?}", item);
            }
        }
    }

    // let mut gltf = GLTF::new();
    // let gltf_string = gltf.to_string().unwrap();
    // println!("{}", gltf_string);
}

fn write_features(mpolys: &[MultiPolygon3], mu_lng: f64, mu_lat: f64) {
    use earcut_rs::{utils_3d::project3d_to_2d, Earcut};
    let mut earcutter = Earcut::new();
    let mut buf3d: Vec<f64> = Vec::new();
    let mut buf2d: Vec<f64> = Vec::new();
    let mut triangles_out: Vec<u32> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();
    let mut vertices: IndexSet<[u32; 3]> = IndexSet::new();

    for mpoly in mpolys {
        for poly in mpoly {
            let num_outer = match poly.hole_indices().first() {
                Some(&v) => v as usize,
                None => poly.coords().len() / 3,
            };

            buf3d.clear();
            buf3d.extend(poly.coords().chunks_exact(3).flat_map(|v| {
                let (lat, lng) = (v[0], v[1]);
                [
                    (lng - mu_lng) * (10000000. * lat.to_radians().cos() / 90.),
                    (lat - mu_lat) * (10000000. / 90.),
                    v[2],
                ]
            }));

            if project3d_to_2d(&buf3d, num_outer, &mut buf2d) {
                // earcut
                earcutter.earcut(&buf2d, poly.hole_indices(), 2, &mut triangles_out);
                // indices and vertices
                indices.extend(triangles_out.iter().map(|idx| {
                    let vbits = [
                        (buf3d[*idx as usize * 3] as f32).to_bits(),
                        (buf3d[*idx as usize * 3 + 1] as f32).to_bits(),
                        (buf3d[*idx as usize * 3 + 2] as f32).to_bits(),
                    ];
                    let (index, _) = vertices.insert_full(vbits);
                    index as u32
                }));
            } else {
                println!("WARN: polygon does not have normal");
            }
        }
    }

    // println!("{:?} {:?}", vertices.len(), indices.len());
    // let file = std::fs::File::create("out.ply").unwrap();
    // let mut writer = std::io::BufWriter::new(file);

    // writer
    //     .write_all(
    //         PLY_HEADER_TEMPLATE
    //             .replace("{n_verts}", &vertices.len().to_string())
    //             .replace("{n_faces}", &(indices.len() / 3).to_string())
    //             .as_ref(),
    //     )
    //     .unwrap();

    // let mut buf = [0; 12];
    // vertices.iter().for_each(|v| {
    //     LittleEndian::write_u32_into(v, &mut buf);
    //     writer.write_all(&buf).unwrap();
    // });
    // indices.chunks_exact(3).for_each(|v| {
    //     writer.write_u8(3).unwrap();
    //     LittleEndian::write_u32_into(v, &mut buf);
    //     writer.write_all(&buf).unwrap();
    // });

    // writer.flush().unwrap();
}
