//! This example is extract the geometry and attribute for each semantic child element of a CityGML file.

use citygml::FeatureOrData;
use citygml::{CityGMLElement, CityGMLReader, ParseError, SubTreeReader};
use clap::Parser;
use nusamai_geojson::toplevel_cityobj_to_geojson_features;
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

    let context = citygml::ParseContext::default();
    let cityobjs = match CityGMLReader::new(context).start_root(&mut xml_reader) {
        Ok(mut st) => match toplevel_dispatcher(&mut st) {
            Ok(items) => items,
            Err(e) => panic!("Err: {:?}", e),
        },
        Err(e) => panic!("Err: {:?}", e),
    };

    for obj in cityobjs {
        // println!("{:?}", obj.geometries);

        let root = obj.root;
        if let citygml::object::ObjectValue::FeatureOrData(fod) = root {
            // println!("{:?}", fod.geometries);

            // GeometryRefEntryは、Geometries.multipolygonを参照している
            // Geometries.multipolygonをiterして、lenの分だけ取得するような実装が良い？

            if fod.id == Some("bldg_d6ae568b-e0b7-4fa5-abab-4d257ff9ab90".to_string()) {
                println!("{:?}\n", fod.id);
                println!("{:?}\n", obj.geometries);
                println!("{:?}\n", fod.attributes);
            }
        }
    }
}
