use citygml::{CityGMLElement, CityGMLReader, Geometries, ParseError, SubTreeReader};
use clap::Parser;
use nusamai_plateau::models::CityObject;
use std::io::BufRead;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    filename: String,
}

struct TopLevelCityObject<'a> {
    cityobj: citygml::object::FeatureOrData<'a>,
    geometries: Geometries,
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

fn extract_indices(first_obj: &TopLevelCityObject<'_>) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    let mut all_coords: Vec<u32> = Vec::new();
    let mut coords_spans: Vec<u32> = Vec::new();
    let mut all_hole_indices: Vec<u32> = Vec::new();

    for polygon in first_obj.geometries.multipolygon.iter() {
        all_coords.extend(polygon.coords());
        // todo: ロジックがひどいのでそのうち修正する
        // coords_spansに格納されている最後の値に、polygon.coords()の長さを足したものを追加する
        // ただし、最初のpolygonの場合は、coords_spansに何も入っていないので、0を追加する
        coords_spans.push(
            coords_spans
                .last()
                .map(|x| x + polygon.coords().len() as u32)
                .unwrap_or(0),
        );
        // todo: holeを考慮する必要がある
        for hole in polygon.hole_indices() {
            all_hole_indices.push(*hole);
        }
    }
    // coords_spansの先頭の0を削除する
    coords_spans.remove(0);
    (all_coords, coords_spans, all_hole_indices)
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

    // for (i, tlc_obj) in tlc_objs.iter().enumerate() {
    //     let properties = &tlc_obj.cityobj;
    //     let vertices = &tlc_obj.geometries.vertices;
    //     let (indices, coords_spans, all_hole_indices) = extract_indices(tlc_obj);

    //     if !all_hole_indices.is_empty() {
    //         println!("i: {}", i);
    //         println!("properties: {:?}\n", properties);
    //         println!("vertices: {:?}\n", vertices);
    //         println!("indices: {:?}\n", indices);
    //         println!("coords_spans: {:?}\n", coords_spans);
    //         println!("all_hole_indices: {:?}\n", all_hole_indices);
    //     }
    // }

    // 17番目とかがholeを持っていた
    let first_obj = &tlc_objs[17];
    println!("first_obj: {:?}\n", first_obj.geometries);

    let properties = &first_obj.cityobj;
    let vertices = &first_obj.geometries.vertices;
    let (indices, coords_spans, all_hole_indices) = extract_indices(first_obj);

    // 一つのCityObject(TopLevelCityObject・Feature)に必要な情報
    println!("properties: {:?}\n", properties);
    println!("vertices: {:?}\n", vertices);
    println!("indices: {:?}\n", indices);
    println!("coords_spans: {:?}\n", coords_spans);
    println!("all_hole_indices: {:?}\n", all_hole_indices);

    // todo
    // holeも考慮する
    // 地物の中心座標を求める
    // 地物ごとに三角分割
    // 頂点にIDを付与
    // 地物ごとにバイナリバッファを作成
    // 地物ごとにバイナリバッファをファイルに書き出し

    // EXT_structural_metadata
}
