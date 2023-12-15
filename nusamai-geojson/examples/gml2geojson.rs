use citygml::{CityGMLElement, CityGMLReader, Geometries, ParseError, SubTreeReader};
use clap::Parser;
use nusamai_geojson::{geojson_geometry_to_feature, nusamai_to_geojson_geometry};
use nusamai_geometry::{Geometry, MultiPolygon3};
use nusamai_plateau::models::CityObject;
use std::borrow::Cow;
use std::fs;
use std::io::BufRead;
use std::io::BufWriter;

#[derive(Parser)]
struct Args {
    #[clap(required = true)]
    filename: String,
}

struct TopLevelCityObject<'a> {
    cityobj: citygml::object::FeatureOrData<'a>,
    geometries: Geometries,
}

fn make_mpoly3(cityobj: &TopLevelCityObject<'_>) -> MultiPolygon3<'static> {
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

    // for (i, o) in tlc_objs.iter().enumerate() {
    //     println!();
    //     println!("{}", i);
    //     println!("{:?}", o.cityobj);
    //     println!("{:?}", o.geometries);
    // }

    // e.g.,
    // cargo run --example object_to_gltcf ~/Desktop/plateau/22203_numazu-shi_2021_citygml_4_op/udx/bldg/52385628_*_6697_op.gml
    let mpoly = make_mpoly3(&tlc_objs[17]);

    let geojson_features =
        geojson_geometry_to_feature(nusamai_to_geojson_geometry(&Geometry::MultiPolygon(mpoly)));

    let geojson_feature_collection = geojson::FeatureCollection {
        bbox: None,
        features: vec![geojson_features],
        foreign_members: None,
    };
    let geojson = geojson::GeoJson::from(geojson_feature_collection);

    let mut file = fs::File::create("out.geojson").unwrap();
    let mut writer = BufWriter::new(&mut file);
    serde_json::to_writer(&mut writer, &geojson).unwrap();
}
