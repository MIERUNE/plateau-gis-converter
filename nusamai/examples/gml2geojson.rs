//! This example converts a CityGML file to GeoJSON and outputs it to a file

use clap::Parser;
use nusamai::sink::geojson::toplevel_cityobj_to_geojson_features;
use nusamai_citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use nusamai_plateau::TopLevelCityObject;
use std::fs;
use std::io::BufRead;
use std::io::BufWriter;

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
