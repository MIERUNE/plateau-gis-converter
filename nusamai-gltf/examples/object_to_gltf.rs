use citygml::{CityGMLElement, CityGMLReader, Geometries, ParseError, SubTreeReader};
use clap::Parser;
use earcut_rs::{utils_3d::project3d_to_2d, Earcut};
use indexmap::IndexSet;
use nusamai_geometry::MultiPolygon3;
use nusamai_plateau::models::CityObject;
use std::{borrow::Cow, collections::HashMap, io::BufRead};

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    filename: String,
}

struct TopLevelCityObject<'a> {
    cityobj: citygml::object::FeatureOrData<'a>,
    geometries: Geometries,
}

// // 暫定で構造体を定義
#[derive(Debug, Clone, Default)]
struct Triangles {
    pub indices: Vec<u32>,
    pub vertices: IndexSet<[u32; 3]>,
    pub face_normals: Option<Vec<f32>>,
    pub vertex_normals: Option<Vec<f32>>,
    pub vertex_colors: Option<Vec<f32>>,
    pub vertex_ids: Option<Vec<u64>>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Triangles {
    pub fn new(indices: Vec<u32>, vertices: IndexSet<[u32; 3]>) -> Self {
        Self {
            indices,
            vertices,
            ..Default::default()
        }
    }
}

fn toplevel_dispatcher<R: BufRead>(
    st: &mut SubTreeReader<R>,
) -> Result<Vec<(CityObject, Geometries)>, ParseError> {
    let mut items: Vec<(CityObject, Geometries)> = vec![];

    match st.parse_children(|st| match st.current_path() {
        b"core:cityObjectMember" => {
            let mut cityobj: CityObject = Default::default();
            cityobj.parse(st)?;
            let geometries = st.collect_geometries();
            items.push((cityobj, geometries));
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
        Ok(_) => Ok(items),
        Err(e) => {
            println!("Err: {:?}", e);
            Err(e)
        }
    }
}

fn extract_indices(cityobj: &TopLevelCityObject<'_>) -> MultiPolygon3<'static> {
    let mut all_coords: Vec<u32> = Vec::new();
    let mut coords_spans: Vec<u32> = Vec::new();
    let mut all_hole_indices: Vec<u32> = Vec::new();
    let mut holes_spans: Vec<u32> = Vec::new();

    // todo: ロジックがひどいのでそのうち修正する
    for polygon in cityobj.geometries.multipolygon.iter() {
        all_coords.extend(polygon.coords());

        coords_spans.push(all_coords.len() as u32);

        for hole in polygon.hole_indices() {
            all_hole_indices.push(*hole);
        }

        holes_spans.push(all_hole_indices.len() as u32);
    }
    // 最後のcoords_spansを削除
    coords_spans.pop();
    // 最後のholes_spansを削除
    holes_spans.pop();

    // all_coordsに記載の数字をflat_verticesから取得し、フラット化する
    let all_coords_f64: Vec<f64> = all_coords
        .iter()
        .map(|&idx| cityobj.geometries.vertices[idx as usize])
        .flat_map(|v| v.to_vec())
        .collect();

    MultiPolygon3::from_raw(
        Cow::Owned(all_coords_f64.clone()),
        Cow::Owned(coords_spans.clone()),
        Cow::Owned(all_hole_indices.clone()),
        Cow::Owned(holes_spans.clone()),
    )
}

fn main() {
    let args = Args::parse();

    let reader = std::io::BufReader::new(std::fs::File::open(args.filename).unwrap());
    let mut xml_reader = quick_xml::NsReader::from_reader(reader);

    let items = match CityGMLReader::new().start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(items) => items,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    let tlc_objs: Vec<_> = items
        .iter()
        .map(|(o, g)| {
            let cityobj = match o.objectify().unwrap() {
                citygml::object::ObjectValue::FeatureOrData(fod) => fod,
                _ => panic!("Not a FeatureOrData"),
            };

            TopLevelCityObject {
                cityobj,
                geometries: g.clone(),
            }
        })
        .collect();

    // 17番目とかがholeを持っていた
    let first_obj = &tlc_objs[17];

    let first_mpoly = extract_indices(first_obj);
    println!("first_mpoly: {:?}\n", first_mpoly);

    // todo
    // 地物の中心座標を求める
    let (mu_lat, mu_lng) = calc_center(&vec![first_mpoly.clone()]);

    // 地物ごとに三角分割
    let triangles = tessellation(&[first_mpoly.clone()], mu_lng, mu_lat).unwrap();
    // 頂点にIDを付与
    // 地物ごとにバイナリバッファを作成
    // 地物ごとにバイナリバッファをファイルに書き出し

    // EXT_structural_metadata
}

fn tessellation(
    mpolys: &[MultiPolygon3],
    mu_lng: f64,
    mu_lat: f64,
) -> Result<Triangles, Box<dyn std::error::Error>> {
    let mut earcutter = Earcut::new();

    let mut buf3d: Vec<f64> = Vec::new();
    let mut buf2d: Vec<f64> = Vec::new();
    let mut triangles_out: Vec<u32> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();
    let mut vertices: IndexSet<[u32; 3]> = IndexSet::new();

    for mpoly in mpolys.iter() {
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

    return Ok(Triangles::new(indices, vertices));
}

fn calc_center(all_mpolys: &Vec<nusamai_geometry::MultiPolygon<'_, 3>>) -> (f64, f64) {
    // 中心の経緯度を求める
    let (mu_lat, mu_lng) = {
        let (mut mu_lat, mut mu_lng) = (0.0, 0.0);
        let mut num_features = 0;
        for mpoly in all_mpolys {
            let (mut feat_mu_lng, mut feat_mu_lat) = (0.0, 0.0);
            let mut num_verts = 0;
            for poly in mpoly {
                for v in poly.coords().chunks_exact(3) {
                    num_verts += 1;
                    feat_mu_lng += v[0];
                    feat_mu_lat += v[1];
                }
            }
            if num_verts > 0 {
                num_features += 1;
                mu_lat += feat_mu_lng / num_verts as f64;
                mu_lng += feat_mu_lat / num_verts as f64;
            }
        }
        (mu_lat / num_features as f64, mu_lng / num_features as f64)
    };
    println!("{} {}", mu_lat, mu_lng);
    (mu_lat, mu_lng)
}
